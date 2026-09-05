# Structured Review Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/config_generation.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
adl/src/cli/csm_runtime_v3_cmd.rs
adl/tests/csm_runtime_v3_generation.rs
.csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py

## Prompts

- Can any partial candidate or mutable file become authority without a committed receipt?
- Do CSM Guardian kernel status and readiness prove identical generation and digest?
- Can a secret value enter any durable or observable surface?
- Does every named failpoint recover deterministically to either the candidate commit or prior generation?
- Does validation avoid the live Runtime?

## Findings

[
  {
    "id": "r4-p1-kernel-env-not-bound-to-active-receipt",
    "severity": "p1",
    "summary": "Direct kernel startup validates configuration-generation env presence and hex shape but does not compare the supplied generation/digest to validate_active_config_generation for the active init and binary generation before exposing readiness/status.",
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

- none

## Review Result

Revision: Some("git-blake3:040d3ac82450086030fd3f6c8b034e24dd5546b5:84273a639a9f0cb7c30930e098d3d498332199ffba40e73948ab1c3afaaf58eb")

Reviewer: Some("codex-subagent:/root/review_686_runtime_config_generation_r4")

Result: changes_required
