# Structured Task Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair only the V3-A contract/proof artifacts and issue-local validator surfaces needed to address the four recorded #500/#565 findings.

## Deliverables

- Updated predecessor-coverage data with explicit owner issue and proof lane for every retained #161-#163 row.
- Updated CONTRACT.md construction-decision evidence for the measured #162 slice, thresholds or decision criteria, and #163/Decision 11 approval binding.
- Updated proportional-lifecycle default path so retained bind, publication, finish, and cleanup gates cannot be silently omitted.
- Updated or successor V3-A validator that rejects missing/empty/duplicated/broad-only owner/lane data and checks exact-range diff hygiene.
- Focused validation evidence for predecessor mapping, construction decision evidence, lifecycle gate consistency, and exact-range diff hygiene.

## Acceptance

1. AC-1: Every retained #161-#163 predecessor row has an explicit owner issue and proof lane; validation rejects missing, empty, duplicated, or broad-only owner/lane data.
2. AC-2: CONTRACT.md records V3-A construction-decision evidence, including measured #162 construction-slice disposition, thresholds or decision criteria, and #163 approval/Decision 11 binding.
3. AC-3: proportional-lifecycle.json default lifecycle path cannot omit retained bind, publication, finish, or cleanup gates; optional/deferred gate status is explicit and justified.
4. AC-4: The V3-A implementation validator or successor checks git diff --check against the exact PR base/head range, not only the working tree.
5. AC-5: Validation includes exact-range diff hygiene and focused V3-A contract/proof checks.

## Dependencies

- Merged #500/PR #565 provides the historical V3-A artifacts being corrected.
- Review findings from #500/#565 define the corrective scope.
- Sprint 6 #534 requires #571 resolved or explicitly dispositioned before V3-F/#505 authority transition.

## Inputs

- agent-logic/agent-design-language#571
- agent-logic/agent-design-language#500
- agent-logic/agent-design-language#565
- agent-logic/agent-design-language#534
- agent-logic/agent-design-language#505
- docs/csdlc-v3/CONTRACT.md
- docs/csdlc-v3/predecessor-coverage.json
- docs/csdlc-v3/proportional-lifecycle.json
- .csdlc/prepared/issues/500/validate-implementation.rb

## Non Goals

- Reopening, rewriting, or relabeling #500/#565 historical review truth as passing.
- Implementing later v3 runtime/lifecycle slices.
- Moving authority from v2 to v3.
- Broad documentation cleanup outside the V3-A corrective surfaces.
