# Continuity

## Completed

- ARM root library pushed at `5b05e06892dcc6d20d1db7f0324030916b152d6a`.
- Catalyst Rust semantic ABI uses that exact Git revision and native Solana instruction/address types.
- Checked dispatch verifies protocol, program identity, deployment/version support,
  source evidence, valid ARM projection and stable change identity.
- Existing TypeScript event API retained; it is not the semantic ARM API.
- Test adapter is synthetic ABI evidence, not a supported native protocol.
- Minimality audit removed unused Suspend/Resume variants. Revoke and ResetAuthority
  now have verified native targets; no speculative action kinds were added.
- Official token interfaces and harness compatibility compiled with native types.
- Classic SPL delegate Spend projection, native owner Revoke action and canonical
  declared removal implemented. Only exact local fixture deployment is supported.
- Golden raw bytes reproduced by native Approve; native Transfer enforces/consumes
  allowance; native Revoke round trip recompiles to no delegate authority.
- Classic Mint/Freeze/Thaw projection and native authority removal actions implemented.
  Freeze/Thaw are direct operational powers on a mint's token accounts. ARM is unchanged.
- Native InitializeMint2/MintTo reproduce both mint and token golden bytes; clearing
  mint/freeze authority round-trips, preserves unrelated authority and cannot be restored.
- Close authority projection and native explicit-closer reset complete. Reset restores
  the owner fallback; Diff is Changed, not Removed. Mint and Close use AuthorityState's
  keyed native observations, with no duplicate observation model or ARM changes.
- Token-2022 ordinary and Permanent Delegate Spend compile from native golden fixtures.
  Owner and delegate revoke round trips preserve permanent authority. Unknown extension
  semantics and same-principal overlap fail closed; ARM remains unchanged.
- Native CloseAccount fixtures cover rent return, deletion/recompilation, ordinary
  token balance checks, frozen state, wrapped SOL, signatures and recipient aliasing.

## Test status

- Baseline SDK: 3 Bun tests and TypeScript check passed.
- Rust ABI: determinism, requirements, version/program mismatch, evidence propagation,
  malformed/duplicate output, stale action evidence and declared action changes.
- Rust fmt/strict Clippy pass; 6 ABI, 8 delegate, 8 mint, 9 close and 11 Token-2022 tests pass.
- Existing 3 Bun tests and TypeScript check pass.

## Hosted CI

- Cataloger CI is EXTERNALLY BLOCKED by the GitHub account billing lock. No workflow
  steps ran. This external lock does not invalidate exact local gate evidence.
- Owner authorized architecture progression on local checks; no Railway CI substitute.

## Next critical path

- Cataloger correction a88dbd8 removes ARM semantics and clarifies support boundaries.
- Final SUB-0 local checks pass: ARM 12, Cataloger 30 and SDK 42 Rust tests, SDK 3 Bun
  tests and typecheck, formatting, strict Clippy and whitespace checks.
- Freeze/Thaw classification corrected to Direct; ARM schema and native actions unchanged.
- PADDLE UP: Subscriptions maintainer study, three durable research outputs, no adapter.
- No live deployment coverage claimed. Unsupported native subsets remain documented.
- Do not commit the pre-existing untracked package-lock.json.
