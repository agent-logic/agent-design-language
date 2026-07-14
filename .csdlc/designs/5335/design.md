# Issue 5335 design: v0.91.8 ADL Core Rearchitecture planning

Status: candidate for bounded review and C-SDLC design approval.

## Decision

Create v0.91.8 as a bridge milestone dedicated to clean-room replacement of
the monolithic `adl/` crate. Preserve v0.92 as the first-birthday milestone;
v0.92 consumes the resulting platform rather than absorbing its construction.

## Product boundary

The planned replacement contains four bounded surfaces:

1. a typed six-primitives language and schema layer;
2. a pure deterministic compiler to a versioned execution plan;
3. a portable bounded execution state machine with provider/tool ports; and
4. a thin CLI plus narrow independently owned adapters.

Runtime supervision and cognitive services remain owned by Runtime v3.
C-SDLC remains owned by C-SDLC v2. Provider/cloud integrations, demos, and
proof tooling do not enter the default ADL core dependency graph.

## Method

Follow the method proven by Runtime v3 and C-SDLC v2:

- pin an authoritative legacy denominator;
- treat legacy code as behavioral evidence rather than source architecture;
- capture a compact normalized characterization corpus;
- implement independently under explicit LoC and dependency budgets;
- run shadow parity and opt-in soak;
- prove reversible default switching;
- delete by owner band after approval.

## Budgets

- target 90 percent incumbent deletion;
- fail closeout below 80 percent;
- target 20,000 implementation LoC, hard ceiling 30,000;
- target 8,000 test LoC, hard ceiling 15,000;
- at most four core crates and five installed owner binaries;
- warm focused validation under two minutes;
- deterministic non-live validation under ten minutes.

## Setup-issue boundary

Issue 5335 creates the full planned-posture milestone package, source and
downstream handoffs, label, sprint, and reviewed child issue topology. It does
not implement, cut over, delete code, or claim parity/release approval.

## Review questions

- Does the issue topology make deletion an outcome rather than code movement?
- Are product owners non-overlapping?
- Are parity, rollback, and deletion independently gated?
- Is v0.92 scope preserved?
