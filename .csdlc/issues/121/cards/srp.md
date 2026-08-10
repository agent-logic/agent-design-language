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

Revision: Some("git-blake3:1e6d7169a197bfff79c8cd0514977ecd76a28bce:6edd6bd5b3c6b6a28479d15ee475b59f6d669f25021447897fdf1a7fe8cebb9c")

Reviewer: Some("subagent:issue-121-stack-refresh-review")

Result: pass
