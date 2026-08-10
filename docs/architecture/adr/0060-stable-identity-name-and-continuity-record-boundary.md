# ADR 0060: Stable Identity, Name, And Continuity Record Boundary

## Status

Status: **Proposed**

## Context

Names, runtime instances, snapshots, and copied state are insufficient identity
or continuity authorities.

## Decision

Stable identity is rooted in a canonical identity record and digest. Bounded
continuity is a separate ordered record that binds the identity root, prior
head, evidence references, and ambiguity state. A display name is descriptive,
not authority; discontinuity and ambiguity remain explicit.

## Consequences

Identity and continuity can evolve independently while preserving deterministic
verification and fail-closed replay behavior.

## Alternatives Considered

Using a mutable name, process identifier, or copied checkpoint as identity was
rejected because each can fork without preserving authority.

## Source Evidence

- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `adl-runtime-kernel/src/birthday_identity.rs`
- `adl-runtime-kernel/src/birthday_continuity.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/birthday.rs`
- `.csdlc/evidence/5826/birthday_identity-runtime-v3.log`
- `.csdlc/evidence/5827/birthday-continuity-tests.log`

## Supersession Relationships

Refines ADR 0013 and remains compatible with ADR 0058.

## Non-Claims

Does not establish legal identity, universal identity portability, or unlimited
continuity across forks.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
