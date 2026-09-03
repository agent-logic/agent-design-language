# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/real_issue_canary.rs
csdlc-v3/tests/transactions.rs

## Prompts

- Verify remote authority is not caller-forgeable.
- Verify publication refuses stale or missing review truth.
- Verify closing publications produce visible and typed Closes #xxx linkage.
- Verify credentials are redacted and no raw gh lifecycle writes are used.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 GitHub and publication routes remain non-authoritative until explicit V3-F/#505 cutover.
- Live PR #641 closing relation was corrected to #629-only through typed v2 PR update; v3 now rejects body-derived unexpected closing references.
- V3 PR create/update writes remain incomplete; typed C-SDLC v2 remains publication authority until #505.

## Review Result

Revision: Some("git-blake3:c2f4ece7ca8bdffbd27afeee37464f41944f2539:5e13f407cd37a5d910e9141f843e2e6cbe81e4bd3fed42a6b364cdd0aec8634e")

Reviewer: Some("codex-reviewer:review_629_head_c2f4")

Result: pass
