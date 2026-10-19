# v0.93.2 Wp Execution Readiness

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## Opening prerequisites

v0.93.1 #1230 is closed with native coordination completion; #1245 acceptance merged in PR #1295 at 69fae86cde65861cdace95183c2c373dcb09ca01.

## Before each issue

Assign owner, reuse existing issue where applicable, agree acceptance and negative cases, bind execution context and run focused validation.

## Coordination scope

31 planned identities across five dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 21 retained identities plus five explicit Beta 1 product packages and four existing C-SDLC improvements plus independent CI #205; 46 platform identities move to v0.93.3.

## Unresolved readiness

CF-B1-01 must measure actual gaps before downstream issue seeding. CodeFriend #92 already exists; Runtime #112 stays reused as GHB predecessor evidence. Sidecars #1275 and C-SDLC #304 remain separate, and ADL #1226 remains deferred.

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
