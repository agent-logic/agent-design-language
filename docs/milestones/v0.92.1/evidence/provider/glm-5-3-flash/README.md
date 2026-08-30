# GLM-5.3-Flash Provider Profile Evidence

Issue: #578

Profile: `z_ai:glm-5.3-flash`

## Source facts

- Z.ai documents GLM-5.3-Flash as model id `glm-5.3-flash` and exposes it
  through the Z.ai API platform.
- Z.ai's AutoClaw release note identifies GLM-5.3-Flash as the model it
  anonymously evaluated as Ox Alpha under real-world traffic before official
  release.
- Z.ai chat-completions documentation uses
  `https://api.z.ai/api/paas/v4/chat/completions` for the direct API path.
- Existing ADL `z_ai:glm-5` and `z_ai:glm-5-current` profiles intentionally
  retain the established `https://open.bigmodel.cn/api/paas/v4/chat/completions`
  endpoint so #578 does not silently reroute older Z.ai profiles.
- Z.ai and Hugging Face document `reasoning_effort` support for
  `low`, `high`, and `max`, with `max` as the default/reproduction setting.
- Z.ai and Unsloth document evaluation-style sampling around
  `temperature=1.0` and `top_p=0.95`.
- Z.ai's API documentation lists `thinking.clear_thinking` with default
  `true`, while the Hugging Face/chat-template route documents
  `clear_thinking` defaulting to `false`. ADL intentionally chooses explicit
  `false` as the profile default for continuity-preserving long-lived
  reviewer/agent turns and allows runtime override to `true`.
- The model is large enough that local execution is not the default ADL proof
  path here; Unsloth lists local quantized memory requirements around 100GB for
  1-bit and 128GB for 3-bit operation.

## ADL profile decision

ADL adds one general direct-Z.ai provider profile:

```yaml
providers:
  glm53_flash:
    profile: "z_ai:glm-5.3-flash"
agents:
  reviewer:
    provider: "glm53_flash"
    model: "hosted:adl-z-ai:glm-5.3-flash"
```

The profile is not reviewer-specific. Reviewer selection is proof of normal
agent/provider routing through the shared provider-profile machinery from #514.

## Provider-variant boundary

- Direct Z.ai is the primary #578 profile.
- OpenRouter `z-ai/glm-5.3-flash` remains a separate OpenRouter route and is
  not implemented by this issue.
- Ollama `glm-5.3-flash:cloud` remains a separate Ollama-cloud route. It is
  useful for experimentation because it can preserve the local Ollama API
  shape while dispatching remotely, but it adds an extra cloud provider/trust
  boundary and is not the direct Z.ai profile.

## Validation surface

Focused local proof uses deterministic tests and does not require a live
provider credential:

- `profiles::z_ai_glm_5_3_flash_profile_expands_for_reviewer_agent_selection`
- `http_family::zai_glm_5_3_flash_request_materializes_profile_defaults_and_runtime_overrides`
- `.csdlc/prepared/issues/578/reviewer-selection-smoke.sh`

Live Z.ai execution is credential-gated by `ZAI_API_KEY`; absence of that
credential is not claimed as a live model PASS. After the operator identified
the approved local Z.ai key source, the credentialed reviewer-viability smoke
completed successfully:

- Command shape:
  `ZAI_API_KEY=<redacted> .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh`
- Credential source: `/Users/daniel/keys/z.ai.ADL-default.key`; value never
  printed, copied, committed, or serialized.
- Profile: `z_ai:glm-5.3-flash`
- Runtime surface: direct hosted Z.ai API, `glm-5.3-flash`
- Parameters: `reasoning_effort=low`, `clear_thinking=true`,
  `temperature=0.2`, `top_p=0.8`, `max_output_tokens=64`
- Timeout/attempts: 45 seconds, one attempt
- Result: passed, `final_status=ok`, HTTP 200, duration 3748 ms
- Expected output: `GLM reviewer smoke ok`
- Local redacted artifacts:
  `.adl/provider-smoke/glm-5-3-flash/result.json` and
  `.adl/provider-smoke/glm-5-3-flash/provider-run.jsonl`

The same direct profile also completed a bounded reviewer-style verdict task
for the #582 live-proof evidence:

