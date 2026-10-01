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

This is a delegate projection, not complete account coverage. Owner, mint, freeze, close,
Burn and Token-2022 semantics are not projected yet. Spend and Burn share the native
delegated allowance; this adapter emits only Spend and never adds their budgets together.
Remaining allowance does not promise available balance or transaction success. Declared
Diff supports exactly one canonical owner Revoke; other instructions return Unsupported.

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
and remaining work are recorded under docs/research and docs/codex.
