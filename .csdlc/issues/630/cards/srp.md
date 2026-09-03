# Structured Review Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs

## Prompts

- Can any caller manufacture terminal, cleanup, or cutover authority without authenticated evidence?
- Does cleanup actually consult Git worktree registration and preserve distinct outcomes?
- Does cutover remain a non-authoritative decision packet before #505?
- Are all tests and validators exercising behavior instead of strings?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 terminal, cleanup, and cutover routes remain construction-only and non-authoritative until explicit V3-F/#505 cutover.
- Publication text must be refreshed to current head fef864bb6ae6bd58052cb9c2274ac4330346667d before readiness or merge claims.

## Review Result

Revision: Some("git-blake3:fef864bb6ae6bd58052cb9c2274ac4330346667d:96b647c2db01c45549870b7d040ebafa47986e0e5ed1e2e4387d60c25a494c2f")

Reviewer: Some("codex-reviewer:review_630_cleanup_receipts_fef864")

Result: pass
