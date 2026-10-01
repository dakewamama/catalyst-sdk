use crate::{Action, ActionKind, Adapter, Context, Error, Protocol, SourceRequirements};
use arm::*;
use solana_account::Account;
use solana_instruction::Instruction;
use solana_program_pack::Pack;
use solana_pubkey::Pubkey;
use spl_token_interface::{instruction, state::Account as TokenAccount};

pub const PROGRAM_VERSION: &str =
    "sha256:8190d3f7ceb6cb7a7a8d8924bff89f9f611e15ce1f806f2b6237f3311a98f697";
pub const DEPLOYMENT: &str = "fixture:mollusk:token:0.15.1";

pub struct DelegateAdapter;

pub struct State {
    pub address: Pubkey,
    pub account: Account,
    /// Observation of the token account owner's account, not the token delegate.
    pub owner: Account,
    pub delegate: Account,
}

fn decode(state: &State) -> Result<TokenAccount, Error> {
    if state.account.owner != spl_token_interface::id() || state.account.executable {
        return Err(Error::InvalidState("token account program owner".into()));
    }
    TokenAccount::unpack(&state.account.data)
        .map_err(|error| Error::InvalidState(error.to_string()))
}

fn revoke(state: &State) -> Result<Instruction, Error> {
    let account = decode(state)?;
    if account.is_frozen() {
        return Err(Error::UnsupportedOperation);
    }
    // Multisig and program-controlled owners require their own native signer evidence.
    if state.owner.owner != Pubkey::default()
        || !state.owner.data.is_empty()
        || state.owner.executable
    {
        return Err(Error::UnsupportedOperation);
    }
    instruction::revoke(
        &spl_token_interface::id(),
        &state.address,
        &account.owner,
        &[],
    )
    .map_err(|error| Error::InvalidState(error.to_string()))
}

impl Adapter for DelegateAdapter {
    type State = State;

    fn protocol(&self) -> Protocol {
        Protocol {
            name: "spl-token",
            program_id: spl_token_interface::id(),
        }
    }

    fn supports(&self, native: &NativeContext) -> bool {
        native.protocol == "spl-token"
            && native.deployment == DEPLOYMENT
            && native.program_version == PROGRAM_VERSION
            && native.adapter_version == "0.1"
    }

    fn source_requirements(&self, state: &State) -> SourceRequirements {
        let mut accounts = vec![state.address];
        if let Ok(account) = decode(state) {
            accounts.push(account.owner);
            if let Some(delegate) = Option::<Pubkey>::from(account.delegate) {
                accounts.push(delegate);
            }
        }
        SourceRequirements {
            accounts,
            clock: false,
        }
    }

    fn compile_state(&self, state: &State, context: &Context) -> Result<Vec<Authorization>, Error> {
        let account = decode(state)?;
        let Some(delegate) = Option::<Pubkey>::from(account.delegate) else {
            return Ok(vec![]);
        };
        // A multisig principal needs threshold interpretation, not an ordinary signer identity.
        if state.delegate.owner != Pubkey::default()
            || !state.delegate.data.is_empty()
            || state.delegate.executable
        {
            return Err(Error::UnsupportedOperation);
        }
        Ok(vec![Authorization {
            schema_version: SCHEMA_VERSION.into(),
            id: format!("{}:{}:delegate:spend", context.program_id, state.address),
            subject: Subject::Identity(account.owner.to_string()),
            principal: Principal::Identity(delegate.to_string()),
            resource: Resource {
                namespace: "solana:token-account".into(),
                id: state.address.to_string(),
            },
            capability: Capability::Spend,
            constraints: ConstraintExpr::Constraint(Constraint::AmountAtMost {
                asset: Resource {
                    namespace: "solana:mint".into(),
                    id: account.mint.to_string(),
                },
                amount: account.delegated_amount,
            }),
            usage: UsageSemantics::Cumulative {
                remaining: Some(account.delegated_amount),
            },
            lifecycle: if account.is_frozen() {
                Lifecycle::Suspended
            } else {
                Lifecycle::Active {
                    valid_from: None,
                    valid_until: None,
                }
            },
            delegability: Delegability::Unknown,
            authority_kind: AuthorityKind::Direct,
            enforcement: Enforcement::Native,
            observability: Observability::Exact,
            evidence: context.evidence.clone(),
            native_context: context.native.clone(),
        }])
    }

    fn diff_transaction(
        &self,
        instructions: &[Instruction],
        state: &State,
        context: &Context,
    ) -> Result<Vec<AuthorizationChange>, Error> {
        if instructions != [revoke(state)?] {
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
        state: &State,
        context: &Context,
    ) -> Result<Vec<Action>, Error> {
        if !self.compile_state(state, context)?.contains(authorization) {
            return Err(Error::InvalidProjection);
        }
        Ok(vec![Action {
            kind: ActionKind::Revoke,
            authorization_id: authorization.id.clone(),
            instructions: vec![revoke(state)?],
        }])
    }
}
