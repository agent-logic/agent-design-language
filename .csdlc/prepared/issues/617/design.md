# Issue #617 Design: Canonical Agent Names in Runtime API

## Decision

Add a dedicated canonical-name field to `AgentRosterEntry` and project it
unchanged into roster and detail responses. Keep operational ID, canonical
two-part name, display label, and public office as distinct API concepts.

## Compatibility

The change is additive. Existing fields retain their meanings and serialized
names. Dynamic agents use the canonical name admitted by the #602 lifecycle
contract. The startup Shepherd receives a stable canonical name from its
configuration rather than from display-label or operational-ID inference.

## Proof Boundary

Focused roster, control, Observatory, and OpenAPI tests prove both dynamic and
Shepherd behavior, JSON serialization, required schema inventory, and the
absence of field substitution or inferred identity.
