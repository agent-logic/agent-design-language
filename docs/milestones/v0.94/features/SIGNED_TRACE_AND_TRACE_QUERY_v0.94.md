# v0.94 Feature: Signed Trace and Trace Query

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.94`.

## Purpose

Turn trace from a review and audit surface into a signed, queryable
reasoning/provenance substrate that can support secure execution, governance,
and later temporal/self-projection features without becoming hidden state.

## Source Inputs

- `docs/milestones/v0.94/README.md`
- `docs/milestones/v0.94/WBS_v0.94.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- signed trace as a first-class provenance surface
- queryable trace contracts over execution, reasoning, and governance events
- trace contracts for WebSocket-carried ACIP messages and normalized provider
  session events, including connection lifecycle, schema version, sequence,
  policy disposition, replay disposition, and cryptographic disposition
- bounded reasoning/provenance closure rather than free-form historical search
- compatibility with secure execution, policy, identity/auth, and MTT
- a clear canonical home for the trace-query story inside the `v0.94` band

## Non-goals

- replacing earlier trace schema/emission baseline work
- unconstrained analytics or surveillance queries
- public leakage of private state through query convenience
- treating transport logs or provider session logs as signed ADL trace merely
  because they were observed

## WebSocket/ACIP Trace Handoff

`v0.92` may prove binary ACIP over mock or local WebSocket carriers, and may
optionally spike server-side provider WebSocket sessions. `v0.93` owns the
security hardening for those message paths. `v0.94` should make the resulting
events signed and queryable so a reviewer can reconstruct:

- which schema version governed the message
- which connection/session carried it
- which identity, policy, key, and sequence checks were applied
- whether the message was accepted, denied, replayed, malformed, or superseded
- which downstream execution, reasoning, or governance event it caused

The trace model should support conversion back to public JSON evidence through
the relevant public schema catalog without leaking private state.

## Completion Target

`v0.94`
