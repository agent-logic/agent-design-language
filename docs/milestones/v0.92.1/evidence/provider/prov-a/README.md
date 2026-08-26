# PROV-A Evidence

Issue #514 implements the shared provider inference-profile contract in
`adl/src/provider/profiles.rs` and exposes redacted profile projections through
`adl::provider`.

Local proof lanes:

- `profile-schema`: focused validator and Rust provider tests prove profile
  expansion emits bounded model and inference fields, including explicit
  upper bounds for `max_output_tokens` and `timeout_secs`.
- `ollama-materialization`: focused Rust provider tests prove deterministic
  Ollama materialization.
- `invalid-profile`: focused validator and Rust provider tests prove invalid
  parameters fail before activation, including malformed value types,
  over-bound `max_output_tokens`/`timeout_secs`, and conflicting
  `provider_model_id` overrides.
- `last-known-good`: focused validator and Rust provider tests prove profile
  state retains the last known good profile and redacted materialization from
  the previous valid state, and that an invalid candidate leaves the active
  materialization unchanged.
- `redaction`: focused Rust provider tests prove private profile config values
  and raw endpoint URLs do not appear in redacted/materialization projections,
  including nested private-key values.

No credentials, private prompts, provider responses, legal instruments, auth
codes, recovery factors, or paid cloud/provider mutations are retained here.
