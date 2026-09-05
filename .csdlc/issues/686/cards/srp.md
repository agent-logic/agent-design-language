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
    "id": "r3-p1-csm-readiness-config-generation-not-authoritative",
    "severity": "p1",
    "summary": "CSM readiness/status validation accepts arbitrary 64-hex config_generation and config_receipt_digest values without comparing them to validate_active_config_generation for the active init and current binary generation.",
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

Revision: Some("git-blake3:0452ae5019effeee12cd4109b79ed4bb840ca6d4:12418d7c26b6df545b796332d677ef4d235e3edffe567b306677150cf26e1e54")

Reviewer: Some("codex-subagent:/root/review_686_runtime_config_generation_r3")

Result: changes_required
