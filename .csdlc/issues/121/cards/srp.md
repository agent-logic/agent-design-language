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

- Full stacked diff-check reports final blank lines in retained machine stdout logs; the exact source, receipt, test, lint, tamper, and path-safety proofs remain valid.

## Review Result

Revision: Some("git-blake3:03d0854fd749986895c283d114c20a681f624098:a791ba537d35c5877869e61703359c09dbe17e7081f7126d6d61a50eea26c0c7")

Reviewer: Some("subagent:issue-121-exact-head-security-review")

Result: pass
