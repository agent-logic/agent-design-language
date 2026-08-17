# Structured Review Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/tests/estimation_contracts.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/296

## Prompts

- Review every acceptance criterion with code, security, test, and evidence coverage, emphasizing lifecycle authority, stale approval invalidation, artifact TOCTOU and path safety, atomic SPP/VPP parity, append-only history, and exact fresh-review gating.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review treated 53b52e8a819a5549b0fb56eded4c618d76a1fa4a as non-substantive review-assignment metadata over the substantive r7 remediation head.
- Review scope was limited to the standalone CI fixture remediation and #296 lifecycle record after PR #383 failed Gate 10A on noncanonical design reviewer identity.

## Review Result

Revision: Some("git-blake3:2cbb7af54c101775e9ee3873cac64e6ec2276006:a41598c8e66d0d7e632ad0c704428eac31e7b4d0900d288f9d9b03028faac6a9")

Reviewer: Some("fresh-session:a12780a7-6a78-4485-bdff-2bd14fb84957")

Result: pass
