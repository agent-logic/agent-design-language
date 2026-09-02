# Issue 495 design: XCL-01 cross-cloud Runtime Terraform conversion

Status: design candidate for review.

## Boundary

Issue #495 owns one portable Runtime workload contract and explicit AWS/GCP
Terraform implementations for the already-admitted issue #194 and #268
CloudFormation-template denominator.

The pre-bind denominator authority is
`.csdlc/prepared/issues/495/denominator-inventory.md`. That inventory names the
exact source CloudFormation templates, preserves each admitted behavior as a
portable-contract row, and identifies the provider-specific AWS/GCP mapping that
must remain visible after binding.

It consumes:

- #488 AWS-E terminal adoption-register truth.
- #493 GCP-D terminal private-platform-foundation truth.

It does not own:

- #496 AWS-G CloudFormation retirement.
- #494 GCP GPU smoke qualification.
- DRT-D six-resident portability qualification.
- production cutover, DNS, or public exposure.
- credential-bearing live proof without explicit operator authorization.

## Design

The implementation should create three separable surfaces:

1. `infra/runtime-portable/**`
   - provider-neutral workload contract;
   - denominator map for #194 and #268 behavior, seeded from the pre-bind
     denominator inventory;
   - common inputs/outputs and invariants.
2. `infra/aws/runtime/**`
   - AWS Terraform preserving the admitted CloudFormation behavior;
   - explicit AWS IAM, networking, state, tags, deadlines, and rollback notes;
   - CloudFormation templates remain rollback authority until #496.
3. `infra/gcp/workloads/**`
   - GCP Terraform preserving equivalent Runtime workload behavior;
   - explicit GCP service account, network, artifact/state, tags/labels,
     deadline, and cleanup selectors;
   - consumes the #493 platform foundation without mutating it.

Provider-neutrality is a contract layer only. Provider-specific security,
identity, network, state, and cleanup differences stay visible and reviewable.

## Proof plan

Pre-bind proof checks only issue readiness:

- #488 and #493 terminal caches exist and are merged.
- denominator inventory exists and cites the exact #194/#268 CloudFormation
  templates with provider-neutral mapping rows;
- issue body and design declare the owned paths and non-goals.
- paid/live apply and destroy proof is explicitly gated.

Post-bind local proof checks static product truth:

- exact #194/#268 denominator inventory is present;
- portable contract exists and references both provider implementations;
- AWS and GCP implementation docs expose provider-specific differences;
- rollback and zero-residue cleanup evidence is represented;
- no credential material is embedded.

Live parity proof is separate and may run only after explicit operator
authorization for paid/cloud mutation. It must record exact inputs, plans,
deployment identity, cleanup selectors, and independent zero-residue readback.

## Review questions

- Does the design keep #495 scoped to XCL-01 and avoid #494/#496/DRT-D scope?
- Does the denominator inventory make #194/#268 behavior reviewable before
  binding?
- Does it preserve CloudFormation rollback authority until #496?
- Does the portable contract avoid hiding AWS/GCP security differences?
- Does proof truth distinguish static validation from paid/live parity?
