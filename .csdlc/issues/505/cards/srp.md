# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md
docs/csdlc-v3/authority-transition-disposition.json
.csdlc/evidence/505/pre-cutover-rollback-exercise.json
.csdlc/evidence/505/terminal-finish-canary-issue-629-pr641-output.json
.csdlc/evidence/505/terminal-clean-canary-issue-629-pr641-preview-output.json
.csdlc/evidence/505/terminal-clean-canary-issue-629-pr641-removal-denied-output.json
.csdlc/evidence/505/cutover-approval-absent-canary-output.json
.csdlc/prepared/issues/505/validate-authority-transition-prep.rb

## Prompts

- Verify #505 remains pre-bind preparation only until #504 is terminal, reconciled, and ancestral.
- Verify the packet preserves C-SDLC v2 live authority and rejects silent v2 retirement before explicit operator approval.
- Verify requirements #179 and #180 are named in the acceptance denominator and future proof plan.
- Verify the future PR body requirement visibly uses `Closes #505`.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains non-authoritative until explicit #505 operator approval, merge, typed finish, and cleanup reconciliation.
- C-SDLC v2 remains the live lifecycle and rollback authority until that cutover completes.

## Review Result

Revision: Some("git-blake3:4b216568c6674f99641dfffdac99c674e4a294c2:4b3dddb4871d07b7ad98888a0f8091c40e48b0924062ccb068594ccd37113c00")

Reviewer: Some("subagent:/root/review_505_canaries")

Result: pass
