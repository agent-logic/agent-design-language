# Resident provider monitoring

Runtime keeps a successfully validated resident ready until a real provider
execution failure or an explicit readiness invalidation. Elapsed time alone
never starts another generated-content probe. This applies to local providers
and metered cloud models behind Ollama or an OpenAI-compatible gateway.

Startup first checks provider metadata where supported (Ollama `/api/tags`),
then runs one governed inference probe with the resident's orientation.
Preload does not send a separate warmup prompt. The retained `preload.enabled`
setting does not bypass mandatory governed startup validation. OpenAI-compatible
metadata checks establish no readiness claim because there is no universally
supported metadata endpoint; the governed inference supplies that evidence.

`retry_initial_millis` and `retry_max_millis` configure exponential recovery
backoff, capped at the maximum. They are not monitoring intervals. Successful
recovery returns to idle. Shutdown cancels a pending attempt or wait. Resident
provider/model configuration is restart-required: the existing hot-reload guard
rejects those changes, and an approved restart validates the new configuration.
No periodic synthetic-inference monitoring option is provided.

## Observation and accounting

The existing Runtime API now also exposes:

- `GET /v1/metrics/providers`: process-lifetime request counts by agent, provider,
  model and reason, successful/failed counts, and input/output token estimates.
- `GET /v1/health/providers`: last-observed provider reachability, model
  availability and inference readiness as separate nullable signals.

These reads, ordinary health and readiness reads do not invoke models. Null
means no observation; a metadata success does not prove generated inference.
Signals retain last-observed evidence, not a continuous connectivity guarantee.
Health rows are keyed by the complete agent/provider/model identity. If a
resident ID is removed and re-admitted with a different provider or model, its
new observations have a separate row; the previous row remains historical
evidence. A late completion from the old model cannot overwrite the new row.

Reasons are `operator_conversation`, `agent_to_agent`, `startup_probe` and
`recovery_probe`. Compatibility fallback from tool chat to plain generation is
counted as two provider attempts. Failed or cancelled attempts remain counted;
an operator cancellation does not by itself request recovery. An admitted
conversation's execution deadline is a failure and wakes recovery even when
the deadline cancels the underlying provider future. Explicit provider failures
in both conversation and Shepherd routes invalidate shared resident readiness.

Token estimates are UTF-8 bytes divided by four, rounded up. They are labeled
as estimates, are not tokenizer measurements or provider billing receipts, and
can undercount provider-side output from failed/cancelled requests. Counts are
attempts and do not assert that a provider billed each one. Counters reset when
Runtime restarts. Inspect request rate and reasons to detect unwanted usage.

Provider request diagnostics use `adl_event` on stderr; JSON API payloads remain
separate. Accounting retains identities and aggregate counts, not prompts,
responses, endpoints or credentials. Existing Runtime stderr behavior applies;
this change makes no new compatibility-file-log or OpenTelemetry claim.

## Focused proof

Issue #854 uses production recovery and provider paths against a local counting
cloud-model fixture: exactly one startup inference; no additional calls during
100 rounds of health/readiness/provider API polling over 400 virtual seconds;
a real conversation failure followed by one recovery inference; and another
400 virtual seconds without polling. Separate tests cover exponential retry,
shutdown, invalidation during a successful attempt, fallback accounting and
content-free counter snapshots. Fixtures make no paid provider calls.
