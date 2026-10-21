# UTS v1.0 Schema

## Status

Reference baseline for Universal Tool Schema v1.0.

## Required Fields

`UTS v1.0` tool definitions require:

- `schema_version`
- `name`
- `version`
- `description`
- `input_schema`
- `output_schema`
- `side_effect_class`
- `determinism`
- `replay_safety`
- `idempotence`
- `resources`
- `authentication`
- `data_sensitivity`
- `exfiltration_risk`
- `execution_environment`
- `errors`

## Safety Invariants

- `schema_version` must be `uts.v1`.
- If `authentication.required` is true, `authentication.mode` must not be
  `none`.
- If `side_effect_class` is `exfiltration`, `exfiltration_risk` must be `high`.
- If `exfiltration_risk` is `high`, `side_effect_class` must be
  `exfiltration`.

## Non-Authority

UTS v1.0 validity does not imply authorization, execution permission, replay
permission, approval, governance compliance, or runtime safety.

The machine-readable schema lives at
`../../schemas/uts/v1.0/universal_tool_schema.v1.schema.json`.
