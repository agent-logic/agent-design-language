# Gemini decomposition and architecture review for ADL issue #203

You are Gemini acting as an external architecture/refactoring advisor for ADL issue #203.

The operator is concerned that #203 may be too large. Do not approve publication, merge, or closeout. Answer decomposition and architecture advice only.

Return strict JSON only with keys: verdict, reason, proposed_slices, architecture_advice, must_fix_before_publication, notes.

Use verdict one of: split_required, split_recommended, keep_single_pr.

For proposed_slices, include name, purpose, paths, depends_on, validation, risk.

Current total LoC including uncommitted work against origin/main: 56 files, +5157 -477, net +4680.
Current code-only LoC excluding docs/json/md/locks/logs: +2988 -477, net +2511.
Current non-code lifecycle/proof LoC: +2169 -0, net +2169.

## Current status

Branch: codex/203-authority-serving-adapters-preparation
Ahead of origin/main by 27 commits with additional uncommitted remediation.

Pre-PR review found #203 still has architecture-sensitive P1 concerns:

1. Raw-store bypass risk:
   - DistributedCertificateStore::open/activate/authorize/revoke
   - AuthorityLedger::new/apply/authorize_mutation
   - FencingStore::create/open/commit
   These raw low-level operations need sealed/crate-private authority tokens or test-only fixture access so production callers must go through governed authority-serving adapters.

2. Published store authority receipt view is too thin:
   - Prior view exposed only lineage_id, operation_id, generation, result_sha256.
   - It should expose only published OwnerCommit/Fence results and include lineage, action class, adapter version, published generation, canonical result digest, and receipt digest.

Current uncommitted remediation adds authority access token types for certificate/lease/fencing stores and extends the published view.

## Diff stat against origin/main

```text
 .../203-path-repair-sip-required-outcome.json      |  12 +
 .../FAILED_TYPED_CARD_PATH_REPAIR.md               |  38 +
 .csdlc/evidence/203/v1/authority-store-proof.json  |   1 +
 .../evidence/203/v1/identity-authority.stderr.log  |   4 +
 .../evidence/203/v1/identity-authority.stdout.log  | 185 +++++
 .csdlc/issues/203/audit.jsonl                      |  30 +
 .csdlc/issues/203/cards/sip.md                     |  51 ++
 .csdlc/issues/203/cards/sor.md                     |  63 ++
 .csdlc/issues/203/cards/spp.md                     | 107 +++
 .csdlc/issues/203/cards/srp.md                     |  45 ++
 .csdlc/issues/203/cards/stp.md                     |  90 +++
 .csdlc/issues/203/cards/vpp.md                     | 140 ++++
 .csdlc/issues/203/index.json                       | 293 +++++++
 .csdlc/prepared/issues/203/design.md               | 250 ++++++
 .csdlc/prepared/issues/203/produce-proof-receipt.rb| 184 +++++
 .csdlc/prepared/issues/203/validate-proof-receipt.rb|191 +++++
 adl-runtime/src/distributed/authority_protocol.rs  |   2 +-
 .../src/distributed/authority_reconciliation.rs    |  67 +-
 .../src/distributed/authority_store_adapters.rs    | 886 +++++++++++++++++++++
 .../src/distributed/capability_advertisement.rs    |  27 +-
 adl-runtime/src/distributed/certificates.rs        |  23 +
 adl-runtime/src/distributed/fencing.rs             |  30 +-
 adl-runtime/src/distributed/lease.rs               | 113 ++-
 adl-runtime/src/distributed/migration.rs           | 262 ++++--
 adl-runtime/src/distributed/mod.rs                 |   1 +
 adl-runtime/src/distributed/placement.rs           |  75 +-
 adl-runtime/src/distributed/projection.rs          |  65 +-
 adl-runtime/src/distributed/recovery.rs            | 399 ++++++++--
 adl-runtime/src/distributed/resource_weather.rs    |  30 +-
 adl-runtime/src/distributed/snapshot_catalog.rs    |  47 +-
 adl-runtime/src/distributed/transport/core.rs      |  94 ++-
 .../transport/governed/learner_transport/tests.rs  |  16 +-
 .../transport/governed/polis_runtime.rs            |  24 +-
 .../tests/distributed_authority_snapshots.rs       |  44 +-
 .../tests/distributed_capability_advertisement.rs  |  25 +-
 adl-runtime/tests/distributed_certificates.rs      | 139 +++-
 adl-runtime/tests/distributed_fencing.rs           | 185 +++--
 adl-runtime/tests/distributed_guardian.rs          |   9 +-
 .../tests/distributed_identity_lease_authority.rs  | 365 +++++++++
 adl-runtime/tests/distributed_lease.rs             | 297 +++++--
 adl-runtime/tests/distributed_migration.rs         |  18 +-
 adl-runtime/tests/distributed_projection.rs        |  66 +-
 adl-runtime/tests/distributed_recovery.rs          |  20 +-
 adl-runtime/tests/distributed_runtime_transport.rs | 103 ++-
 adl-runtime/tests/distributed_snapshot_catalog.rs  |  15 +-
 adl-runtime/tests/distributed_transport.rs         |  16 +-
 56 files changed, 5157 insertions(+), 477 deletions(-)
```

## Requested output

Assess whether this should stay a single PR or be split. If split, propose the smallest reviewable module decomposition and dependency order. Focus especially on architecture boundaries among:

- sealed raw certificate/lease/fencing stores
- governed authority-serving adapter facade
- migration/recovery integration
- transport/projection/peripheral runtime caller migration
- proof and lifecycle evidence

Also identify must-fix architectural risks before any PR publication.
