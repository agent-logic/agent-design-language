# v0.92.2 Canonical Document Inventory

Status: planning candidate. Inventory completeness is not execution or release proof.

| Surface | File |
|---|---|
| C-SDLC simplification first sprint | [C_SDLC_V3_SIMPLIFICATION_PLAN.md](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md) |
| Statistical study | [REPORT.md](cognitive-sdlc/statistical-review-2026-09-08/REPORT.md) |
| Study methods | [METHODS.md](cognitive-sdlc/statistical-review-2026-09-08/METHODS.md) |
| Study recommendations | [RECOMMENDATIONS.md](cognitive-sdlc/statistical-review-2026-09-08/RECOMMENDATIONS.md) |
| Source provenance | [source-promotion-manifest.json](cognitive-sdlc/source-promotion-manifest.json) |
| Entry point | [README.md](README.md) |
| Vision | [VISION_v0.92.2.md](VISION_v0.92.2.md) |
| Adopted shared contracts | [ADOPTED_DESIGN_CONTRACTS_v0.92.2.md](ADOPTED_DESIGN_CONTRACTS_v0.92.2.md) |
| Existing issue reconciliation | [EXISTING_ISSUE_RECONCILIATION_v0.92.2.md](EXISTING_ISSUE_RECONCILIATION_v0.92.2.md) |
| Atomic task contracts | [ATOMIC_TASK_CONTRACTS_v0.92.2.md](ATOMIC_TASK_CONTRACTS_v0.92.2.md) |
| Atomic task manifest | [ATOMIC_TASK_CONTRACTS_v0.92.2.json](ATOMIC_TASK_CONTRACTS_v0.92.2.json) |
| Planning validator | [validate_planning.py](validate_planning.py) |
| Atomic task validator | [validate_atomic_tasks.py](validate_atomic_tasks.py) |
| Launch binding validator | [validate_launch_bindings.py](validate_launch_bindings.py) |
| Concrete creation selections | [CREATION_SELECTIONS_v0.92.2.md](CREATION_SELECTIONS_v0.92.2.md) |
| Complete reviewed launch map | [complete reviewed launch map](../../../.csdlc/evidence/864/all-issue-launch.json) |
| Creation batch contract | [ISSUE_CREATION_BATCHES_v0.92.2.json](ISSUE_CREATION_BATCHES_v0.92.2.json) |
| Design | [DESIGN_v0.92.2.md](DESIGN_v0.92.2.md) |
| Decisions | [DECISIONS_v0.92.2.md](DECISIONS_v0.92.2.md) |
| Work breakdown | [WBS_v0.92.2.md](WBS_v0.92.2.md) |
| Sprint plan | [SPRINT_v0.92.2.md](SPRINT_v0.92.2.md) |
| Planned issue catalog | [PLANNED_ISSUE_CATALOG_v0.92.2.md](PLANNED_ISSUE_CATALOG_v0.92.2.md) |
| Issue wave | [WP_ISSUE_WAVE_v0.92.2.yaml](WP_ISSUE_WAVE_v0.92.2.yaml) |
| Execution specifications | [WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml](WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml) |
| Execution readiness | [WP_EXECUTION_READINESS_v0.92.2.md](WP_EXECUTION_READINESS_v0.92.2.md) |
| TBD scheduling reconciliation | [TBD_SCHEDULING_RECONCILIATION_v0.92.2.md](TBD_SCHEDULING_RECONCILIATION_v0.92.2.md) |
| TBD source audit manifest | [TBD_SOURCE_AUDIT_MANIFEST_v0.92.2.txt](TBD_SOURCE_AUDIT_MANIFEST_v0.92.2.txt) |
| Feature index | [features/README.md](features/README.md) |
| Feature/proof coverage | [FEATURE_PROOF_COVERAGE_v0.92.2.md](FEATURE_PROOF_COVERAGE_v0.92.2.md) |
| Quality gate | [QUALITY_GATE_v0.92.2.md](QUALITY_GATE_v0.92.2.md) |
| Demo matrix | [DEMO_MATRIX_v0.92.2.md](DEMO_MATRIX_v0.92.2.md) |
| Milestone checklist | [MILESTONE_CHECKLIST_v0.92.2.md](MILESTONE_CHECKLIST_v0.92.2.md) |
| ADR plan | [ADR_PLAN_v0.92.2.md](ADR_PLAN_v0.92.2.md) |
| Release plan | [RELEASE_PLAN_v0.92.2.md](RELEASE_PLAN_v0.92.2.md) |
| Release notes | [RELEASE_NOTES_v0.92.2.md](RELEASE_NOTES_v0.92.2.md) |
| Successor handoff | [NEXT_MILESTONE_HANDOFF_v0.92.2.md](NEXT_MILESTONE_HANDOFF_v0.92.2.md) |

