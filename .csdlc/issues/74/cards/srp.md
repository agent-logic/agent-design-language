# Structured Review Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/tests/gate2.rs
.csdlc/issues/74
.csdlc/prepared/issues/74

## Prompts

- Does the test contain the exact retired claim field from the report?
- Is skipping based only on issue, branch, and canonical worktree irrelevance?
- Do relevant corruption and genuine collisions remain fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This is regression proof of the existing relevance-first implementation; broader topology integration remains GitHub CI evidence.

## Review Result

Revision: Some("git-blake3:52b847e497741954cf5470a5657fa8f41404a035:66a9beca6e70e36abf8d5cd06ae88a99832ee5b5e80bacc8d50eafed883f59b5")

Reviewer: Some("subagent:74-exact-head-review")

Result: pass
