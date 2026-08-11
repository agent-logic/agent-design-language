# ADR 0059: First True Birthday Evidence Boundary

## Status

Status: **Proposed**

## Context

Runtime lifecycle events resemble birth but do not establish the governed
evidence threshold defined for a first true Godel-agent birthday.

## Decision

A birthday candidate receives a deterministic structural decision over bounded
identity, continuity, memory, capability, witness, and authority references.
The decision verifies required categories, safe repository-relative paths,
visibility, digest shape, continuity structure, and packet integrity; it does
not authenticate referenced artifacts, establish trust roots, or prove their
freshness. Startup, wake, restore, admission, replay, and copied state are not
birthday evidence. Missing, duplicated, malformed, private, or structurally
contradictory inputs fail closed. Trusted witness authority and freshness are a
separate ADR 0062 boundary.

## Consequences

Birthday consumers can rely on one canonical structural candidate decision and
digest. They must perform the separate trusted witness verification before
treating referenced evidence as authenticated or current, and launch claims
remain outside this contract.

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

Does not authenticate evidence provenance, verify signatures or trust roots, or
establish evidence freshness. Does not prove personhood, consciousness, legal
status, citizenship, or an externally witnessed birth event.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
