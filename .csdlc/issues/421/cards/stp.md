# Structured Task Prompt

Template: 1.0.0

Issue: 421

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #421 typed C-SDLC intentional-deletion deliverables readiness defect only.

## Deliverables

- Typed intentional-deletion marker or classification accepted by readiness.
- Base-vs-candidate deletion proof for exact governed paths.
- Negative missing-path regressions that remain fail-closed.
- Focused validation lane proving ordinary validator deliverables remain required-present.

## Acceptance

1. AC-1: A typed intentional-deletion deliverable can remain in exact affected-area, deliverable, and review scope without requiring the deleted file to exist at candidate HEAD.
2. AC-2: Validation proves the path existed at the governed base and is deleted in the exact candidate; absent, renamed, untracked, unrelated, or mistyped missing paths still fail closed.
3. AC-3: Ordinary test and validator deliverables retain current required-present behavior.
4. AC-4: Focused positive and adversarial regressions cover the #414-style deleted test and false deletion claims.
5. AC-5: Exact-head review, publication, topology, and terminal authority are not weakened.

## Dependencies

- Issue #414 is blocked until #421 is terminal, canonical, ancestral, and installed.

## Inputs

- GitHub issue #421
- GitHub issue #414 reproduction
- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate2.rs

## Non Goals

- Changing #414 product code, evidence, design, publication, or AWS behavior.
- Changing #268 or #269.
- Treating arbitrary missing files as deferred or accepted.
- Broad redesign of C-SDLC validation or review scope.
