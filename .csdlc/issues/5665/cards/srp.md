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
infra/runtime-v3/runtime-api-5665.toml
docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json
.csdlc/evidence/5665/runtime-v3-wss-focused.log
.csdlc/evidence/5665/runtime-v3-strict-clippy.log
.csdlc/evidence/5665/wp12-wrapper-tombstone-check.log
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
    "id": "P1-net-loc-reduction-unproven",
    "severity": "p1",
    "summary": "Net physical LoC reduction is not proved and appears unsatisfied for the full requested scope: the WP-12 wrapper dropped from 241 to 13 lines, but the issue branch remains net positive from main once the Runtime API and proof surface are included.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "P2-review-scope-omitted-lifecycle-index",
    "severity": "p2",
    "summary": "The initially assigned Locke review scope omitted .csdlc/issues/5665/index.json even though the review was asked to cover lifecycle state.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "Superseded by this direct csdlc-review record, whose scope includes .csdlc/issues/5665/index.json."
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The strict Clippy retained log is terse and records Cargo success rather than echoing the full command; the command was rerun and the typed SOR records the argv.
- #5663 and #5664 protected-path records were not visible in this worktree; visible #5657 protected paths were disjoint from the changed paths.

## Review Result

Revision: Some("git-blake3:37d4f5555d38f5e958c40990cef42e1e569bb7ce:61865b3d6d2e14ae70ec54d511ed2a3347fd0a742a1aba20cfa6becc7d6b1670")

Reviewer: Some("Locke")

Result: changes_required
