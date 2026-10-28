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
  `true` as the profile default for responsive single-turn reviewer/runtime
  dispatch and allows runtime override to `false` only for continuity-preserving
  long-running agent turns that also preserve unmodified `reasoning_content`.
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
the operator-approved ZAI_API_KEY source, the credentialed reviewer-viability smoke
completed successfully:

- Command shape:
  `ZAI_API_KEY=<redacted> .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh`
- Credential source: operator-approved `ZAI_API_KEY` source; the credential
  value and machine-local filename are not printed, copied, committed, or
  serialized.
- Profile: `z_ai:glm-5.3-flash`
- Runtime surface: direct hosted Z.ai API, `glm-5.3-flash`
- Parameters: `reasoning_effort=low`, `clear_thinking=true`,
  `temperature=0.2`, `top_p=0.8`, `max_output_tokens=64`
- Timeout/attempts: 45 seconds, one attempt
- Result: passed, `final_status=ok`, HTTP 200, duration 1714 ms
- Expected output: `GLM reviewer smoke ok`
- Retained redacted proof:
  `docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/live-proof-redacted-summary.json`

The same direct profile also completed a bounded reviewer-style verdict task
for the #582 live-proof evidence:

- Command shape:
  `ZAI_API_KEY=<redacted> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request <local ignored reviewer-proof request> --out <local ignored reviewer-proof result> --log <local ignored reviewer-proof run log>`
- Result: passed, `final_status=ok`, HTTP 200, duration 2405 ms
- Model verdict:
  `VERDICT: PASS; FINDINGS: none; LIMITATIONS: Single-run smoke with max_attempts=1 and 64-token output confirms connectivity only, not sustained reliability or reasoning quality of the glm-5.3-flash endpoint.`
- Retained redacted proof:
  `docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/live-proof-redacted-summary.json`

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
reviewer prompt evidence favors `low` as the default:

- `low`: passed for the direct reviewer-viability smoke, the bounded
  reviewer-style verdict task, and all five open-PR triage calls.
- `high`: failed on the #582 PR-review prompt with HTTP 200 but empty text
  output under a 90-second timeout and 1400-token output budget, then passed
  when rerun with `max_output_tokens=8192` and a 180-second timeout. The
  successful call completed in 36240 ms and returned a bounded
  `NEEDS_HUMAN_REVIEW` triage verdict.
- `max`: failed twice on the #582 PR-review prompt with HTTP 200 but empty text
  output under small output budgets, then passed when rerun with
  `max_output_tokens=16384` and a 240-second timeout. The successful call
  completed in 151326 ms and returned a bounded `NEEDS_HUMAN_REVIEW` triage
  verdict.
- `medium`: not a documented GLM-5.3-Flash `reasoning_effort` value. ADL must
  not pass it through to Z.ai; if a human-facing medium preset is needed, map
  it internally to `high` with a larger output-token budget and timeout.

The current reviewer-trial default should therefore remain `low` with
`clear_thinking=true`. `high` is the practical "medium-depth" reviewer tier
when paired with a larger output budget. `max` remains valid for explicit deep
review only, but its minutes-scale latency makes it a poor default for ordinary
runtime dispatch.

## Reviewer quality characterization

The first open-PR reviewer smoke was intentionally conservative but not a fair
quality test: it supplied a truncated, lifecycle-heavy PR packet and therefore
handicapped the model on code-local defects. A follow-up quality experiment used
small, complete, focused packets around the exact endpoint-regression surface
from the failed #582 review and the repaired current code. The quality claim is
therefore packet-specific: the evidence proves behavior on the exact packets
identified below, not a blanket replacement for ADL exact-head approval review.

The baseline human/Codex review failure was:

- Existing `z_ai:glm-5` and `z_ai:glm-5-current` profiles must continue using
  `https://open.bigmodel.cn/api/paas/v4/chat/completions`.
