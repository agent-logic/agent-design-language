# Structured Task Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Build one complete release-tail census and gap-analysis packet, then emit one fail-closed admission decision.

## Deliverables

- Immutable release-tail admission JSON
- Findings-first gap analysis in Markdown and JSON
- Exact issue, PR, revision, ancestry, artifact, and owner denominator
- Updated demo matrix only where observed evidence requires correction

## Acceptance

1. AC-1: Every planned v0.92.1 issue and retained predecessor dependency is enumerated with no silent omission
2. AC-2: Every required lane is reviewed, merged, ancestral, and indexed by exact revision and artifact
3. AC-3: Acceptance criteria and execution specifications are compared with real production implementation and execution evidence
4. AC-4: Test-only, placeholder, unused, partially wired, do-nothing, or unexecuted implementations are identified and cannot pass admission
5. AC-5: Missing validation, stale review, documentation drift, collision, and closeout overclaim findings carry severity, evidence, uncertainty, disposition, and owner
6. AC-6: Gap results are classified as release blockers, durable proof gaps, routed work, stale readiness documentation, or non-blocking quality concerns
7. AC-7: No unresolved P0/P1 gap or unowned material gap remains in an admitted candidate
8. AC-8: Machine-readable and human-readable gap reports and the admission record agree on denominator, revisions, findings, and decision

## Dependencies

- #498 CORP-D
- #496 AWS-G
- #494 GCP-E
- #495 XCL-01
- #499 RUST-01
- #505 V3-F
- #508 DRT-C
- #509 DRT-D
- #51 podcast mini-sprint
- #510 HOT-01
- #512 OBS-B
- #513 DEC-01
- #515 PROV-B

## Inputs

- agent-logic/agent-design-language#516
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md
- .csdlc/issues/**
- .git/csdlc-v2/derived-terminal/**

## Non Goals

- Implement child fixes
- Create duplicate remediation issues
- Perform live cloud or runtime mutations
- Approve the release
