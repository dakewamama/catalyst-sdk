# Subscriptions fixture contract

Baseline and verified semantics: [SUBSCRIPTIONS_SEMANTICS.md](SUBSCRIPTIONS_SEMANTICS.md).
These are fixtures to capture through native execution before adapter claims,
not fabricated golden files or a claim that Catalyst already supports Subscriptions.
Upstream test paths refer to the pinned source in that document.

Every captured case must include program/token ELF hashes, source revisions,
account addresses/owners/raw bytes before and after, clock slot/timestamp, native
instruction bytes/accounts, outcome and expected ARM. Use absolute clocks and
independent slots. Keep successful and rejected transitions; failures must leave
state unchanged. Unsupported versions and absent dependency evidence are separate.

| Fixture | Native operation and assertion | Existing upstream evidence |
| --- | --- | --- |
| Technical authority | Initialize; ATA delegates finite `u64::MAX` to PDA, without granting that amount to a merchant | `test_initialize_subscription_authority.rs`, `test_revoke_subscription_authority.rs` |
| Fixed | Create cap 100; pull 40 to a third party; remaining 60; pull 61 fails; delegator revokes; recompile removes effective grant | `test_create_fixed_delegation.rs`, `test_transfer_fixed_delegation.rs`, `test_revoke_delegation.rs` |
| Recurring | Cap 100, period 30; pull 60; remaining 40; exceed rejected; skip periods and prove only one current cap, not accumulation | `test_transfer_recurring_delegation.rs` |
| Finite boundary | Test expiry minus one, expiry and expiry plus one; exhausted final period never receives another cap | Same file; host helper `catch_up_at_exact_expiry_boundary_succeeds` already executed |
| Subscription | Owner plus allowed puller share one cap; destination wallet whitelist checked; arbitrary third party caller rejected | `test_transfer_subscription.rs` |
| Pending cancellation | Cancel midway; cutoff at period boundary; pull remains possible before cutoff and fails at cutoff | `test_cancel_subscription.rs`, `test_transfer_subscription.rs` |
| Resume | Resume pending cancellation with observed cutoff; consumed amount and period preserved; stale expiry/generation rejected | `test_resume_subscription.rs` |
| Immediate cancellation | Both signatures and observed period start; cutoff now; single subscription stopped while another on same authority survives | `test_cancel_subscription_now.rs` |
| Active but inactive spending | Preserve subscription state while directly revoking token delegate, freezing/emptying source or closing authority; do not report collectible Spend | `test_transfer_recurring_delegation.rs`, README collectability section |
| Generation | Close/recreate in same slot versus later slot; old binding can survive only matching generation; sponsor recovery waits past creation slot | `test_co_init_slot.rs`, `test_revoke_abandoned_delegation.rs`, `test_revoke_abandoned_subscription.rs` |
| Plan lineage and administration | Sunset keeps existing pull rights; removing puller changes principal set; term mismatch/closed plan blocks collection; reject stale plan edit | `test_update_plan.rs`, `test_transfer_subscription.rs`, `test_subscribe.rs` |
| Global revoke | Revoke matching token approval and close authority; recompile all dependent grant states; foreign token delegate remains | `test_revoke_subscription_authority.rs` |

Capture fixed, recurring and subscription inputs from successful native creates,
not handwritten protocol layouts. The upstream harness creates some token fixtures
by installing native account data; record that distinction. Catalyst goldens should
reproduce the relevant state through official instructions when practical.

Additional negative variants: wrong owner/kind/length/version, authority address or
generation mismatch, missing signature, recipient violation, foreign delegate,
sponsor recipient mismatch, mint/token-program mismatch and failed CPI rollback.
Observe a removed account explicitly; absence from an incomplete scan is not revoke.

Token-2022 fee/hook examples are dependency tests, not automatic extension support.
First adapter scope should use classic SPL and an explicitly verified Token-2022
subset. Hooks must either supply complete interpreted dependencies or remain
unsupported for exact effective claims. Never add hook authorization to ARM simply
because a TransferContext exists.

Retained-signature scenarios require an actually held, still-valid transaction or
equivalent native instruction sequence with pinned state. Different-timestamp stale
approvals are already covered upstream; same-second/same-period/ABA and held init
bundles are documented residual behavior. Preserve them as limits until locally
reproduced. No fabricated model gap may be inferred from a test name or audit label.

## Acceptance

Raw native state must decode through the official client, compile to stable ARM,
and retain sufficient evidence for replay. Native revoke/cancel/resume must execute,
re-observe and recompile. A declared Diff must agree with the observed semantic
change, including shared authority effects and preserved unrelated permissions.
All existing SPL goldens must still pass. Unknown deployments remain unsupported.

SBF execution is the next proof step. Previous toolchain download failure is an
environment limitation, not passing round-trip evidence. No Subscriptions SVM
fixture or Catalyst semantic golden has been captured in this study milestone.
