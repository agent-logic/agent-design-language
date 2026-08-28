# Issue 578 design

Status: design ready for fresh-session review.

## Intent

Add GLM-5.3-Flash as a general-purpose direct Z.ai provider profile through the shared provider-profile machinery from #514. The profile must be usable by reviewer selection, but it is not a reviewer-only profile.

## Profile contract

- Profile name: `z_ai:glm-5.3-flash`
- Stable ADL model identity: `hosted:adl-z-ai:glm-5.3-flash`
- Provider family: `z_ai`
- Provider model id: `glm-5.3-flash`
- Endpoint: current documented Z.ai chat-completions endpoint `https://api.z.ai/api/paas/v4/chat/completions`.
- Auth: existing `ZAI_API_KEY` provider-family contract; credentials must not be serialized into profile or invocation evidence.

## Default parameter contract

Use source-grounded defaults that are good for general agent/reviewer use and allow runtime overrides:

- `reasoning_effort`: default `max`; runtime override allowed only for `low`, `high`, or `max`.
- `thinking.clear_thinking`: explicit boolean default chosen by ADL rather than hidden provider behavior. Use `false` by default to preserve reasoning continuity for long-running agent/reviewer turns; allow runtime override to `true` for shorter chat-like or context-cost-sensitive calls.
- `temperature`: default `1.0`; runtime override allowed within the provider's documented range.
- `top_p`: default `0.95`; runtime override allowed within the provider's documented range.
- `max_tokens`: default should remain ADL/provider-config bounded; GLM-5.3-Flash-specific validation must reject values above `131072`.

## Implementation shape

1. Extend the provider profile registry with `z_ai:glm-5.3-flash`.
2. Add profile/config materialization so GLM-5.3-Flash defaults are explicit in the provider target and can be overridden at runtime.
3. Extend the Z.ai HTTP request builder to include supported optional fields only after validation:
   - `reasoning_effort`
   - nested `thinking.clear_thinking`
   - `temperature`
   - `top_p`
   - `max_tokens`
4. Keep direct Z.ai distinct from OpenRouter and Ollama:
   - OpenRouter `z-ai/glm-5.3-flash` is useful evidence but not implemented here.
   - Ollama `glm-5.3-flash:cloud` is a possible future cloud-backed profile/smoke route, not the primary #578 profile.
5. Prove reviewer selection by exercising an ADL reviewer-agent fixture that names the general profile and resolves to `hosted:adl-z-ai:glm-5.3-flash`. A live provider call is credential-gated and must not be claimed when no credential is present.

## Validation

- Profile test proves profile name, ADL identity, provider model id, endpoint, and redacted materialization.
- HTTP-family test proves exact request JSON for defaults and runtime overrides.
- Negative test proves invalid `reasoning_effort` and `max_tokens > 131072` fail before network dispatch.
- Reviewer-selection smoke proves a reviewer-agent fixture can select the named general profile and records live dispatch skip truthfully when `ZAI_API_KEY` is absent.

## Non-overlap

Issue #578 must not modify #446 or #455 scope. Any discovered need for OpenRouter provider pinning, Ollama cloud profile support, or local-model shadow execution must be routed as follow-on work unless it is already required for the direct Z.ai GLM-5.3-Flash profile.
