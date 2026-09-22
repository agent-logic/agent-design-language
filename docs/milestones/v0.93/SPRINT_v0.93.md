# v0.93 Sprint Plan

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Status

First-pass planning under #1047. v0.93 is not open; no extraction or feature implementation is authorized by this package.

## How To Use

These are ordered planning phases, not calendar commitments or newly created sprint issues. The logical dependency graph is authoritative; split phase-sized groups into bounded sprints after capacity and ownership decisions.

## Sprint Overview

| Phase | Rows | Exit result |
|---|---|---|
| Opening | WP-01 | Explicit opening and ADL coordination/audit/decision identities; destination identities follow RD-12 bootstrap through RD-13 |
| Repository split | RD-01 through RD-13 in dependency order | Independently qualified product lockset and rollback |
| CodeFriend launch | CF-01 through CF-07; installed product depends on RV-08 | Authorized live Beta 1 launch, tester admission, verification and rollback |
| Runtime foundation | RV-01 through RV-11, mandatory | Accepted plugin lifecycle and native/process/WASM proof |
| Governance and security | GOV-01 through GOV-16; WP-S1 through WP-S6 | Installed evidence-backed behavior in owning repositories |
| Artifact templates | CT-01–CT-10 | Complete branded artifact catalog accepted before CF-05 |
| Citizen continuity/reproduction | CM-01–CM-04 | Installed identity, lineage and recovery qualification before INTEGRATE |
| Qualification | DEMO-GOV, DEMO-SEC, INTEGRATE, QUALIFY | Independent installed milestone qualification |
| Release tail | TAIL-01 through TAIL-10 | Accepted review/remediation/planning and authorized release |

## Sprint Goals

Split repositories without breaking Beta 1; complete Runtime v4, launch CodeFriend Beta 1, and deliver the retained governance/security scope on qualified contracts.

## Sprint Goal

No source move overlaps v0.92.2 coding or v0.93 feature implementation. Establish the independent products before new behavior grows.

## Planned Scope

The 83 candidates are a first-pass denominator, not a promise that all fit. Runtime v4 was omitted by the old package despite explicit handoff routing; operator direction now makes all eleven Runtime v4 outcomes mandatory for release. CodeFriend launch is included as CF-01 through CF-07; audience, environment and limits are resolved before launch execution. See DECISIONS_v0.93.md.

## Work Plan

Use PLANNED_ISSUE_CATALOG_v0.93.md and its exact dependencies. RD-04 follows complete-milestone C-SDLC proof. GOV-01 waits for the accepted Runtime contract; all native, process and WASM adapters must complete; adapter deferral cannot satisfy v0.93.

## Execution Policy

One accountable repository and writing owner per issue. Cross-repository parent coordination never substitutes for native issue/PR evidence. Preserve exact commit/artifact/version links, authentic recovery proof and independent review. Feature starts require RD-11; milestone opening requires accepted #925, the 15-minute break and operator authorization.

## Cadence Expectations

Measure build/review time after RD-01; do not infer duration from issue count. Recheck changed contracts before each consumer starts. Report completed behavior, failures and uncertain effects separately.

## Risks / Dependencies

Unresolved public ADL/private Runtime coupling, registry access, C-SDLC recovery, plugin activation design and feature capacity can block the relevant phase. #922 supplies final residuals; this first pass does not close #922 or shift Beta 1 defects into v0.93.

## Demo / Review Plan

Requalify Beta 1 after extraction, then execute governance/security/plugin demonstrations against the installed lockset. Each includes a negative and interruption/recovery boundary where applicable.

## Closeout Bar

Complete installed integration and independent qualification precede the canonical ten-step release tail. No ceremony work may hide unfinished implementation or unsafe evidence disclosure.

## Exit Criteria

Every admitted candidate has accepted output or explicit operator-approved disposition; required product behavior cannot be silently deferred to make the milestone green.
