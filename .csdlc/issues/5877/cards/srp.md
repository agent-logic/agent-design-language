# Structured Review Prompt

Template: 1.0.0

Issue: 5877

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime/src/distributed/projection.rs
adl-runtime/tests/distributed_projection.rs
docs/api/runtime-v3/v1/distributed.openapi.json

## Prompts

- Is the implementation confined to exclusive paths?
- Do exact tests prove the named behavior and negatives?
- Are receipts exact-revision and digest bound?
- Does rollback restore one authoritative owner without weakening security?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:bedde511d6523d1e7e4f8c5189f7dd8198ab50b5:45d7ce54be48bd7712b0aeda4bb5fe7feec048eebd1fa2856e42f05e32ff7c2f")

Reviewer: Some("subagent:Raman")

Result: pass
