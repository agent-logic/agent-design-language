# v0.92 Features

This directory contains the canonical feature contracts for the completed
v0.92 milestone. Each document describes one bounded capability, its evidence
boundary, and any work deliberately assigned to a later milestone.

## Feature Documents

- [Cognitive profiles](ACP_COGNITIVE_PROFILES_v0.92.md)
- [Adaptive Learning DAG](ADAPTIVE_LEARNING_DAG_v0.92.md)
- [ACIP schema and WebSocket transport](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md)
- [Distributed Guardian and polis](DISTRIBUTED_GUARDIAN_POLIS_v0.92.md)
- [Cross-polis continuity and migration](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md)
- [First-birthday demo and governance handoff](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md)
- [Stable identity and continuity](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md)
- [Memory, capability, and witnesses](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md)
- [Memory Palace context topology](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md)
- [Observatory and Unity integration](OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md)
- [Provider-neutral multi-agent proof](PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md)
- [Runtime launch and resilience](RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md)
- [First true Gödel-agent birthday](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md)

## WP Coverage Map

| WPs | Feature coverage |
| --- | --- |
| WP-08, WP-09, WP-10 | [FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md) |
| WP-03 | [RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md](RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md) |
| WP-04 | [DISTRIBUTED_GUARDIAN_POLIS_v0.92.md](DISTRIBUTED_GUARDIAN_POLIS_v0.92.md) |
| WP-09, WP-10, WP-17 | [IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md), [CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md) |
| WP-11, WP-12, WP-15 | [MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md) |
| WP-11, WP-16 | [MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md) |
| WP-13 | [ACP_COGNITIVE_PROFILES_v0.92.md](ACP_COGNITIVE_PROFILES_v0.92.md) |
| WP-13A | [ADAPTIVE_LEARNING_DAG_v0.92.md](ADAPTIVE_LEARNING_DAG_v0.92.md) |
| WP-14 | [ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md) |
| WP-18, WP-18B, WP-19 | [FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md) |
| Deferred from v0.92; backlog #84 with #122 (v0.92.1) and #251 (backlog) dependencies | [OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md](OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md) |
| WP-18B | [PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md](PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md) |

## v0.92 Completion Gate

Every in-scope feature document in this index must have a landed owning issue and
exact-revision implementation, validation, review, and integration evidence
before the milestone can pass WP-22 or enter WP-25 internal review. A feature
that remains `planned`, lacks real proof, or is replaced by fixtures or
synthetic success is a release blocker. Observatory/Unity is the explicit
operator-approved exception: it is outside the v0.92 completion claim and
routed to backlog `#84`, `#122` (`v0.92.1`), and backlog `#251`.

## Supporting Work Tracks

WP-01/WP-01B planning and docs, WP-02 repository copies, WP-02A CI,
WP-05 through WP-07 workflow tooling, WP-20 proof coverage, WP-21/WP-21A code
quality, WP-22/WP-23 quality and docs, WP-24/WP-24A publication, and WP-25
through WP-30 review/release work support the feature package but are not
standalone product features. Their omission from the feature table is
intentional, not forgotten scope.

The feature contracts are navigation and scope authorities. Exact completion
evidence remains in the owning issues, merged changes, validation records, and
the milestone's quality and review packets.
