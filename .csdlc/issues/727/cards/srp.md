# Structured Review Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.gitignore
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_LIVE_APPLY_RUNBOOK.md
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json
docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_REDACTED_LIVE_APPLY_PROOF.md

## Prompts

- Verify exact resource denominator and absence of unrelated mutations.
- Verify redaction, rollback, cost and saved-plan binding.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Raw Terraform plan/apply/readback logs are retained only under ignored .adl/requests/727; tracked proof is deliberately redacted and hash-bound.
- Terraform backend currently uses deprecated dynamodb_table locking parameter; warning observed but apply/readback succeeded.

## Review Result

Revision: Some("git-blake3:03e9de180e75e87c4deddaa0a28d5a7ce3aafc45:22ac7d77d8000dacad6faf7a093b215a20b168f594c8d9caf2d6c9bbd6b659c5")

Reviewer: Some("codex-727-live-review")

Result: pass
