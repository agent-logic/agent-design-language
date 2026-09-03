# Structured Task Prompt

Template: 1.0.0

Issue: 507

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #507 DRT-B only; produce the six-resident UTS qualification implementation, evidence, and validators. Do not absorb #508, #509, #114/#115 descendants, cloud-provider setup, or optional paid launch beyond an explicitly authorized bounded proof.

## Deliverables

- Deterministic DRT-B six-resident qualification contract/proof surface
- Focused continuity, dehydrate/restore, replay, reclamation, and cleanup evidence under docs/milestones/v0.92.1/evidence/runtime/drt-b/
- .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh
- Issue-owned validation lane for six-resident UTS, continuity reclamation, cost/resource envelope checks, and cleanup-zero checks
- Typed C-SDLC v2 cards proving dependency, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: Requirements 183 and 184 are mapped to explicit DRT-B checks
2. AC-2: Six distinct residents complete assigned UTS work with exact identity and lineage receipts
3. AC-3: Dehydrate and restore preserve exact population, resident identity, lineage, and workload receipt set
4. AC-4: Reclamation, cost/resource-envelope, and cleanup-zero predicates are proven or truthfully gated when a paid GPU proof is required
5. AC-5: #506 and #345 predecessor authority is consumed read-only without absorbing their implementation scope
6. AC-6: Fresh exact-head review has no actionable findings before publication

## Dependencies

- #506 DRT-A terminal/merged distributed qualification contract; derived terminal cache observed at merge badcf9067da6eb46fc9f59e9da8b11a41e2f24f6
- #345 AWS GPU Shepherd proof-runner authority closed in GitHub; no local derived-terminal cache observed during bootstrap

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#DRT-B
- .git/csdlc-v2/derived-terminal/506.json
- adl-runtime/src/qualification/**
- adl-runtime-kernel/src/**
- adl/tools/run_issue268_**
- docs/milestones/v0.92.1/evidence/runtime/drt-a/**
- docs/milestones/v0.92.1/evidence/runtime/drt-b/**

## Non Goals

- Final DRT-C distributed Runtime qualification
- GCP DRT-D portability qualification
- Unbounded soak
- Production cloud/provider cutover
- New continuity architecture unrelated to the DRT-B proof
- Credential discovery or retention
