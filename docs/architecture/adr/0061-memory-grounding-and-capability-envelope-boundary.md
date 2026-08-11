# ADR 0061: Memory Grounding And Capability Envelope Boundary

## Status

Status: **Proposed**

## Context

Birthday and continuity decisions need bounded memory and capability inputs,
not unrestricted recollection or ambient tool authority.

## Decision

Memory grounding uses canonical, cited, continuity-bound references. The
capability envelope is a versioned record bound to the accepted birthday and
identity evidence. Private, stale, uncited, future, mismatched, or unauthorized
inputs fail closed.

## Consequences

Runtime consumers receive explicit memory and capability boundaries without
turning Memory Palace into identity or granting ambient capabilities.

## Alternatives Considered

Embedding arbitrary memory blobs or inferred tool access in identity records
was rejected because it destroys provenance and least authority.

## Source Evidence

- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `adl-runtime-kernel/src/identity_memory.rs`
- `adl-runtime-kernel/src/memory_palace.rs`
- `adl-runtime-kernel/src/capability_envelope.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/identity_memory.rs`
- `adl-runtime-kernel/tests/memory_palace.rs`
- `adl-runtime-kernel/tests/capability_envelope.rs`
- `.csdlc/evidence/5828/memory_palace-runtime-v3.log`
- `.csdlc/evidence/5829/capability_envelope-runtime-v3.log`
- `.csdlc/evidence/5829/native-validation-manifest.json`

## Supersession Relationships

Refines ADR 0058 and preserves ADR 0015 capability authority.

## Non-Claims

Does not prove complete autobiographical memory or unrestricted retrieval. The
envelope does not grant capabilities, invoke providers, models, tools, or
skills, expose credentials, prove unlimited capacity, or establish birthday
authority.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
