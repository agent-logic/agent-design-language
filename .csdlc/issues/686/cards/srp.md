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
    "id": "r5-p1-interrupted-reload-recovery-blocked-by-config-preflight",
    "severity": "p1",
    "summary": "Start/reload run active configuration-generation validation before reconcile_interrupted_reload, so a crash after active init replacement but before active-generation ref activation can strand recovery behind a mismatched active ref.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "r5-p2-candidate-receipt-stored-outside-active-generation-store",
    "severity": "p2",
    "summary": "Reload provisions a candidate receipt beside the candidate file even though post-activation validation reads the receipt beside the active init path, so candidates from another directory cannot become valid active generations.",
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

Revision: Some("git-blake3:ff92652c4d1537d5d2557ab63f317a2a3a8b2372:48c290d1f395846260e577c1042c5e504fc85b739ced0bcfd6def1bc24493305")

Reviewer: Some("codex-subagent:/root/review_686_runtime_config_generation_r5")

Result: changes_required
