# v0.95 Demo Matrix

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Candidate demo planning only.

## Candidate Coverage Summary

| Demo ID | Candidate demo | Milestone claim | Primary proof surface | Status |
| --- | --- | --- | --- | --- |
| D1 | MVP walkthrough flagship | The full MVP story is coherent across runtime, governance, evaluation, and operator surfaces. | walkthrough packet plus catalog/runbook | Planned candidate |
| D2 | Dashboard and compression review path | Dashboard/reporting surfaces truthfully summarize milestone state without becoming release authority. | HTML dashboard/report pair | Planned candidate |
| D3 | Distributed execution integration proof | Distributed execution remains bounded, reviewable, and subordinate to secure execution. | integration packet plus boundary report | Planned candidate |
| D4 | Capability-evidence proof | Shepherd/Gemma and capability-testing evidence surfaces are evidence-backed rather than narrative-only; Aptitude Atlas productization is out of MVP scope. | evaluator/report bundle | Planned candidate |
| D4b | CodeFriend external-repo proof | CodeFriend v1 and adapter v2 can run a bounded external-repo review through ADL with redaction and manifest evidence, consuming the complete v1 build plan and pre-v0.92 obligation boundary from `#4756`. | CodeFriend proof packet plus `CODEFRIEND_V1_BUILD_PLAN.md` and `runtime_v2.codefriend_adapter_obligations.v1` handoff | Planned candidate |
| D5 | Editor boundary proof | Web editor baseline works with the validated lifecycle and the Zed decision is explicit. | editor integration packet and decision note | Planned candidate |

## Coverage Rules

- Every user-facing `v0.95` feature should map to a walkthrough or supporting
  demo surface.
- Every operator-facing or evaluator-facing `v0.95` feature should map to a
  bounded proof packet even if it is not a flashy demo.
- The MVP walkthrough should integrate the milestone instead of acting as a
  disconnected showcase reel.

## Explicit MVP reconciliation outputs — #1407

WP-11 consumes the full127-row feature acceptance/disposition map and exact predecessor owner lockset, then records feature freeze,1.0 boundary and every residual/exclusion. WP-12 maps required features to actual reused owner proof or a named gap. WP-13 records bounded public-doc/style cleanup disposition and final review; no forced closeout-only PR.

Before the relevant consumer executes or MVP acceptance proceeds, resolve D01 Shepherd/Gemma evaluated-versus-trained output, D02 required payments subset, D03 UTS/SDK and independent ATEC/AVP consumption, D04 speculative backend, D05 optional Zed and D06 old logistic split. D08 decision-model pilots remain proposed/unallocated. An open decision is not proof, an exclusion or authority to implement. All options and deferred proposals remain in the [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md).
