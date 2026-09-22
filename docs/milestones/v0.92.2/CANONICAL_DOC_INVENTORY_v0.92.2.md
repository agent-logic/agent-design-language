# v0.92.2 Canonical Document Inventory

Status: documentation handoff audit in progress under #917. Inventory completeness is not execution or release proof.

[Reviewer entry point](evidence/issue-917/HANDOFF.md) and [digest manifest](evidence/issue-917/HANDOFF_MANIFEST.json) identify the exact inspected documents, dependencies and unresolved acceptance boundaries.

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

The statistical study directory and `evidence/issue-523/` preserve historical observations and source provenance. Their original counts and recommendations are not the current task denominator and are not rewritten by this correction. The tracked simplification plan retains its original contract and current bounded completion/deferral overlay; its source-promotion manifest identifies the original promoted bytes rather than certifying later edits.

## Sidecar accounting

Podcast #671 is now assigned to v0.93, as is the deferred SIM-09 pilot #875. Both retain their original identities and authorization requirements. The historical 69-task creation roster is preserved for traceability; it is not a live milestone-membership count. The original sidecar admission and 82-membership census are creation-time history, not current scope.
## Complete milestone Markdown coverage

The tables above describe canonical planning surfaces; they are not the entire documentation denominator. #917 additionally reads and checks **every Markdown file recursively under this milestone**, including the nested documents below. The handoff validator derives this complete set from the filesystem and rejects an omitted or newly added Markdown file until the manifest is refreshed. Exact file bytes and classifications are in the handoff manifest.

“Historical evidence” means the original claim, candidate and review retain their source-time meaning; file presence is not current implementation proof. Current corrections are dated addenda or explicit superseding pointers, never fabricated historical results. Machine-readable proof archives remain their owners' evidence; this docs pass checks references and declared proof boundaries, not fresh execution of every historical proof.

| Nested document | Review treatment |
|---|---|
| [adr/issue-911/README.md](adr/issue-911/README.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-911/decision-dispositions.md](adr/issue-911/decision-dispositions.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-911/decision-inventory.md](adr/issue-911/decision-inventory.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-911/inventory-review.md](adr/issue-911/inventory-review.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-911/review.md](adr/issue-911/review.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-911/supersession-map.md](adr/issue-911/supersession-map.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-945/BETA1_DELIVERY.md](adr/issue-945/BETA1_DELIVERY.md) | Proposal history and accepted #945 successor authority checked separately |
| [adr/issue-945/README.md](adr/issue-945/README.md) | Proposal history and accepted #945 successor authority checked separately |
| [cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md) | Historical source/review evidence; preserve original proof limits |
| [cognitive-sdlc/REVISION_4_REVIEW_HANDOFF.md](cognitive-sdlc/REVISION_4_REVIEW_HANDOFF.md) | Historical source/review evidence; preserve original proof limits |
| [cognitive-sdlc/issue-1064/REMOVAL_PLAN.md](cognitive-sdlc/issue-1064/REMOVAL_PLAN.md) | Historical source/review evidence; preserve original proof limits |
| [cognitive-sdlc/statistical-review-2026-09-08/METHODS.md](cognitive-sdlc/statistical-review-2026-09-08/METHODS.md) | Historical source/review evidence; preserve original proof limits |
| [cognitive-sdlc/statistical-review-2026-09-08/RECOMMENDATIONS.md](cognitive-sdlc/statistical-review-2026-09-08/RECOMMENDATIONS.md) | Historical source/review evidence; preserve original proof limits |
| [cognitive-sdlc/statistical-review-2026-09-08/REPORT.md](cognitive-sdlc/statistical-review-2026-09-08/REPORT.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/MERGE_LINKAGE_849.md](evidence/MERGE_LINKAGE_849.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/conversion-rehearsal-872/README.md](evidence/conversion-rehearsal-872/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-1109/README.md](evidence/issue-1109/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-1129/README.md](evidence/issue-1129/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-1132/README.md](evidence/issue-1132/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-523/README.md](evidence/issue-523/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-523/simplification-addition.md](evidence/issue-523/simplification-addition.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/issue-916/GAP_ANALYSIS.md](evidence/issue-916/GAP_ANALYSIS.md) | Dated evolving qualification checkpoint; not release acceptance |
| [evidence/issue-917/HANDOFF.md](evidence/issue-917/HANDOFF.md) | Current review handoff and local validation contract |
| [evidence/qual-evidence-902/README.md](evidence/qual-evidence-902/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/qual-inventory-899/README.md](evidence/qual-inventory-899/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/qual-provider-901/README.md](evidence/qual-provider-901/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/qual-resident-900/README.md](evidence/qual-resident-900/README.md) | Historical source/review evidence; preserve original proof limits |
| [evidence/qual-runtime-852/README.md](evidence/qual-runtime-852/README.md) | Historical source/review evidence; preserve original proof limits |
| [repository-decomposition/CLAUDE_REVIEW_1.md](repository-decomposition/CLAUDE_REVIEW_1.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/CLAUDE_REVIEW_2.md](repository-decomposition/CLAUDE_REVIEW_2.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/FINDINGS.md](repository-decomposition/FINDINGS.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/GEMINI_REVIEW_1.md](repository-decomposition/GEMINI_REVIEW_1.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/PLAN.md](repository-decomposition/PLAN.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/REVIEWED_PLAN_848.md](repository-decomposition/REVIEWED_PLAN_848.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/rd01-977/AUDIT.md](repository-decomposition/rd01-977/AUDIT.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |
| [repository-decomposition/rd01-977/REVIEW.md](repository-decomposition/rd01-977/REVIEW.md) | Planning/audit scope; Claude F10 fix and no-extraction boundary checked |

## Nested decision validation inputs

- [#911 proposal validator](adr/issue-911/validate_packet.py) preserves original proposed-record and source identity.
- [#945 acceptance validator](adr/issue-945/validate_packet.py) checks all twelve accepted decisions and their approval bindings.
- [#945 content manifest](adr/issue-945/candidate-content.json) distinguishes unchanged accepted decision bytes from supporting planning documents refreshed by #917.

Run both validators with `--self-test` from the repository root. A refreshed supporting-document digest is not new ADR acceptance and must not alter accepted records or approval hashes.
