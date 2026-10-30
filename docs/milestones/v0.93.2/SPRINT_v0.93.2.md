# v0.93.2 Sprint

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## Sprint Overview

31 planned identities across five dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 21 retained identities plus five explicit Beta 1 product packages and four existing C-SDLC improvements plus independent CI #205; 46 platform identities move to v0.93.3.

## Work Plan

### Sprint 1: C-SDLC lifecycle simplification

WP-01, CSDLC-296, CSDLC-297, CSDLC-298, CSDLC-299.

### Sprint 2: Acceptance and security contracts

CF-B1-01, WP-S1.

### Sprint 3: Beta 1 improvements and enterprise security

CF-B1-02, CF-B1-03, WP-S2, WP-S3, WP-S4, WP-S5, CF-B1-04, CF-B1-05, WP-S6, DEMO-SEC, CI-205.

### Sprint 4: Candidate integration and qualification

INTEGRATE, QUALIFY.

### Sprint 5: Authorized launch and release review

CF-92, TAIL-01, TAIL-02, TAIL-03, TAIL-04, TAIL-05, TAIL-06, TAIL-07, TAIL-08, TAIL-09, TAIL-10.

## Execution Policy

No calendar promises. Use parallel owners only for independent accepted inputs; do not rerun tests or provider work unnecessarily. Each package has one accountable owner.

## Risks / Dependencies

Security must retain supported-consumer invariants after broad governance moves. A graph change alone does not establish compatibility.

## Closeout Bar

Accept the sprint outcomes and recorded adverse cases. Administrative cleanup may run asynchronously but cannot substitute for product acceptance.

## Existing C-SDLC improvements

These four existing issues are included in v0.93.2 without duplication. The operator approved the plan and proceeding with C-SDLC #296–299 first. Other work remains unactivated.

- [C-SDLC #296](https://github.com/agent-logic/cognitive-sdlc/issues/296): Use one editable issue source and remove projection-drift gates — planned Sprint 1.
- [C-SDLC #297](https://github.com/agent-logic/cognitive-sdlc/issues/297): Preserve proof and review across metadata-only edits — planned Sprint 1.
- [C-SDLC #298](https://github.com/agent-logic/cognitive-sdlc/issues/298): Remove mandatory portable-review attestation and publication receipt gates — planned Sprint 1.
- [C-SDLC #299](https://github.com/agent-logic/cognitive-sdlc/issues/299): Simplify lifecycle commands with optional start and consolidated delivery — planned Sprint 1.

#299 depends on #296, #297 and #298. These run first so later work benefits from the simplification; CodeFriend assessment depends on #299. The milestone quality gate includes their completion. Existing #304 remains an additional separately tracked remount-reliability fix; it is not one of these four improvements. CI #205 is included independently, outside this four-issue group. The four issues have no GitHub milestone assigned at this planning snapshot; metadata reconciliation is outstanding.

## Independent CI work

[CI-205 / C-SDLC #205](https://github.com/agent-logic/cognitive-sdlc/issues/205) must also complete in v0.93.2. It is an independent Sprint 3 track, not part of or dependent on lifecycle simplification #296–299. It covers CI across code and non-paper documentation repositories; papers remain excluded. The milestone quality gate requires its completion. Its existing v0.93.1 title and unassigned GitHub milestone await metadata reconciliation. No execution is started.
