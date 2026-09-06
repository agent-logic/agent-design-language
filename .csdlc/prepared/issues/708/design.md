# Versioned agent-orientation Runtime resource design

Issue #708 makes the existing Axioma Polis welcome package a first-class
Runtime resource. The canonical authored input remains
`docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md`; this issue consumes that
document and does not rewrite it.

The Runtime loads one validated orientation resource containing a declared
version, digest algorithm, digest, source provenance, and exact deliverable
content. A deterministic projection is permitted only when its projection
identifier and resulting content are themselves versioned and digest-bound.
The digest recorded on an agent must cover the exact bytes injected into that
agent's initial system context, not merely the source file or current global
configuration.

Admission snapshots the active valid orientation before the first model turn,
injects the snapshot as explicitly non-authoritative orientation, and stores
the delivered version and digest on the agent record. Existing agents retain
their recorded snapshot across reload. A valid reload affects only later
admissions; an invalid reload preserves the last valid resource.

Runtime projections expose the stored per-agent version and digest. The
Observatory displays those values as provenance and never describes the
welcome package as granting authority. Runtime policy, operator authority,
admission, and Layer 8 remain higher-priority boundaries.

Implementation should use the smallest existing Runtime configuration,
admission, agent-record, projection, and Observatory seams. It must not create
a general prompt-template framework.
