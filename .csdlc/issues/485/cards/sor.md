# Structured Output Record

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Finalize AWS-B access and billing baseline with redacted read-only evidence, retained administrator continuity, agent IAM guardrail posture, AWS CLI 2.35+ Agent Toolkit proof, CloudTrail/CloudWatch attribution readbacks, billing visibility evidence, and post-review remediation for sessionToken redaction/current typed-state evidence.

## Artifacts

- docs/operations/cloud/aws/access-billing/AWS_ACCESS_BILLING_BASELINE.md
- docs/milestones/v0.92.1/evidence/cloud/aws-b/
- docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh
- infra/aws/account-foundation/README.md
- .csdlc/prepared/issues/485/validate-aws-b-baseline.sh
- .csdlc/prepared/issues/485/validate-typed-state.sh
- .csdlc/evidence/485/

## Execution

- Added AWS access and billing baseline documentation for corporate recovery, identity census, agent IAM guardrails, audit attribution, and billing visibility.
- Added issue-owned redacted readback collector and retained AWS-B evidence under docs/milestones/v0.92.1/evidence/cloud/aws-b.
- Recorded current AWS CLI 2.36.32 Agent Toolkit floor proof and refreshed read-only AWS readbacks using the approved agent-logic-admin profile.
- Bound #485 to codex/485-aws-access-billing-baseline in the FastWork issue worktree.
- Fixed R3 P1 by redacting normal and escaped CloudTrail sessionToken/secretAccessKey fields and tightening the validator against raw sessionToken evidence.
- Fixed R3 P2 by refreshing retained .csdlc/evidence/485 typed-state evidence to prove the current implemented generation.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "root-recovery"
    ],
    "purpose": "Refresh root recovery and administrator continuity readback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/root-recovery.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "identity-census"
    ],
    "purpose": "Refresh IAM identity census readback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/identity-census.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-toolkit-configuration"
    ],
    "purpose": "Refresh AWS CLI and approved-profile STS readback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/agent-toolkit-configuration.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-iam-guardrails"
    ],
    "purpose": "Refresh agent IAM guardrail readback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/agent-iam-guardrails.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-activity-audit"
    ],
    "purpose": "Refresh CloudWatch and CloudTrail attribution readback with credential-field redaction.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/agent-activity-audit.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "billing-readback"
    ],
    "purpose": "Refresh billing and cost visibility readback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/billing-readback.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/485/validate-aws-b-baseline.sh"
    ],
    "purpose": "Reject credential material, raw sessionToken values, and unintended mutation verbs in retained AWS-B evidence.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/credential-redaction.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/485/validate-typed-state.sh"
    ],
    "purpose": "Prove current typed issue integrity after R3 remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/485/typed-state.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
