# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/csdlc-v3/v3-command-manifest.json

## Prompts

- Can any route claim proof, parity, soak, or install success without durable bounded evidence?
- Does shadow refuse broad equivalence and report exact mismatches?
- Does install plan a stable one-binary artifact without selector mutation before #505?
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
