# ADR 0062: Witness And Birthday Receipt Authority Boundary

## Status

Status: **Deferred**

## Context

The milestone requires witness and receipt authority beyond the implemented
memory and capability slices.

## Decision

Defer this decision until the named witness and receipt implementation proves
signer authority, canonical content, replay rejection, revocation behavior, and
consumer verification.

## Consequences

Current birthday records cannot claim externally authoritative witnessing.

## Alternatives Considered

Promoting fixture shapes or planning prose was rejected because neither proves
the signing and verification path.

## Source Evidence

- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`

## Validation Evidence

- `adl-runtime-kernel/tests/birthday.rs`

## Supersession Relationships

May later refine ADR 0016 and ADR 0053; it supersedes neither now.

## Non-Claims

No authoritative witness network, signed birthday receipt, revocation path, or
external attestation is claimed.

## Approval Boundary

Executable witness and receipt proof plus human review are required before this
record can become Proposed.
