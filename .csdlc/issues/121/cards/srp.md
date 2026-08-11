# Structured Review Prompt

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/lease.rs
adl-runtime/tests/distributed_lease.rs
.csdlc/issues/121
.csdlc/evidence/121
.csdlc/prepared/issues/121
.csdlc/publication/121.intent.json

## Prompts

- Can quorum fence and revoke an unavailable holder without its private activation key?
- Are next-epoch and applied-index transitions exact and atomic?
- Does every restart preserve the portable fence floor until safe activation?
- Do holder-authorized operations still prove the correct activation key?
- Are negative cases derived from executed output and exact-head bound?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:f729263b04820f730841059680a3902b1b99b1e3:df6d256432fa1dd77bd1f86e175a1d8467ae25bc1d8f38624f149c4ec5ded494")

Reviewer: Some("subagent:issue-121-proof-repair-review")

Result: pass