- The rejected candidate changed the shared Z.ai endpoint constant to
  `https://api.z.ai/api/paas/v4/chat/completions` and reused that constant for
  both existing profiles and the new `z_ai:glm-5.3-flash` profile.
- The correct repair gives GLM-5.3-Flash its own `api.z.ai` endpoint constant
  while retaining the legacy endpoint for the existing GLM-5 profiles.

Local credentialed quality probes, using the operator-approved `ZAI_API_KEY`
source without serializing the credential value or source filename, produced:

| Probe | Candidate | Request sha256 | Effort | Result | Duration | Quality signal |
| --- | --- | --- | --- | --- | ---: | --- |
| `old-endpoint-regression-low` | rejected PR head `724515224c36e55bed59e16c08e99d559271fa7d` | `22276c77f89ec5e5de0e3a627a5ff18ded42f8e3dc27f24802a2d036220d9c31` | `low` | `FAIL` | 8752 ms | Correctly found both `z_ai:glm-5` and `z_ai:glm-5-current` were rerouted to `api.z.ai`; included one harmless caveat about externally verifying the new Flash host. |
| `old-endpoint-regression-high` | rejected PR head `724515224c36e55bed59e16c08e99d559271fa7d` | `23cfe04eab78a62bb8da103b5e560f3ed2c13f76d16ddfb60e38392d594ae76e` | `high` | `FAIL` | 7668 ms | Correctly found the breaking endpoint change, tied it to the migration scope boundary, and identified the new Flash entry as structurally fine. |
| `current-endpoint-repair-high` | repaired worktree head `c9759517d30b9ea14175be6d6fdaf9d019183593` | `daec9511fedded4f188de98ac5b5231b48b12f3d09eb6851e270fe52e635cec5` | `high` | `PASS` | 10240 ms | Correctly recognized the legacy endpoint was preserved for `z_ai:glm-5` and `z_ai:glm-5-current`, and did not repeat the stale endpoint finding. |

The corresponding response and run-log sha256 digests were:

- `old-endpoint-regression-low-result.json`:
  `a30a311fa1582411737328e613d76d0b47af10025ea6b24dee137decf62014a9`
- `old-endpoint-regression-low-run.jsonl`:
  `687d895b9801c7b6515d3fdbe94970a412d27fb99e777fd6b06c8d361a3329ce`
- `old-endpoint-regression-high-result.json`:
  `4c434c2f818474cad2649b11785c14f366c493b05386101a0ab0fecee496f09e`
- `old-endpoint-regression-high-run.jsonl`:
  `c65831cb869f77b77dd00739d34ce475734b129743b3fcb9de9b552a36786120`
- `current-endpoint-repair-high-result.json`:
  `6b46ccc3a1506074da7499d4b81639a9bdbe04b29d36c2b571a49c8b0e2f210d`
- `current-endpoint-repair-high-run.jsonl`:
  `5cd08d46e77271703c4fab4673a5d9424087bbf6859d731906f14abf4a325107`

This moves the reviewer-quality conclusion from "not proven" to "useful when
the review harness supplies a bounded, complete packet." The model should still
not replace ADL exact-head review or required CI inspection, but it is viable as
a first-pass reviewer for focused code packets and as a second-opinion reviewer
when the harness explicitly records packet completeness, exact candidate, effort
level, output budget, and limitations.

## References

- Z.ai AutoClaw GLM-5.3-Flash release note:
  <https://autoclaw.z.ai/blog/model/glm-5.3-flash/>
- Z.ai GLM-5.3-Flash guide: <https://docs.z.ai/guides/vlm/glm-5.3-flash.md>
- Z.ai chat-completions API: <https://docs.z.ai/api-reference/llm/chat-completion.md>
- Hugging Face model card: <https://huggingface.co/zai-org/GLM-5.3-Flash>
- Unsloth GLM-5.3-Flash guide: <https://unsloth.ai/docs/models/glm-5.3-flash>
- OpenRouter variant page: <https://openrouter.ai/z-ai/glm-5.3-flash>
