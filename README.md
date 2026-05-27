# RocoResolvedProtocol

Shared Rust models for Roco combat history upload, validation, and future
resolved-history merging.

This crate is intentionally schema-focused. It should not decode game network
packets and should not contain live combat runtime logic. RocoAI produces
`CombatHistoryObserved`; this crate defines the shared JSON contract and helper
functions that validate or merge observed histories.

## Documents

- [Architecture](documents/01_architecture.md)
- [Combat History Schema Rules](documents/02_combat_history_schema.md)

## Development

```powershell
cargo fmt
cargo check --locked
cargo test
```