## Feature Documents

| Surface | File |
|---|---|
| Feature index | [features/README.md](features/README.md) |
| Product shell | [features/PRODUCT_SHELL_AND_OPERATOR_CONTROLS_v0.92.2.md](features/PRODUCT_SHELL_AND_OPERATOR_CONTROLS_v0.92.2.md) |
| Portable adapter | [features/PORTABLE_ADAPTER_V2_v0.92.2.md](features/PORTABLE_ADAPTER_V2_v0.92.2.md) |
| Evidence core | [features/EVIDENCE_CORE_v0.92.2.md](features/EVIDENCE_CORE_v0.92.2.md) |
| Architecture cognition | [features/ARCHITECTURE_COGNITION_v0.92.2.md](features/ARCHITECTURE_COGNITION_v0.92.2.md) |
| Executable governance | [features/EXECUTABLE_GOVERNANCE_v0.92.2.md](features/EXECUTABLE_GOVERNANCE_v0.92.2.md) |
| Multi-perspective review | [features/MULTI_PERSPECTIVE_REVIEW_v0.92.2.md](features/MULTI_PERSPECTIVE_REVIEW_v0.92.2.md) |
| Longitudinal memory | [features/LONGITUDINAL_REVIEW_MEMORY_v0.92.2.md](features/LONGITUDINAL_REVIEW_MEMORY_v0.92.2.md) |
| Governed publication | [features/GOVERNED_PUBLICATION_v0.92.2.md](features/GOVERNED_PUBLICATION_v0.92.2.md) |
| Beta 1 qualification | [features/BETA1_QUALIFICATION_v0.92.2.md](features/BETA1_QUALIFICATION_v0.92.2.md) |
| Supporting tracks | [features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md](features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md) |

## Validation Contract

The package validator should require every inventory row, parse both YAML files, reject unresolved placeholders and machine-local paths, verify all 69 rows match proved native identities, with admitted existing bindings reused exactly once and no unassigned row, enforce the exact ten-step tail, and resolve all relative Markdown links.

## Current scope and historical evidence

Active Markdown projections, both execution YAML files and the atomic-task manifest must agree on the 69-task wave, 69 assigned core issues (nine preexisting and 60 newly created), with zero unassigned tasks, eight splits, eleven completion contracts and seven retained planning tasks. Feature owner tables and release proof must use the split consumers rather than the former bundled owners.

The statistical study directory and `evidence/issue-523/` preserve historical observations and source provenance. Their original counts and recommendations are not the current task denominator and are not rewritten by this correction. The tracked simplification plan remains active planning; its source-promotion manifest identifies the original promoted bytes rather than certifying later edits.

## Sidecar accounting

The core-plan inventory remains exactly 69 tasks. Existing #671 is separately admitted milestone sidecar membership, with its own action approvals, outside the core startup gate and dependency graph. Final membership accounting must distinguish the 69 core identities from this one sidecar rather than dropping it or altering the atomic denominator.
