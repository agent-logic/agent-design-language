# Structured Review Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/reviews/v0.92/external-review-5847
.csdlc/evidence/5847
.csdlc/prepared/issues/5847
.csdlc/issues/5847

## Prompts

- Is the handoff self-contained, exact-revision bound, digest-reproducible, publication-safe, and explicit about reviewer authority?
- Would any source change, missing evidence, unsafe private state, or unsupported approval claim fail closed before dispatch?
- Was an actual reviewer-authored report received and preserved without rewriting?
- Does the separate findings index preserve every returned item and route it to WP-27?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- WP-26 proves external-review intake and provenance only; the seven unique findings remain owned by #315/WP-27, with #471 as its child issue.

## Review Result

Revision: Some("git-blake3:5d5097fa8fb7d01116853ba9bdd18eaf6078e1ad:9df30ca40fe8f9c37e52c9c84bd39ef56fa9c15ca50b6095431f6687562ab91e")

Reviewer: Some("subagent:/root/review_314_prepr")

Result: pass
