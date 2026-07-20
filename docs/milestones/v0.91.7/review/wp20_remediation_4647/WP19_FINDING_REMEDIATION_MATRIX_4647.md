# WP-20 Remediation Matrix for WP-19 Findings (#4647)

Status: in_progress_remediated_locally

Issue: #4647

Source register: `docs/milestones/v0.91.7/review/external_review_4646/FINDINGS_REGISTER.md`

Exact retained verification artifact:
`docs/milestones/v0.91.7/review/V0917_EXTERNAL_REVIEW_VERIFICATION_2026-07-19.md`

## Disposition Matrix

| Finding | Disposition | Remediation / Evidence |
| --- | --- | --- |
| WP19-01 | fixed | `adl/tools/run_authoritative_coverage_lane.sh` isolates `CARGO_LLVM_COV_TARGET_DIR` by validated run id and keeps sibling run profiles out of cleanup scope. `adl/tools/test_run_authoritative_coverage_lane.sh` proves sibling profile survival. |
| WP19-02 | fixed | `AwsBedrockProvider` now requires `ADL_AWS_BEDROCK_ACCOUNT_SHA256` or `config.expected_account_sha256`, compares it to the STS account hash before invocation, and records `account_hash_verified` instead of false `sts_verified`. |
| WP19-03 | fixed | Runtime API auth-event write failures now return `500 Internal Server Error` instead of being discarded; `runtime_api_auth_event_write_failure_fails_closed` proves the policy. |
| WP19-04 | fixed | Runtime API redaction now includes cloud/account-id key variants and the redaction regression includes `account_id`, `aws_account`, and `cloud_account_identifier`. |
| WP19-05 | fixed | The authoritative coverage runner captures partition failure status, attempts workspace/runtime summary reports, then exits with the recorded failure. The fake-cargo regression injects a partition failure and verifies both reports are attempted. |
| WP19-06 | fixed | Local Ollama streaming now buffers partial UTF-8 before invoking stream callbacks and drains malformed bytes with one replacement so later valid chunks are not blocked. `ollama_streaming_buffers_split_multibyte_utf8` proves split multibyte output and invalid-byte recovery. |
| WP19-07 | fixed | The #5571 audit remains historical. Current replacement dispatch authority is `external_review_4646/REVIEW_CORPUS.v1.txt` plus `PUBLICATION_SAFE_MANIFEST.md`, which limits publication to the replacement corpus. |
| WP19-08 | fixed | Provider endpoint validation parses IPv4/IPv6 loopback hosts with `IpAddr::is_loopback`; bracketed `[::1]` bearer endpoint coverage was added to `http_family` tests. |
| WP19-09 | fixed | `V0917_SPRINT_REVIEW_REGISTER.md` no longer directs operators to resolve already-closed #5406 and now consumes the closed terminal evidence. |
| WP19-10 | fixed | Provider invocation artifact locks now write lease metadata, recover stale lock directories, and classify post-provider lock acquisition failure as non-retryable partial-success to avoid duplicate billable retries. |
| WP19-11 | fixed | `adl/src/provider/http_family.rs` maps to the `provider_hardening` coverage-impact lane with a regression in `test_check_coverage_impact.sh`. |
| WP19-12 | fixed | The v0.91.8 review handoff digest procedure now hashes sorted `git ls-tree` object records rather than filenames only. |
| WP19-13 | fixed | `WP_ISSUE_WAVE_v0.91.7.yaml` now records WP-21A as closed in both summary and detail truth; YAML parse passed. |
| WP19-14 | fixed_by_current_code | `resolve_main_runtime_api_listener` parses and rejects non-loopback binds before `TcpListener::bind`; `validate_loopback_bind` remains as post-bind defense. Existing networking/runtime tests cover the pre-bind loopback gate. |
| WP19-15 | fixed | Authenticated GET now computes one identity-aware runtime API response through `runtime_api_get_response` instead of building and replacing a second projection. |
| WP19-16 | fixed | `exact_pid_is_live` now returns `unknown` for EPERM/unattributable liveness rather than treating it as live. |
| WP19-17 | fixed | Unauthenticated OPTIONS responses return before `record_request` and omit `x-csm-admission`, so they no longer consume bounded shutdown/test request budget or disclose admission state. |
| WP19-18 | fixed | `emit_runtime_api_client_error` now routes through the existing sanitizer before separator masking. |
| WP19-19 | fixed | `runtime_api_axum_response` forces HTTP 500 when serialization fallback is used. |
| WP19-20 | fixed | `PUBLICATION_SAFE_MANIFEST.md` now says no operator-specific paths and explicitly declares synthetic fixture exceptions. |
| WP19-21 | fixed | `RELEASE_TRUTH_GATE_STATUS_5544.md` now self-identifies as a historical snapshot superseded by the current sprint review register. |
| WP19-22 | fixed | v0.91.8 setup docs retain #5383 as closed historical setup authority; `setup/5383/DIAGRAM.mmd` no longer presents #5383 as pending. |

## Focused Validation Run

- `cargo fmt`
- `bash adl/tools/test_run_authoritative_coverage_lane.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_check_coverage_impact.sh`
- `cargo test -p adl provider::http_family::tests:: -- --test-threads=1 --nocapture`
- `cargo test -p adl provider::local::tests::ollama_streaming_buffers_split_multibyte_utf8 -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_options_does_not_consume_test_request_budget -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_redacts_secret_and_host_path_event_payloads -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_auth_event_write_failure_fails_closed -- --nocapture`
- `ruby -e 'require "yaml"; require "date"; YAML.safe_load(File.read("docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml"), permitted_classes: [Date], aliases: true); puts "yaml-ok"'`

## Non-Claims

- No AWS operation was performed.
- WP-23 remains blocked until WP-20 publication, exact-revision review, CI, and truthful closeout complete.
- This matrix records local remediation state only until the PR exact head is reviewed and merged.
