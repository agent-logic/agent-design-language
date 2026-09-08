# Structured Review Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.gitignore
.csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh
.csdlc/prepared/issues/727/validate-issue-727-readiness.sh
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_LIVE_APPLY_RUNBOOK.md
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_REDACTED_LIVE_APPLY_PROOF.md
docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh

## Prompts

- Verify exact resource denominator and absence of unrelated mutations.
- Verify redaction, rollback, cost and saved-plan binding.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Raw Terraform plan/apply/readback logs and account identity evidence are retained only under ignored .adl/requests/727; tracked proof is deliberately redacted and hash-bound.
- C-SDLC issue records are self-staling under review assignment/record/publication and are excluded from the exact review scope by typed tooling policy; lifecycle truth is validated separately by csdlc-validate/doctor and the typed review/publish guards.
- No additional Terraform apply was performed during remediation; live AWS interaction was read-only readback using the approved agent-logic-admin profile.

## Review Result

Revision: Some("git-blake3:f6c30b020de6faf5c84a967f097d37d3892cafaa:1d007b72f4b6236acaaefdee756a433623ea0958c35e99e4d65c7c52836c0e50")

Reviewer: Some("codex-727-remediation-review")

Result: pass
