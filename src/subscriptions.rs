use crate::{
    spl::{check_authority, AuthorityState},
    Action, ActionKind, Adapter, Context, Error, Protocol, SourceRequirements,
};
use ::subscriptions::{
    accounts::{FixedDelegation, RecurringDelegation, SubscriptionAuthority},
    instructions::RevokeDelegation,
    types::Header,
    SUBSCRIPTIONS_ID,
};
use arm::*;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_pack::Pack;
use solana_pubkey::Pubkey;
use spl_token_interface::state::{Account as TokenAccount, Mint};

pub const PROGRAM_VERSION: &str =
    "sha256:a59467f0b2d0a0211b06ddf9a3f3141a90eaea6faeeaa3d19541288eaa082107";
pub const DEPLOYMENT: &str = "fixture:mollusk:subscriptions:0.5.0";

pub const ADAPTER_VERSION: &str = "0.2";

pub struct DelegationAdapter;

pub struct DelegationState {
    pub delegation: (Pubkey, Account),
    pub authority: (Pubkey, Account),
    pub source: AuthorityState,
    pub mint: (Pubkey, Account),
    pub token_program_version: String,
    pub unix_timestamp: Option<i64>,
}

enum Delegation {
    Fixed(FixedDelegation),
    Recurring(RecurringDelegation),
}

impl Delegation {
    fn header(&self) -> &Header {
        match self {
            Self::Fixed(grant) => &grant.header,
            Self::Recurring(grant) => &grant.header,
        }
    }

    fn terms(&self, now: Option<i64>) -> Result<(u64, UsageSemantics, Lifecycle), Error> {
        let (amount, usage, start, expiry) = match self {
            Self::Fixed(grant) => (
                grant.amount,
                UsageSemantics::Cumulative {
                    remaining: Some(grant.amount),
                },
                None,
                grant.expiry_ts,
            ),
            Self::Recurring(grant) => {
                let now = now.ok_or(Error::InsufficientEvidence)?;
                let period = i64::try_from(grant.period_length_s)
                    .ok()
                    .filter(|period| *period > 0)
                    .ok_or_else(|| Error::InvalidState("recurring period".into()))?;
                let anchor = grant.current_period_start_ts;
                let mut start = anchor;
                let mut pulled = grant.amount_pulled_in_period;
                if pulled > grant.amount_per_period {
                    return Err(Error::InvalidState("recurring consumption".into()));
                }
                if now >= anchor && (grant.expiry_ts == 0 || now <= grant.expiry_ts) {
                    // Inclusive expiry retains the last period starting strictly before it.
                    let until = if grant.expiry_ts == 0 {
                        now
                    } else {
                        now.min(grant.expiry_ts.saturating_sub(1))
                    };
                    let elapsed = until
                        .checked_sub(anchor)
                        .ok_or(Error::UnsupportedOperation)?;
                    if elapsed >= period {
                        start = anchor
                            .checked_add(elapsed / period * period)
                            .ok_or(Error::UnsupportedOperation)?;
                        pulled = 0;
                    }
                }
                if start.checked_add(period).is_none() {
                    return Err(Error::UnsupportedOperation);
                }
                (
                    grant.amount_per_period,
                    UsageSemantics::Recurring {
                        period_seconds: grant.period_length_s,
                        anchor_unix_seconds: anchor,
                        observed_period_start: start,
                        remaining: Some(grant.amount_per_period - pulled),
                    },
                    Some(anchor),
                    grant.expiry_ts,
                )
            }
        };
        Ok((
            amount,
            usage,
            Lifecycle::Active {
                valid_from: start,
                // Native expiry is inclusive; i64::MAX has no later representable instant.
                valid_until: if expiry == 0 {
                    None
                } else {
                    expiry.checked_add(1)
                },
            },
        ))
    }
}

fn absent(account: &Account) -> bool {
    account.owner == Pubkey::default()
        && account.lamports == 0
        && account.data.is_empty()
        && !account.executable
}

