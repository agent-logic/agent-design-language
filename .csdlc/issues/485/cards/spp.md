# Structured Planning Prompt

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #485 from current origin/main after #484 PR #556 merge, preserve the accepted AWS-A inventory as dependency input, capture a redacted AWS access and billing baseline, validate local redaction/no-mutation and coverage, obtain fresh review, then publish one closing PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm #484 live closure and current main ancestry, then bootstrap and bind #485 in the existing FastWork worktree.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify approved account identity and AWS CLI version without printing credentials.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Record corporate recovery, identity census, and administrator-continuity baseline.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record Agent Toolkit, agent IAM guardrail, CloudWatch, CloudTrail, billing, budget, anomaly, export, and cost-attribution readbacks.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Validate redaction, no-mutation posture, diff hygiene, and lifecycle truth; obtain fresh exact-head review and publish.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- No credential material is captured.
- Existing administrator access remains in place until replacement is proven.
- Human, workload, and agent identities remain distinguishable.
- Agent access defaults to read-only unless a later operator-approved lane proves otherwise.
- Billing/cost evidence is visible without exposing sensitive payment data.
- Downstream AWS-C and deferred #122 scope remain separate.

## Risks

- The active AWS profile could point at the wrong account.
- Billing APIs may require payer/root permissions that are not available to the read-only profile.
- Some audit readbacks may lag CloudTrail or CloudWatch ingestion.
- Agent Toolkit setup could imply AWS resource creation if not constrained to documentation/configuration and read-only checks.
- Evidence could accidentally include sensitive ARNs, account IDs, payment metadata, or command history.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/485/design.md

Digest: e37e23e073732cac34573fb7bed7c8d169a46f1ad930bbdb9a57f01fa6c744d5

## Diagram

.csdlc/prepared/issues/485/diagram.mmd

Digest: d63606ef1e768660e36bcbb80de7d053aaea89e77858bdbe24026c0003a150bc

## Stop Conditions

- Replacement access is unproven and a change would remove existing administrator access.
- Billing target is ambiguous.
- Credentials would enter evidence.
- A required AWS operation would mutate state without explicit typed approval.
- Fresh review finds unresolved actionable issues.

## Handoff

Proceed only after doctor readiness.
