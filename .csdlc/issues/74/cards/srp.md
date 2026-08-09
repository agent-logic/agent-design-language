# Structured Review Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/74
.csdlc/prepared/issues/74/record-exact-head-gate2-validation.json
.csdlc/prepared/issues/74/record-exact-head-clippy-validation.json

## Prompts

- Does the test contain the exact retired claim field from the report?
- Is skipping based only on issue, branch, and canonical worktree irrelevance?
- Do relevant corruption and genuine collisions remain fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The historical PR #80 publication intent is non-authoritative after typed recovery and must be atomically replaced by csdlc-publish for this repair.

## Review Result

Revision: Some("git-blake3:2d064702c290fc18356f9673c701393f5473bbe9:9193c0a5e6ac7706f91924d13144f9d5d7c01d036a56b178567911fbbf44e808")

Reviewer: Some("subagent:Ampere")

Result: pass
