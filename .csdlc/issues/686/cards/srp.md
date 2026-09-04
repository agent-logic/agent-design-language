# Structured Review Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
.csdlc/evidence/686

## Prompts

- Can any partial candidate or mutable file become authority without a committed receipt?
- Do CSM Guardian kernel status and readiness prove identical generation and digest?
- Can a secret value enter any durable or observable surface?
- Does every named failpoint recover deterministically to either the candidate commit or prior generation?
- Does validation avoid the live Runtime?

## Findings

[
  {
    "id": "r1-p1-stale-retained-evidence",
    "severity": "p1",
    "summary": "Retained .csdlc/evidence/686 logs record sha=30a1bb05e3ac7c80d20010d8cb3a6207ce0e0cf6 instead of the reviewed exact implementation head 8f770c0c3e820eee81d206ff3a284cfbbc247236.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "r1-p1-secret-path-values-in-receipt",
    "severity": "p1",
    "summary": "ConfigGenerationReceipt.secret_references stores actual _path values, and the focused test blesses /secret/runtime/control.pub.key, contradicting AC-4's requirement that secret values not enter receipts or retained evidence.",
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

Revision: Some("git-blake3:8f770c0c3e820eee81d206ff3a284cfbbc247236:3f650a56663e4c5d114e948f6f5365a0e75d1a5d9b7698e8ebd1e78ba9221772")

Reviewer: Some("codex-subagent:/root/review_686_runtime_config_generation")

Result: changes_required
