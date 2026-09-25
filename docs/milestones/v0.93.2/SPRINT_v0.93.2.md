# v0.93.2 Sprint Plan

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Planned and unopened. The operator approved the scope split; this package does not approve execution, provider spend, publication or release.

## How To Use

Four proposed outcome waves, with no implied fixed duration. Assign owners and capacity before activation.

## Sprint Overview

| Sprint | Outcome | Tasks | Count |
|---|---|---|---|
| 1 | Open and qualify Runtime v4; bounded Python reduction | WP-01, RV-01–RV-11, PY-01 | 13 |
| 2 | Governed actors and enterprise security | GOV-01–GOV-16, WP-S1–WP-S6 | 22 |
| 3 | Citizen continuity and integrated demonstrations | CM-01–CM-04, DEMO-GOV, DEMO-SEC | 6 |
| 4 | Integration, independent qualification and release | INTEGRATE, QUALIFY, TAIL-01–TAIL-10 | 12 |

## Sprint Goals

Deliver the complete remaining platform scope on accepted product boundaries.

## Sprint Goal

Maintain CodeFriend Beta 1 continuity through the Runtime upgrade.

## Planned Scope

53 planned candidates: 40 inherited implementation/demo tasks plus 13 version-specific opening, integration, qualification and release-tail tasks. Existing #875 remains a separately gated sidecar and is not counted in 53. Podcast #671 is completed in v0.92.2 (PR #1174).

## Work Plan

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated. Honor all graph dependencies within and across sprints. All native/process/WASM plugin outcomes remain required.

## Execution Policy

One repository and writing owner per task; use native lifecycle state and exact issue/PR evidence. No extra receipt or review layer is created by this allocation.

## Cadence Expectations

Report accepted outcomes and unresolved failures; stop duplicate repairs and preserve unknown remote effects. Counts are not capacity estimates.

## Risks / Dependencies

A 22-task second wave may need parallel owners. Existing #875 requires its original pilot authorization and qualification. It is not automatically authorized or counted as a core task. #671 completed in v0.92.2.

## Demo / Review Plan

Installed recovery, adversarial and compatibility proof precede independent qualification. Review findings are remediated after reviews, in TAIL-06.

## Closeout Bar

TAIL-01–TAIL-10 execute in order, including external review, findings disposition and explicit release authorization.

## Exit Criteria

Each task and sidecar has truthful final disposition, with no launch or qualification claim from planning alone.

## Existing Observatory backlog

Existing #1145 (Everyone Observatory) is separately routed to v0.93.2, outside the 53 core candidates. Preserve its existing task and authority; resolve its scope, dependencies and disposition at opening instead of creating a duplicate or assuming completion. #875 retains its original gates and is excluded from the core count. #671 is completed in v0.92.2 and has no successor allocation.
