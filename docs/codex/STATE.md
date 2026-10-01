# Continuity

## Completed

- ARM root library pushed at `5b05e06892dcc6d20d1db7f0324030916b152d6a`.
- Catalyst Rust semantic ABI uses that exact Git revision and native Solana instruction/address types.
- Checked dispatch verifies protocol, program identity, deployment/version support,
  source evidence, valid ARM projection and stable change identity.
- Existing TypeScript event API retained; it is not the semantic ARM API.
- Test adapter is synthetic ABI evidence, not a supported native protocol.

## Test status

- Baseline SDK: 3 Bun tests and TypeScript check passed.
- Rust ABI: determinism, requirements, version/program mismatch, evidence propagation,
  malformed/duplicate output, stale action evidence and declared action changes.
- Rust fmt/strict Clippy pass; 6 ABI tests pass. Existing Bun tests/typecheck pass.

## Real blockers

- Separate indexer checkout and architecture book remain unidentified.

## Next critical path

- Verify Cataloger repository identity and implement deterministic resolver after ABI push.
- SUB-0 locked until SPL native fixtures and revoke round trip are verified and pushed.
- Do not commit the pre-existing untracked package-lock.json.
