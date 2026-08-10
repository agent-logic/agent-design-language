# Structured Review Prompt

Template: 1.0.0

Issue: 5870

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/fencing.rs
adl-runtime/tests/distributed_fencing.rs
.csdlc/evidence/5870/derive-negative-cases.rb
.csdlc/prepared/issues/5870/validate-proof-receipt.rb
.csdlc/evidence/5870/remediation-v5/execution-proof.json

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

Revision: Some("git-blake3:0b166fa34e145b6890ddb1b43a5727f2b01d1f02:3c8a27264ea96f313b363b9c523a82d7e7f0d7faa9c8e93fed470e6169f51210")

Reviewer: Some("subagent:portable_review_5870")

Result: pass
