# v0.92 ADR Candidate Index

This issue #143 packet records review candidates only. Accepted architecture
authority remains in `docs/adr/`; none of these records is Accepted.

| Candidate | Status | Decision boundary | Primary evidence |
| --- | --- | --- | --- |
| ADR 0059 | Proposed | First true birthday evidence | `adl-runtime-kernel/tests/birthday.rs` |
| ADR 0060 | Proposed | Stable identity and bounded continuity | `adl-runtime-kernel/tests/birthday.rs` |
| ADR 0061 | Proposed | Memory grounding and capability envelope | `.csdlc/evidence/5829/native-validation-manifest.json` |
| ADR 0062 | Proposed | Witness and birthday receipt authority | `.csdlc/evidence/5833/local-validation-manifest.json` |
| ADR 0063 | Proposed | ACP cognitive-profile authority | `.csdlc/evidence/144/local-validation-manifest.json` |
| ADR 0064 | Proposed | Adaptive-learning DAG governance | `.csdlc/evidence/5831/native-validation-manifest.json` |
| ADR 0065 | Proposed | ACIP schema and governed projection | `.csdlc/evidence/283/evidence-manifest.json`; replacement authority `.csdlc/evidence/209/local-validation-manifest.json` |
| ADR 0066 | Deferred | Operational distributed Guardian authority and fencing | `.csdlc/evidence/284/evidence-manifest.json` |
| ADR 0067 | Proposed | Runtime transport and TLS stack | `adl-runtime/tests/distributed_transport.rs` |
| ADR 0068 | Deferred | Birthday-to-governance handoff | `.csdlc/evidence/285/evidence-manifest.json` |
| ADR 0069 | Deferred | Observatory governed Runtime consumer | `.csdlc/evidence/286/adr0069-evidence-reconciliation.md` |
| ADR 0070 | Proposed | Cross-polis continuity transfer planning | `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md` |
| ADR 0071 | Deferred | Provider-neutral multi-agent proof | `.csdlc/evidence/287/evidence-manifest.json` |

Promotion requires separate human approval and a separate change to
`docs/adr/`. #288 promoted only ADR 0065 to Proposed from terminal replacement
authority. ADR 0066, ADR 0068, ADR 0069, and ADR 0071 remain Deferred because
their #284, #285, #286, and #287 reconciliation packets retain explicit
residual gaps.
