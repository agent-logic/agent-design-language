# Structured Task Prompt

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #494 GCP-E only; produce and execute one bounded GCP L4 smoke readiness decision with cleanup proof. Do not implement DRT-D six-resident qualification, XCL-01 parity, AWS work, production traffic, Observatory, Unity, or credential-bearing evidence outside the approved paid lane.

## Deliverables

- infra/gcp/workloads/gpu-smoke Terraform/root scripts for one disposable On-Demand L4 smoke workload
- docs/milestones/v0.92.1/evidence/cloud/gcp-e retained readiness decision and proof packet
- .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh
- typed C-SDLC v2 cards proving dependency, paid authorization, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: Paid launch has separate authorization and a USD 20 ceiling
2. AC-2: Exact inputs and hardware are retained
3. AC-3: GPU inference and headroom are proven
4. AC-4: All owned resources are independently absent afterward
5. AC-5: Fresh exact-head review has no actionable findings before publication

## Dependencies

- GCP-D #493 terminal/merged private platform foundation truth; derived terminal cache observed with PR #587 merge c0bf217934508d6dbc70d78633e6a95d5ddd9d06

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#GCP-E
- docs/milestones/v0.92.1/features/GCP_ACCOUNT_MOVE_IN_v0.92.1.md
- docs/milestones/v0.92.1/evidence/cloud/gcp-d/
- infra/gcp/platform/
- .git/csdlc-v2/derived-terminal/493.json

## Non Goals

- Six-resident qualification
- Spot substitution
- Production deployment
- Persistent public exposure or DNS
- AWS changes
- XCL-01 cross-cloud parity
- DRT-D portability qualification
- Unity proof
- Observatory implementation
- Credential disclosure
