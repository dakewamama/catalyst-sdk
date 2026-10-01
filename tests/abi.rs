use arm::*;
use catalyst_sdk::{
    self as sdk, Action, ActionKind, Adapter, Context, Error, Protocol, SourceRequirements,
};
use solana_instruction::Instruction;
use std::cell::Cell;

struct FixtureAdapter {
    calls: Cell<u32>,
    corrupt: bool,
    duplicate: bool,
}
impl FixtureAdapter {
    fn new() -> Self {
        Self {
            calls: Cell::new(0),
            corrupt: false,
            duplicate: false,
        }
    }
}
struct State {
    remaining: Option<u64>,
}

fn context() -> Context {
    Context {
        program_id: Default::default(),
        native: NativeContext {
            protocol: "fixture".into(),
            deployment: "fixture:local:deployment-1".into(),
            program_version: "1".into(),
            adapter_version: "0.1".into(),
        },
        evidence: EvidenceBundle {
            references: vec!["fixture:account:source".into()],
            observed_at: "fixture:slot:1".into(),
        },
    }
}

impl Adapter for FixtureAdapter {
    type State = State;
    fn protocol(&self) -> Protocol {
        Protocol {
            name: "fixture",
            program_id: Default::default(),
        }
    }
    fn supports(&self, native: &NativeContext) -> bool {
        native == &context().native
    }
    fn source_requirements(&self, _: &State) -> SourceRequirements {
        SourceRequirements {
            accounts: vec![Default::default()],
            clock: false,
        }
    }
    fn compile_state(&self, state: &State, context: &Context) -> Result<Vec<Authorization>, Error> {
        self.calls.set(self.calls.get() + 1);
        let Some(remaining) = state.remaining else {
            return Ok(vec![]);
        };
        let resource = Resource {
            namespace: "fixture:account".into(),
            id: "source".into(),
        };
        let mut authorization = Authorization {
            schema_version: SCHEMA_VERSION.into(),
            id: "fixture:source:delegate:spend".into(),
            subject: Subject::Identity("owner".into()),
            principal: Principal::Identity("delegate".into()),
            resource: resource.clone(),
            capability: Capability::Spend,
            constraints: ConstraintExpr::Constraint(Constraint::AmountAtMost {
                asset: resource,
                amount: remaining,
            }),
            usage: UsageSemantics::Cumulative {
                remaining: Some(remaining),
            },
            lifecycle: Lifecycle::Active {
                valid_from: None,
                valid_until: None,
            },
            delegability: Delegability::None,
            authority_kind: AuthorityKind::Direct,
            enforcement: Enforcement::Native,
            observability: Observability::Exact,
            evidence: context.evidence.clone(),
            native_context: context.native.clone(),
        };
        if self.corrupt {
            authorization.evidence.observed_at = "fixture:wrong-observation".into();
        }
        if self.duplicate {
            return Ok(vec![authorization.clone(), authorization]);
        }
        Ok(vec![authorization])
    }
    fn diff_transaction(
        &self,
        instructions: &[Instruction],
        state: &State,
        context: &Context,
    ) -> Result<Vec<AuthorizationChange>, Error> {
        self.calls.set(self.calls.get() + 1);
        if instructions.len() != 1 || instructions[0].data != [0] {
            return Err(Error::UnsupportedOperation);
        }
        Ok(self
            .compile_state(state, context)?
            .into_iter()
            .map(|authorization| AuthorizationChange::Removed {
                authorization: Box::new(authorization),
            })
            .collect())
    }
    fn actions(
        &self,
        authorization: &Authorization,
        _: &State,
        _: &Context,
    ) -> Result<Vec<Action>, Error> {
        self.calls.set(self.calls.get() + 1);
        Ok(vec![Action {
            kind: ActionKind::Revoke,
            authorization_id: authorization.id.clone(),
            instructions: vec![Instruction::new_with_bytes(
                Default::default(),
                &[0],
                vec![],
            )],
        }])
    }
}

