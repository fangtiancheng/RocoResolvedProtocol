# RocoResolvedProtocol Architecture

This crate defines shared combat history schemas and merge helpers. It sits
between live clients such as RocoAI and offline/server-side tools that store,
validate, compare, or merge combat histories.

## 1. Scope

In scope:

- `CombatHistoryObserved`: client-perspective combat history uploaded by RocoAI.
- `CombatHistoryResolved`: merged history built from multiple observed
  histories.
- Shared domain enums and structs used by both observed and resolved schemas.
- Upload/test-mode API DTOs.
- Compatibility validation before merging.
- Deterministic merge helpers for already-compatible histories.

Out of scope:

- Decoding ADF/socket packets.
- Reading Flash/AS directly.
- Managing live combat phases.
- Projecting UI DTOs.
- Rendering battle logs.
- Repairing malformed observed histories silently.

## 2. Relationship With RocoAI

RocoAI owns live combat runtime:

- protocol decode
- `CombatSideRegistry`
- `CombatBattleFacts`
- observed history recording
- UI projection and logs

RocoResolvedProtocol owns shared schema:

- stable serde layout
- typed combat history values
- side/participant helper functions
- merge validation and resolved output models

RocoAI may depend on this crate for types and small pure helpers, but this crate
must not depend on RocoAI.

## 3. Data Model Layers

### Observed

Observed history is what one client actually knew. It may contain partial
opponent data, but it should not fabricate hidden information. If RocoAI cannot
observe a value, observed history should model that absence explicitly.

### Resolved

Resolved history is the merged view. It may combine both players' observed
histories and can contain fuller information than either single client saw.

Current merge code is intentionally conservative: it validates compatible
histories and merges snapshots that exist from both sides. Do not treat the
current merge helper as a full repair engine.

### API

`models_api.rs` defines JSON DTOs for upload and experiment/test-mode flows.
These DTOs should remain transport-safe and avoid live runtime concepts.

## 4. Side Resolution

`models_common.rs` exposes side identity helpers:

- `CombatHistoryParticipantType::from_raw`
- `CombatHistorySideIdentity::from_raw`
- `resolve_combat_history_side_identity`
- `opposite_combat_history_side`

These helpers are pure schema/domain functions. They are not a replacement for
RocoAI's runtime `CombatSideRegistry`.

Rules:

- If raw participant type is unknown, return `Err`.
- If side cannot be resolved, return `Err`.
- `CombatHistorySideHint::Unknown` is allowed only where the schema explicitly
  represents partial observation.
- Do not use `Unknown` as a fallback for failed business resolution.

## 5. Error Handling

Use hard errors for required schema/domain data:

- unknown raw enum values
- impossible indexes
- invalid participant side identity
- incompatible histories during merge validation

Avoid:

- `_ => 0`
- `_ => Unknown`
- `unwrap_or_default()` on required fields
- lossy merge behavior that hides disagreements

If a real protocol value is not understood yet, either add a named enum variant
with a comment explaining the evidence, or return an error until the value is
understood.

## 6. Versioning

`CombatHistoryObserved.schema_version` is the client upload contract. Schema
changes should be intentional:

- Add optional fields when possible.
- Avoid renaming serialized fields casually.
- Keep `serde(rename_all = "...")` stable.
- Document breaking changes before changing upload/storage consumers.

## 7. Review Checklist

When changing this crate, check:

- Is this a shared schema concern, not a live runtime concern?
- Does the type represent observed or resolved data clearly?
- Are raw protocol values converted at a clear boundary?
- Does unknown data mean "not observed" or "bug/unrecognized value"?
- Does merge validation catch disagreements before merge output is produced?
- Can RocoAI consume the change without duplicating schema types locally?
