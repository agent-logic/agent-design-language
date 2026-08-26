# Structured Task Prompt

Template: 1.0.0

Issue: 480

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Create and verify the exact issue wave; do not implement any child issue.

## Deliverables

- docs/milestones/v0.92.1/evidence/wp-01/creation-plan.json
- docs/milestones/v0.92.1/evidence/wp-01/partial-receipt.json
- docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json
- .csdlc/prepared/issues/480/validate-wave-creation.rb

## Acceptance

1. AC-1: Exactly 45 ordered creation slots and complete issue specifications are derived from merged authority.
2. AC-2: Duplicate IDs, operation keys, conflicting titles, unresolved dependencies, and extra slots fail before mutation.
3. AC-3: Every create is followed by exact live readback and immutable receipt append.
4. AC-4: Partial failure resumes from verified live state without duplicate creation or renumbering.
5. AC-5: Existing #51, #84, #122, #251, #261-#264, #342, and #345 are reconciled without replacement and #269 remains excluded.
6. AC-6: Final independent live readback proves exactly 45 unique children with exact title, labels, milestone, dependencies, and body/spec identity.
7. AC-7: Independent exact-head review passes before publication.

## Dependencies

- #432 merged and ancestral
- PR #479 merged as aa5766d71864713b97210abdb5aa8e5c2481ed31
- operator declaration opening v0.92.1

## Inputs

- agent-logic/agent-design-language#480
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_READINESS_v0.92.1.md

## Non Goals

- Implement child work
- Delete or recycle issue numbers
- Recreate existing issues
- Activate v0.93
- Tag or release mutation
