# Structured Review Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/721/**
.csdlc/evidence/v3-defects/backlog-requests/**
csdlc-v3/README.md
csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/proof_parity_install_commands.rs
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

- none

## Review Result

Revision: Some("git-blake3:375067551b44afdcdc3aef19e1bf1c25632424be:abbb991f5815724a47dee4cdcfd48753753a9a81b084078d1273433b60e7baae")

Reviewer: Some("/root/review_721_ci_repair_retry")

Result: pass
