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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
