# Provider Inference Profiles

ADL provider profiles materialize into explicit provider identity and bounded
inference parameters before runtime activation.

The shared profile contract is:

- `provider_model_id` binds the provider-native model selected by the profile.
- `temperature`, `top_p`, `max_output_tokens`, and `timeout_secs` are present
  after expansion and validated before activation. Compatibility overrides are
  bounded to `temperature` in `[0.0, 2.0]`, `top_p` in `[0.0, 1.0]`,
  `max_output_tokens` no greater than `32768`, and `timeout_secs` no greater
  than `600`.
- Ollama profiles use `materialization_policy: deterministic_ollama_v1`,
  `temperature: 0.0`, `top_p: 1.0`, `max_output_tokens: 512`,
  `timeout_secs: 120`, and `deterministic_seed: 0`.
- `profile_state.schema: adl.provider_profile_state.v1` retains the
  `last_known_good_profile` plus a redacted
  `last_known_good_materialization` from the previous valid state when one is
  supplied and requires `validate_before_activation`.
- Review and evidence packets use
  `adl.provider_profile_redacted_projection.v1`, which redacts credential,
  token, key, password, passphrase, PIN, prompt, secret, auth, and
  private-payload config surfaces, including nested private keys.
- Deterministic byte evidence uses
  `adl.provider_profile_materialization_projection.v1`, a canonical sorted
  provider/config projection that records only `base_url_present` instead of
  serializing raw endpoint URLs. Raw `AdlDoc` provider maps keep their public
  runtime shape and are not the serialized proof boundary.

## GLM-5.3-Flash

`z_ai:glm-5.3-flash` is a first-class direct Z.ai profile for the hosted
GLM-5.3-Flash API. It is a general provider profile, not a reviewer-only
profile. Reviewer agents select it the same way any other ADL agent selects a
profile-backed provider.

The materialized defaults are:

- `type: z_ai`
- `default_model: hosted:adl-z-ai:glm-5.3-flash`
- `provider_model_id: glm-5.3-flash`
- `endpoint: https://api.z.ai/api/paas/v4/chat/completions`
- `reasoning_effort: max`
- `clear_thinking: false`
- `temperature: 1.0`
- `top_p: 0.95`
- `max_output_tokens: 4096`
- `timeout_secs: 120`

Runtime overrides are intentionally narrow and validated before dispatch:

- `reasoning_effort` may be `low`, `high`, or `max`.
- `clear_thinking` must be a boolean. ADL explicitly defaults it to `false`
  for continuity-preserving long-lived agent and reviewer turns, even though
  the direct Z.ai API default differs; short chat-like calls may set it to
  `true`.
- `temperature` must be in `[0.0, 1.0]`.
- `top_p` must be in `[0.01, 1.0]`.
- `max_output_tokens` may be raised as high as `131072` for GLM-5.3-Flash.

The direct Z.ai profile is separate from provider variants:

- Existing `z_ai:glm-5` and `z_ai:glm-5-current` profiles preserve the
  established `https://open.bigmodel.cn/api/paas/v4/chat/completions`
  endpoint. The newer `https://api.z.ai/api/paas/v4/chat/completions`
  endpoint is scoped to `z_ai:glm-5.3-flash`.
- OpenRouter's `z-ai/glm-5.3-flash` route is a distinct OpenRouter-backed
  provider path and is not materialized by this profile.
- Ollama's `glm-5.3-flash:cloud` route is a distinct Ollama-cloud transport
  choice and is not materialized by this profile.

Provider profiles are configuration contracts only. They do not authorize a
paid provider call, cloud mutation, credential disclosure, or provider-specific
acceptance claim.
