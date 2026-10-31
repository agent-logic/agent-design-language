> **Current successor routing (2026-10-30):** [#1405 allocation](../v0.93.3/REORGANIZATION_1405.md): Runtime/#1352 v0.93.3; tooling and governance/continuity foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Older placement below is historical. v0.93.2 product scope is unchanged.

# v0.93.2 CodeFriend Beta 1 launch and enterprise security

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## Purpose

CodeFriend Beta 1 launch and enterprise security. 31 planned identities across five dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 21 retained identities plus five explicit Beta 1 product packages and four existing C-SDLC improvements plus independent CI #205; 46 platform identities move to v0.93.3.

## Scope

Python reduction, affect and humour remain deferred outside both allocations. Existing historical evidence is retained. No repository split is repeated; no sibling checkout is required to validate public ADL.

## Document Map

- [Vision](VISION_v0.93.2.md)
- [Design](DESIGN_v0.93.2.md)
- [Decisions](DECISIONS_v0.93.2.md)
- [Wbs](WBS_v0.93.2.md)
- [Sprint](SPRINT_v0.93.2.md)
- [Demo Matrix](DEMO_MATRIX_v0.93.2.md)
- [Milestone Checklist](MILESTONE_CHECKLIST_v0.93.2.md)
- [Release Plan](RELEASE_PLAN_v0.93.2.md)
- [Release Notes](RELEASE_NOTES_v0.93.2.md)
- [Quality Gate](QUALITY_GATE_v0.93.2.md)
- [Feature Proof Coverage](FEATURE_PROOF_COVERAGE_v0.93.2.md)
- [Wp Execution Readiness](WP_EXECUTION_READINESS_v0.93.2.md)
- [Adr Plan](ADR_PLAN_v0.93.2.md)
- [Next Milestone Handoff](NEXT_MILESTONE_HANDOFF_v0.93.2.md)
- [Features](features/README.md)

[Execution graph](EXECUTION_PLAN_v0.93.2.json), [specifications](WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml), and [issue wave](WP_ISSUE_WAVE_v0.93.2.yaml) carry exact acceptance and dependencies.

## Execution boundary

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. Owners, accepted dependencies, scoped resources and a tracked issue are required before each implementation begins.

## Existing C-SDLC improvements

These four existing issues are included in v0.93.2 without duplication. The operator approved the plan and proceeding with C-SDLC #296–299 first. Other work remains unactivated.

- [C-SDLC #296](https://github.com/agent-logic/cognitive-sdlc/issues/296): Use one editable issue source and remove projection-drift gates — planned Sprint 1.
- [C-SDLC #297](https://github.com/agent-logic/cognitive-sdlc/issues/297): Preserve proof and review across metadata-only edits — planned Sprint 1.
- [C-SDLC #298](https://github.com/agent-logic/cognitive-sdlc/issues/298): Remove mandatory portable-review attestation and publication receipt gates — planned Sprint 1.
- [C-SDLC #299](https://github.com/agent-logic/cognitive-sdlc/issues/299): Simplify lifecycle commands with optional start and consolidated delivery — planned Sprint 1.

#299 depends on #296, #297 and #298. These run first so later work benefits from the simplification; CodeFriend assessment depends on #299. The milestone quality gate includes their completion. Existing #304 remains an additional separately tracked remount-reliability fix; it is not one of these four improvements. CI #205 is included independently, outside this four-issue group. The four issues have no GitHub milestone assigned at this planning snapshot; metadata reconciliation is outstanding.

## Independent CI work

[CI-205 / C-SDLC #205](https://github.com/agent-logic/cognitive-sdlc/issues/205) must also complete in v0.93.2. It is an independent Sprint 3 track, not part of or dependent on lifecycle simplification #296–299. It covers CI across code and non-paper documentation repositories; papers remain excluded. The milestone quality gate requires its completion. Its existing v0.93.1 title and unassigned GitHub milestone await metadata reconciliation. No execution is started.

## Approved sidecars

The operator explicitly included these two existing issues in v0.93.2. They remain outside the 31 core work packages and numbered sprint counts; total tracked scope is 33 identities. Reuse both issues without duplication. Implementation remains unactivated; inclusion in the plan is not a completion claim. Record each sidecar disposition at milestone review.

- [ADL-1275](https://github.com/agent-logic/agent-design-language/issues/1275): Reconcile post-split repository adapters and native lifecycle admission.
- [CSDLC-304](https://github.com/agent-logic/cognitive-sdlc/issues/304): Preserve C-SDLC installation identity across filesystem remounts.

## Created issue index

See [all issues and sprint umbrellas](ISSUES_v0.93.2.md). Cards are deferred until each sprint is prepared.
