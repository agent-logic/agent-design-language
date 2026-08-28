# Issue 488 design — AWS-E resource adoption register

Status: design ready for review.

Issue #488 produces one accepted AWS resource adoption register reconciled with live state. The register is the deliverable; this issue does not delete, import, tag, rewrite, or retire resources by itself.

## Scope

#488 owns `docs/operations/cloud/aws/adoption/**`, `docs/milestones/v0.92.1/evidence/cloud/aws-e/**`, `.csdlc/prepared/issues/488/**`, and `.csdlc/evidence/488/**`.

It may inspect `infra/aws/**` and prior AWS evidence to classify resources, but must not absorb downstream implementation authority.

## Non-goals

- #489 AWS Runtime platform modules.
- #495 cross-cloud Runtime Terraform conversion.
- #496 CloudFormation retirement.
- Website rewrite or public-edge changes.
- Speculative cleanup.
- Credential disclosure.

## Register model

Each admitted durable AWS resource gets one row with stable identity, service, region, observed source, current authority, intended authority, disposition, evidence reference, deletion authority, retention recovery requirement, and follow-on issue.

Allowed dispositions are `retain`, `import`, `replace`, `retire-later`, `ephemeral`, and `frozen-unknown`.

The one management authority invariant is mandatory. Ambiguous authority must be `frozen-unknown` or routed to a follow-on, never silently accepted as dual-managed.

## Cleanup and deletion gate

Cleanup is not authorized unless an explicit register row proves exact non-use evidence, retention recovery, deletion authority, and that the resource is not website-owned, public-edge-owned, historical-evidence-owned, or downstream Terraform/CloudFormation authority.

## Readback posture

Live readback is read-only through the approved Agent Logic business AWS profile `agent-logic-admin`. It must not print credential material, secret values, raw account dumps, or sensitive payloads. Retained proof uses aggregate counts, hashes, and disposition summaries rather than raw credentials.

## Dependency and downstream boundary

#487 is terminal by merge `1d31016a8df3cf07a4c3f2e6acd2694bd10570c2`. #489, #495, and #496 consume this register later. CloudFormation retirement remains #496. Runtime platform modules remain #489. Cross-cloud conversion remains #495.

## Validation

Prebind validation:

- `bash .csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh .`
- `bash .csdlc/prepared/issues/488/run-aws-e-readback.sh --lane=static`

Postbind validation:

- the same register validator after implementation creates the register and proof surfaces;
- optional read-only AWS inventory lane with `AWS_PROFILE=agent-logic-admin`;
- fresh exact-head review before publication.

## Failure policy

Fail closed if a resource may belong to website or retained evidence, if dual management is possible, if deletion authority is missing, if live and declared state cannot be reconciled, if evidence would expose credentials or sensitive values, or if #488 starts implementing #489, #495, or #496 scope.
