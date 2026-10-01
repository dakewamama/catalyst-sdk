# Reuse boundaries

| Responsibility | Owner | Decision | Remaining Velon work |
| --- | --- | --- | --- |
| ARM serialization | Serde | DEPEND, already used | Semantic validation and schema fixtures |
| Solana keys, instructions and signer/writable metas | Official Solana SDK crates | DEPEND, already used | Select native action; preserve its meaning |
| SPL account, mint and multisig decoding | spl-token-interface | DEPEND, source verified; not installed in SDK yet | Account owner/version checks and ARM translation |
| Token-2022 base state and extension decoding | spl-token-2022-interface | DEPEND, source verified; not installed in SDK yet | Inspect extension inventory; reject unknown semantics |
| Native revoke and authority instruction construction | Official token interfaces | PUBLIC API, source verified | Identify the actual controller and required signers |
| Exact deployment and adapter provenance | Cataloger | Project-owned | Bounded verified history; unknown versions rejected |
| Native authority to common meaning | Catalyst and ARM | Project-owned | Translation, evidence and native fixture conformance |

Source verification is not execution evidence. The token interface versions inspected
are 3.0.0 and 3.1.2. They require Solana 3.x instruction types; the current SDK and
Cataloger pins are 2.x. Resolve and test this compatibility before adding an adapter.
Do not implement a second instruction model or conversion framework to hide the gap.
A client crate version does not establish the deployed program version.

Token-2022 base unpacking leaves extension data lazy. Successful unpack is not proof
that every extension is valid or understood. Enumerate extensions, propagate malformed
data errors, and reject unknown semantic effects. A helper that maps extension errors
to None must not become an exact absence claim.

The ABI remains small: one required adapter trait, native state owned by each adapter,
checked dispatch and native instruction output. It has no decoder registry, intent
compiler, signer, policy engine or RPC framework. Only Revoke is advertised as an action
kind; other actions require a real native target before being added.

Ingestion, developer clients and execution infrastructure remain behind their existing
source gates. No dependency decision for those layers is established by this audit.
Choose the provider only after inspecting its relevant source and proving the needed
behavior. The retained webhook service is legacy runtime code, not a new generic indexer.
