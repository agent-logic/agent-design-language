# Issue 496 design

Status: proposed for design review.

## Scope

Issue #496 decides the bounded AWS-G CloudFormation retirement disposition for
exactly these retained template denominators:

- `adl/tools/issue194_private_network.cloudformation.json`
- `adl/tools/issue268_runtime_qualification.cloudformation.yaml`

The issue does not delete either template. The output is a retirement ledger and
validator that classify the templates, current repo consumers, Terraform
replacement evidence, rollback posture, retained evidence, and live-stack
readback boundary.

## Dependency truth

- AWS-F #489 is closed by merged PR evidence at merge
  `69ba35e066d1389a9f194659acb066a7dca82a40` and provides the AWS Runtime
  platform module denominator consumed by AWS-G.
- XCL-01 #495 is closed by merged PR #590 at merge
  `c78c60f5a45a87a96159d4910a831b69b62b042c`; a root derived-terminal cache was
  not present during bootstrap, so #496 records the live GitHub/ancestry basis
  explicitly rather than inventing cache truth.

## Implementation plan

1. Create `docs/milestones/v0.92.1/evidence/cloud/aws-g/`.
2. Add `aws-g-cloudformation-retirement-ledger.md` with one Markdown table row
   per template, repo consumer/reference, rollback path, replacement evidence
   path, retained evidence path, and live-stack boundary. Each row must carry an
   explicit disposition token so the validator proves classification rather than
   mere path mention.
3. Add `validate-aws-g-cloudformation-retirement.sh` to fail closed unless the
   ledger:
   - inventories both templates,
   - names #489 and #495 dependency evidence, including merge SHAs,
   - classifies every current repo consumer/reference path found by the
     validator with one of `rollback`, `source-denominator`,
     `terraform-replacement`, `retained-evidence`, or `follow-on`,
   - preserves rollback authority and retained historical evidence,
   - refuses template deletion and forced retirement,
   - records live stack readback as a non-claim unless explicit live evidence is
     later added.
4. Run the validator, `git diff --check`, exact-head review, typed publication,
   CI, and finish.

## Non-goals

- Deleting CloudFormation templates.
- Destroying or changing AWS resources.
- Reopening #489/#495 implementation scope.
- Website Terraform changes.
- Production cutover.
- Credential disclosure.

## Acceptance mapping

- AC-1 is satisfied by explicit template inventory.
- AC-2 is satisfied by validator-backed consumer classifications.
- AC-3 is satisfied by dependency evidence, Terraform replacement references,
  rollback retention, and no forced retirement.
- AC-4 is satisfied by live-stack non-claim/fail-closed wording.
- AC-5 is satisfied only after fresh exact-head review has no actionable
  findings.
