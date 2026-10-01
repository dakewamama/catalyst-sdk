# Source ledger

## Native instruction types

- Source: solana-sdk crates, https://github.com/anza-xyz/solana-sdk.
- Versions: solana-instruction 3.4.0 and solana-pubkey 4.2.0, exact Cargo pins/lockfile.
- Published source commits: instruction `1d4d322b9eac89e6c8069464e0c184d72584106c`,
  pubkey `5b985fd7b60de1c845c25bb2d4fc16e19c9ee6ab` (crate .cargo_vcs_info.json).
- License: Apache-2.0, verified from published crate manifests.
- Inspection: TARGETED SOURCE, published instruction fields, AccountMeta signer/writable
  requirements and new_with_bytes constructor; dependencies executed through ABI tests.
- Reuse: DEPEND. No source copied. Native account metadata carries signing requirements;
  Catalyst does not introduce its own instruction or public-key representation.
- Upgrade review: native adapter client dependencies must agree with these type versions.

## ARM

- Source: https://github.com/dakewamama/arm.
- Revision: 3b91100314473b616b5fd6f77214c6b6282a1dfb.
- Inspection: EXECUTED LOCALLY (model and all fixtures/tests); project-owned dependency.
- Reuse: DEPEND. No separate Catalyst ARM domain model or handwritten TS mirror.

## Token client interfaces

- Sources: https://github.com/solana-program/token and
  https://github.com/solana-program/token-2022.
- Published packages inspected: spl-token-interface 3.0.0, commit
  `46eaacd2c13629eb3436354ed650a8df8b202bc4`; spl-token-2022-interface 3.1.2,
  commit `0e74c157aebd5dbdef8035b526a56f0e416f9fb6`.
- License: Apache-2.0, verified in both published manifests.
- Inspection: TARGETED SOURCE, not executed locally. Read published state decoding,
  instruction revoke builders, extension inventory/access and PermanentDelegate helper.
- Tests inspected: classic test_instruction_unpack_panic/proptest; Token-2022
  get_extension_types_with_opaque_buffer and malformed TransferHookAccount access tests.
- Experience: use public Pack APIs and native builders. Base unpacking leaves TLV lazy;
  a helper using get_extension().ok() loses malformed-versus-absent information.
- Reuse: DEPEND for typed decoding; PUBLIC API for native builders. No source copied.
- Compatibility: both interfaces compiled with instruction 3.4.0/pubkey 4.2.0 in an
  isolated proof. Upstream solana-address 1.1.0 re-exports Address 2.x; type equality
  was checked by assignments and native instruction return types, without conversion.
- Classic interface now EXECUTED LOCALLY through Approve, Transfer and Revoke fixtures.
- Token-2022 interpretation and live deployed-version matching remain unverified.

## Native classic token fixture

- Source: https://github.com/anza-xyz/mollusk, published mollusk-svm and
  mollusk-svm-programs-token 0.15.1, commit f432ef136ee9779d2a814ebf2b80f44c10607255.
- License: Apache-2.0, verified from the published LICENSE files (manifest uses license-file).
- Inspection: EXECUTED LOCALLY. Read token helper APIs, update script, loader setup,
  process_instruction and result account/error types. Upstream fixture notes capture
  mainnet-beta slot 347196212; that note does not establish current chain coverage.
- Token ELF SHA256: 8190d3f7ceb6cb7a7a8d8924bff89f9f611e15ce1f806f2b6237f3311a98f697.
  Test asserts the digest before golden compilation. Only the named local fixture
  deployment is supported; the catalog still contains no live deployment records.
- Native source checked: spl-token 9.0.0 processor.rs transfer, approve, revoke,
  validate_owner; interface state Pack/layout and revoke/approve/transfer builders.
- Experience: signature checks happen in the native processor; frozen Revoke fails;
  the allowance is cumulative and native consumption clears delegation at zero.
- Reuse: DEPEND/PUBLIC API for fixture ELF loading and instruction execution. No source
  code copied. Upstream fixture lockfile was a resolution seed; the project lockfile
  retains only its own dependency graph. No program crate is a runtime dependency.

## Historical program source comparison

- Published spl-token 8.0.0 at `1f61242183ea9b43d37b47a1b4f745eb1beb32de` and
  spl-token-2022 8.0.1 at `4a10133df35b9c8635c2d553043b166e240a0f9f`.
- License: Apache-2.0, verified from manifests. TARGETED SOURCE, not executed locally.
- Relevant files: state.rs, instruction.rs, processor.rs process_revoke, extension/mod.rs
  and extension/permanent_delegate.rs. Native clients match the currently pinned 2.x types.
- Verified source: both reject revoke on frozen state and clear delegate/allowance.
  Classic revoke validates the owner; this Token-2022 version also accepts the current
  delegate as revoking authority. Do not infer identical controllers from shared opcodes.
- Reuse: PATTERN ONLY for interpreting the distinction; no source copied. These archived
  versions are not proof of current deployed behavior or a reason to choose an old client.
