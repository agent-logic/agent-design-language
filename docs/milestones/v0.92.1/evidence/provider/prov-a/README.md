# PROV-A Evidence

Issue #514 implements the shared provider inference-profile contract in
`adl/src/provider/profiles.rs` and exposes redacted profile projections through
`adl::provider`.

Local proof lanes:

- `profile-schema`: focused Rust provider tests prove profile expansion emits
  bounded model and inference fields.
- `ollama-materialization`: focused Rust provider tests prove deterministic
  Ollama materialization.
- `invalid-profile`: focused Rust provider tests prove invalid parameters fail
  before activation, including malformed value types and conflicting
  `provider_model_id` overrides.
- `last-known-good`: source and tests prove profile state retains the last
  known good profile from the previous valid state with validate-before-
  activation semantics.
- `redaction`: focused Rust provider tests prove private profile config values
  do not appear in redacted projections, including nested private-key values.

No credentials, private prompts, provider responses, legal instruments, auth
codes, recovery factors, or paid cloud/provider mutations are retained here.
