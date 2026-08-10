# v0.92 ADR Candidate Index

This issue #143 packet records review candidates only. Accepted architecture
authority remains in `docs/adr/`; none of these records is Accepted.

| Candidate | Status | Decision boundary | Primary evidence |
| --- | --- | --- | --- |
| ADR 0059 | Proposed | First true birthday evidence | `adl-runtime-kernel/tests/birthday.rs` |
| ADR 0060 | Proposed | Stable identity and bounded continuity | `adl-runtime-kernel/tests/birthday.rs` |
| ADR 0061 | Proposed | Memory grounding and capability envelope | `adl-runtime-kernel/tests/identity_memory.rs` |
| ADR 0062 | Deferred | Witness and birthday receipt authority | `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md` |
| ADR 0063 | Deferred | ACP cognitive-profile authority | `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` |
| ADR 0064 | Deferred | Adaptive-learning DAG governance | `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md` |
| ADR 0065 | Proposed | ACIP schema and governed projection | `adl-runtime-kernel/tests/production_acip_wss.rs` |
| ADR 0066 | Proposed | Distributed Guardian authority and fencing | `adl-runtime/tests/distributed_guardian.rs` |
| ADR 0067 | Proposed | Runtime transport and TLS stack | `adl-runtime/tests/distributed_transport.rs` |
| ADR 0068 | Deferred | Birthday-to-governance handoff | `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md` |
| ADR 0069 | Deferred | Observatory governed Runtime consumer | `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md` |
| ADR 0070 | Proposed | Cross-polis continuity transfer planning | `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md` |
| ADR 0071 | Deferred | Provider-neutral multi-agent proof | `docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md` |

Promotion requires separate human approval and a separate change to
`docs/adr/`. Deferred records require their named executable proof before they
can become Proposed.
