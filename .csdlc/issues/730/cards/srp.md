# Structured Review Prompt

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

exact-head pre-publication review at aefcc1e584d7a4912ce3e95edeebbbac01044a75
GCP-B1 impersonation-only Terraform bootstrap correction
typed recover_review metadata-tail state
fail-closed live proof authorization and cleanup safety
static validation, no-auth failure, static-key rejection, residue, and diff hygiene evidence

## Prompts

- Can any production path still select a static key?
- Can the plan or rollback touch resources outside the exact denominator?
- Does recovery prove immutable content and clean backend reinitialization?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live GCP apply was not executed because the exact git-common authorization artifact is not present; the live lane fails closed before mutation without it.

## Review Result

Revision: Some("git-blake3:aefcc1e584d7a4912ce3e95edeebbbac01044a75:6fdac4c245792ab9457ee76153c3086a7686aec338a0ad4f5ac1d95e8a550aa3")

Reviewer: Some("codex-subagent:/root/review_730_prelive")

Result: pass
