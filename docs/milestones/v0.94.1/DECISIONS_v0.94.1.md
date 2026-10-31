# v0.94.1 Decisions

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning decisions for `v0.94.1`.

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | Payments and settlement belong in `v0.94.1`. | Accepted for planning | The cluster is too large and distinct to remain floating. | Prevents roadmap spill into `v0.95`. |
| D-02 | `x402` and Lightning are adapter-boundary work, not implicit substrate assumptions. | Accepted for planning | The rails need explicit contracts and bounded demos. | Forces a reviewable interface. |
| D-03 | Financial claims remain bounded and non-production. | Accepted for planning | Avoids overclaiming live payment deployment. | Keeps demos and docs honest. |
