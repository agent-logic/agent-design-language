# Structured Task Prompt

Template: 1.0.0

Issue: 431

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Plan and review the initial v0.92.1 milestone package; stop before product execution, issue migration, release approval, or replacement of WP-28 authority.

## Deliverables

- docs/milestones/v0.92.1
- docs/planning/ADL_FEATURE_LIST.md
- .csdlc/prepared/issues/431/design.md
- .csdlc/prepared/issues/431/diagram.mmd
- .csdlc/prepared/issues/431/validate_preparation_bundle.py
- .csdlc/prepared/issues/431/validate_planning_package.py
- .csdlc/issues/431
- .csdlc/evidence/431

## Acceptance

1. AC-1: Every required v0.92.1 planning surface has truthful planned status, sources, ownership, and unresolved-input classification.
2. AC-2: WBS, sprint, issue wave, readiness, proof, demo, release, ADR, and handoff surfaces agree on initial work packages and dependencies.
3. AC-3: Active tracked work, unfinished carryovers, backlog candidates, and provenance-only inputs remain distinguishable.
4. AC-4: No unfinished v0.92 dependency is represented as terminal or ancestral without exact evidence.
5. AC-5: WP-28 #316 remains unchanged and receives an explicit handoff of ready, pending, and operator-owned decisions.
6. AC-6: Focused structure, YAML, link, placeholder, routing, and diff validation passes and one bounded review resolves or routes findings.
7. AC-7: The package names exactly six execution lanes—corporate/IP, C-SDLC v3, distributed Runtime qualification, podcast/Podcast Studio, Axum hot reload, and Observatory redesign—with explicit independence and dependency boundaries.
8. AC-8: Issue #432 is an opening prerequisite and every changed tracked planning artifact has zero dependency on .adl paths.
9. AC-9: The handoff assigns CodeFriend Beta 1 to v0.92.2 and requires integrated beta availability by v0.95 without importing broad SaaS scope.
10. AC-10: Runtime v4 is recorded only as a rebaseline risk and non-scope, never as an implicit v0.92.1 dependency or delivered capability.
11. AC-11: Observatory redesign is grounded in the operator decision retained by issue #431, rejects invented data, and gates implementation on stable consumed Runtime authority APIs.

## Dependencies

- Existing docs/milestones/v0.92.1 package
- Read-only live v0.92 and v0.92.1 issue inventory
- Read-only WP-28 #316 and WP-28A #317 boundaries

## Inputs

- docs/milestones/v0.92.1
- agent-logic/agent-design-language#431
- agent-logic/agent-design-language#316
- agent-logic/agent-design-language#317
- agent-logic/agent-design-language#432
- agent-logic/agent-design-language#51
- agent-logic/agent-design-language#261
- agent-logic/agent-design-language#262
- agent-logic/agent-design-language#263
- agent-logic/agent-design-language#264
- agent-logic/agent-design-language#342

## Non Goals

- Editing, replacing, closing, or absorbing WP-28 #316 or WP-28A #317
- Implementing v0.92.1 runtime, product, tooling, or release work
- Moving issues or promoting backlog candidates beyond the operator-authorized hot-reload allocation
- Claiming release approval or late-v0.92 terminal evidence prematurely