fn delegation(state: &DelegationState) -> Result<Option<Delegation>, Error> {
    let account = &state.delegation.1;
    if absent(account) {
        return Ok(None);
    }
    if account.owner != SUBSCRIPTIONS_ID || account.executable || account.data.len() < 2 {
        return Err(Error::InvalidState("delegation owner or header".into()));
    }
    if account.data[1] != 1 {
        return Err(Error::UnsupportedVersion);
    }
    match account.data[0] {
        2 if account.data.len() == 187 => FixedDelegation::from_bytes(&account.data)
            .map(Delegation::Fixed)
            .map(Some)
            .map_err(|error| Error::InvalidState(error.to_string())),
        3 if account.data.len() == 211 => RecurringDelegation::from_bytes(&account.data)
            .map(Delegation::Recurring)
            .map(Some)
            .map_err(|error| Error::InvalidState(error.to_string())),
        2 | 3 => Err(Error::InvalidState("delegation length".into())),
        _ => Err(Error::UnsupportedOperation),
    }
}

fn revoke(state: &DelegationState) -> Result<Instruction, Error> {
    let grant = delegation(state)?.ok_or(Error::UnsupportedOperation)?;
    let header = grant.header();
    check_authority(&state.source, header.delegator)?;
    let recipients = if header.payer == header.delegator {
        vec![]
    } else {
        vec![AccountMeta::new(header.payer, false)]
    };
    Ok(RevokeDelegation {
        authority: header.delegator,
        delegation_account: state.delegation.0,
    }
    .instruction_with_remaining_accounts(&recipients))
}

impl Adapter for DelegationAdapter {
    type State = DelegationState;

    fn protocol(&self) -> Protocol {
        Protocol {
            name: "subscriptions",
            program_id: SUBSCRIPTIONS_ID,
        }
    }

    fn supports(&self, native: &NativeContext) -> bool {
        native.protocol == "subscriptions"
            && native.deployment == DEPLOYMENT
            && native.program_version == PROGRAM_VERSION
            && native.adapter_version == ADAPTER_VERSION
    }

    fn source_requirements(&self, state: &DelegationState) -> SourceRequirements {
        let mut accounts = vec![
            state.delegation.0,
            state.authority.0,
            state.source.address,
            state.mint.0,
        ];
        if let Ok(account) = TokenAccount::unpack(&state.source.account.data) {
            if !accounts.contains(&account.owner) {
                accounts.push(account.owner);
            }
        }
        if let Ok(Some(grant)) = delegation(state) {
            if !accounts.contains(&grant.header().delegatee) {
                accounts.push(grant.header().delegatee);
            }
        }
        SourceRequirements {
            accounts,
            clock: state.delegation.1.data.first() == Some(&3),
        }
    }

