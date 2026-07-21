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
    "summary": "The selector now serializes transitions and routes stop through instance-owned supervisor state rather than externally signaling a stored PID.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  },
  {
    "id": "selector-descendant-shutdown",
    "severity": "p1",
    "summary": "Replacement waits for a stopped receipt emitted only after guardian exit and descendant cleanup; timeout fails closed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  },
  {
    "id": "websocket-token-revocation",
    "severity": "p1",
    "summary": "Authenticated sessions revalidate credentials on every refresh and close after rotation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  },
  {
    "id": "continuity-proof-overclaim",
    "severity": "p2",
    "summary": "The proof restarts candidate and prior runtimes through the cryptographic restore path and verifies generation-2 key, integrity, and lineage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  },
  {
    "id": "proof-cleanup-liveness",
    "severity": "p2",
    "summary": "Proof cleanup uses confirmed selector shutdown and preserves the proof directory on unconfirmed termination.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  },
  {
    "id": "observatory-stale-close-state",
    "severity": "p2",
    "summary": "The browser handles close events, clears matching socket identity, and downgrades live state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- A selector killed with SIGKILL can leave a stale lock requiring explicit operator recovery; the stale lock fails closed and cannot launch a competing Runtime.
- Runtime v3 is 12,842 physical lines, a reviewed necessary and nonduplicative +159 delta over the prior 12,683 reviewed point and +633 over the pinned 12,209 baseline.

## Review Result

Revision: Some("git-blake3:0b5280459a97427cfef8fe478f532b2842f6c2bd:a7805563e9e143a5a5d47162b508742e30374146a503cec3796e8c4d68b9e08a")

Reviewer: Some("subagent:019f8692-79df-7fe0-98bd-8d42df9b5f1a")

Result: pass
