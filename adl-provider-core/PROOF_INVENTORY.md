# Provider core proof inventory — issue #855

This package owns provider transport/DTO/profile validation proof. It has no ADL
or Runtime package dependency. ADL document traversal and the #876 watcher remain
in ADL; Runtime lifecycle, persistence and watcher ownership remain in kernel.

## Local unit denominator and PVF classification

The extraction preserves 79 existing provider/model/substrate unit tests in this
leaf. Eleven new tests bring the Unix leaf denominator to 90 tests. Ten ADL document/
remote-retry compatibility tests, eight #876 reload tests and one ADL substrate
manifest test remain in their existing ADL modules. Counts describe selected
source tests, not paid-provider executions or installed Runtime proof.

All eleven new tests are PVF local-contract/transport proofs: deterministic
fixture input, bounded CPU/memory, loopback HTTP only where transport is needed,
no external network or paid model calls, no production credentials. They are
release-required for this extraction and provider-boundary change. Required
loopback fixtures fail if socket binding is unavailable; they do not silently
report skipped transport as passing coverage.

| New test (unique suffix) | Proof role | Resource profile |
| --- | --- | --- |
| `complete_map_validation_rejects_entire_candidate_and_redacts_input` | Complete replacement map, malformed/credential rejection, redacted failure | CPU only |
| `explicit_openai_chat_format_uses_existing_adapter_payload_and_parser` | Existing generic adapter's explicit chat wire format and generated output | One loopback request |
| `native_http_failure_categories_never_include_remote_body` | Typed credential/quota/model/transport categories; no remote-body disclosure | Five loopback requests |
| `adapter_does_not_follow_endpoint_redirects` | No redirect-based endpoint-policy bypass | One loopback request |
| `ollama_metadata_verification_checks_installed_model_without_generation` | Installed-model metadata success/missing-model failure; no inference | Two loopback GET requests, 1 MiB/10 s adapter limits |
| `explicit_chat_format_allows_only_numeric_private_plaintext_targets` | Private numeric local HTTP accepted; public/unchecked host rejected | Constructor only, no request |
| `five_adapter_transport_matrix_serializes_generates_and_classifies_failures` | Five real adapter implementations: OpenAI, Anthropic, Vertex Google, Ollama, OpenAI-compatible; request body, generated text, quota and invalid-response categories, actual 256-token wire limits | Fifteen loopback requests, 5 s per request |

The five-provider matrix invokes `build_provider_for_id` and the existing
transport implementations. It supplies a synthetic fixture token in one private
test environment variable, restores that variable and the invocation-artifact
setting, and never consults credential files or ADC. The fixture supplies Vertex
project/location fields explicitly; no project or cloud location is inferred.

Additional bounded-call/trust tests use the same release-required PVF classification:

| New test (unique suffix) | Proof role | Resource profile |
| --- | --- | --- |
| `runtime_bedrock_sets_one_attempt_and_rejects_external_credential_process_chains` | SDK retry configuration, unsupported external credential process/source chain rejection | Fake SDK profile filesystem only |
| `runtime_limits_are_opt_in_strict_and_do_not_raise_existing_caps` | Explicit positive upper bounds, malformed configuration rejection, unchanged absent-field defaults | CPU only |
| `authentication_capture_bounds_time_output_and_redacts_errors` | Authentication subprocess timeout, kill/reap and 64 KiB capture limit | Unix local subprocesses; 50 ms timeout fixture |
| `operator_ca_errors_are_bounded_and_redacted` | Missing, oversized and malformed process-owner CA source rejects without paths/content | Temporary public-test input files; no network |

## Runtime limits and operator trust

