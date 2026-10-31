# v0.95 Milestone Design

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward planning. This document defines the intended MVP-convergence shape of
`v0.95`.

## Problem Statement

The roadmap now contains strong later-band feature contracts, but without a
coherent convergence milestone those surfaces risk staying legible only to
people who already know the project history.

`v0.95` must solve the integration problem:

- make the MVP legible
- make the proof surfaces discoverable
- keep the control plane and editor/operator surfaces truthful
- avoid adding new architecture while closing the story

## Goals

- converge the later roadmap bands into one reviewable MVP package
- make dashboard, evaluation, distributed execution, walkthrough, and editor
  surfaces mutually consistent
- preserve explicit dependency and non-goal boundaries

## Non-goals

- building a new substrate below the already planned governance/security layers
- pretending optional editor choices are architectural necessities
- using `v0.95` as a catch-all for unfinished earlier work

## Scope

### In scope

- dashboard/compression reporting
- Shepherd/Gemma evaluator/training closure
- capability-testing evidence consumption and Aptitude Atlas post-`v0.95`
  boundary
- CodeFriend v1 and portable adapter v2 proof packaging, consuming the
  complete build plan and pre-v0.92 boundary from `#4756`
- distributed execution integration closure
- walkthrough/catalog convergence
- control-plane Rust migration and tooling hardening
- web editor baseline and Zed decision boundary

### Out of scope

- new greenfield platform domains
- payment-rail expansion beyond what `v0.94.1` already owns
- revisiting earlier milestone fundamentals unless a contradiction is found

## Proposed Design

The milestone should be structured as one convergence package with four linked
lanes:

1. Operator/read-only surfaces:
   dashboard, compression reporting, and reviewer entrypoints.
2. Evaluation surfaces:
   Shepherd/Gemma follow-on, capability-testing evidence consumption, and the
   Aptitude Atlas post-`v0.95` product boundary.
3. Platform integration surfaces:
   distributed execution, CodeFriend external-repo proof packaging from the
   `#4756` obligation boundary, and
   control-plane hardening.
4. User/editor surfaces:
   walkthrough/catalog, web editor baseline, and Zed decision.

The key design rule is that all four lanes must remain subordinate to the same
deterministic execution and review truth model.

## Risks and Mitigations

- Risk: `v0.95` becomes a vague “everything else” milestone.
  - Mitigation: keep every work area tied to one existing tracked feature doc.
- Risk: dashboard/editor polish gets ahead of control-plane truth.
  - Mitigation: treat workflow and lifecycle correctness as a dependency, not a
    follow-on.
- Risk: reviewer and customer narratives diverge.
  - Mitigation: require one coherent walkthrough/catalog package plus explicit
    proof surfaces.

## Validation Plan

- verify every `v0.95` feature row maps to the milestone package
- verify the demo matrix and checklist cover the same feature set
- verify no `v0.95` planning file remains a raw template
- verify the milestone remains bounded by `v0.94` / `v0.94.1` dependencies

## Explicit MVP reconciliation outputs — #1407

WP-11 consumes the full127-row feature acceptance/disposition map and exact predecessor owner lockset, then records feature freeze,1.0 boundary and every residual/exclusion. WP-12 maps required features to actual reused owner proof or a named gap. WP-13 records bounded public-doc/style cleanup disposition and final review; no forced closeout-only PR.

Before the relevant consumer executes or MVP acceptance proceeds, resolve D01 Shepherd/Gemma evaluated-versus-trained output, D02 required payments subset, D03 UTS/SDK and independent ATEC/AVP consumption, D04 speculative backend, D05 optional Zed and D06 old logistic split. D08 decision-model pilots remain proposed/unallocated. An open decision is not proof, an exclusion or authority to implement. All options and deferred proposals remain in the [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md).
