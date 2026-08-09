# Structured Review Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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

Revision: Some("git-blake3:b07302a56c8a5ae1a88831e5920f7ae1831056db:ffea47c6678ee3a13a9efe4915882ac064b2aa525bfe8262498f3857af2a87be")

Reviewer: Some("subagent:74-exact-head-rereview")

Result: pass
