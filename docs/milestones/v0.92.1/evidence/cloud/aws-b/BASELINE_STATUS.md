# AWS-B baseline status

Issue: #485

Status: draft, not yet accepted; AWS CLI floor and typed bind are satisfied, but exact-head review/publication are still pending.

## Completed readbacks

The readback collector wrote redacted evidence under `readbacks/` for:

- root recovery and administrator continuity
- identity census
- Agent Toolkit/AWS CLI configuration
- agent IAM guardrail posture
- CloudWatch and CloudTrail attribution
- billing, budget, anomaly, export, and cost-attribution visibility

## AWS CLI floor

AC-3 requires AWS CLI 2.35 or newer for the approved Agent Toolkit path. The retained readback currently shows:

```text
aws-cli/2.36.32 Python/3.14.7 Darwin/25.5.0 source/arm64
```

This satisfies the required floor. The baseline cannot be accepted or published as complete until validation, exact-head review, publication, and shepherd gates complete.

## Non-claims

- No AWS mutation was performed.
- No administrator access was removed or replaced.
- No Terraform apply/import/destroy was performed.
- No #122 public exposure, Route53, ACM, CloudFront, or WSS ownership was changed.
