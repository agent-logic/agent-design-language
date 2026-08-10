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

Revision: Some("git-blake3:6ddf15e4c5ec8341cb316d63e2a8fddc2a6c8e10:db56583e172b5ac80bcdfdb575aa82f40c14745161204c37dc414724b9958b56")

Reviewer: Some("subagent:issue-121-exact-head-security-review")

Result: pass
