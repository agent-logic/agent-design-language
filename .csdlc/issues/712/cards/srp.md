# Structured Review Prompt

Template: 1.0.0

Issue: 712

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/712/design.md
.csdlc/prepared/issues/712/diagram.mmd
.csdlc/prepared/issues/712/validate-runtime-startup-simplification.sh
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config_generation.rs
adl-runtime-kernel/src/continuity.rs
adl-runtime-kernel/src/live_continuity.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/continuity.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/parity_b_live_kernel.rs
adl-runtime-kernel/tests/production_acip_wss.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
adl-runtime/tests/guardian_cli.rs
adl-runtime/tests/runtime_guardian_lifecycle.rs
adl/src/cli/csm_runtime_v3_cmd.rs
adl/src/lib.rs
adl/tests/csm_runtime_v3_generation.rs
docs/tooling/START_CSM_RUNBOOK.md

## Prompts

- Is any receipt choreography still required for ordinary startup?
- Can invalid config bind a listener?
- Can reload lose the last known-good config or create two children?
- Did simplification preserve real API, ACIP, identity, replay, and audit security?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:4f7ea161331068d5142a922d05e68af78a85addf:81d9db568431c609e883c6115d4d8bc3cf5d608ba8ec438cf259763deaac88e1")

Reviewer: Some("codex:/root/review_712_startup_simplification")

Result: pass
