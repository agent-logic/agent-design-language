# #484 design — AWS resource ownership inventory

## Purpose

Produce one accepted all-region AWS resource ownership inventory for the Agent Logic business AWS account without mutating AWS resources, Terraform state, GitHub state, or credential material.

## Scope

Issue #484 owns:

- `docs/operations/cloud/aws/inventory/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-a/**`

It classifies discovered AWS resources by owner and lifecycle disposition, including retained assets that must not be inferred disposable.

## Approach

1. Verify the approved AWS account identity through the configured business profile.
2. Enumerate enabled regions and service surfaces with read-only AWS CLI calls.
3. Preserve redacted command summaries and normalized inventory tables under the owned evidence paths.
4. Classify each discovered resource as owned, externally owned, frozen-unknown, or not observed for the inspected surface.
5. Validate that the inventory has no unclassified discovered resource and that evidence does not contain credentials.

## Non-goals

- No Terraform apply.
- No resource import.
- No cleanup or deletion.
- No website or DDNS state migration.
- No credential, token, or secret capture.

## Review focus

Review should confirm that the inventory denominator is explicit, the AWS account/region basis is evidence-backed, each discovered resource has a disposition, and the evidence path is redacted and issue-scoped.
