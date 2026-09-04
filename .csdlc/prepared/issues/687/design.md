# Issue #687 design: explicit provider inference readiness

## Decision

Introduce one serialized `InferenceReadinessState` used by Runtime v3 agent
roster evidence and projections. The state describes whether the configured
provider/model can execute real inference; it is not a generic component or
process-health label.

## State contract

The closed state set is:

- `unimplemented`: no production adapter exists for the configured provider;
- `unavailable`: a supported provider or model cannot currently be reached or
  found and recovery may succeed without changing the declaration;
- `model_loading`: a supported provider/model is undergoing its bounded preload
  and inference probe;
- `failed`: the supported adapter completed a probe but returned invalid,
  rejected, or otherwise non-ready inference behavior;
- `ready`: preload and real governed inference have succeeded.

Only `ready` is communication-eligible. The roster's existing `state`,
`health`, `availability`, and `activity` strings are derived from the typed
inference state at one boundary so they cannot contradict it.

## Runtime integration

Resident Shepherd recovery reports typed attempt failures. Unsupported provider
adapters are `unimplemented`; transport or missing-model failures are
`unavailable`; a failed governed inference probe is `failed`; successful preload
and probe is `ready`. `model_loading` remains the retry/start transition.

Dynamic-agent health refresh retains the concrete Ollama verification failure
instead of flattening it to a boolean, then maps it to the same taxonomy.
Existing provider/model identifiers remain visible only where current roster
policy already permits them. Canonical names, display names, and agent IDs are
unchanged.

## Production-credit boundary

The existing production assembly validator remains fail-closed for missing
operational adapter bindings. This change adds focused proof that an
`unimplemented` or otherwise non-ready inference state never becomes
communication-eligible and never receives `ready` projection credit. It does
not add a placeholder executor or implement a provider.

## Scope and validation

The implementation is limited to Runtime-kernel readiness types, roster/control
projection, resident Shepherd recovery classification, and focused deterministic
tests. Validation uses no live Runtime, provider credentials, provider calls,
AWS, or other cloud resources.

