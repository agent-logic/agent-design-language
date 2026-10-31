# v0.95 Sprint Plan

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning sprint outline. The final sprint and WP sequence will be
authored during `v0.95` `WP-01`.

## Sprint Goal

Close the MVP convergence band: dashboard/reporting, evaluator/training,
capability-testing evidence, CodeFriend proof packaging, distributed
integration, walkthrough/catalog, control-plane hardening, and the final editor
boundary.

## Planned Scope

- make the `v0.95` feature package executable as one coherent milestone rather
  than a loose checklist of later ideas
- preserve the dependency chain from `v0.92` / `v0.93` / `v0.94` / `v0.94.1`
  instead of re-arguing earlier architecture
- finish with an explicit MVP boundary and a reviewable ceremony package

## Candidate Execution Shape

| Order | Candidate tranche | Milestone role | Status |
| --- | --- | --- | --- |
| 1 | Dashboard/reporting + evaluator/evidence convergence | Make non-user-facing MVP surfaces explicit first. | Planned |
| 1b | CodeFriend v1 / adapter v2 proof packaging | Ensure external-repo proof is ready for MVP consumption by implementing the v0.95 side of the `#4756` complete v1 build plan and pre-v0.92 obligation boundary. | Planned |
| 2 | Distributed integration + walkthrough/catalog + control-plane hardening | Build the integrated launch-shape story. | Planned |
| 3 | Web editor baseline + Zed decision + final convergence/review tail | Resolve the editor boundary and close the MVP packet. | Planned |

## Cadence Expectations

- keep the milestone on the standard ADL WP rhythm, ending in demo, quality,
  docs/review, and ceremony
- prefer bounded integrated proofs over a large number of disconnected demos
- treat user-facing storytelling and reviewer-facing proof posture as equally
  important in this milestone

## Risks / Dependencies

- Dependency: `v0.94` and `v0.94.1` must already have explicit tracked closure
  packages.
  - Risk: MVP convergence can become a silent overflow bucket for earlier work.
  - Mitigation: keep `v0.95` limited to explicit convergence, integration, and
    decision-boundary surfaces.
- Dependency: the control-plane and editor surfaces must remain workflow-truthful.
  - Risk: polished editor or dashboard work could drift from the validated
    lifecycle.
  - Mitigation: keep all operator/editor features explicitly subordinate to the
    existing control-plane truth model.

## Demo / Review Plan

- Demo artifact: one MVP walkthrough plus bounded supporting demos for
  distributed execution, dashboard/reporting, and editor boundary.
- Review posture: reviewer/customer-ready convergence packet with explicit MVP
  non-goals.
- Sign-off expectation: milestone closes only when user-facing, operator-facing,
  and reviewer-facing surfaces all tell the same story.

## Explicit MVP reconciliation outputs — #1407

WP-11 consumes the full127-row feature acceptance/disposition map and exact predecessor owner lockset, then records feature freeze,1.0 boundary and every residual/exclusion. WP-12 maps required features to actual reused owner proof or a named gap. WP-13 records bounded public-doc/style cleanup disposition and final review; no forced closeout-only PR.

Before the relevant consumer executes or MVP acceptance proceeds, resolve D01 Shepherd/Gemma evaluated-versus-trained output, D02 required payments subset, D03 UTS/SDK and independent ATEC/AVP consumption, D04 speculative backend, D05 optional Zed and D06 old logistic split. D08 decision-model pilots remain proposed/unallocated. An open decision is not proof, an exclusion or authority to implement. All options and deferred proposals remain in the [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md).
