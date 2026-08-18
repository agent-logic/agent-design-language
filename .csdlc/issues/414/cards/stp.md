# Structured Task Prompt

Template: 1.0.0

Issue: 414

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the resident adapter, interruption/storage/restore bridge, and truthful local CPU habitability proof over existing continuity authority.

## Deliverables

- .csdlc/prepared/issues/414/design.md
- .csdlc/prepared/issues/414/diagram.mmd
- .csdlc/prepared/issues/414/design-bindings.json
- .csdlc/prepared/issues/414/validate_preparation_bundle.py
- .csdlc/issues/414
- .csdlc/evidence/414
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime/src/lib.rs
- adl/Cargo.toml
- adl/Cargo.lock
- adl/src/lib.rs
- adl/src/resident_shepherd_spot_continuity.rs
- adl/src/bin/adl_resident_shepherd_continuity.rs
- adl/src/runtime_v2/agent_lifecycle_state.rs
- adl/src/runtime_v2/citizen.rs
- adl/src/runtime_v2/contracts.rs
- tools/aws_remote_validation/scripts/remote_validation_runner.sh
- tools/aws_remote_validation/src/aws_remote_validation.rs
- adl/tools/aws_spot_artifact_finalize.py
- adl/tools/test_aws_spot_artifact_finalize.sh
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/test_run_aws_spot_remote_validation_lane.sh
- adl/tools/run_issue414_cpu_shepherd_continuity.sh
- adl/tools/run_issue414_llama_baseline.sh
- adl/tools/issue414_spot_dehydrate_callback.sh
- adl/tools/issue414_restore_and_admit.sh
- adl/tools/issue414_s3_linux_bootstrap.py
- adl/tools/test_issue414_s3_linux_bootstrap.py

## Acceptance

1. AC-1: The adl integration orchestrator maps every admitted resident agent exactly once into the existing Runtime-v2 snapshot, post-restore rehydration report, per-agent CSM capsule, and signed singleton live_kernel authorities without inventing a lifecycle, capsule, participant, service, lineage, or recovery system; no successful rehydration report exists before actual restore.
2. AC-2: A confirmed IMDSv2 interruption closes admission before the existing ACTIVE -> SUSPENDED -> DORMANT Spot path; ACTIVE -> QUIESCENT -> ACTIVE remains separate habitability behavior; one absolute deadline bounds every status, stop, capsule, and checkpoint boundary through a TERM-then-KILL child cap; any partial failure preserves closed admission and every stop intent and emits no termination readiness.
3. AC-3: Signed continuity is retained on a dedicated Runtime volume distinct from build cache; safe single-component agent IDs, every capsule, and exact model/configuration bindings validate before complete-population restore; the actual Runtime-v2 rehydration report is created after restore, then the durable global admission-open pointer commits before any local stop intent is cleared, with fail-closed rollback.
4. AC-4: Deterministic tests prove multiple distinct resident agents with no missing or duplicate activation; one retained nonqualifying reference run proves two distinct agents on pinned llama3.1:8b Q4 perform useful work before signed dehydration and deterministic continuation after validated restore.
5. AC-5: Identity, lifecycle state, sequence, predecessor, state digest, model artifact digest, quantization, configuration digest, completed-task digest, and continuation-request digest remain exact across recovery.
6. AC-6: Tamper, partial population, rollback, unsafe agent path, model or volume substitution, oversized or busy resident, legacy missing subrecord, and deadline cases fail closed before mutation/admission.
7. AC-7: Operator-visible redacted receipts expose lifecycle, admission, checkpoint, actual rehydration, task, latency, population, and resource state without prompts, secrets, model weights, or private-state authority; historical three-model attempts remain explicitly classified non-proving.
8. AC-8: A reviewed Linux/x86 bootstrap contract binds the approved S3 bucket, exact executing reviewed Git SHA, exact runner and continuity-binary SHA256, create-only immutable installer objects, Ollama 0.31.1 Linux-amd64 object, SHA256 model-store manifests, and llama3.1:8b/qwen3:8b think:false/phi4-mini matrix; S3 is bootstrap cache only. Exact three-model 8-vCPU/64-GiB r7i.2xlarge measurement and artifact staging/fetch are deferred to #268; Mac MLX/Metal blobs and #269 execution are forbidden.

## Dependencies

- Existing Runtime v2 lifecycle/snapshot/rehydration authority on current main
- Existing LiveContinuity signed checkpoint and restore authority on current main
- Terminal/canonical/ancestral #414 is consumed by #256, #341, and #268

## Inputs

- agent-logic/agent-design-language#414
- adl/src/runtime_v2/agent_lifecycle_state.rs
- adl/src/runtime_v2/types.rs
- adl/src/runtime_v2/snapshot.rs
- adl/src/long_lived_agent.rs
- adl/src/csm_continuity_capsule.rs
- adl-runtime-kernel/src/live_continuity.rs

## Non Goals

- New continuity state, snapshot schema, lineage model, or recovery system
- Model-weight or prompt serialization
- GPU, paid #268 launch, On-Demand fallback, or #269 mutation
- External-model dependency for admission, continuity, useful work, or pass/fail