#[test]
fn typed_state_compiles_deterministically_with_source_evidence() {
    let adapter = FixtureAdapter::new();
    let context = context();
    let state = State {
        remaining: Some(u64::MAX),
    };
    let first = sdk::compile_state(&adapter, &state, &context).unwrap();
    let second = sdk::compile_state(&adapter, &state, &context).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    assert_eq!(first[0].evidence, context.evidence);
    assert_eq!(first[0].native_context, context.native);
    assert_eq!(
        adapter.source_requirements(&state),
        SourceRequirements {
            accounts: vec![Default::default()],
            clock: false
        }
    );
    assert_eq!(adapter.protocol().program_id, context.program_id);
}

#[test]
fn unknown_versions_reject_before_any_adapter_operation() {
    let adapter = FixtureAdapter::new();
    let state = State { remaining: Some(5) };
    let supported = context();
    let a = sdk::compile_state(&adapter, &state, &supported)
        .unwrap()
        .remove(0);
    let calls = adapter.calls.get();
    for field in 0..5 {
        let mut unknown = context();
        match field {
            0 => unknown.native.program_version = "future".into(),
            1 => unknown.native.deployment = "other".into(),
            2 => unknown.native.adapter_version = "future".into(),
            3 => unknown.native.protocol = "other".into(),
            _ => unknown.program_id = solana_pubkey::Pubkey::new_from_array([1; 32]),
        }
        assert_eq!(
            sdk::compile_state(&adapter, &state, &unknown),
            Err(Error::UnsupportedVersion)
        );
        assert_eq!(
            sdk::diff_transaction(&adapter, &[], &state, &unknown),
            Err(Error::UnsupportedVersion)
        );
        assert_eq!(
            sdk::actions(&adapter, &a, &state, &unknown),
            Err(Error::UnsupportedVersion)
        );
    }
    assert_eq!(adapter.calls.get(), calls);
}

#[test]
fn missing_evidence_and_corrupt_projection_reject() {
    let adapter = FixtureAdapter::new();
    let state = State { remaining: Some(5) };
    let mut missing = context();
    missing.evidence.references.clear();
    assert_eq!(
        sdk::compile_state(&adapter, &state, &missing),
        Err(Error::InsufficientEvidence)
    );
    assert_eq!(adapter.calls.get(), 0);
    let corrupt = FixtureAdapter {
        calls: Cell::new(0),
        corrupt: true,
        duplicate: false,
    };
    assert_eq!(
        sdk::compile_state(&corrupt, &state, &context()),
        Err(Error::InvalidProjection)
    );
}

#[test]
fn duplicate_authorization_identity_rejects_the_whole_projection() {
    let adapter = FixtureAdapter {
        calls: Cell::new(0),
        corrupt: false,
        duplicate: true,
    };
    assert_eq!(
        sdk::compile_state(&adapter, &State { remaining: Some(5) }, &context()),
        Err(Error::InvalidProjection)
    );
}

#[test]
fn declared_revoke_uses_standard_instructions_and_retains_removed_evidence() {
    let adapter = FixtureAdapter::new();
    let state = State { remaining: Some(5) };
    let context = context();
    let a = sdk::compile_state(&adapter, &state, &context)
        .unwrap()
        .remove(0);
    let actions = sdk::actions(&adapter, &a, &state, &context).unwrap();
    let diff = sdk::diff_transaction(&adapter, &actions[0].instructions, &state, &context).unwrap();
    assert_eq!(
        diff,
        vec![AuthorizationChange::Removed {
            authorization: Box::new(a)
        }]
    );
    assert_eq!(
        sdk::diff_transaction(&adapter, &[], &state, &context),
        Err(Error::UnsupportedOperation)
    );
    assert!(
        sdk::compile_state(&adapter, &State { remaining: None }, &context)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn action_requires_the_current_observation_not_stale_evidence() {
    let adapter = FixtureAdapter::new();
    let state = State { remaining: Some(5) };
    let mut context = context();
    let a = sdk::compile_state(&adapter, &state, &context)
        .unwrap()
        .remove(0);
    context.evidence.observed_at = "fixture:slot:2".into();
    assert_eq!(
        sdk::actions(&adapter, &a, &state, &context),
        Err(Error::InvalidProjection)
    );
}
