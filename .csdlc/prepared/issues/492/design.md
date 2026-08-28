# Issue #492 design: GCP-C organization and billing baseline

## Outcome

Issue #492 produces one operational GCP organization and billing baseline for the approved Agent Logic GCP denominator, using #490 decision evidence and #491 Terraform bootstrap as inputs.

The result is a governed baseline, not a production activation. It must prove folder/project denominator, billing linkage, scoped policy posture, administrative ownership, budget/export observability, and label/cost attribution without silently changing unrelated POC resources.

## Dependencies

- #490 GCP hierarchy and cost decision is terminal.
- #491 GCP Terraform bootstrap is terminal and ancestral to the implementation base.

## Owned paths

- `infra/gcp/organization/**`
- `docs/operations/cloud/gcp/organization-billing/**`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-c/**`
- `.csdlc/prepared/issues/492/**`
- `.csdlc/evidence/492/**`

## Non-goals

- Runtime deployment.
- GPU launch or GPU quota qualification.
- AWS organization changes.
- Broad production hierarchy cutover beyond the approved denominator.
- Static service-account-key creation.
- Credential disclosure, credential copying, or credential material retention.

## Baseline contract

The implemented baseline must record:

- approved organization, folder, project, and billing account denominator;
- corporate group ownership for new managed projects;
- explicit cost-attribution labels and budget/export observability;
- scoped organization-policy impact review;
- Terraform backend and deployment identity source inherited from #491;
- exact readback commands and retained evidence locations;
- explicit unchanged status for existing POC resources unless the issue admits a reviewed exception.

## Validation lanes

Pre-bind lanes are local and non-mutating:

- `prebind-gcp-org-billing-packet` checks design, diagram, dependency inputs, owned paths, and stop conditions.
- `prebind-gcp-org-readback-static` checks the readback entrypoint without credentials or cloud mutation.
- `prebind-review-readiness` proves the packet is ready for design review.

Post-bind lanes are deferred until implementation exists:

- `gcp-c-organization-static` runs the validator in `--phase=postbind` mode and validates Terraform/docs/evidence contract plus unchanged POC boundaries.
- `gcp-c-readback` performs approved read-only `gcloud` project, billing, optional folder, and optional organization-policy readbacks without printing names, IDs, or credentials.
- `exact-head-review` records a fresh implementation review before publication.

## Stop conditions

Stop before bind or implementation if:

- #491 terminal cache or ancestry is not current;
- the live issue contract changes the approved denominator;
- any plan requires broad policy impact outside reviewed scope;
- individual-only ownership remains the only way forward;
- cost attribution or billing export cannot be represented truthfully;
- credential material would need to be read, copied, printed, or committed.
