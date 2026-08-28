# Structured Task Prompt

Template: 1.0.0

Issue: 490

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one GCP hierarchy/cost decision register and retained redacted read-only evidence packet.

## Deliverables

- Organization/folder/project/billing/region decision
- POC and long-term ownership distinction
- First workload hard cost ceiling
- Quota-not-capacity statement
- Redaction and no-mutation validation proof
- Fresh exact-head review and closing PR

## Acceptance

1. AC-1: Organization, folder, project, billing, and region are exact or explicitly blocked.
2. AC-2: POC and long-term ownership are explicit.
3. AC-3: The first workload has a hard cost ceiling.
4. AC-4: Quota is not treated as capacity.
5. AC-5: Evidence contains no credentials, secrets, raw token contents, or mutation commands.
6. AC-6: Fresh exact-head review has zero actionable findings and publication truth closes only issue #490.

## Dependencies

- none

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/490
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#GCP-A
- docs/milestones/v0.92.1/features/GCP_ACCOUNT_MOVE_IN_v0.92.1.md
- .csdlc/prepared/issues/490/design.md

## Non Goals

- API enablement
- Project creation
- Paid launch
- Terraform apply
- Static service-account key creation
