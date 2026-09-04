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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
