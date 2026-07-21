# Structured Review Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5590/audit.jsonl
.csdlc/issues/5590/index.json
.csdlc/prepared/issues/5590/run_filtered_test_lane.sh
.csdlc/prepared/issues/5590/run_operational_selector_transition.sh
.csdlc/prepared/issues/5590/transition-guardian-binary-claim.json
.csdlc/prepared/issues/5590/transition-implementation-claim.json
.csdlc/prepared/issues/5590/transition-lockfile-claim.json
.csdlc/prepared/issues/5590/transition-operational-proof-claim.json
.csdlc/prepared/issues/5590/transition-websocket-dependency-claim.json
adl-runtime-kernel/Cargo.lock
adl-runtime-kernel/Cargo.toml
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
adl/tools/run_runtime_v3_operational_proof.sh
adl/tools/runtime_v3_operational_selector.sh
adl/tools/test_runtime_v3_operational_selector.sh
demos/v0.91.7/html-observatory/app.js

## Prompts

- Does one init model and one Axum/rustls router truthfully cover local and remote access without hard-coded addresses or HTTP?
- Do HTTP and WebSocket Observatory paths share authentication, origin, authority, frame, redaction, and live-state contracts?
- Does discovery report the actual listener and configured public HTTPS base for default, non-default, and ephemeral ports?
- Does the external guardian distinguish intentional stop, invalid config, bounded retry, pressure serialization, and checkpoint restore without sidecars?
- Does Vector own collection/export while Runtime stderr, health, control, and shutdown survive collector absence?
- Is rollback explicit, reviewed, evidence-preserving, and free of Runtime v2 source edits, automatic cutover, AWS, or deployment claims?
- Do S1 through S6 and all lanes cover AC-1 through AC-8 with no deferred or fixture-only parity credit?

## Findings

[
  {
    "id": "selector-process-ownership",
    "severity": "p1",
    "summary": "The operational selector trusts a reusable bare PID and lacks serialization across stop, launch, and state replacement.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "selector-descendant-shutdown",
    "severity": "p1",
    "summary": "Forced selector shutdown kills only the guardian PID and can bypass guardian descendant cleanup, orphaning the kernel.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "websocket-token-revocation",
    "severity": "p1",
    "summary": "An authenticated Observatory WebSocket remains authorized after bearer-token rotation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "continuity-proof-overclaim",
    "severity": "p2",
    "summary": "The operational proof reports signed continuity without exercising cryptographic restore verification, key identity, integrity, and lineage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "proof-cleanup-liveness",
    "severity": "p2",
    "summary": "Failed-proof cleanup can remove the proof tree before guardian and descendant termination is confirmed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "observatory-stale-close-state",
    "severity": "p2",
    "summary": "The browser does not downgrade live Observatory status after a clean or policy WebSocket close.",
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

- control.rs is 1,248 lines after the issue delta and should be watched for future decomposition; no additional module growth is authorized by this review.

## Review Result

Revision: Some("git-blake3:680f5908818c353f8d7df054ad9a87884adbac0f:c11c9725081bd67bf9b48c0ad08ab9ce6007a1a620c2918934d5236fe8852dbc")

Reviewer: Some("subagent:019f8692-79df-7fe0-98bd-8d42df9b5f1a")

Result: changes_required
