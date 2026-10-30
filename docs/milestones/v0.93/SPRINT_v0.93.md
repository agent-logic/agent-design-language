> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# v0.93 Sprint Plan

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Status

Numbered sprint allocation under #922, continuing the #1047 baseline. v0.93 is not open; no extraction or feature implementation is authorized by this package.

## How To Use

There are **8 proposed sprints covering 83 single-task candidates**. Sprint 1 is exclusively the opening and repository split; all feature work waits for accepted RD-11. These are bounded delivery waves, not fixed-duration promises or created sprint issues. The canonical graph owns membership and dependency order. Named owners and capacity must be resolved before activating each sprint.

## Sprint Overview

| Sprint | Goal | Tasks | Count | Exit result |
|---|---|---|---|---|
| 1 | Repository split, qualification and acceptance | WP-01, RD-01, RD-02, RD-12, RD-13, RD-03, RD-04, RD-05, RD-06, RD-09, RD-10, RD-08, RD-07, RD-11 | 14 | Complete split lockset accepted at RD-11 before any feature work. |
| 2 | Runtime v4 and template foundations | RV-01, RV-02, RV-03, RV-09, RV-10, RV-11, RV-04, RV-05, RV-06, RV-07, RV-08, CF-01, CT-01, CT-02, CT-09, CT-08, CT-07, CT-06, PY-01 | 19 | Installed Runtime v4 qualified; launch requirements, common template foundation and four families accepted; Python tranche disposition recorded. |
| 3 | CodeFriend product and artifact preparation | CF-02, CF-03, CF-04, CT-03, CT-10 | 5 | Deployable product, tester onboarding, launch recovery and branded artifact preparation/export accepted. |
| 4 | Template qualification and Beta 1 launch | CT-04, CT-05, CF-05, CF-06, CF-07 | 5 | Complete template catalog and external launch candidate qualified; Beta 1 launched under explicit authority. |
| 5 | Governance and security | GOV-01, GOV-02, GOV-03, GOV-04, GOV-05, GOV-06, GOV-07, GOV-08, GOV-09, GOV-10, GOV-11, GOV-12, GOV-13, GOV-14, GOV-15, GOV-16, WP-S1, WP-S2, WP-S3, WP-S4, WP-S5, WP-S6 | 22 | Constitutional/social governance and enterprise security behavior accepted, including replayable security drill. |
| 6 | Citizen migration/reproduction and demonstrations | CM-01, CM-02, CM-03, CM-04, DEMO-GOV, DEMO-SEC | 6 | Citizen migration/reproduction recovery and installed governance/security demonstrations accepted. |
| 7 | Milestone integration and qualification | INTEGRATE, QUALIFY | 2 | Complete installed product lockset integrated and independently qualified. |
| 8 | Release reviews, remediation, successor planning and closeout | TAIL-01, TAIL-02, TAIL-03, TAIL-04, TAIL-05, TAIL-06, TAIL-07, TAIL-08, TAIL-09, TAIL-10 | 10 | Canonical TAIL-01–TAIL-10 completed in order, including reviews before accepted remediation and authorized release. |

## Sprint Goals

Split repositories without breaking Beta 1; complete Runtime v4, launch CodeFriend Beta 1, and deliver the retained governance/security scope on qualified contracts.

## Sprint Goal

No source move overlaps v0.92.2 coding or v0.93 feature implementation. Establish the independent products before new behavior grows.

## Planned Scope

The 83 candidates are a first-pass denominator, not a promise that all fit. Runtime v4 was omitted by the old package despite explicit handoff routing; operator direction now makes all eleven Runtime v4 outcomes mandatory for release. CodeFriend launch is included as CF-01 through CF-07; audience, environment and limits are resolved before launch execution. See DECISIONS_v0.93.md.

## Work Plan

Use PLANNED_ISSUE_CATALOG_v0.93.md and its exact dependencies. Task listing inside a sprint is not blanket start authority: complete each predecessor before its consumer. Runtime and CodeFriend/template lanes may progress independently after RD-11; governance/security follow their own gates. Do not hold CodeFriend launch for unrelated governance work once CF-05/CF-06 and launch authority are satisfied. RD-04 follows complete-milestone C-SDLC proof. GOV-01 waits for the accepted Runtime contract; all native, process and WASM adapters must complete; adapter deferral cannot satisfy v0.93.

## Execution Policy

One accountable repository and writing owner per issue. Cross-repository parent coordination never substitutes for native issue/PR evidence. Preserve exact commit/artifact/version links, authentic recovery proof and independent review. Feature starts require RD-11; milestone opening requires accepted #925, the 15-minute break and operator authorization.

## Cadence Expectations

The operator accepted eight outcome-based sprints. Counts vary with the coherent outcome: Sprint 5 contains 22 individually scoped governance/security tasks; Sprint 7 contains two qualification tasks. Grouping reduces coordination boundaries, not effort. Assign named owners and estimate capacity before activation; no dates or fixed duration are promised. Recheck changed contracts before each consumer starts. Report completed behavior, failures and uncertain effects separately.

## Risks / Dependencies

Unresolved public ADL/private Runtime coupling, registry access, C-SDLC recovery, plugin activation design and feature capacity can block the relevant phase. #922 supplies final residuals; this first pass does not close #922 or shift Beta 1 defects into v0.93.

## Demo / Review Plan

Requalify Beta 1 after extraction, then execute governance/security/plugin demonstrations against the installed lockset. Each includes a negative and interruption/recovery boundary where applicable.

## Closeout Bar

Complete installed integration and independent qualification precede the canonical ten-step release tail. No ceremony work may hide unfinished implementation or unsafe evidence disclosure.

## Exit Criteria

Every admitted candidate has accepted output or explicit operator-approved disposition; required product behavior cannot be silently deferred to make the milestone green.

## Existing issue sidecars

These are two existing v0.93 issues in addition to the 83 core candidates; do not recreate them or merge their tasks into sprint issues. Both wait until the split is accepted.

- **#875: proposed observation window, Sprints 2–6.** The live five-journey pilot requires refreshed SIM-07/SIM-08 qualification and separate exact-window transition authorization. The first five eligible consecutive journeys determine completion, including failures and abandonments; sprint placement is not authorization or proof of completion. Reconcile its disposition before Sprint 7 qualification.
- **#671: proposed window, Sprints 4–6.** Podcast registration retains terminal #261/#262/#263/#660 dependencies and explicit public-launch/provider-submission authority. Provider delays or missing approval remain visible; report actual disposition before Sprint 7 without claiming publication or inventing a core release dependency.

## Ownership and sizing at activation

Repository ownership remains the canonical issue row's repository. Before each sprint starts, assign one named writing owner per issue, identify an independent reviewer, check exact prerequisite acceptance, and size the work against available capacity. Those names and effort estimates are unresolved and must not be fabricated. Use existing issue/PR evidence for sprint acceptance; this allocation adds no receipt system or extra review rounds. A sprint coordinator tracks its tasks without absorbing their implementation. Final #921 residuals may require an explicit plan revision before opening; no required scope is silently deferred.

## Consolidation decision

The operator accepted eight sprints in place of sixteen. All 83 task identities, scopes and dependency edges are preserved. Sprint 7 owns integration and qualification; release review remediation stays in Sprint 8 after TAIL-04/TAIL-05, preserving the canonical review-before-remediation order. No new sprint ceremony or review layer is introduced.
