# Structured Review Prompt

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/485
.csdlc/prepared/issues/485
.csdlc/evidence/485
docs/milestones/v0.92.1/evidence/cloud/aws-b
docs/operations/cloud/aws/access-billing/AWS_ACCESS_BILLING_BASELINE.md
infra/aws/account-foundation/README.md

## Prompts

- Is #484 dependency truth accurately represented without re-owning AWS-A inventory scope?
- Does the baseline prove recovery, identity separation, agent guardrails, audit attribution, and billing visibility from retained evidence?
- Does the Agent Toolkit section avoid authorizing unbounded AWS mutation?
- Are credentials, payment data, and secret material excluded from retained evidence?
- Are #486 and #122 scopes clearly out of bounds?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not inspect credential stores and did not perform GitHub, AWS, or lifecycle writes.
- Reviewer did not rerun AWS readback collection; review validated retained evidence and local validators only.
- Live worktree includes assignment-only metadata commit bd95a4f5 after the reviewed substantive commit; scoped artifacts are unchanged by that top metadata commit.

## Review Result

Revision: Some("git-blake3:9ec2e5e453a0a4a300fcf3e0871e7827939d577e:a8c6d9c85916c6ecbd7845a744bacd38054989493f2d1d3453aaf945c6fac13a")

Reviewer: Some("fresh-session:f0bf501e-d7a2-48cd-b983-da5f21564b5e")

Result: pass
