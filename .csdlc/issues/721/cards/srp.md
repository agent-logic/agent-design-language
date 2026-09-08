# Structured Review Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/operational_cli_commands.rs

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

Revision: Some("git-blake3:af5cf0c33cb4f66b148804f7cc5a7fdb531f98aa:77eaf334debd91e8fb4856f0684e4cd81a26e37e2c73dacd8bf570feb4abfd4a")

Reviewer: Some("/root/review_721_authority_binding_fix")

Result: pass
