# v0.95 Milestone Vision

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward planning. This is a milestone-vision document, not an implementation
closeout record.

## Overview

`v0.95` is the milestone where ADL should stop feeling like a collection of
strong subsystems and start feeling like one coherent MVP platform.

Earlier milestones established:

- deterministic runtime and review surfaces
- citizen state, governance, secure execution, and payments allocation bands
- evaluation, publication, and productization pressure-release work

`v0.95` should turn those bands into one launch-shape story with clear proof,
clear boundaries, and clear operator/reviewer entrypoints.

## Strategic Focus

`v0.95` should strengthen five pillars:

1. MVP convergence rather than new scope growth.
2. Reviewer/customer legibility through walkthroughs, dashboards, and explicit
   proof posture.
3. Model-evaluation maturity through Shepherd/Gemma follow-on work and tracked
   capability-testing evidence, while Aptitude Atlas productization remains
   post-`v0.95`.
4. Hardened execution/control-plane substrate suitable for the MVP boundary.
5. Explicit editor/operator story across the web baseline and the Zed decision.

## Intended Outcome

After `v0.95`, ADL should be able to say:

- what the MVP is
- what surfaces prove it
- which operator/editor path is required
- which optional integrations remain optional
- which product surfaces, including Aptitude Atlas productization, remain
  post-MVP
- how the runtime, governance, execution, evaluation, and demo layers fit
  together

## Non-goals

`v0.95` should not:

- add a new greenfield architectural domain
- quietly absorb unfinished `v0.94` or `v0.94.1` foundation work
- treat polish alone as proof
- let optional host/editor preference masquerade as MVP necessity

## Long-Term Direction

The point of `v0.95` is not just “more features.” It is coherence.

This milestone should prepare the platform for a later public-facing phase by
making the system understandable as one bounded product: deterministic,
governable, observable, secure, evaluable, and demoable.

## Explicit MVP reconciliation outputs — #1407

WP-11 consumes the full127-row feature acceptance/disposition map and exact predecessor owner lockset, then records feature freeze,1.0 boundary and every residual/exclusion. WP-12 maps required features to actual reused owner proof or a named gap. WP-13 records bounded public-doc/style cleanup disposition and final review; no forced closeout-only PR.

Before the relevant consumer executes or MVP acceptance proceeds, resolve D01 Shepherd/Gemma evaluated-versus-trained output, D02 required payments subset, D03 UTS/SDK and independent ATEC/AVP consumption, D04 speculative backend, D05 optional Zed and D06 old logistic split. D08 decision-model pilots remain proposed/unallocated. An open decision is not proof, an exclusion or authority to implement. All options and deferred proposals remain in the [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md).
