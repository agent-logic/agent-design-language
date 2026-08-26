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

Provider profiles are configuration contracts only. They do not authorize a
paid provider call, cloud mutation, credential disclosure, or provider-specific
acceptance claim.
