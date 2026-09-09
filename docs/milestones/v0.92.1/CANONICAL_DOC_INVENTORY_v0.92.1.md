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

The original creation specification below is retained as planning history. WP-01/#480 created the execution wave; `evidence/wp-01/final-creation-receipt.json` is the exact creation mapping. Its source-time issue states do not establish present acceptance. #84 and #251 are deferred backlog; #122 and #345 retain their owned scope. Current issue bodies and reviewed evidence govern later amendments. Closed #431, #433-#439 and #457 remain historical provenance.

## Release Closeout Review Denominator

The planning package above is only part of the release documentation review. The
active release-plan template, `docs/templates/planning/1.1.0/release_plan.md`,
requires the following additional surfaces to be reviewed. Inclusion is a review
obligation, not a claim that a gate has passed.

| Surface | Review obligation |
|---|---|
| Root and component README files; all AGENTS guidance | Reconcile current entry points and operational authority; preserve explicitly historical guidance as historical |
| CHANGELOG.md, REVIEW.md, docs/README.md, CONTRIBUTING.md, adl/CONTRIBUTING.md | Separate released behavior, current work, and deferred scope |
| adl/Cargo.toml, adl/kernel/Cargo.toml, csdlc-v3/Cargo.toml | Verify package identity and version intent; do not equate every package version with the milestone number |
| docs/planning/ADL_FEATURE_LIST.md | Map delivered, deferred, and unproven claims to source evidence |
| Coverage/test, Rust module, and active gap/risk trackers | Identify the current tracker or record its absence explicitly; require current quality evidence from TAIL-01 |
| Integration gap analysis and admission evidence | Distinguish a completed diagnostic from a passing admission decision |
| Closed-issue records and review dispositions | Reconcile observed issue/PR state with retained records; preserve historical proof |
| End-of-milestone report and successor handoff | Record residual risks, deferred work, and exact revision references before ceremony |

TAIL-02 records each surface's revision, disposition, and supporting evidence in
its review packet. Missing evidence remains an open finding. Final handoff waits
for the TAIL-01 passing quality result and reviewed merge; parallel documentation
corrections do not establish release readiness.

## Canonical Release Tail

The serial denominator matches the preceding milestone standard: TAIL-01 Quality gate; TAIL-02 Documentation review and external-review handoff; TAIL-03 Publication finalization; TAIL-04 Internal review; TAIL-05 External / third-party review; TAIL-06 Review findings remediation; TAIL-07 Next-milestone planning; TAIL-08 Next-milestone closeout plan; TAIL-09 Next milestone review pass; and TAIL-10 Release ceremony.

## Validation Boundary

The issue-owned planning validator must require every surface in this inventory, parse both YAML contracts, preserve the full predecessor and existing-issue denominator, reject tracked local-path dependencies, verify the exact 45-entry number-free creation catalog including AWS-A through AWS-G, GCP-A through GCP-E, XCL-01, RUST-01, DEC-01, PROV-A, PROV-B, and DRT-D, and enforce the ten-step serial release tail. Passing planning validation is not implementation or release proof.
