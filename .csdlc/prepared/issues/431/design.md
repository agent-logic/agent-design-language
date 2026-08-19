# Issue #431 Design — Initial v0.92.1 Planning Sidecar

## Purpose

Refresh the already-existing v0.92.1 milestone package early enough to guide
issue shaping and dependency decisions before v0.92 WP-28 begins. This sidecar
creates a reviewed initial baseline; WP-28 #316 remains unchanged and later
updates that baseline with final v0.92 evidence.

## Existing Package

The repository already contains the canonical v0.92.1 README, vision, design,
decisions, WBS, sprint, issue wave, demonstration matrix, checklist, release
plan and notes, quality gate, proof coverage, execution readiness,
specifications, ADR plan, handoff, feature documents, source notes, and planned
issue packets. The sidecar audits and refreshes these surfaces rather than
creating a competing package.

## Planning Model

Every input is classified as one of:

- active tracked v0.92.1 work;
- explicit v0.92 carryover awaiting terminal evidence;
- operator-promoted v0.92.1 scope awaiting issue-wave allocation;
- rejected, superseded, or out-of-scope input retained only as provenance.

The WBS and issue-wave YAML must agree on identifiers, dependencies,
deliverables, proof expectations, parallel groups, and serial gates. Planning
documents remain planned—not implemented or release-approved.

The refreshed package has six independently executable lanes after its
opening prerequisites:

1. corporate and intellectual-property transfer;
2. C-SDLC v3 implementation and controlled cutover;
3. distributed multi-agent Runtime qualification, using UTS as a governed
   workload rather than creating a separate UTS architecture program;
4. podcast publication and Podcast Studio execution; and
5. validated Axum configuration hot reload with last-known-good fallback; and
6. an Observatory redesign sprint covering information architecture, operator
   workflows, governed interaction, accessibility, responsive behavior, and
   evidence-grounded visual design.

The Observatory lane may design against declared Runtime authority contracts,
but implementation must wait for stable consumed APIs. It does not authorize
Runtime v4, backend contract rewrites, or invented data.

Issue #432 is an opening repository-authority prerequisite: tracked planning,
policy, tooling, and validation contracts must not depend on `.adl` paths.

## Relationship to WP-28 And v0.92.2

This sidecar does not edit, replace, close, or absorb WP-28 #316 or WP-28A
#317. It produces an explicit handoff containing the initial package revision,
resolved inputs, outstanding late-v0.92 evidence, and operator decisions. WP-28
later accepts or revises that plan and owns final next-milestone reconciliation.

The next milestone is planned as v0.92.2 CodeFriend Beta 1. v0.92.1 must
produce a reviewed handoff for a usable bounded beta in v0.92.2, with beta
availability and integrated product proof required by v0.95. Runtime v4 is a
named planning risk and possible future rebaseline, not committed v0.92.1
scope.

## Owned Paths

- `docs/milestones/v0.92.1/**`
- `docs/planning/ADL_FEATURE_LIST.md`
- `.csdlc/issues/431/**`
- `.csdlc/prepared/issues/431/**`
- `.csdlc/evidence/431/**`

All v0.92 implementation, lifecycle, release, provider, and product paths are
read-only.

## Initial Inputs

- live issue and label inventory for v0.92 and v0.92.1;
- current v0.92.1 package;
- terminal and open v0.92 runtime/observatory evidence;
- operator decisions promoting Axum configuration hot reload into v0.92.1;
- live v0.92.1 podcast issues #51, #261-#264, and #342;
- the operator-authorized Observatory redesign decision retained by issue #431;
- issue #432 repository-authority cleanup and the no-`.adl` dependency rule;
- v0.92.2 CodeFriend Beta 1 successor requirements;
- WP-28 #316 and WP-28A #317 boundaries.

## Validation

Use focused planning-template structure checks, YAML parsing, relative-link and
repo-path validation, placeholder/status scans, live routing comparisons, and
diff hygiene. No broad Rust/runtime suite is required unless executable tooling
changes.

## Stop Conditions

- A proposed edit would move or create execution issues without operator
  authority.
- The sidecar would claim unfinished v0.92 work terminal.
- The sidecar would replace or mutate WP-28 authority.
- Package documents and issue-wave dependency truth cannot be reconciled.
- An unapproved backlog-only idea is treated as active execution scope.
- Runtime v4 is silently made a release dependency or implementation lane.
