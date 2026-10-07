# Subscriptions fit against ARM 0.1

ARM reference: `3b91100314473b616b5fd6f77214c6b6282a1dfb`.
Native baseline: [SUBSCRIPTIONS_SEMANTICS.md](SUBSCRIPTIONS_SEMANTICS.md).
This is a representation review, not implemented adapter conformance.

REPRESENTABLE means existing fields describe the fact. PARTIAL means description
is possible but current evaluation or evidence does not prove the full effective
claim. NOT REPRESENTABLE requires native fixture evidence and an exact output
that would otherwise lie. No ARM change is proposed by this study.

| Verified native fact | Fit | Existing representation and limit |
| --- | --- | --- |
| Native token approval to authority PDA | REPRESENTABLE | Identity principal, source token-account resource, Spend and finite Cumulative budget; Direct technical authority, distinct from a person |
| Fixed delegatee spending | REPRESENTABLE | Identity principal, Spend, Cumulative remaining, native enforcement; Derived lineage to technical authority |
| Fixed allowance permits partial pulls | REPRESENTABLE | Cumulative, not OneShot |
| Recurring cap and phase | REPRESENTABLE | AmountAtMost plus Recurring period, phase anchor and observed period remaining; current start supplies phase, not invented historical creation time |
| Owner and allowed pullers share subscription cap | REPRESENTABLE | One AnyOf principal and one Recurring budget; separate full-cap grants would lie |
| Destination wallets differ from callers | REPRESENTABLE | Recipient constraint using allowed wallet identities, independently of acting principal; all-zero native list means no recipient restriction |
| Subject, source and asset | REPRESENTABLE | Subscriber/delegator subject, canonical source-account resource, mint asset; native addresses remain adapter evidence |
| Future start | REPRESENTABLE | NotBefore and/or Active valid_from when native evidence supplies it |
| Inclusive finite expiry | REPRESENTABLE | Half-open ARM boundary at end plus one, with checked arithmetic; maximum i64 means no representable later timestamp, not wrapping arithmetic |
| Pending cancellation | REPRESENTABLE | Active until exclusive cutoff; native cancellation label stays in evidence, not Suspended while pulls remain valid |
| Cancel-now joint approval | REPRESENTABLE | AllOf subscriber/owner principal for authority control, separate from their spending records |
| Resume preserves consumption | REPRESENTABLE | Changed lifecycle with identical observed period usage; no invented reset |
| Plan owner management | REPRESENTABLE | Separate Administrative ModifyAuthority/RevokeAuthority on scoped policy resources; never pretend every field or immutable term is editable |
| Sponsor funds rent without control | REPRESENTABLE | Evidence and separate recovery permission if emitted; payer identity must not imply Spend or authority management |
| Sunset preserves existing pulls | REPRESENTABLE | Current Spend remains active subject to finite end; new-grant management is a separate native fact |
| Authority/delegation generation bindings | REPRESENTABLE | Raw evidence retains generation; NativeContext retains interpretation versions. Derived parent IDs must be scoped to the observed generation |
| Unknown native version | REPRESENTABLE | Unsupported before projection; recovery compatibility does not authorize future semantic interpretation |
| Evidence and provenance | REPRESENTABLE | EvidenceBundle plus NativeContext links to immutable raw observations and versioned interpretation |
| Complete effective spending across dependencies | PARTIAL | Derived parents describe lineage; ARM availability evaluator returns Unknown for derived records, rather than resolving native dependencies |
| Shared approval consumed across different delegations | PARTIAL | Keep one technical budget and dependent grants; ARM does not allocate independent simultaneous budgets or evaluate their intersection |
| Recurrence after last observed period | PARTIAL | Current evaluator returns Unknown outside the observed window; adapter must derive refreshed native period state before reporting known remaining allowance |
| Finite final-period boundary at exact expiry | PARTIAL | Native helper retains the prior period through inclusive expiry; ordinary ARM observed-window evaluation becomes Unknown at its next boundary. Preserve native limit and evidence, do not manufacture a fresh cap |
| Frozen/empty source, revoked approval, hook/extension rules | PARTIAL | Native dependency evaluation belongs to adapter; incomplete interpretation cannot produce Exact effective authority |
| Revocation versus retained signed actions | PARTIAL | Current authority can be removed/changed; ARM current-state records do not prove all unobserved future signed actions permanently invalid |
| Historical original creation/intent | PARTIAL | Account snapshot can lose original period anchor or signing intent. Evidence absence must remain unknown; no inferred grant intent |

## Concrete conservative boundary

Executed upstream helper fixture: start 0, consumption 100, cap 100, period 30,
finite expiry 90, evaluation time 90. Native helper advances to period start 60
and allows its final-period cap. With that native phase and inclusive expiry
translated to valid_until 91, ARM's generic recurring window still ends at 90,
so its evaluator cannot confirm availability there. The safe result is Unknown,
not a false denial presented as exact native behavior and not a new period at 90.

This demonstrates an evaluation limit, not a proven schema impossibility.
An SVM-captured state plus exact expected ARM output is still required before
proposing a general model refinement. It does not justify editing ARM now.

## Failures requiring schema change

No NOT REPRESENTABLE fact has been established with the required native fixture,
desired semantic output and proof that the current model must lie. Declarative
fit does not settle exact adapter output; the fixture contract is the next gate.
Do not introduce Subscriptions nouns, generation counters or generic policy types
into ARM to anticipate that work.

## Smallest adapter path

Use official generated account types and native instruction builders. Validate
deployment, owner, kind, account version and dependency evidence around decoding;
generated typed fields alone do not prove these. Keep protocol calculations in
the adapter. Begin with fixed state and native revoke, then recurring boundaries,
then subscriptions sharing technical authority. Compile technical/effective/control
records separately. Expose only management actions proven by native round trips.

Reuse the existing Catalyst ABI and evidence types. No new decoder framework,
RPC provider, intent compiler or ARM abstraction is justified by this study.
Adapter implementation follows study review; no adapter code is part of this commit.
