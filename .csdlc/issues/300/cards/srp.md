# Structured Review Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/tests/projection_recovery_integration.rs
.csdlc/issues/300
.csdlc/evidence/300/bridge-fed-r4
.csdlc/requests/300/recover-review-curie-r3.json
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

- Reviewer did not rerun cargo, Clippy, fmt, or diff-check; PASS is based on inspection-only exact-head review of immutable commit b21078fd31e76ef6ba1e8b4a72be6bca3854f1a0 and recorded r4 evidence.

## Review Result

Revision: Some("git-blake3:b21078fd31e76ef6ba1e8b4a72be6bca3854f1a0:0099780c89d015c3b7b007ce9bb74ef22751bfa71dfb1bfa5c3c99f85f08a815")

Reviewer: Some("fresh-session:lovelace-300-bridge-fed-r4")

Result: pass