`config.runtime_max_attempts: 1` opts into Runtime's bounded-call policy. It
disables reqwest automatic retries and configures AWS SDK one-attempt operation
policy. Runtime Bedrock bounds the overall authentication/request duration and
identity loading; it rejects selected/source-profile `credential_process`
because the SDK cannot terminate that subprocess on timeout. Ordinary ADL
behavior without this setting is preserved. Runtime Vertex ADC uses a 10-second
bounded token command with a 64 KiB in-memory output limit and discarded stderr;
on Unix its isolated process group is killed and the child reaped on failure.
No real credentials or subprocess output are written to proof artifacts.

`config.runtime_max_output_tokens` is an optional integer from 1 through 32768.
It is an upper bound, so a smaller pre-existing native output limit wins. The
five-provider matrix sets normal limits to 1024 and Runtime's cap to 256, then
asserts **the captured request fields**, not merely in-memory configuration:

| Adapter | Serialized cap field |
| --- | --- |
| OpenAI | `max_output_tokens` |
| Anthropic | `max_tokens` |
| Vertex Google | `generationConfig.maxOutputTokens` |
| Ollama HTTP | `options.num_predict` |
| OpenAI-compatible chat | `max_tokens` |

Generic legacy `{prompt}` transport rejects a Runtime output cap because it has
no declared token-limit wire contract. Omitting the Runtime cap preserves legacy
payloads. Native DeepSeek/Kimi/OpenRouter/Bedrock/Z.ai use the same cap helper.
These are output limits; this package does not claim generic input tokenization.

`ADL_PROVIDER_CA_FILE` is an optional process-owner PEM trust bundle, limited to
64 KiB. It appends trusted roots to all HTTP-family reqwest clients and never disables
certificate verification or redirects. Provider/agent configuration cannot set
this trust source. Failure messages omit the path and PEM contents. The installed
HTTPS harness must separately prove successful custom-CA TLS handshakes; the leaf
unit test proves the bounded rejection cases without mutating global trust.

## Commands and evidence boundaries

Run from the issue worktree with its trusted warmed target:

```sh
CARGO_TARGET_DIR=adl/target cargo test --manifest-path adl-provider-core/Cargo.toml --locked --offline --lib
CARGO_TARGET_DIR=adl/target cargo test --manifest-path adl/Cargo.toml --locked --offline --lib provider::
CARGO_TARGET_DIR=adl/target cargo test --manifest-path adl/Cargo.toml --locked --offline --lib provider_substrate::
```

The selected leaf matrix proves adapter serialization and handling of controlled
responses. It does not prove hosted credentials, vendor availability, provider
billing, installed binary composition, dynamic Runtime restoration/removal,
operator/A2A routing, or release CI selection. Those remain separate #855
integration and demonstration gates. CI/path inventories must explicitly select
this new leaf package; an ADL test invocation alone does not execute leaf tests.

Runtime registry capability declarations deliberately do not infer tools,
streaming, token accounting or health probes from model names/profile claims.
Ollama HTTP alone provides installed-model metadata validation here. Other
adapters report configuration validation without claiming model availability.

## Runtime registry and lifecycle proof

`adl-runtime-kernel/src/control/provider_registry_tests.rs` contains eight bounded,
deterministic, release-required provider-lane tests. Their resource profile is
local CPU/disk with generated fixture text and no network or paid calls:

- Sixth adapter admission, metadata-only refresh, generated helper execution,
  replacement, checkpoint, migration digest rejection, removal and rehydration.
  This test alone does not prove the signed conversation or installed CLI path.
- Unknown adapter, unsupported required capability and malformed credential
  reference rejection before admission or completion side effects.
- Removed named definitions fail closed; conflicting stable/native model mapping
  rejects; already prepared calls retain their old generation.
- Metadata-only refresh does not erase an observed inference failure.
- Pre-cancelled registry dispatch issues no completion and preserves prior
  generated readiness when accounting the cancellation.
- Demo full-prompt byte, process-lifetime call and stop-after-failure caps reject
  before another completion; re-preparing a binding does not reset those caps.

