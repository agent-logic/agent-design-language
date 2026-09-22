# v0.93 Sprint Plan

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Status

Numbered sprint allocation under #922, continuing the #1047 baseline. v0.93 is not open; no extraction or feature implementation is authorized by this package.

## How To Use

There are **16 proposed sprints covering 83 single-task candidates**. Sprints 1–3 are exclusively the opening and repository split; all feature work waits for accepted RD-11. These are bounded delivery waves, not fixed-duration promises or created sprint issues. The canonical graph owns membership and dependency order. Named owners and capacity must be resolved before activating each sprint.

## Sprint Overview

| Sprint | Goal | Tasks | Count | Exit result |
|---|---|---|---|---|
| 1 | Repository split: decisions and destinations | WP-01, RD-01, RD-02, RD-12, RD-13 | 5 | Approved cutover contract, destination repositories and issue identities. |
| 2 | Repository split: foundational extraction | RD-03, RD-04, RD-05 | 3 | Public contracts, C-SDLC and Runtime/Observatory qualified in their owning repositories. |
| 3 | Repository split: consumers and acceptance | RD-06, RD-09, RD-10, RD-08, RD-07, RD-11 | 6 | All product consumers qualified; superseded source retired; complete split lockset accepted. |
| 4 | Runtime transition foundation and product contracts | RV-01, RV-02, RV-03, CF-01, CT-01, PY-01 | 6 | Native generation transitions accepted; launch requirements and artifact catalog agreed; Python tranche disposition recorded. |
| 5 | Plugin continuity and template families | RV-09, RV-10, RV-11, CT-02, CT-06, CT-07, CT-08, CT-09 | 8 | State/configuration migration and safe removal accepted; four template families use the common branded foundation. |
| 6 | Runtime v4 installed qualification | RV-04, RV-05, RV-06, RV-07, RV-08 | 5 | Native/process/WASM behavior, provider migration and installed recovery qualified. |
| 7 | Deployable CodeFriend and citizen foundations | CF-02, GOV-01, GOV-02, GOV-03, GOV-06, CM-01 | 6 | Deployable product, actor boundaries, rights/standing and private ToM accepted; citizen continuity contract agreed. |
| 8 | Artifact preparation and launch operations | CF-03, CF-04, CT-03, CT-10, GOV-04, GOV-07, GOV-08, GOV-11 | 8 | Tester onboarding, launch recovery, branded exports, constitutional review, ToM continuity and delegation accepted. |
| 9 | Artifact qualification and governed cognition | CT-04, CT-05, GOV-05, GOV-09, GOV-12, GOV-13, WP-S1 | 7 | Full artifact catalog qualified; appeals, reputation, upstream cognition, communication and zero-trust boundary accepted. |
| 10 | CodeFriend Beta 1 launch and action security | CF-05, CF-06, CF-07, GOV-10, GOV-14, WP-S2, WP-S3 | 7 | Qualified Beta 1 launched under explicit authority; social memory/contracts, action authorization and key lifecycle accepted. |
| 11 | Citizen migration, reproduction and isolation | GOV-15, GOV-16, WP-S4, WP-S5, CM-02, CM-03 | 6 | Guild/health behavior, incident evidence, protected data, single-identity migration and distinct descendants accepted. |
| 12 | Recovery and integrated demonstrations | WP-S6, CM-04, DEMO-GOV, DEMO-SEC | 4 | Security drill, citizen recovery and installed governance/security demonstrations complete. |
| 13 | Milestone integration and qualification | INTEGRATE, QUALIFY | 2 | Independent installed milestone qualification accepts the complete product lockset. |
| 14 | Release evidence and documentation | TAIL-01, TAIL-02, TAIL-03 | 3 | Quality gate, reviewed documentation and publication artifacts finalized. |
| 15 | Milestone review and remediation | TAIL-04, TAIL-05, TAIL-06 | 3 | Internal and external reviews resolved through accepted remediation or explicit permitted dispositions. |
| 16 | Successor planning and release ceremony | TAIL-07, TAIL-08, TAIL-09, TAIL-10 | 4 | Successor planning and closeout plans reviewed; authorized release ceremony completes the milestone. |

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

Each sprint contains 2–8 core tasks. This bounds the issue count, not effort: repository extraction, adapter work and catalog-wide qualification may take longer than other tasks. Measure capacity and build/review time during the split; split an oversized sprint before activation while preserving single-task issues and dependency order. No dates, staffing or fixed duration are promised. Recheck changed contracts before each consumer starts. Report completed behavior, failures and uncertain effects separately.

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

- **#875: proposed observation window, Sprints 4–12.** The live five-journey pilot requires refreshed SIM-07/SIM-08 qualification and separate exact-window transition authorization. The first five eligible consecutive journeys determine completion, including failures and abandonments; sprint placement is not authorization or proof of completion. Reconcile its disposition before Sprint 13 qualification.
- **#671: proposed window, Sprints 10–12.** Podcast registration retains terminal #261/#262/#263/#660 dependencies and explicit public-launch/provider-submission authority. Provider delays or missing approval remain visible; report actual disposition before Sprint 13 without claiming publication or inventing a core release dependency.

## Ownership and sizing at activation

Repository ownership remains the canonical issue row's repository. Before each sprint starts, assign one named writing owner per issue, identify an independent reviewer, check exact prerequisite acceptance, and size the work against available capacity. Those names and effort estimates are unresolved and must not be fabricated. Use existing issue/PR evidence for sprint acceptance; this allocation adds no receipt system or extra review rounds. A sprint coordinator tracks its tasks without absorbing their implementation. Final #921 residuals may require an explicit plan revision before opening; no required scope is silently deferred.
