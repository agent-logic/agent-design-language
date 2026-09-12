# Provider and profile hot loading

ADL can run a production provider reload owner against an explicit
provider-only sidecar. The CSM `adl_workflow` production cycle starts the owner
when `workflow.run_args.provider_reload_sidecar_path` is set. Relative sidecar
paths resolve against the ADL workflow file's parent directory. The owner reuses
the runtime kernel config reload watcher instead of adding a second watcher or
registry.

The sidecar accepts only:

- `schema: adl.provider_reload_sidecar.v1`
- optional `version`
- `providers`

Workflow, task, tool, authority, executable-step, and credential-value surfaces
are intentionally outside the sidecar boundary. Provider credentials must remain
stable references such as environment-variable names or governed provider auth
objects. Raw token, secret, password, API key, client-secret, private-key,
bearer-token, or credential-shaped values are rejected before a candidate can
become active, including suspicious values under neutral containers such as
`auth.value`.

On each accepted edit, the owner validates and materializes a complete candidate
document, including provider profile expansion and last-known-good promotion,
then publishes one immutable provider snapshot with a provider-level generation.
Invalid edits retain the prior complete snapshot and record only a bounded
redacted diagnostic tied to the current provider-level generation.

The execution runner consults the current provider reload snapshot immediately
before building the provider for a local step. That means:

- a valid edit is available to later inference without restarting the process;
- an in-flight step keeps the provider snapshot it selected before dispatch;
- concurrent readers see either the old complete snapshot or the new complete
  snapshot, never a mixed provider map.

The reload owner is deliberately provider-scoped. It does not reload signing
keys, database pools, model weights, workflow authority, tools, or executable
workflow content.

## Editable definition compatibility and proof (#876)

The existing `adl.provider_reload_sidecar.v1` format is retained; no second
registry, watcher, or schema is introduced. `schema` may be omitted for legacy
sidecars; when present it must match that value. `version` is optional, and
`providers` must be nonempty. Each provider accepts the existing `id`, `profile`,
`type` (or `kind`), `base_url`, `default_model`, and `config` fields. Unknown
provider or top-level fields fail closed. A profile-only definition is expanded
before its concrete substrate and adapter constructor are validated. Profile and explicit identity
fields cannot be mixed; bounded profile overrides remain under `config`.

For example, the existing local HTTP adapter can consume:

```yaml
schema: adl.provider_reload_sidecar.v1
providers:
  primary:
    profile: ollama:phi4-mini
    config:
      endpoint: http://127.0.0.1:11434/api/generate
      temperature: 0.0
```

The endpoint is instance data. No special provider ID or host branch selects
this definition. The Ollama HTTP adapter sends the expanded model and temperature;
other profile settings retain their existing adapter-specific semantics. This
change does not make every profile setting executable on every adapter (for
example, the local Ollama CLI does not transmit temperature).

Free-form config strings are checked recursively, including arrays and neutral
nested keys. Raw key prefixes, bearer values, private-key markers and opaque
credential-shaped strings are rejected. This conservative opaque-string check
can reject long arbitrary config strings; it is not a general secret detector.
Declared model identities and local-shadow model, rule-set and evidence-path
fields retain their existing length range, but still reject explicit credential
markers. Shadow evidence paths also retain constructor rejection of absolute
and traversal paths. Only the existing `auth.env`, `api_key_env`,
`auth_env`, and `token_env` reference locations bypass the opaque-string check,
and their values must be environment-variable names. This validation does not
add a new authentication mechanism or resolve credentials during reload. The
public `expected_account_sha256` field remains a validated 64-digit hexadecimal
account digest, not a credential value.

Initial parse/validation failure and rejected replacement diagnostics carry
bounded category messages with redacted input details. Raw parser exceptions,
provider IDs, paths, unknown fields and candidate values are not propagated into
these diagnostic messages. A rejection preserves the complete prior document,
provider generation and digest. Admission reuses existing adapter constructors
(including local-shadow configuration validation); it does not call completion,
resolve token/ADC values, launch a provider process or write shadow evidence.
Existing non-secret runtime defaults, such as timeout or AWS region/profile
configuration, retain their constructor validation behavior. Editing definitions performs no inference or
health probes; actual requests remain explicit dispatch work.

The deterministic #876 proof is
`execute::tests::provider_definitions::editable_provider_definitions_drive_real_dispatch_and_atomic_reload`:
it calls the production execution runner and reload owner, holds one real local
HTTP request at a channel barrier, replaces both endpoint and profile, observes
the new model/temperature on a second endpoint, then dispatches again against the
last-known-good snapshot after invalid profile expansion. Concurrent readers
check the complete two-provider map, and endpoint request counts exclude hidden
probe calls. The complementary
`provider::reload::tests::provider_definitions_reject_nested_credentials_and_redact_loader_errors`
executes initial loader rejection and the watcher parser against malformed,
unsupported, incomplete and nested credential inputs, preserving generation and
bounded redacted diagnostics. A third admission compatibility test verifies credential references and shadow
configuration without dispatch or evidence creation. These are required provider-platform PVF integration
or contract gates using bounded local CPU, filesystem and loopback only; they
provide no hosted-provider qualification or paid-cloud evidence.
