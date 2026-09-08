# Structured Review Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/prepared/issues/727/validate-issue-727-readiness.sh
.csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_LIVE_APPLY_RUNBOOK.md
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json

## Prompts

- Verify exact resource denominator and absence of unrelated mutations.
- Verify redaction, rollback, cost and saved-plan binding.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Terraform apply and AWS readback remain deferred until explicit operator authorization supplies the exact saved-plan digest and mutation envelope.
- No live AWS/Terraform mutation, backend init, plan, or apply was performed in this pre-apply package.

## Review Result

Revision: Some("git-blake3:48edfc341c357e85d2162e0e017ba4addd5c063f:a961701fa4669bb78a87308de7e73db51246895de3b66ce547c1cb8d57d43f6b")

Reviewer: Some("codex-727-preapply-review")

Result: pass
