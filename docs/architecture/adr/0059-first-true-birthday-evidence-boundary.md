# ADR 0059: First True Birthday Evidence Boundary

## Status

Status: **Proposed**

## Context

Runtime lifecycle events resemble birth but do not establish the governed
evidence threshold defined for a first true Godel-agent birthday.

## Decision

A birthday is a deterministic decision over a bounded candidate containing the
required identity, continuity, memory, capability, witness, and authority
references. Startup, wake, restore, admission, replay, and copied state are not
birthday evidence. Missing, stale, duplicated, or contradictory evidence fails
closed.

## Consequences

Birthday consumers can rely on one canonical decision and digest while launch
claims remain separate from the implemented decision contract.

## Alternatives Considered

Treating process start or first model response as birth was rejected because
neither carries the required evidence or replay boundary.

## Source Evidence

- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `adl-runtime-kernel/src/birthday.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/birthday.rs`
- `.csdlc/evidence/5825/birthday-runtime-v3.log`

## Supersession Relationships

Refines ADR 0016 without superseding its moral-evidence boundary.

## Non-Claims

Does not prove personhood, consciousness, legal status, citizenship, or an
externally witnessed birth event.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
