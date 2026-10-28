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

The saved plan is the mutation boundary. The authorization validator hashes the
saved `.tfplan` bytes directly, compares that digest to both the sidecar and the
signed packet, runs `terraform show -json` against those exact bytes, and
requires the resulting canonical JSON to match the signed reviewed projection.
A plan generated before authorization
must not be regenerated, edited, or replaced after authorization. If source,
backend, workspace, variables, provider lockfiles, or ambient identity change,
discard the authorization request and produce a new plan/digest for operator
review.

Keep backend config, tfvars, saved plans, state, and raw provider output in
repo-local ignored paths under `.adl/requests/727/`. Do not use `/private/tmp`
for #727 artifacts.

## Authentic operator approval

The editable JSON packet is not authority by itself. Production validation
uses the fixed external trust anchor
`$HOME/keys/adl-cloud-authorization.allowed_signers` and OpenSSH namespace
`adl-cloud-authorization-v1`. The allowed-signers file must be outside the
repository and not group- or world-writable. It contains the approved operator
principal and Ed25519 public key; the corresponding private key is never stored
in the repository or passed to provider-running automation.

After filling the v2 packet, produce its canonical unsigned payload with:

```sh
python3 .csdlc/prepared/issues/815/verify-cloud-authorization.py canonicalize \
  --packet .adl/requests/727/operator-authorization.json \
  > .adl/requests/727/operator-authorization.payload
ssh-keygen -Y sign -f <external-operator-private-key> \
  -n adl-cloud-authorization-v1 \
  .adl/requests/727/operator-authorization.payload
```

Place the armored detached signature in
`operator_authorization.signature.value`, then delete the unsigned payload and
its detached sidecar. The signature covers repository, issue, exact AWS account
ID, region, Terraform root/workspace/state key, expiry, resource allowlist,
cost/rollback bounds, actual plan digest, and reviewed plan-JSON digest. The
validator also compares the signed account ID to authenticated STS readback, so
the packet cannot be replayed in another AWS account.

## Apply stop lines

Stop before mutation if any of the following is true:

- `AWS_PROFILE` is not `agent-logic-admin`;
- caller identity is not the approved Agent Logic business account;
- region is not `us-west-2`;
- backend/workspace/state key are ambiguous or differ from the authorization;
- the plan digest differs from `operator-authorization.json`;
- the digest sidecar differs from the actual `.tfplan` bytes;
- `terraform show -json` from those bytes differs from the signed reviewed projection;
- the detached operator signature is absent, forged, expired, or untrusted;
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
