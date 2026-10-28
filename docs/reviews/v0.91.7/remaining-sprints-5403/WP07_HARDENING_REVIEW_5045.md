# WP-07 CSM Runtime Hardening Follow-On Review

Issue: #5045
Review issue: #5403
Status: blocked with findings
Remediation: #5408

## Findings

### P1: Emergency stop does not authenticate or authorize its caller

`adl/src/long_lived_agent.rs:1445` accepts any non-empty authorization string,
and line 1483 merely hashes that string for evidence. The positive case at
`adl/tests/cli_smoke/agent.rs:3957` uses an arbitrary ticket string; the
negative case at line 4114 only omits `--operator`.

Impact: anyone able to invoke the local command against an agent specification
can satisfy the governed-stop authorization requirement without presenting
verifiable authority.

Disposition: open. Route a #5005 security remediation that verifies caller
identity and authorization against a governed trust source, with forgery and
wrong-authority negative tests.

### P1: API Gateway `passed` proof omits required routes and failure behavior

The retained summary at
`docs/milestones/v0.91.7/review/runtime/csm_api_gateway_bridge_5039/live_20260710T004221Z/api_gateway_bridge_summary.json:23`
supports only `$default`; `/status`, `/health`, `/ready`, `/metrics`, `/events`,
and `/chronosense` remain planned. The sole positive call is
`/api-gateway-bridge` with readiness `unknown` around line 42. Around line 60,
only missing-token denial is exercised; throttling, malformed request, upstream
failure, and degraded-state behavior remain declarations.

Impact: the `passed` classification does not prove the remote runtime routes or
failure matrix required by #5039.

Disposition: open. Route a #5039 live-proof completion issue or downgrade the
packet to a bounded API Gateway connectivity/authentication smoke.

### P1: The sprint closed while its serial final gate remained blocked

`docs/milestones/v0.91.7/review/runtime/FINAL_CSM_RUNTIME_COHERENCE_GATE_4906.md:9`
records `blocked_with_evidence`. Line 74 requires integrated closure or explicit
operator-approved deferral before readiness consumption. The sprint register
preserves that unresolved boundary at
`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:99`.

Impact: umbrella closure cannot support final WP-07 readiness, because its own
serial completion gate did not pass and no approved release disposition is
retained.

Disposition: open. Keep #5045 closed only as a coordination wrapper; route and
resolve the #4906 release-readiness disposition before consuming WP-07 as
clean.

## Child Coverage

Reviewed #5005, #5042, #4977, #4979, #5003, #4985, #4974, #5040, #5039,
#5041, and final gate #4906. All declared children are live-closed through
merged PRs with successful required checks. The #5068-derived rearchitecture
wave was explicitly moved to WP-07A and is not credited to #5045.

Previously found and fixed defects, including #5005 CI escalation routing and
Clippy failures and #5041 readiness/documentation findings, are not counted
above.

## Validation And Limits

No tests or mutating commands were run during the read-only specialist pass.
Historical local SRP/SOR cards were unavailable after v1 sunset and were not
reconstructed from PR prose. All three findings above are review-discovered;
no test-discovered defect is counted above.

## Review Result

Blocked with findings. The emergency-stop authority defect and incomplete
remote bridge proof are active P1s, while the unresolved #4906 serial gate
prevents final WP-07 readiness consumption.
