# ADR Plan — v0.92.1

Create or update ADRs only for durable architectural decisions:

- C-SDLC v3 state, migration, and compatibility authority
- distributed Runtime continuity or qualification boundaries not already governed
- hot-reload atomicity, validation, and stateful-resource exclusions
- Observatory projection and authenticity boundaries
- Runtime v4 rebaseline decisions, if triggered
- Runtime v2/v3 authority, compatibility, migration, and rollback boundaries established by DEC-01
- provider inference-profile ownership and the boundary between authoritative execution and local-model shadow comparison
- GCP portability qualification differences from the canonical AWS-backed six-resident contract, if durable differences are accepted

Corporate records, podcast operator choices, UI styling, historical #457 provenance, cloud-run receipts, and routine issue sequencing are not ADRs.

## Source-grounded disposition packet

Draft disposition at source baseline `bf617859982f8b9613737344400626eb90768930`.
Four new candidates are tracked by issue #745. Existing
accepted ADRs remain accepted; new candidates are Proposed. No candidate is
promoted by this table.

| Topic | Disposition | Record and reason |
| --- | --- | --- |
| C-SDLC v3 state, migration, compatibility authority | New candidate | [ADR 0072](../../architecture/adr/0072-csdlc-v3-native-authority.md) records native authority and explicit rollback; proposed successor to ADR 0056's generation selection. |
| Distributed Runtime continuity or qualification | Existing architecture covered; qualification-specific decision deferred | [ADR 0054](../../adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md) and [ADR 0055](../../adr/0055-runtime-v3-unified-redb-state.md) cover local ownership and recovery scope. The [qualification feature](features/DISTRIBUTED_MULTI_AGENT_RUNTIME_QUALIFICATION_v0.92.1.md) makes UTS a workload; no new Runtime architecture is introduced. [ADR 0066](../../architecture/adr/0066-distributed-guardian-membership-authority-and-fencing-boundary.md) already owns the Deferred operational membership/continuity/fencing decision, and [ADR 0070](../../architecture/adr/0070-cross-polis-continuity-transfer-planning-boundary.md) owns the Proposed cross-polis transfer planning boundary. Preserve those statuses and their promotion gates; route qualification evidence to those records before proposing any genuinely new difference. A run status alone does not discharge their gates. |
| Hot-reload atomicity, validation, exclusions | New candidate | [ADR 0073](../../architecture/adr/0073-validated-configuration-snapshot-reload.md) records whole validated snapshots and stateful-resource exclusions. |
| Observatory projection and authenticity | Update existing deferred record | [ADR 0069](../../architecture/adr/0069-observatory-governed-runtime-consumer-boundary.md) retains its dual-client proof gate and adds v0.92.1 evidence. ADRs 0048/0054 remain the accepted boundaries. |
| Runtime v4 rebaseline | Deferred; not triggered within this contract | [Milestone decisions 7 and 12](DECISIONS_v0.92.1.md) exclude v4 and require explicit replanning. DEC-01 is not that rebaseline. |
| DEC-01 Runtime v2/v3 authority and compatibility | New candidate | [ADR 0074](../../architecture/adr/0074-runtime-generation-source-ownership.md) records source ownership, compatibility bridge, and bounded migration/rollback dry runs. |
| Provider profiles and shadow comparison | New candidate | [ADR 0075](../../architecture/adr/0075-provider-profile-and-shadow-authority.md) refines ADRs 0004/0041; ADR 0071 remains deferred. |
| GCP portability differences | No new accepted-difference ADR established; deferred | The [GCP feature](features/GCP_SIX_RESIDENT_QUALIFICATION_v0.92.1.md) preserves the AWS contract and separate provider identity/cost/cleanup evidence. [DRT-D evidence](evidence/runtime/drt-d/qualification.json) records a provider-specific run, not a decision changing resident semantics. A reviewed accepted difference and its portability consequences are required before a new ADR. |

The [release-tail gap report](evidence/integration/gap_analysis_report.md) is
retained evidence of review and semantic-proof debt. Neither curation nor
provider run receipts resolve it. Distributed/GCP deferral here concerns new
architectural claims, not deletion of the existing qualification requirements.

Acceptance requires the normal issue-bound review/publication path. The
unresolved candidate decisions retain their own approval gates; publication of
this disposition table alone does not make them accepted.
