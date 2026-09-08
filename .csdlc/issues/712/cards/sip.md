# Structured Intent Prompt

Template: 1.0.0

Issue: 712

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime startup Apache-simple and reliable by removing the cross-binary prepared-receipt protocol.

## Required Outcome

One canonical config and one stable Runtime binary start under one Guardian supervisor; Kernel validates and reports the loaded config hash; reload is atomic.

## Scope

- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config_generation.rs
- focused startup and reload tests
- docs/tooling/START_CSM_RUNBOOK.md

## Authority

- Issue #712 owns Runtime startup and reload simplification
- Security at authenticated API, signed ACIP, agent identity, replay, and audit boundaries remains intact
- No provider, Observatory, checkpoint, or C-SDLC redesign

## Assumptions

- none

## Operator Constraints

- One canonical config file
- One canonical Runtime binary with a stable name
- Exactly one Guardian-supervised Runtime child
- No prepared receipt handshake
- Never write tracked implementation on main
- Preserve live Wuji rollback until acceptance passes
