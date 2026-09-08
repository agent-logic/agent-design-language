# Structured Review Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/721/**
.csdlc/evidence/v3-defects/backlog-requests/**
csdlc-v3/README.md
csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/milestones/v0.92.1/evidence/csdlc-v3/issue-721/**

## Prompts

- Review whether issue-create can duplicate live issues under retry, timeout, or marker search failure.
- Review whether terminal authority reporting is truthful only when canonical v3 authority actually allowed persistence.
- Review whether docs expose a real command path without suggesting raw gh lifecycle writes.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Broad csdlc-v3 all-target proof/shadow/install failures remain outside #721 scope and are tracked by #723.

## Review Result

Revision: Some("git-blake3:82bd07886f83197694288eb2d93164a462cb8bb3:b92fd5a1b3190fbed16d883282cb1ffaeb6664f369ef61fe12158cf8a487108f")

Reviewer: Some("fresh-session:/root/review_721_retry")

Result: pass
