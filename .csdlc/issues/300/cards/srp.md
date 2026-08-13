# Structured Review Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/tests/projection_recovery_integration.rs
.csdlc/evidence/300/bridge-fed-r4
.csdlc/requests/300/recover-review-curie-r3.json
.csdlc/requests/300/recover-review-lovelace-scope-dead-end.json
.csdlc/requests/300/recover-review-hopper-request-metadata.json
.csdlc/requests/300/recover-review-lamarr-assignment-request-metadata.json
.csdlc/requests/300/replace-sor-bridge-fed-r4.json

## Prompts

- Are both prerequisite terminal and ancestry gates exact and fail closed before bind?
- Does every production mutation and durability boundary have before/after restart proof?
- Can any mock, constant, path, or self-authored receipt become authority?
- Are symlink, repeated-inode, ancestor-swap, destination-race, recovery/cleanup, ordinary-commit, and sentinel cases explicit?
- Does scope remain one new integration test target plus issue-local records?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not rerun cargo, Clippy, fmt, or diff-check; PASS is based on inspection-only exact-head review of immutable commit 2e0a8e67d7b910f3cab6b755343726cbfb0746bd and recorded r4 evidence.

## Review Result

Revision: Some("git-blake3:2e0a8e67d7b910f3cab6b755343726cbfb0746bd:eb75cadc0288d9b651586398ad9f797e1fa4b2649cb862a1510328c6119180eb")

Reviewer: Some("fresh-session:meitner-300-bridge-fed-r7")

Result: pass
