# v0.95 Decisions

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning decision log. These are current milestone-shaping decisions,
not final implementation acceptances.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact |
| --- | --- | --- | --- | --- | --- |
| D-01 | Treat `v0.95` as MVP convergence rather than a new architecture milestone. | accepted | The feature list and roadmap already assign the remaining large new domains to earlier milestones. | Let `v0.95` absorb overflow from any band. | Keeps the MVP boundary explicit. |
| D-02 | Require a web editor baseline even if Zed remains optional. | accepted | Editor/operator capability needs one required MVP floor. | Treat Zed or another host-specific editor as the only path. | Preserves portability and workflow truth. |
| D-03 | Keep distributed execution subordinate to secure-execution and review boundaries. | accepted | Integration is valuable only if it stays deterministic and reviewable. | Recast distributed execution as a separate orchestration architecture. | Protects the platform’s core trust story. |
| D-04 | Treat dashboard/reporting and evaluation-platform surfaces as first-class MVP work. | accepted | These surfaces are now explicit feature rows and cannot remain implied. | Leave them as backlog-like supporting notes. | Aligns milestone package with feature-list truth. |

## Open Questions

- Should Zed ship as an MVP integration, or should `v0.95` close with an
  explicit defer/drop decision?
- Which `v0.94.1` payments/economic surfaces are truly required inputs for MVP
  convergence, and which remain adjacent but non-blocking?

## Explicit MVP reconciliation outputs — #1407

WP-11 consumes the full127-row feature acceptance/disposition map and exact predecessor owner lockset, then records feature freeze,1.0 boundary and every residual/exclusion. WP-12 maps required features to actual reused owner proof or a named gap. WP-13 records bounded public-doc/style cleanup disposition and final review; no forced closeout-only PR.

Before the relevant consumer executes or MVP acceptance proceeds, resolve D01 Shepherd/Gemma evaluated-versus-trained output, D02 required payments subset, D03 UTS/SDK and independent ATEC/AVP consumption, D04 speculative backend, D05 optional Zed and D06 old logistic split. D08 decision-model pilots remain proposed/unallocated. An open decision is not proof, an exclusion or authority to implement. All options and deferred proposals remain in the [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md).
