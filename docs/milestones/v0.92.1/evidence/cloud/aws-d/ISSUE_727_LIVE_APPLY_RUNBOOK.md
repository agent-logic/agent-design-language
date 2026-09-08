# Issue #727 AWS-D-R live apply runbook

Issue #727 is the guarded live-application lane for the account-foundation
Terraform root that was reviewed and merged through #487 / PR #574. This
runbook keeps the mutation envelope first-class: preflight and planning may be
prepared locally, but no Terraform apply is allowed until the operator supplies
`.adl/requests/727/operator-authorization.json` with the exact saved-plan digest
and bounded authority named below.

## Non-mutating readiness proof

Run from the bound issue worktree:

```sh
bash .csdlc/prepared/issues/727/validate-issue-727-readiness.sh .
AWS_PROFILE=agent-logic-admin \
  bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=static
```

The first command checks the issue projection, the reviewed AWS-D static
contract, the retained authorization gate, and credential-like retained content.
The second command proves the readback entrypoint is available without making
cloud calls.

## Authorization envelope

Use
`docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json`
as the reviewable template. Copy it to
`.adl/requests/727/operator-authorization.json` only after the operator has
reviewed the exact saved plan. The `.adl/requests/727/` copy is the local
authorization file consumed by the validator and must not be committed. It must
name:

- the verified `agent-logic-admin` Agent Logic business-account identity;
- region `us-west-2`;
- Terraform root `infra/aws/account-foundation`;
- workspace `aws-d-account-foundation-live`;
- state key `v0.92.1/aws-d/account-foundation/account-foundation.tfstate`;
- the SHA-256 digest of the exact saved plan to apply;
- the explicit set of permitted Terraform resources;
- mutation deadline, cost ceiling, and rollback/destroy disposition.

Then run:

```sh
bash .csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh .
```

If that validator fails, do not apply.

## Saved-plan discipline

The saved plan is the mutation boundary. A plan generated before authorization
must not be regenerated, edited, or replaced after authorization. If source,
backend, workspace, variables, provider lockfiles, or ambient identity change,
discard the authorization request and produce a new plan/digest for operator
review.

Keep backend config, tfvars, saved plans, state, and raw provider output in
repo-local ignored paths under `.adl/requests/727/`. Do not use `/private/tmp`
for #727 artifacts.

## Apply stop lines

Stop before mutation if any of the following is true:

- `AWS_PROFILE` is not `agent-logic-admin`;
- caller identity is not the approved Agent Logic business account;
- region is not `us-west-2`;
- backend/workspace/state key are ambiguous or differ from the authorization;
- the plan digest differs from `operator-authorization.json`;
- the plan contains resources outside the permitted resource list;
- estimated cost or logging volume exceeds the operator ceiling;
- rollback/destroy authority is unclear;
- output would retain account IDs, ARNs, emails, access keys, secret values,
  Terraform state, provider debug logs, or raw AWS JSON.

## Post-apply readback

After applying the exact saved plan, run the governed readback:

```sh
AWS_PROFILE=agent-logic-admin \
  bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=aws-readonly
```

Retain only the redacted summary needed to prove CloudTrail, KMS-backed audit
bucket encryption/versioning/retention, AWS Config, IAM Access Analyzer,
SNS/EventBridge findings route, and owner/destination tags.
