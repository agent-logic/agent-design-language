# Issue #617 Design: Canonical Agent Names in Runtime API

## Decision

Add the serialized field `name` to `AgentRosterEntry` and project it unchanged
into roster and detail responses. Keep operational `id`, canonical two-part
`name`, display `label`, and public `role`/office as distinct API concepts.

Dynamic agents take `name` from the validated `AgentAdmissionRequest.name` and
carry it through `AgentSample` and `AgentRuntimeEvidence`. The startup Shepherd
takes its canonical name from a required `[resident_shepherd]` Runtime init
configuration section. `RuntimeInitConfig` validates it with the same canonical
two-part-name rule; the production binary passes the configured identity into
`AgentPopulationFeed::resident_shepherd`. No roster layer infers it from ID,
display label, office, provider, or model.

## Compatibility

The wire field is `name`. The entry schema identifier remains
`adl.runtime_v3.agent_roster_entry.v1` because this is an additive JSON member;
existing fields retain their meanings and serialized names. Outbound roster and
detail responses always include nonempty canonical `name`, and the OpenAPI
schema marks it required. `AgentRosterEntry` deserialization uses a default only
to read previously recorded v1 payloads that lack `name`; a focused legacy JSON
test proves that read compatibility, while a new-output test proves the Runtime
never emits the empty compatibility default. Standard JSON clients that ignore
unknown members remain compatible; strict clients that reject additive members
are explicitly outside the v1 compatibility guarantee.

## Proof Boundary

Focused configuration, roster, control, Observatory, and OpenAPI tests prove
both dynamic and Shepherd behavior, legacy-payload deserialization, current JSON
serialization, required schema inventory, and the absence of field substitution
or inferred identity.