    fn compile_state(
        &self,
        state: &DelegationState,
        context: &Context,
    ) -> Result<Vec<Authorization>, Error> {
        if state.token_program_version != crate::spl::PROGRAM_VERSION {
            return Err(Error::UnsupportedVersion);
        }
        let token_id = spl_token_interface::id();
        if state.source.account.owner != token_id
            || state.source.account.executable
            || state.mint.1.owner != token_id
            || state.mint.1.executable
        {
            return Err(Error::InvalidState("token account or mint owner".into()));
        }
        let source = TokenAccount::unpack(&state.source.account.data)
            .map_err(|error| Error::InvalidState(error.to_string()))?;
        Mint::unpack(&state.mint.1.data).map_err(|error| Error::InvalidState(error.to_string()))?;
        let (authority_address, bump) =
            SubscriptionAuthority::find_pda(&source.owner, &source.mint);
        let source_address = Pubkey::find_program_address(
            &[
                source.owner.as_ref(),
                token_id.as_ref(),
                source.mint.as_ref(),
            ],
            &solana_pubkey::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        )
        .0;
        if state.mint.0 != source.mint
            || state.authority.0 != authority_address
            || state.source.address != source_address
        {
            return Err(Error::InvalidState(
                "source, mint or authority binding".into(),
            ));
        }
        let authority = if absent(&state.authority.1) {
            None
        } else {
            let account = &state.authority.1;
            if account.owner != SUBSCRIPTIONS_ID
                || account.executable
                || account.data.len() != 106
                || account.data.first() != Some(&0)
            {
                return Err(Error::InvalidState(
                    "authority kind, owner or length".into(),
                ));
            }
            let authority = SubscriptionAuthority::from_bytes(&account.data)
                .map_err(|error| Error::InvalidState(error.to_string()))?;
            if authority.user != source.owner
                || authority.token_mint != source.mint
                || authority.bump != bump
            {
                return Err(Error::InvalidState("authority identity".into()));
            }
            Some(authority)
        };
        let grant = delegation(state)?;
        if let Some(grant) = &grant {
            let header = grant.header();
            let (authority, mint) = match grant {
                Delegation::Fixed(grant) => (grant.subscription_authority, grant.mint),
                Delegation::Recurring(grant) => (grant.subscription_authority, grant.mint),
            };
            if header.delegator != source.owner
                || mint != source.mint
                || authority != authority_address
            {
                return Err(Error::InvalidState("delegation binding".into()));
            }
            check_authority(&state.source, grant.header().delegatee)?;
        }
        let approved = Option::<Pubkey>::from(source.delegate) == Some(authority_address);
        let asset = Resource {
            namespace: "solana:mint".into(),
            id: source.mint.to_string(),
        };
        let technical = Authorization {
            schema_version: SCHEMA_VERSION.into(),
            id: format!(
                "{}:{}:technical:spend",
                context.program_id, state.source.address
            ),
            subject: Subject::Identity(source.owner.to_string()),
            principal: Principal::Identity(authority_address.to_string()),
            resource: Resource {
                namespace: "solana:token-account".into(),
                id: state.source.address.to_string(),
            },
            capability: Capability::Spend,
            constraints: ConstraintExpr::Constraint(Constraint::AmountAtMost {
                asset: asset.clone(),
                amount: source.delegated_amount,
            }),
            usage: UsageSemantics::Cumulative {
                remaining: Some(source.delegated_amount),
            },
            lifecycle: if source.is_frozen() {
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
        };
        let mut authorizations = if approved {
            vec![technical.clone()]
        } else {
            vec![]
        };
        if let Some(grant) = grant {
            let (amount, usage, lifecycle) = grant.terms(state.unix_timestamp)?;
            let active = approved
                && !source.is_frozen()
                && source.amount != 0
                && source.delegated_amount != 0
                && authority.is_some_and(|authority| authority.init_id == grant.header().init_id);
            let parent = technical.id.clone();
            let mut authorization = technical;
            authorization.id = format!(
                "{}:{}:{}:spend",
                context.program_id,
                state.delegation.0,
                grant.header().init_id
            );
            authorization.principal = Principal::Identity(grant.header().delegatee.to_string());
            authorization.constraints =
                ConstraintExpr::Constraint(Constraint::AmountAtMost { asset, amount });
            authorization.usage = usage;
            authorization.authority_kind = AuthorityKind::Derived {
                parents: vec![parent],
            };
            authorization.lifecycle = if active {
                lifecycle
            } else {
                Lifecycle::Suspended
            };
            authorizations.push(authorization);
        }
        Ok(authorizations)
    }

    fn diff_transaction(
        &self,
        instructions: &[Instruction],
        state: &DelegationState,
        context: &Context,
    ) -> Result<Vec<AuthorizationChange>, Error> {
        if instructions != [revoke(state)?] {
            return Err(Error::UnsupportedOperation);
        }
        Ok(self
            .compile_state(state, context)?
            .into_iter()
            .filter(|authorization| {
                matches!(authorization.authority_kind, AuthorityKind::Derived { .. })
            })
            .map(|authorization| AuthorizationChange::Removed {
                authorization: Box::new(authorization),
            })
            .collect())
    }

    fn actions(
        &self,
        authorization: &Authorization,
        state: &DelegationState,
        context: &Context,
    ) -> Result<Vec<Action>, Error> {
        if !self.compile_state(state, context)?.contains(authorization) {
            return Err(Error::InvalidProjection);
        }
        if !matches!(authorization.authority_kind, AuthorityKind::Derived { .. }) {
            return Ok(vec![]);
        }
        Ok(vec![Action {
            kind: ActionKind::Revoke,
            authorization_id: authorization.id.clone(),
            instructions: vec![revoke(state)?],
        }])
    }
}
