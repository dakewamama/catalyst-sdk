# SUB-0 review

Status corrected on 2026-10-03: the earlier unlock decision below is withdrawn.
It checked local results and recorded upstream refs without inspecting hosted CI.
Cataloger main jobs never started because GitHub reported an account billing lock.
Subscriptions work is held until Cataloger hosted CI is green. The boundary correction
removes Cataloger's ARM dependency and makes the support interval explicit.

Reviewed on 2026-10-03 against pushed revisions:

- ARM: d1e2e6ba752ffd7d67b6c55a15c57bffbb2cf15c; semantic dependency remains
  pinned to 5b05e06892dcc6d20d1db7f0324030916b152d6a.
- Cataloger: d60d49260ffaeda3478fab6b3fd854347c4cd0a4.
- Catalyst: 323b873706a2da47d08cd19faed6f2349e84d24c.

## Gate evidence

| Requirement | Evidence |
| --- | --- |
| ARM 0.1 frozen | Ten canonical fixtures, serialization and conservative evaluation tests; no token-driven schema change |
| Catalyst ABI | Checked dispatch, native instruction types, evidence propagation and unsupported-version tests |
| Cataloger resolver | Finite slot intervals, upgrade boundaries, gaps, deterministic resolution and adapter provenance tests |
| SPL and Token-2022 interpretation | Official state interfaces, checked raw and semantic goldens, hashed native ELF execution |
| Native revoke round trip | Classic owner revoke; Token-2022 owner and delegate revoke; declared removal agrees with recompilation |
| Unknown versions fail closed | Checked dispatch rejects unsupported contexts before native decoding |
| Protocol-neutral ARM | No native protocol types, identifiers, RPC or storage dependencies in ARM source |
| Verification | fmt, strict Clippy and workspace tests pass in all three repositories; SDK Bun tests and TypeScript check pass |
| Milestones pushed | Reviewed revisions match locally recorded upstream main refs |

Rust test totals: ARM 12, Cataloger 30 (7 resolver and 23 retained runtime),
Catalyst 42. SDK also passes 3 Bun tests. Commands run in each repository:

```sh
cargo fmt --check
cargo clippy --offline --locked --workspace --all-targets -- -D warnings
cargo test --offline --locked --workspace
git diff --check
```

## Cross-repository findings

SPL close-authority reset changes the principal to the owner rather than removing
Close permission. The existing Changed representation handles this without a schema
change. Mint/freezing powers remain distinct from spending authority. Token-2022
Permanent Delegate bypasses ordinary allowance; overlapping principals are explicitly
Unsupported rather than assigned an independently consumed budget.

A second native program adapter was added without editing ARM or the adapter trait.
Official decoders, builders and native execution own byte layouts and enforcement.
AuthorityState is shared only where the observed account shape actually matches.
No RPC provider, decoder framework, intent compiler or generic policy engine was added
in anticipation of later protocols. No further extraction or abstraction is justified
by these fixtures.

## Decision and limits

The earlier review passed only the documented local fixture checks. It did not satisfy
the hosted CI gate and cannot authorize further Subscriptions work.
This decision does not advertise live Solana support. Cataloger contains no verified
live deployment records. Controllers requiring multisig or program semantics, unreviewed
Token-2022 extensions and overlapping delegate roles remain unsupported. Individual
Exact projections do not imply complete protocol or address coverage.

The retained Cataloger webhook binary is not a completed index pipeline. Its known
runtime test/amount issues remain recorded separately. No address API, general
transaction simulation Diff or permission compiler is claimed by this gate.
