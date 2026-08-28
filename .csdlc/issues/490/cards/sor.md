# Structured Output Record

Template: 1.0.0

Issue: 490

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Accepted the GCP-A hierarchy and cost decision from refreshed read-only GCP evidence, preserved no-mutation and redaction proof, and documented the company-controlled execution identity plan for #491+.

## Artifacts

- docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/readbacks
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/run-readonly-decision-readbacks.sh
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/gcp-execution-identity-plan.md
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/operator-reauth-proof-packet.md
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/readiness-repair.md
- .csdlc/prepared/issues/490/validate-gcp-a-decision.sh

## Execution

- Retained read-only GCP hierarchy, billing, IAM, service, quota, and network readbacks under the #490 evidence directory.
- Accepted the long-term GCP organization, folder, project, billing account, region, data-residency, quota, and cost-envelope decision where readbacks support it.
- Recorded the #491+ Terraform execution identity recommendation without mutating GCP or creating service-account keys.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/490/validate-gcp-a-decision.sh",
      "."
    ],
    "purpose": "Prove the issue-owned GCP-A decision denominator is locally coherent and safe to review.",
    "outcome": "passed",
    "evidence_ref": "decision-denominator.log"
  },
  {
    "command": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "490"
    ],
    "purpose": "Prove #490 typed state remains coherent before exact-head review.",
    "outcome": "passed",
    "evidence_ref": "typed-doctor.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