`control::layer8_conversation_ingress_tests::sixth_registered_provider_uses_real_canonical_a2a_dispatch`
is a separate release-required deterministic test through actual ControlService
conversation admission, canonical ingress, production operation executor, signed
agent-to-agent delivery, recipient generation and initiating-agent continuation.
It asserts canonical peer identity and exactly three registered-adapter calls.
The existing affected control suite also exercises actual in-flight operator
cancellation versus an admitted execution deadline through the new registry.

Run `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib control::`.
The five-vendor installed binary/CLI matrix and separately authorized hosted
execution remain distinct from these deterministic library proofs. CI runs the
leaf tests directly: testing the ADL dependent package does not execute its
provider-core dependency's unit tests.

The `registered_provider_contract_has_no_vendor_admission_allowlist` OpenAPI test is a release-required, deterministic CPU-only API contract proof: provider-neutral admission, reference-only credentials, and redacted capability/health projection. The optional bounded demonstration envelope rejects concurrent same-provider dispatch without queuing, counts complete prompts, and stops future dispatch after transport or invalid-output failure. It does not actively cancel an already dispatched HTTP call.

The real signed sixth-provider dispatch regression also proves failed inference → authenticated operator retry → generated success → restored A2A eligibility, and first-failure demo budget refusal without another transport call. Health polling never performs this retry. `named_local_http_definition_admits_without_hosted_vendor_policy` preserves #876 named generic HTTP definitions through shared candidate validation and actual ControlService admission; CPU-only numeric-loopback construction, no generated request. Malformed or incomplete optional budget keys fail closed.

`metadata_publication_preserves_newer_inference_and_replacement` is a deterministic, release-required blocked-metadata ordering test (bounded local threads, no network). It proves current success/failure wins over stale asynchronous checks and replacement identity cannot be overwritten. The signed sixth-provider test also rejects malformed declared actions before success accounting or signed dispatch. Continuations normalize the same protocol and reject a second action; semantic failures trip any configured first-failure demonstration stop.

## Runtime local CLI review-fix proof

Four new `provider::local::tests::runtime_cli_*` tests raise the observed Unix
leaf denominator from 90 to 94 (94 passed, zero failures/ignored/filtered).
All four are deterministic, release-required provider-lane regression proofs.
Their resource profile is bounded local CPU, pipes, temporary files and fake
subprocesses; no Ollama installation, model, credentials, network or paid calls.

- Constructor proof preserves the public legacy Ollama struct literal and
  non-Runtime factory behavior, enforces Runtime timeout configuration, and
  rejects a declared token-output cap with `unsupported_capability`.
- Stalled-stdin proof supplies 1 MiB (larger than pipe capacity) to a subprocess
  that never reads it and observes the 100 ms deadline within a 2 s upper bound.
- Descendant proof observes timeout after the group leader exits, checks that
  the leader is reaped, and proves the background child cannot create its marker.
- Pipe/output proof round-trips UTF-8 input, rejects output above the production
  4 MiB limit, and checks that failed-child stderr is absent from diagnostics.

Focused command: `CARGO_TARGET_DIR=adl/target cargo test --manifest-path adl-provider-core/Cargo.toml --locked --offline --lib provider::local::tests::runtime_cli` (4/4 passed).
Compatibility command: `CARGO_TARGET_DIR=adl/target cargo test --manifest-path adl-provider-core/Cargo.toml --locked --offline --lib` (94/94 passed).
These library proofs establish Runtime CLI supervision on Unix and preserved
legacy construction, not installed-binary or hosted inference qualification.

The existing Vertex thinking configuration test now accepts documented `thinking_budget: 0` (disabled thinking) and still rejects malformed/negative values. The five-adapter transport matrix asserts actual Vertex `thinkingBudget: 0` together with the 256-token output cap. Model support follows the provider contract: https://docs.cloud.google.com/vertex-ai/generative-ai/docs/thinking . This supports the approved Gemini 2.5 Flash demonstration without expanding its model or spending bounds. Final post-fix local leaf run: 94/94 and all-targets Clippy pass.
