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
    "id": "r8-p2-denominator-token-presence-overclaim",
    "severity": "p2",
    "summary": "The #686 denominator lane only checks loose literal token presence while the SOR claims it proves the configuration-generation handoff/recovery contract. Tighten the denominator or narrow the validation claim before publication.",
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

- Publication remains blocked until the denominator proof/claim mismatch is fixed and freshly reviewed.

## Review Result

Revision: Some("git-blake3:9438b58bb5eb5950bb693666358c6bb4f93fdf0c:21ace91b6be9b6bc4b1b415c02c68df3d6b6e666551240bad86815eb2f65a1bd")

Reviewer: Some("codex-subagent:/root/review_686_runtime_config_generation_r8")

Result: changes_required
