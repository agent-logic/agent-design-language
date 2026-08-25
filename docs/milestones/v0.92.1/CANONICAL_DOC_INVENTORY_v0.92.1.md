# v0.92.1 Canonical Document Inventory

Status: planning candidate. This inventory proves package completeness; it does not create issues, approve execution, or claim release readiness.

## Canonical Planning Package

| Surface | File | Purpose |
|---|---|---|
| Entry point | [README.md](README.md) | Milestone status, scope, boundaries, and document map |
| Vision | [VISION_v0.92.1.md](VISION_v0.92.1.md) | Intended milestone outcome |
| Design | [DESIGN_v0.92.1.md](DESIGN_v0.92.1.md) | Cross-lane architecture and authority boundaries |
| Decisions | [DECISIONS_v0.92.1.md](DECISIONS_v0.92.1.md) | Planning decisions and non-claims |
| Work breakdown | [WBS_v0.92.1.md](WBS_v0.92.1.md) | Work-package decomposition |
| Sprint plan | [SPRINT_v0.92.1.md](SPRINT_v0.92.1.md) | Dependency-ordered sprint sequence |
| Planned issue catalog | [PLANNED_ISSUE_CATALOG_v0.92.1.md](PLANNED_ISSUE_CATALOG_v0.92.1.md) | Existing issues and number-free WP-01 creation plan |
| Issue wave | [WP_ISSUE_WAVE_v0.92.1.yaml](WP_ISSUE_WAVE_v0.92.1.yaml) | Machine-readable package dependencies |
| Execution specifications | [WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml](WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml) | Lane outcomes and proof contracts |
| Execution readiness | [WP_EXECUTION_READINESS_v0.92.1.md](WP_EXECUTION_READINESS_v0.92.1.md) | Preconditions for opening execution |
| Retirement ledger | [WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml](WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml) | Preserved #149-#190 requirement routing |
| Feature index | [features/README.md](features/README.md) | Index of milestone feature surfaces |
| Feature/proof coverage | [FEATURE_PROOF_COVERAGE_v0.92.1.md](FEATURE_PROOF_COVERAGE_v0.92.1.md) | Feature-to-proof ownership |
| Quality gate | [QUALITY_GATE_v0.92.1.md](QUALITY_GATE_v0.92.1.md) | Required integrated validation |
| Demo matrix | [DEMO_MATRIX_v0.92.1.md](DEMO_MATRIX_v0.92.1.md) | Demonstration and non-claim boundaries |
| Milestone checklist | [MILESTONE_CHECKLIST_v0.92.1.md](MILESTONE_CHECKLIST_v0.92.1.md) | Completion checklist |
| ADR plan | [ADR_PLAN_v0.92.1.md](ADR_PLAN_v0.92.1.md) | Architecture-decision routing |
| Release plan | [RELEASE_PLAN_v0.92.1.md](RELEASE_PLAN_v0.92.1.md) | Canonical release-tail gates |
| Release notes | [RELEASE_NOTES_v0.92.1.md](RELEASE_NOTES_v0.92.1.md) | Draft release-note surface |
| Successor handoff | [NEXT_MILESTONE_HANDOFF_v0.92.1.md](NEXT_MILESTONE_HANDOFF_v0.92.1.md) | v0.92.2 CodeFriend Beta 1 handoff |

## Issue-Creation Boundary

Existing issue authority in this package is #432, #51, #261, #262, #263, #264, #342, #251, #122, #84, and #345. Closed #431 is planning provenance only. WP-01 remains a number-free milestone-opening ID after this package merges and until the operator separately declares v0.92.1 ready and creates it; every CORP, AWS, GCP, XCL, RUST, V3, DRT, HOT, OBS, DEC, PROV, INT, and TAIL entry remains number-free until WP-01 creates the ordered wave. Closed #433-#438 and redundant #439 are not execution authority. #457 is historical provider-profile provenance only. #269 remains excluded/backlogged. Closed #188, #190, and #189 route respectively to convergence/quality, successor planning, and final ceremony.

## Canonical Release Tail

The serial denominator matches the preceding milestone standard: TAIL-01 quality gate, TAIL-02 docs/release truth, TAIL-03 publication finalization, TAIL-04 internal review, TAIL-05 external review, TAIL-06 remediation/preflight, TAIL-07 next-milestone planning, TAIL-08 next-milestone closeout planning, TAIL-09 next-milestone planning review, and TAIL-10 release ceremony.

## Validation Boundary

The issue-owned planning validator must require every surface in this inventory, parse both YAML contracts, preserve the full predecessor and existing-issue denominator, reject tracked local-path dependencies, verify the exact 45-entry number-free creation catalog including AWS-A through AWS-G, GCP-A through GCP-E, XCL-01, RUST-01, DEC-01, PROV-A, PROV-B, and DRT-D, and enforce the ten-step serial release tail. Passing planning validation is not implementation or release proof.
