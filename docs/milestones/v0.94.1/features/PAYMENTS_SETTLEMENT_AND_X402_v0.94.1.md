# v0.94.1 Feature: Payments, Settlement, and `x402`

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.94.1`.

## Purpose

Give payments, settlement, accounting, and adapter work one explicit milestone
home after governance and secure-execution foundations are in place. This band
should turn the earlier bounded-economics baseline and later `x402` /
Lightning planning into a coherent, reviewable payment-lane contract.

## Source Inputs

- `docs/milestones/v0.94.1/README.md`
- `docs/milestones/v0.94.1/features/README.md`
- `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md`
- `docs/milestones/v0.90.4/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded payment-adapter interface and settlement-rule home for MVP
- explicit relationship among accounting schema, economic trace events, ledger
  families, and payment authorization
- adapter-level planning for `x402` and Lightning without collapsing the whole
  polis into a payments product
- reviewer-facing truth boundaries for demos, settlement, and non-claims

## Non-goals

- claiming production payments are already live
- collapsing bounded economics, contract-market work, and payment rails into
  one undifferentiated “market” story
- external financial, regulatory, or custody claims beyond the bounded adapter
  and settlement architecture
- silent expansion into unrestricted market-making or billing infrastructure

## Completion Target

`v0.94.1`
