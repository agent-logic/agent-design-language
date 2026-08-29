# Structured Task Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #493 GCP-D only; produce private platform Terraform, runbook, proof, and validator surfaces. Do not implement GCP-E, XCL-01, Observatory, Unity, production traffic, or AWS changes.

## Deliverables

- infra/gcp/platform Terraform root/module for private platform foundation
- docs/operations/cloud/gcp/platform-foundation operator runbook
- docs/milestones/v0.92.1/evidence/cloud/gcp-d retained proof packet
- .csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh
- typed C-SDLC v2 cards proving dependency, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: No unintended public route address or ingress exists
2. AC-2: Human and workload identities are separate
3. AC-3: State, artifacts, models, continuity evidence, and logs have separate owners
4. AC-4: A disposable non-GPU workload cleanup path has deterministic selectors and zero-residue proof
5. AC-5: Fresh exact-head review has no actionable findings before publication

## Dependencies

- GCP-C #492 terminal/merged organization and billing baseline; derived terminal cache observed with PR #580 merge b9a98710e2a0a50565c3835386f7f6a348a26eae

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#GCP-D
- docs/operations/cloud/gcp/organization-billing/README.md
- infra/gcp/organization/
- .git/csdlc-v2/derived-terminal/492.json
- docs/milestones/v0.92.1/evidence/cloud/gcp-c/

## Non Goals

- GPU qualification
- Production traffic
- Shared VPC expansion
- Unity proof
- Observatory implementation
- Cross-cloud Runtime Terraform conversion
- Static service-account-key creation
- Credential disclosure
