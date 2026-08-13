# Structured Review Prompt

Template: 1.0.0

Issue: 349

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/349
.csdlc/evidence/349

## Prompts

- Does the design preserve every #79 exact-path and fail-closed predicate?
- Is deferred admission limited exactly to initialized and ready before bind?
- Does the regression follow doctor's actual advertised advance_ready sequence?
- Can any bound or later packet misinterpret deferred planning as execution proof?
- Is #342 completely read-only and outside all owned paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Fresh metadata-head review confirmed substantive code/test identity with the prior PASS and retained exact local proof. Hosted CI remains deferred to publication.

## Review Result

Revision: Some("git-blake3:54ffa5c4d30069c6ad70f25d55a5e59585d6f564:a01d00dc29653c29e3222900e4fe95d99d1aaf4cd4d9ad82e63e5695816e7e53")

Reviewer: Some("/root/issue_349_prepare/fresh_metadata_review")

Result: pass
