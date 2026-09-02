# Structured Review Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/mod.rs
adl/tests/provider_shadow_isolation.rs
adl/tests/provider_shadow_comparison.rs
adl/tests/provider_shadow_fallback.rs
adl/tests/provider_shadow_open_pr_review.rs
docs/milestones/v0.92.1/evidence/provider/prov-b/local-model-shadow-comparison.json
docs/milestones/v0.92.1/evidence/provider/prov-b/open-pr-shadow-review-smoke.json
.csdlc/prepared/issues/515

## Prompts

- Can any shadow result mutate or replace the authoritative result?
- Are authority and shadow paths represented distinctly enough for reviewers and validators?
- Are comparison inputs and rules exact and deterministic?
- Do shadow failures preserve authoritative outputs and state?
- Does evidence redact credentials, private payloads, prompts, and host-local paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
