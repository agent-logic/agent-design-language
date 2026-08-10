# Structured Review Prompt

Template: 1.0.0

Issue: 5871

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime/src/distributed/capability_advertisement.rs
adl-runtime/tests/distributed_capability_advertisement.rs

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

Revision: Some("git-blake3:183bb3ae2451d2092a1075f9611113c03237de91:b1532647057e02aed736142b3f497fb954ba5b734298d749a83e04d897c32c6a")

Reviewer: Some("Codex independent security/correctness reviewer")

Result: pass
