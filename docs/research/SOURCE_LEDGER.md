# Source ledger

## Native instruction types

- Source: solana-sdk crates, https://github.com/anza-xyz/solana-sdk.
- Versions: solana-instruction 2.3.3 and solana-pubkey 2.4.0, exact Cargo pins/lockfile.
- Published source commits: instruction `f6cd41ac1e4a45d4a94f62d6abcaecb202c51cfa`,
  pubkey `483720e23010213a2d611ea1295bd72b3fd98979` (crate .cargo_vcs_info.json).
- License: Apache-2.0, verified from published crate manifests.
- Inspection: TARGETED SOURCE, published instruction fields, AccountMeta signer/writable
  requirements and new_with_bytes constructor; dependencies executed through ABI tests.
- Reuse: DEPEND. No source copied. Native account metadata carries signing requirements;
  Catalyst does not introduce its own instruction or public-key representation.
- Upgrade review: native adapter client dependencies must agree with these type versions.

## ARM

- Source: https://github.com/dakewamama/arm.
- Revision: 5b05e06892dcc6d20d1db7f0324030916b152d6a.
- Inspection: EXECUTED LOCALLY (model and all fixtures/tests); project-owned dependency.
- Reuse: DEPEND. No separate Catalyst ARM domain model or handwritten TS mirror.
