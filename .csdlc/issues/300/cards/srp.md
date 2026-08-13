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

- Reviewer did not rerun cargo, Clippy, fmt, or diff-check; PASS is based on inspection-only exact-head review of immutable commit e340b75013e9b93933acdef830dec1657e330f43 and recorded r4 evidence.

## Review Result

Revision: Some("git-blake3:e340b75013e9b93933acdef830dec1657e330f43:fc3aa8c83bc7671958b537232d78e72aedc1fe588a61d016fe26cdb1f175c371")

Reviewer: Some("fresh-session:lamarr-300-bridge-fed-r6")

Result: pass
