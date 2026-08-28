# Structured Review Prompt

Template: 1.0.0

Issue: 488

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/488
.csdlc/prepared/issues/488
.csdlc/evidence/488
docs/operations/cloud/aws/adoption
docs/milestones/v0.92.1/evidence/cloud/aws-e

## Prompts

- Does the design keep #488 to adoption-register truth without absorbing AWS-F, XCL-01, or AWS-G implementation/retirement work?
- Are one-owner resource authority, website/evidence preservation, cleanup authority, and live-vs-declared reconciliation specified with machine-checkable proof?
- Does the live AWS readback plan avoid mutation and avoid printing or retaining credentials/sensitive values?
- Are frozen-unknown and follow-on dispositions truthful rather than hidden acceptance of unresolved dual ownership?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer confirmed the prior P1 inventory-readonly stub finding is resolved by the live AWS readback script and evidence contract.
- Reviewer confirmed the prior P3 trailing-whitespace finding is resolved in the adoption register and AWS-E proof document.
- Reviewer inspected exact head and targeted proof; live AWS was not rerun by the reviewer to avoid rewriting retained evidence, so future cloud drift remains a normal operational risk.
- GitHub CI remains the hosted integration gate after typed publication.

## Review Result

Revision: Some("git-blake3:5cf661679a50cb58dde806c01336c0351e8e1f3f:f89fefe0a4c06952cd3ac653b925c710a53b64b9830c8edff182ce2b72712c83")

Reviewer: Some("fresh-session:b2d4e572-12de-40dd-97a4-61c3e9b18887")

Result: pass
