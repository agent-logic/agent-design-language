# Structured Review Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/remote_publication_commands.rs
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

- C-SDLC v3 finish, clean, and cutover remain construction-only until explicit V3-F/#505 cutover; no live terminal authority is granted by this issue.
- Live PR publication and GitHub closeout still require typed v2 publication, PR-state verification, merge/finish, and cleanup after this review.

## Review Result

Revision: Some("git-blake3:9b6f08bfaa28651d2f632418fe409477b2975d97:da90166c7a28bcd39a34b376384085215bdfe1809ba2b6f15d6e9e7826a3388e")

Reviewer: Some("codex-reviewer:review_630_terminal_guard_fix")

Result: pass
