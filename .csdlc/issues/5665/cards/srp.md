# Structured Review Prompt

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/runtime_api.rs
adl-runtime/tests/runtime_api_wss.rs
adl-runtime/Cargo.toml
adl-runtime/Cargo.lock
adl/Cargo.lock
adl/src/bin/run_wp12_acip_websocket_transport_proof.rs
adl/src/bin/run_v0916_acip_aee_memory_integration.rs
adl/src/bin/run_v0916_integrated_runtime_soak.rs
adl/src/bin/run_v0916_runtime_failure_injection.rs
adl/src/bin/run_v0917_integrated_resilience_failure_injection.rs
infra/runtime-v3/runtime-api-5665.toml
docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json
.csdlc/evidence/5665/runtime-v3-wss-focused.log
.csdlc/evidence/5665/runtime-v3-strict-clippy.log
.csdlc/evidence/5665/wp12-wrapper-tombstone-check.log
.csdlc/evidence/5665/runtime-v2-wrapper-tombstone-checks.log
.csdlc/evidence/5665/runtime-v3-loc-measurement.md
.csdlc/issues/5665/index.json

## Prompts

- Does the WSS proof exercise a real Axum/Tokio/Rustls API path rather than URL, fixture, or metadata proof?
- Are authentication, bidirectional frames, token rotation, token revocation, and shutdown covered?
- Are Observatory health states and telemetry fields truthful and sink-bounded?
- Does the feature/adapter matrix avoid unresolved claimed features?
- Did the change stay disjoint from #5657/#5663/#5664 protected paths and preserve the API-only runtime boundary?

## Findings

[
  {
    "id": "P2-feature-matrix-proof-uses-test-local-subset",
    "severity": "p2",
    "summary": "The committed feature/adapter matrix claims health-state and telemetry rows, but the WSS feature_matrix proof served only a test-local subset and never parsed or compared the committed JSON artifact.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- #5663 and #5664 issue records were not present in this commit, so protected-path disjointness for those is limited to visible repo evidence; #5657 visible protected paths are disjoint.

## Review Result

Revision: Some("git-blake3:a1cd93f29b80669bea015b00b88178dcf3bc60c8:87bbf819caab77f31c1de855e4d4935c8235c47b1f85dc4ca071f2e3ccc14e1d")

Reviewer: Some("Noether")

Result: changes_required
