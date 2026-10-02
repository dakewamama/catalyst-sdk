# catalyst-sdk

Catalyst translates typed native authority into ARM meaning. The Rust library is the
semantic adapter boundary; the existing Bun/TypeScript package decodes lifecycle event
bytes and retains its original API. Event decoding alone does not establish authority.

## Semantic adapter ABI

Implement Adapter with a native State type and six methods: protocol, supports,
source_requirements, compile_state, diff_transaction and actions. No intent compiler
exists yet. Native decoding belongs to official/generated clients, not this trait.

Runtime callers use the crate-level compile_state, diff_transaction and actions functions.
The trait methods are interpretation hooks; directly calling them bypasses the checked
dispatch boundary. Checked dispatch rejects a protocol/program mismatch or unsupported
native deployment/version before invoking a hook. Evidence must include references and
an observation position. Outputs must validate against ARM and retain the exact supplied
provenance/evidence bundle. Compile output cannot contain duplicate authorization IDs;
Changed output must preserve identity; management actions must target that authorization
and contain instructions. Empty output is valid when there is no authority/action.

Context binds a native program key to Cataloger-resolved NativeContext and source evidence.
Adapters must check known deployments, program versions and their own adapter version.
The runtime is responsible for ensuring Context describes the supplied native state;
the ABI does not authenticate RPC responses or establish freshness/finality.

SourceRequirements lists native account keys and whether clock evidence is needed.
Actions contain standard solana-instruction instructions, including native signer and
writable account metadata. The SDK neither signs nor submits transactions. Declared Diff
is an adapter statement; simulation agreement and execution are later milestones.

The synthetic test adapter proves determinism, explicit requirements, failure before
interpretation, evidence propagation, standard instructions and removal representation.
It is not a supported native protocol or a native revoke round trip.

ARM is pinned to its committed Git revision. Its domain model is reused directly; no
parallel semantic model is maintained here. Solana instruction/public-key dependencies
are pinned; adapter native clients must use compatible versions.

Verified reuse boundaries and the native compatibility proof are recorded in
[REUSE_BOUNDARIES.md](docs/research/REUSE_BOUNDARIES.md). Suspend and Resume have no native
implementation yet and are not exposed as action kinds.

## SPL delegate slice

spl::DelegateAdapter interprets classic SPL token account delegate Spend authority.
State retains native account observations; the runtime binds owner/delegate observations
to the keys returned by source_requirements and retains their evidence. Official Pack
decodes state and the official revoke builder constructs instructions. The owner revoke
action requires an observed ordinary system account; multisig/program-controlled owners
are unsupported for actions. Multisig/program-controlled delegates are unsupported for
interpretation until their principal semantics are implemented.

Support is limited to the exact ELF digest and local deployment named by spl constants.
The ELF comes from Mollusk's pinned token fixture crate, not a live deployment lookup.
Cataloger must establish live deployment coverage before this adapter can advertise it.
Unknown versions fail before decoding. Do not relabel a live observation as this fixture.

The checked-in raw state is reproduced by native Approve execution and has a complete
ARM golden. Mollusk executes Revoke; recompilation removes the authorization and agrees
with its declared Diff. Native Transfer rejects an amount above the delegated allowance
and consumes that allowance. Frozen state is suspended, zero allowance is inactive, and
native unsigned/wrong-owner/frozen revoke failures leave state unchanged.

This is a delegate projection, not complete account coverage. Owner,
Burn semantics are not projected by this classic adapter. Spend and Burn share the native
delegated allowance; this adapter emits only Spend and never adds their budgets together.
Remaining allowance does not promise available balance or transaction success. Declared
Diff supports exactly one canonical owner Revoke; other instructions return Unsupported.

MintAdapter consumes AuthorityState observations for the same verified classic program.
It projects Mint on the mint resource and administrative Freeze/Thaw on that mint's
token accounts. It does not infer a human owner from mint state. Native authority
accounts must be supplied by key; missing, duplicate, multisig or program-owned
observations fail closed. These are separate native powers, not Spend permissions.

Mint actions use official SetAuthority instructions to clear the selected authority.
Clearing mint authority permanently fixes supply against further minting; clearing
freeze authority permanently removes both Freeze and Thaw, including the ability to
thaw existing frozen accounts. Native execution proves neither authority can be restored
after clearing. The SDK builds these actions; it does not submit them. Each declared
Diff must match one canonical removal instruction and retains the removed evidence.

The raw mint golden is reproduced by native InitializeMint2 and MintTo. Native tests
verify supply/balance changes, freeze/thaw effects on the existing delegate projection,
signature/controller failures, removal/recompilation agreement and irreversible removal.

CloseAdapter shares AuthorityState's keyed native observations with MintAdapter.
It projects explicit close authority, or the token account owner when the native close
authority is absent. Clearing an explicit close authority restores the owner; it does
not remove Close permission. That action is ResetAuthority, and its declared Diff is
Changed, or empty when the effective principal already equals the owner. Missing
fallback-owner evidence prevents proposing or declaring the reset.

Close is technical authority, not a guarantee of transaction success: ordinary token
accounts must have zero token balance; wrapped SOL can close with a balance. Frozen
empty accounts can close, but their close authority cannot be reset until thawed.
These distinctions are verified by native execution. System/incinerator token ownership
and multisig/program-owned controllers remain unsupported. Native CloseAccount simulation
returns lamports to an explicitly selected beneficiary and re-observation of the deleted
account yields no Close authorization. The SDK does not propose a beneficiary or submit
a Close transaction from observed state; human destination intent is not inferred.

## Token-2022 delegate slice

Token2022Adapter uses official extension decoding and native instruction builders.
State requires the token account, its matching mint and keyed controller observations
from the same observed bank. It projects ordinary cumulative Spend and mint-wide
Permanent Delegate Spend. Only the exact local ELF digest named by token2022 constants
is supported; this is not live deployment coverage or complete protocol coverage.

Account extensions and mint extensions other than PermanentDelegate are unsupported.
Malformed extension data is an error, never evidence that authority is absent. Mint,
Freeze, Close and Burn powers are not projected by this adapter. Multisig and program
controllers remain unsupported. ARM is unchanged.

Native tests prove that Permanent Delegate bypasses the ordinary delegated allowance.
When both delegate roles name the same principal, compilation returns Unsupported
rather than claiming the ordinary allowance is independently consumed. Freezing one
account suspends its ordinary delegation; it does not revoke mint-wide authority.
Native balance and frozen-account checks still apply to permanent transfers.

Both the owner and ordinary delegate can revoke ordinary delegation. Canonical declared
Diff agrees with native execution and recompilation; Permanent Delegate survives.
Permanent Delegate management actions are not exposed yet. Golden account and mint
bytes are reproduced by native initialization, minting and approval.

## Checks

```sh
cargo fmt --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
bun install --frozen-lockfile
bun test
bun run tsc --noEmit
git diff --check
```

The TypeScript public entry point remains index.ts. It exports decodeEvent, the event
types and discriminators. The Rust public entry point is src/lib.rs. Source decisions
and remaining work are recorded under docs/research.
