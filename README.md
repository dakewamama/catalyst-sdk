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

Verified reuse boundaries and the outstanding native type-version check are recorded in
[REUSE_BOUNDARIES.md](docs/research/REUSE_BOUNDARIES.md). Suspend and Resume have no native
implementation yet and are not exposed as action kinds.

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
