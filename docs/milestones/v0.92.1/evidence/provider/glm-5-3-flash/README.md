# GLM-5.3-Flash Provider Profile Evidence

Issue: #578

Profile: `z_ai:glm-5.3-flash`

## Source facts

- Z.ai documents GLM-5.3-Flash as model id `glm-5.3-flash` and exposes it
  through the Z.ai API platform.
- Z.ai chat-completions documentation uses
  `https://api.z.ai/api/paas/v4/chat/completions` for the direct API path.
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

Live Z.ai execution remains credential-gated by `ZAI_API_KEY`; absence of that
credential is not claimed as a live model PASS.

## References

- Z.ai GLM-5.3-Flash guide: <https://docs.z.ai/guides/vlm/glm-5.3-flash.md>
- Z.ai chat-completions API: <https://docs.z.ai/api-reference/llm/chat-completion.md>
- Hugging Face model card: <https://huggingface.co/zai-org/GLM-5.3-Flash>
- Unsloth GLM-5.3-Flash guide: <https://unsloth.ai/docs/models/glm-5.3-flash>
- OpenRouter variant page: <https://openrouter.ai/z-ai/glm-5.3-flash>
