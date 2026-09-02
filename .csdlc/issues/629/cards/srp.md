# Structured Review Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/remote_publication_commands.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/629

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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
