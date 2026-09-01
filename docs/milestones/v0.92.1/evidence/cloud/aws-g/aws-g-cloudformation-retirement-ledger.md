# AWS-G CloudFormation retirement ledger (#496)

Issue: #496  
Decision date: 2026-09-01  
Scope: `adl/tools/issue194_private_network.cloudformation.json` and `adl/tools/issue268_runtime_qualification.cloudformation.yaml`

## Decision

The two CloudFormation templates are retained as rollback/source-denominator evidence. #496 does not authorize deleting either template, destroying any live AWS resource, or declaring an active stack retired.

CloudFormation rollback authority remains retained. No template deletion is authorized by #496. Live stack retirement is not claimed.

## Dependency evidence

- AWS-F #489: merged by `69ba35e066d1389a9f194659acb066a7dca82a40`; provides the AWS Runtime platform module denominator.
- XCL-01 #495: merged by `c78c60f5a45a87a96159d4910a831b69b62b042c`; provides the cross-cloud Runtime Terraform conversion denominator.

## Disposition vocabulary

- `source-denominator`: canonical historical input or source denominator for this retirement decision.
- `rollback`: retained rollback path or rollback proof.
- `terraform-replacement`: Terraform replacement or parity evidence.
- `retained-evidence`: immutable/historical evidence that remains valid but is not active implementation authority.
- `follow-on`: referenced by future or adjacent work; no deletion or retirement authority here.
- `future-deletion-authority`: deletion can only happen in a later, separately authorized issue after live-stack and rollback evidence are proven.

## consumer-census

| Reference | Disposition | Evidence |
| --- | --- | --- |
| `adl/tools/issue194_private_network.cloudformation.json` | source-denominator, rollback, retained-evidence | Issue #194 private-network CloudFormation denominator retained for rollback and historical proof. |
| `adl/tools/issue268_runtime_qualification.cloudformation.yaml` | source-denominator, rollback, retained-evidence | Issue #268 runtime-qualification CloudFormation denominator retained for rollback and historical proof. |
| `.csdlc/issues/194/cards/sor.md` | retained-evidence | Historical #194 output card references the issue #194 template as delivered evidence. |
| `.csdlc/issues/194/cards/sor.values.json` | retained-evidence | Structured historical #194 output values reference the issue #194 template. |
| `.csdlc/issues/194/cards/spp.values.json` | retained-evidence | Structured historical #194 plan values reference the issue #194 template. |
| `.csdlc/issues/194/cards/srp.md` | retained-evidence | Historical #194 review card references the issue #194 template. |
| `.csdlc/issues/194/cards/srp.values.json` | retained-evidence | Structured historical #194 review values reference the issue #194 template. |
| `.csdlc/issues/194/index.json` | retained-evidence | Historical lifecycle index carries #194 template evidence references. |
| `.csdlc/issues/268/audit.jsonl` | retained-evidence | Historical #268 audit trail references the issue #268 qualification template. |
| `.csdlc/issues/268/cards/sor.md` | retained-evidence | Historical #268 output card references the issue #268 template. |
| `.csdlc/issues/268/cards/sor.values.json` | retained-evidence | Structured historical #268 output values reference the issue #268 template. |
| `.csdlc/issues/268/cards/srp.md` | retained-evidence | Historical #268 review card references the issue #268 template. |
| `.csdlc/issues/268/cards/srp.values.json` | retained-evidence | Structured historical #268 review values reference the issue #268 template. |
| `.csdlc/issues/268/index.json` | retained-evidence | Historical lifecycle index carries #268 template evidence references. |
| `.csdlc/issues/495/cards/stp.md` | terraform-replacement, retained-evidence | #495 uses both templates as CloudFormation denominators while recording Terraform replacement scope. |
| `.csdlc/issues/495/cards/stp.values.json` | terraform-replacement, retained-evidence | Structured #495 task values preserve the denominator-to-Terraform conversion relationship. |
| `.csdlc/prepared/issues/194/design.md` | retained-evidence | Historical #194 design names the private-network template denominator. |
| `.csdlc/prepared/issues/495/denominator-inventory.md` | terraform-replacement, retained-evidence | #495 denominator inventory maps CloudFormation denominators to Terraform replacement evidence. |
| `adl/tools/issue194_private_wuji_aws_runner.sh` | rollback, retained-evidence | Historical runner can still target the #194 template path for rollback/proof reproduction. |
| `adl/tools/run_issue268_six_hour_spot_qualification.sh` | rollback, retained-evidence | Historical #268 qualification runner references the CloudFormation template path. |
| `adl/tools/test_issue194_private_network_template.sh` | retained-evidence | Static validator for the retained #194 template. |
| `adl/tools/test_issue268_runtime_qualification_cloudformation.py` | retained-evidence | Static validator for the retained #268 template. |
| `adl/tools/test_run_issue268_six_hour_spot_qualification.sh` | retained-evidence | Shell proof around the historical #268 runner/template linkage. |
| `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml` | follow-on, retained-evidence | Sprint specification records AWS-G scope and the two CloudFormation denominators. |
| `docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh` | terraform-replacement, retained-evidence | XCL-01 validator references denominators while proving replacement evidence. |
| `docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md` | terraform-replacement, retained-evidence | XCL-01 proof references the denominators and replacement boundary. |
| `docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json` | retained-evidence | WP-01 issue-creation evidence preserves the AWS-G issue contract. |
| `docs/milestones/v0.92.1/evidence/wp-01/operations/015-aws-g-observed.json` | retained-evidence | WP-01 observed issue evidence preserves the AWS-G issue contract. |
| `docs/milestones/v0.92.1/evidence/wp-01/requests/015-aws-g-request.json` | retained-evidence | WP-01 request evidence preserves the AWS-G issue contract. |
| `infra/runtime-portable/runtime-workload-contract.v1.json` | terraform-replacement, follow-on | Portable runtime workload contract is a replacement denominator; no CloudFormation deletion authority is implied. |

## Retirement boundary

The accepted #496 disposition is retain/defer, not delete/retire. Terraform replacement evidence exists via #489/#495, but live-stack retirement, active-stack ownership, and deletion authority remain out of scope for #496.

Future template deletion or live-stack retirement requires separate issue authority that proves:

1. no active stack still depends on the template,
2. Terraform rollback/parity has been proven for the intended environment,
3. the operator explicitly authorizes deletion or retirement,
4. credential material remains outside repository evidence.

