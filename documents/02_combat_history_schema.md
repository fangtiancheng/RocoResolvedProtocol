# Combat History Schema Rules

This document records rules for extending `models_common.rs`,
`models_observed.rs`, `models_resolved.rs`, and merge helpers.

## 1. Naming

Use names that describe the history layer:

- `CombatHistoryObserved*` for client-perspective data.
- `CombatHistoryResolved*` for merged/full-view data.
- `CombatHistory*` for shared domain structs and enums.
- `*Upload*`, `*Experiment*`, or `TestMode*` for API DTOs.

Avoid names that hide the layer boundary, such as a generic `CombatSpiritState`
for both observed and resolved data when the fields differ semantically.

## 2. Observed Fields

Observed fields must be client-observable.

Good observed fields:

- local player's full skill PP and panel data
- opponent values revealed by combat packets
- action events seen by this client
- display overlays shown to this client
- packet source metadata needed for debugging

Bad observed fields:

- hidden opponent PP invented from static data
- enemy panel stats guessed from pet id
- server-only state not present in client packets
- merged data from the other player's upload

If a field is useful but not client-observable, put it in resolved or leave it
out.

## 3. Resolved Fields

Resolved fields may combine both perspectives. They should be explicit about
what is fully known versus still unavailable.

Current resolved structs mostly use complete side data from the participant who
owns that side. If future merge logic supports repair/conflict tracking, prefer
adding typed provenance or conflict fields instead of silently picking one side.

## 4. Unknown Values

There are two different cases:

- Not observed: the client genuinely does not know the value. Model this with
  `Option`, empty collections, or a schema variant that clearly means partial
  observation.
- Unknown protocol value: the code saw a value that should be known but is not
  mapped. Return `Err`.

Do not mix these cases.

`CombatHistorySideHint::Unknown` exists for partial side hints in schema data.
It should not be used as a convenience fallback when side resolution fails.

## 5. Raw Numeric Values

Raw numbers should not leak through domain schemas when a stable enum is cheap.

Allowed raw values:

- stable ids such as `spirit_id`, `skill_id`, `item_id`
- protocol source metadata such as `cmd_id`
- fields intentionally not decoded yet and documented as such

Prefer enums for:

- participant type
- action kind
- finish reason
- field effect
- abnormal state
- equipment type/attribute
- property stage
- change-spirit kind
- restrain hint

## 6. Merge Validation

Validation should run before merge output is trusted.

It should catch:

- different `battle_id`
- mismatched participant identity
- visible spirit identity mismatch
- HP/max HP mismatch when both sides observed the value
- panel/stat disagreement when both sides provide the value
- snapshot disagreement that would make resolved output ambiguous

Validation should not auto-repair disagreements. If repair is needed later, add
a separate repair layer with explicit provenance.

## 7. Merge Output

Merge helpers should be deterministic and conservative:

- merge only compatible histories
- keep stable round ordering
- do not invent snapshots for rounds missing from one side unless a repair policy
  is explicitly added
- prefer owned-side complete data over opponent-side partial data
- preserve display state only when it has clear ownership/meaning

## 8. Serde Compatibility

Schemas are consumed outside this crate, so serialized names matter.

Rules:

- Keep `#[serde(rename_all = "camelCase")]` for JSON object fields unless there
  is a documented breaking change.
- Keep tagged enum styles stable.
- Add optional fields before required fields when evolving persisted JSON.
- Do not remove old fields without coordinating storage and RocoAI upload code.

## 9. Test Expectations

For new schema helpers, add tests for:

- accepted raw values
- rejected raw values
- side resolution success and failure
- validation mismatch messages
- merge output shape for the changed field

Tests should assert errors for invalid values instead of silently accepting
fallback output.
