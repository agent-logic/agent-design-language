# #490 design — GCP hierarchy and cost decision

## Purpose

Produce one accepted GCP hierarchy and cost-envelope decision for Agent Logic cloud work without mutating GCP resources, enabling APIs, creating projects, or capturing credentials.

## Scope

Issue #490 owns:

- `docs/operations/cloud/gcp/decisions/**`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-a/**`

It records the exact observed organization/folder/project/billing basis, region and quota constraints, cost ceiling, and explicit POC versus long-term ownership choices.

## Approach

1. Use the operator-approved `gcloud auth login` account context; do not require a static service-account key.
2. Run read-only `gcloud` readbacks for active identity, project, billing account visibility, org/folder/project observations, policy/IAM summaries, quota/capacity, and region decision inputs.
3. Preserve redacted evidence and a decision register under owned paths.
4. Validate that quota is not treated as capacity and that the first workload has a hard cost ceiling.

## Non-goals

- No API enablement.
- No project creation.
- No Terraform apply.
- No paid launch.
- No static service-account key creation.

## Review focus

Review should confirm that identity/billing ambiguity fails closed, cost ceilings are explicit, quota is not overclaimed, and no credential material or mutation commands enter evidence.
