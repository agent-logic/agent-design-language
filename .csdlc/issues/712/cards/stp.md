# Structured Task Prompt

Template: 1.0.0

Issue: 712

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remove cross-binary startup receipt choreography and replace it with direct config validation, loaded-hash readiness, and atomic reload.

## Deliverables

- Simplified CSM start/reload path
- Single-child Guardian launch path
- Kernel direct validation and loaded hash reporting
- Focused cold-start/restart/reload proof
- Updated runbook
- Reviewed PR

## Acceptance

1. AC-1: Cold start requires only the canonical config and canonical Runtime binary
2. AC-2: Guardian launches exactly one Runtime child and supervises restart
3. AC-3: Kernel validates before serving and reports the exact loaded config hash
4. AC-4: Reload atomically installs a valid candidate or leaves prior config and process intact
5. AC-5: No prepared receipt handshake remains across CSM, Guardian, and Kernel
6. AC-6: Existing roster, model-backed Shepherd, all-agent A2A, API auth, signed ACIP, observability, and checkpoints remain intact
7. AC-7: Focused proof, live Wuji acceptance, review, and publication pass

## Dependencies

- #707 implementation and live proof
- Canonical com.agentlogic.adl-runtime-v3 service

## Inputs

- agent-logic/agent-design-language#712
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- docs/tooling/START_CSM_RUNBOOK.md

## Non Goals

- No new orchestration framework
- No compatibility copy of the receipt protocol
- No cloud-specific Runtime fork
- No unrelated provider or UI work
