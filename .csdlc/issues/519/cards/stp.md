# Structured Task Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one exact-revision publication-candidate packet; linkage and redaction checks are evidence inputs.

## Deliverables

- docs/milestones/v0.92.1/evidence/release/tail-03
- .csdlc/prepared/issues/519/validate-publication-candidate.rb

## Acceptance

1. AC-1: The exact reviewed candidate revision and artifact digests are recorded
2. AC-2: Publication artifacts contain correct and unambiguous closing relationships
3. AC-3: The packet passes redaction and path-portability checks
4. AC-4: No merge, tag, release, or issue-close mutation occurs

## Dependencies

- TAIL-02/#518 reviewed merge is required for final acceptance; operator explicitly authorized preparatory work before merge.

## Inputs

- agent-logic/agent-design-language#519
- agent-logic/agent-design-language#518
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-03
- docs/milestones/v0.92.1/evidence/release/**

## Non Goals

- Merge
- Tag
- Release
- Issue closure
- Product or documentation repair
