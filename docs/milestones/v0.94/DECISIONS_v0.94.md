# v0.94 Decisions

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning decisions for `v0.94`.

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | `v0.94` owns secure execution and trust convergence. | Accepted for planning | These surfaces are too large to remain implicit inside `v0.93`. | Gives the roadmap a real post-governance security/runtime band. |
| D-02 | `v0.94` owns bounded mental time travel / temporal self-projection. | Accepted for planning | MTT depends on identity, trace, chronosense, memory, and bounded simulation. | Makes MTT a named roadmap feature rather than a floating research note. |
| D-03 | Payments, settlement, and `x402` are out of scope for `v0.94`. | Accepted for planning | Financial rails deserve their own follow-on band. | Reserves them for `v0.94.1`. |
