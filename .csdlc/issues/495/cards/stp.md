# Structured Task Prompt

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #495 XCL-01 only; produce the portable Runtime workload contract, explicit AWS/GCP Terraform implementations, parity/rollback proof surfaces, and retained evidence. Do not implement AWS-G retirement, GCP-E GPU smoke, DRT-D qualification, production cutover, or credential-bearing live proof without explicit authorization.

## Deliverables

- infra/runtime-portable portable workload contract and module documentation
- infra/aws/runtime Terraform implementation/adapters for the admitted #194/#268 denominator
- infra/gcp/workloads Terraform implementation/adapters consuming GCP-D foundation truth
- docs/milestones/v0.92.1/evidence/cloud/xcl-01 retained proof packet
- docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh
- typed C-SDLC v2 cards proving dependency, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: The issue #194 and #268 CloudFormation-template denominator is inventoried exactly
2. AC-2: The portable workload contract is provider-neutral and does not hide provider-specific security or identity differences
3. AC-3: AWS Terraform preserves the admitted template behavior while retaining CloudFormation rollback authority
4. AC-4: GCP Terraform preserves the equivalent Runtime workload behavior against the reviewed GCP-D foundation
5. AC-5: Parity, rollback, disposable deployment, and zero-residue proof are represented truthfully, with paid/live proof explicitly gated
6. AC-6: Fresh exact-head review has no actionable findings before publication

## Dependencies

- AWS-E #488 terminal/merged adoption-register truth; derived terminal cache observed with PR #576 merge a6b404cd6e74d7528745325036ceb1a85fd47bd2
- GCP-D #493 terminal/merged platform-foundation truth; derived terminal cache observed with PR #587 merge c0bf217934508d6dbc70d78633e6a95d5ddd9d06

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#XCL-01
- docs/milestones/v0.92.1/features/CROSS_CLOUD_TERRAFORM_CONVERSION_v0.92.1.md
- docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md
- infra/aws/runtime/
- infra/gcp/platform/
- .git/csdlc-v2/derived-terminal/488.json
- .git/csdlc-v2/derived-terminal/493.json
- adl/tools/issue194_private_network.cloudformation.json
- adl/tools/issue268_runtime_qualification.cloudformation.yaml

## Non Goals

- CloudFormation retirement decision
- Deleting CloudFormation templates or historical evidence
- GCP GPU readiness smoke test
- Six-resident distributed Runtime qualification
- Production cutover
- One Terraform resource graph spanning both providers
- Credential disclosure
