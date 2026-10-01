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
- Compatibility: published interfaces use Solana 3.x instructions. Current project
  instruction/key pins are 2.x; local compatibility proof is still required.
- Excluded claims: no deployed-version match, no supported adapter, no executed revoke.

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
