# Issue #640 Design: Model-backed Resident Shepherd

## Decision

Extend the required `resident_shepherd` Runtime configuration with an explicit
provider profile, model identifier, endpoint reference, and preload policy. The
Shepherd remains the Runtime-owned control-plane resident introduced through
the existing governed-operation boundary; only its reasoning executor changes
from the native placeholder to the configured provider adapter.

Startup admission and inference readiness are distinct. Runtime startup requires
a non-empty set of valid resident Shepherd configurations and creates one
resident for each unique configured canonical identity. The Shepherd advertises
`model_loading` until a provider
health check and model preload probe succeed, then advertises `ready`. A
temporary provider or model failure moves only the Shepherd to `degraded` and
starts lifetime recovery using bounded configurable probes and backoff
intervals; it does not terminate the
Runtime or make unrelated admitted agents unavailable.

The preload policy is provider-neutral. The initial Wuji profile may select
Ollama and `qwen3:8b`, but configuration and API surfaces must not hard-code
either. Local-provider preload should request durable residency where the
provider supports it, while restart recovery always repeats the health and
preload proof rather than assuming the model is still loaded.

## Configuration and authority

The configuration file is the sole startup authority for the resident
Shepherd's canonical name, provider profile, model, endpoint reference, and
preload policy. Provider credentials remain outside serialized configuration
and API output. Provider/model identity and readiness are projected through the
agent roster/detail API as non-secret health metadata.

The canonical-name projection delivered by #617 is a required execution-base
dependency. #640 must start from a `main` commit containing #617 rather than
reimplementing or carrying that PR as an accidental stack.

## Reliability boundary

- No millisecond-scale launch deadline is permitted.
- Startup/preload budgets are configurable and generous enough for cold local
  models; timeout expiry degrades and retries the Shepherd instead of killing
  the Runtime.
- Provider requests use the existing governed operation and cancellation
  boundaries.
- Recovery is idempotent and cannot duplicate a configured canonical identity.
- Runtime shutdown remains the only authority that terminates the resident
  Shepherd recovery task.

## Readiness truth table

| Shepherd model state | Resident entry | Runtime `/v1/ready` | Observatory snapshot/feed |
| --- | --- | --- | --- |
| `model_loading` | admitted, not inference-ready | remains globally ready when all non-Shepherd blockers pass | agrees on `model_loading` and global readiness |
| `ready` | admitted and inference-ready | remains globally ready | agrees on `ready` and global readiness |
| `degraded` | admitted, inference unavailable, retry active | remains globally ready when all non-Shepherd blockers pass | agrees on `degraded`, retry state, and global readiness |

The Runtime readiness endpoint, its `blocking_reasons`, roster/detail responses,
and Observatory snapshot/feed must be derived from one health snapshot so they
cannot disagree. Shepherd inference readiness gates Shepherd work, not unrelated
Runtime admission, API availability, or other agents.

## Proof boundary

Focused tests prove configuration validation, provider/model selection,
loading-to-ready transitions, governed inference, degraded isolation,
idempotent recovery, restart preload, and agreement among `/v1/ready`,
`blocking_reasons`, roster/detail health, and Observatory snapshot/feed. A
bounded Wuji acceptance proves an
Ollama-backed Shepherd survives a Runtime restart, becomes ready without a
manual `ollama run`, reports truthful non-secret health, and completes one
governed inference. Broad workspace validation is deferred to CI because this
issue changes one Runtime subsystem and its API contract.
