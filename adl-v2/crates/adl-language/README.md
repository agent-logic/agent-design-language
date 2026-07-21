# ADL language core

`adl-language` is the side-effect-free ADL v2 source-language boundary. It
defines exactly six primitives: providers, tools, agents, tasks, workflows,
and one run. It parses strict YAML or JSON, rejects duplicate and unknown
fields, validates language-level references and state dependency cycles, emits
stable diagnostics, generates JSON Schema, and produces deterministic
canonical JSON.

The crate has no compiler, execution, provider, network, filesystem-mutation,
clock, environment, Runtime, or C-SDLC authority. Pattern expansion,
`ExecutionPlan`, execution, record signing, and CLI behavior belong to later
work packages.

## API

- `parse_yaml` / `parse_json`: strict syntax and duplicate-key parsing.
- `parse_and_validate_yaml` / `parse_and_validate_json`: complete language
  validation.
- `validate`: pure semantic validation.
- `canonical_json` / `canonical_bytes`: deterministic source-model output.
- `json_schema`: generated Draft 2020-12-compatible schema value.

The characterization parity test consumes the reviewed #5337 corpus directly
and records why compiler-, CLI-, execution-, and signing-owned cases are not
WP-04 language claims.

