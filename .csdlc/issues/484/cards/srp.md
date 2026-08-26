# Structured Review Prompt

Template: 1.0.0

Issue: 484

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/484
.csdlc/prepared/issues/484
.csdlc/evidence/484
docs/milestones/v0.92.1/evidence/cloud/aws-a
docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md

## Prompts

- Is the approved account and region basis explicit and evidence-backed?
- Does every discovered resource have an owner or frozen-unknown disposition?
- Are website Terraform and issue evidence classified separately?
- Does the inventory avoid inferring retained assets disposable?
- Does retained evidence avoid credentials and mutation commands?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer was read-only and did not run AWS credentialed calls, mutate lifecycle state, publish, or record review.
- The post-reviewed current HEAD 913b20551245c167f40d15e1f13e020cbf8c82c4 contains metadata-only review-assignment truth above the reviewed substantive revision.

## Review Result

Revision: Some("git-blake3:1f81b33ffba9a652983071372466e53413f1a989:ff98161bc76e73273823d30dadc8534e9adff08d60f56757144e81bd8b47150c")

Reviewer: Some("fresh-session:e364169b-9eba-40df-8cc4-b5ac0d8de826")

Result: pass
