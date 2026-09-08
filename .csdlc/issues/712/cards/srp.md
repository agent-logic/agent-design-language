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
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/config_generation.rs
adl-runtime-kernel/src/continuity.rs
adl-runtime-kernel/src/live_continuity.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/continuity.rs
adl-runtime-kernel/tests/control.rs
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

- No live Runtime process or paid provider execution was performed in this exact-head review; those surfaces were outside the assigned immutable local review scope.

## Review Result

Revision: Some("git-blake3:e0d80849f65503e3e24354586c78530ad816af96:41af4cd2f194ded5c1963a107f071babcc1125c5f7ff0cd71b6cb4e13dadde1f")

Reviewer: Some("codex:/root/review_716_final2")

Result: pass
