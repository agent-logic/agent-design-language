# Observatory and Unity Consumer Integration

## Status

Explicitly removed from the `v0.92` completion claim by operator scope authority.
Remaining Unity work is owned by backlog issue `#84`; its public exposure
dependency `#122` is scheduled for `v0.92.1`, while TLS 1.2 dependency `#251`
remains backlog. The Observatory remains a separate application. This document
is retained as the later-work contract, not as a planned `v0.92` deliverable.

## Purpose

Make the HTML Observatory and Unity client real consumers of the same versioned
Runtime projection and event stream without embedding UI code or private state
inside Runtime v3.

## Required Behavior

- Read-only projection APIs are available to clients without a write session;
  authenticated login and explicit authority are required for writes.
- HTTP snapshots and authenticated full-duplex WSS events share versioned
  schemas, stable identifiers, ordering/correlation rules, reconnect behavior,
  and bounded backpressure.
- The Runtime exposes redacted public/operator/reviewer projections, never raw
  private citizen state, keys, or sealed checkpoints.
- HTML and Unity preserve their existing approved designs while binding every
  control, menu, proof link, packet link, and operator action to real behavior.
- Proof and packet links open independently; presentation modes never widen
  authority or data access.
- TLS trust, API discovery, WSS reconnect, stale data, unavailable services,
  authorization refusal, and Runtime restart are explicit client states.

## Proof

When `#84` is executed, it must run both clients against the actual Runtime API and WSS stream,
exercise reads and authenticated writes, prove redaction and refusal cases,
verify reconnect after Guardian-owned restart, and retain browser plus native
Unity evidence without fixture substitution.

No part of this proof is required for `v0.92` release credit. `#84`, `#122`,
and `#251` remain authoritative for later execution.

## Non-Goals

- No Observatory HTML served from Runtime.
- No design change without explicit operator approval.
- No client-side private-state access or authority bypass.
- No Unity-only fork of Runtime schemas or platform behavior.
