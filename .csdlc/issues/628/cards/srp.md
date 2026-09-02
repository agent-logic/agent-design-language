# Structured Review Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/local/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/628
.csdlc/evidence/628
.csdlc/issues/627
.csdlc/evidence/627

## Prompts

- Verify the eight local routes are implemented or explicitly fail closed according to #628 scope.
- Verify no v3 local route delegates operational authority to v2 before #505.
- Verify missing local lifecycle state has clear typed recovery semantics.
- Verify no csdlc-v2 source changes are present.

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