- Command shape:
  `ZAI_API_KEY=<redacted> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash/reviewer-proof-request.json --out .adl/provider-smoke/glm-5-3-flash/reviewer-proof-result.json --log .adl/provider-smoke/glm-5-3-flash/reviewer-proof-run.jsonl`
- Result: passed, `final_status=ok`, HTTP 200, duration 2405 ms
- Model verdict:
  `VERDICT: PASS; FINDINGS: none; LIMITATIONS: Single-run smoke with max_attempts=1 and 64-token output confirms connectivity only, not sustained reliability or reasoning quality of the glm-5.3-flash endpoint.`
- Local redacted artifacts:
  `.adl/provider-smoke/glm-5-3-flash/reviewer-proof-result.json` and
  `.adl/provider-smoke/glm-5-3-flash/reviewer-proof-run.jsonl`

## Open-PR reviewer smoke

To probe reviewer usefulness beyond a single canned verdict, Ox Alpha /
GLM-5.3-Flash was also run locally against the current ADL open PR set. The
run was read-only and posted no GitHub comments. Each request included PR
metadata, exact head SHA, changed-file list, and a bounded diff excerpt; all
five PR diffs exceeded the excerpt cap, so the outcome is triage evidence, not
an exact-head approval.

| PR | Exact head | Provider result | Model verdict | Limitation |
| --- | --- | --- | --- | --- |
| #588 | `e9816192741ae1b35f678c5b363fb5567605fcf5` | ok, 9667 ms | `NEEDS_HUMAN_REVIEW` | Core Rust/test implementation was outside the truncated excerpt. |
| #586 | `822c81e9d0ad15e479960de542d419e64c80e1f9` | ok, 7922 ms | `NEEDS_HUMAN_REVIEW` | Draft PR; episode package and validator bodies were outside the truncated excerpt. |
| #585 | `8e0c2c217c42c8719fac55de07a4fe498900fac7` | ok, 8664 ms | `NEEDS_HUMAN_REVIEW` | Contract and validator changes were outside the truncated excerpt. |
| #584 | `a5474e6d4cecf90b989c36e5845acd91dbfa3ead` | ok, 12906 ms | `NEEDS_HUMAN_REVIEW` | Substantive skill/docs/validator changes were outside the truncated excerpt. |
| #582 | `497271aceb9bc17fed7469fb48681d658a8af14e` | ok, 8993 ms | `NEEDS_HUMAN_REVIEW` | The remote PR diff was reviewed before the new live-proof evidence commit was pushed; the large diff was truncated. |

This proves the profile can be used for quick PR-review triage and can
truthfully refuse approval when the supplied evidence is insufficient. It does
not replace ADL exact-head review.

## Reviewer effort characterization

The profile supports runtime `reasoning_effort` overrides, but the live
reviewer prompt evidence favors `low` for now:

- `low`: passed for the direct reviewer-viability smoke, the bounded
  reviewer-style verdict task, and all five open-PR triage calls.
- `max`: failed twice on the #582 PR-review prompt with HTTP 200 but empty text
  output, once under the original 45-second timeout and again under a 180-second
  timeout.
- `high`: failed on the same #582 PR-review prompt with HTTP 200 but empty text
  output under a 90-second timeout.

The current reviewer-trial default should therefore remain `low`.
`high`/`max` remain valid API parameters but are not operationally viable for
this PR-review prompt shape until a separate repair or prompt/materialization
change proves otherwise.

## References

- Z.ai AutoClaw GLM-5.3-Flash release note:
  <https://autoclaw.z.ai/blog/model/glm-5.3-flash/>
- Z.ai GLM-5.3-Flash guide: <https://docs.z.ai/guides/vlm/glm-5.3-flash.md>
- Z.ai chat-completions API: <https://docs.z.ai/api-reference/llm/chat-completion.md>
- Hugging Face model card: <https://huggingface.co/zai-org/GLM-5.3-Flash>
- Unsloth GLM-5.3-Flash guide: <https://unsloth.ai/docs/models/glm-5.3-flash>
- OpenRouter variant page: <https://openrouter.ai/z-ai/glm-5.3-flash>
