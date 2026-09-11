# [v0.92.2][CF-EVIDENCE] Admit and retain governed evidence through the production adapter

## One complete result and dependency

The production local adapter invokes a durable evidence admission/store boundary enforcing stable identity, immutable provenance, redaction before retention/model use, retention/deletion and the shared finding/run contract. Depends on CF-ADAPTER's merged packet (canonical number supplied at batch creation). The contract must merge before CF-REVIEW, CF-MEMORY and CF-UX consumers execute; this task does not implement their review/matching/publication algorithms.

## Source and proposed owned paths

Use the predecessor's selected local product entrypoint and packet reader. Proposed new ownership: `adl/src/codefriend/evidence/mod.rs`, `identity.rs`, `store.rs`, `redaction.rs`, versioned fixtures in `adl/tests/fixtures/codefriend/evidence/`, focused `adl/tests/codefriend_evidence.rs`, and the minimum adapter-to-store wiring. Resolve existing redaction utilities before adding a duplicate implementation. Read all record semantics in `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and `features/EVIDENCE_CORE_v0.92.2.md`. Own versioned schemas and canonical test vectors alongside the production boundary, not a free-standing paper contract. Retain proof under the assigned issue's `.csdlc/evidence/` directory.

## Acceptance and proving cases

1. Run actual local ingestion into the production store, restart/open it and read back admitted evidence with stable object identity, content digest, source revision, repo-relative location, immutable provenance and declared redaction/retention policy. Repeat/relocate identical source and prove identity stability; tampering and identity collisions fail closed.
2. Apply secret redaction/omission before retained bytes or model exposure. Execute hostile embedded instructions and nested credential negatives; inspect output, logs and store to prove prohibited raw material never entered retention. Existing unsafe packets are rejected rather than silently retained. These tests prove declared fixtures, not universal prompt-injection or secret prevention.
3. Execute actual retention expiry/deletion and subsequent denied retrieval with controlled clocks. State metadata/tombstone/provenance retention rules, avoiding raw deleted content; prove an interrupted write cannot expose partially admitted evidence or fabricate success.
4. Deliver the complete versioned shared contract: run identity/repository/revision/scope/inclusions/exclusions/lane versions/provider route/completion/failures; evidence identity/digest/location/revision/redaction/retention/provenance; finding stable match identity/perspective/rule/severity+rationale/confidence/evidence/inference/scope/limitations; comparison baseline/current/version/match reason/delta or not-comparable; publication exact run/finding and manifest digests/render versions/claims/approval target/withheld state. Provider route identity excludes credentials.
5. Canonical finding identity is distinct from evidence identity and cannot rely solely on mutable prose or line number. Cross-consumer test vectors reject collisions, missing evidence, incompatible versions, changed scope and false resolution from partial/narrower runs. They preserve complete/incomplete/failed/cancelled/withheld states. CF-MEMORY owns actual matching, CF-UX approval enforcement and review owns lane execution; their later use of the same fixtures is an explicit prerequisite, not a dependency cycle or claim they already exist.
6. Execute adapter admission plus store readback and the reusable review/memory/renderer contract conformance suite. Record production consumer invocation; a schema, isolated identity helper or packet without retention/deletion execution cannot close this task. Scope/finding/renderer/target changes must be representable as approval invalidation; do not implement full publication here.

## PVF and exclusions

PVF: deterministic local production integration and contract/security negatives; role: admission, identity/provenance, redaction before retention, actual expiry/deletion and shared-consumer compatibility; resources: bounded CPU/disk with injected clocks and isolated store, no provider/cloud; gate: required pre-consumer Beta evidence boundary. Stop on raw secret retention, mutable identity, unverifiable provenance, unsupported contract ambiguity or missing/failed proof. Exclude public evidence hosting, review/synthesis algorithms, longitudinal matching implementation and full publication; supporting schema work alone is not delivery.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `stable_identity`, `immutable_provenance`, `fail_closed_redaction`, `explicit_retention`, `finding_run_contract_merged_before_consumers`, `adapter_packet_admitted`, `durable_evidence_readback`, `retention_delete_executed`, `consumer_conformance`.

pvf: `determinism`, `tamper_detection`, `redaction_negative_suite`, `retention_contract`, `cross_consumer_conformance`, `adapter_packet_admitted`, `durable_evidence_readback`, `retention_delete_executed`, `consumer_conformance`, `schema_only_rejected`, `secret_before_retention_rejected`, `tampered_provenance_rejected`.

stop_conditions: `unredacted_secret`, `mutable_identity`, `unverifiable_provenance`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `public_evidence_hosting`.

