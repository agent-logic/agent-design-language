# Structured Intent Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Resolve the four current V3-F review-freshness gaps with independent exact-head review and full detached locked suite proof.

## Required Outcome

V3-F-ac-1 through V3-F-ac-4 bind the same immutable reviewed source blobs and passing suite receipt; stale substitutions fail.

## Scope

- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
- docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py
- docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current
- .csdlc/evidence/771
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/commands/remote/tests.rs

## Authority

- Native v3 default; operator authorized typed v2 for #771 after defect #776.

## Assumptions

- none

## Operator Constraints

- Keep main inspection-only. Work beneath FastWork policy parent. Preserve other sessions.
