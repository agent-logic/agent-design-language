# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/real_issue_canary.rs

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
- This change binds PR title into authenticated v3 PR readback receipts; it does not make v3 perform PR title writes before cutover.

## Review Result

Revision: Some("git-blake3:6e959dc51ee7e33b0ed5233fa8a12056eba5c362:e2c7218d67b81ea1fe6ab1e26d4c1585151bcf914eaa76449a8e2996eee760d9")

Reviewer: Some("codex-reviewer:review_629_title_readback")

Result: pass
