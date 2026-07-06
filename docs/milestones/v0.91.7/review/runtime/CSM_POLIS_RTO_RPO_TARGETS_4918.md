# CSM Polis RTO/RPO Targets (#4918)

Status: `measured_recovery_objective_packet`

This packet defines bounded recovery-time objective (RTO) and recovery-point
objective (RPO) classes for the current CSM/Polis runtime surface. It consumes
current WP-07 evidence only and separates measured local/cloud proof from
production non-claims.

## Source Ledger

| Surface | Evidence | Use |
| --- | --- | --- |
| CSM owner split and daemon | `docs/milestones/v0.91.7/review/runtime/csm_4890/README.md`, `RUNTIME_DAEMON_SUPERVISION_4885.md` | CSM is runtime owner; daemon writes recoverable state and checkpoint artifacts. |
| Continuity capsule | `docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json` | Portable capture/stage/restore format, retained runtime roles, negative cases. |
| AWS restore/fire-up | `docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/aws_remote_restore_fireup_summary.json` | Measured EC2 Spot restore/fire-up timing and cleanup truth. |
| Runtime API | `docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md` | `/health`, `/ready`, `/metrics`, `/events` observability checkpoints. |
| No-sparrow observability | `docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/README.md` | Local event-retention coverage and telemetry non-claims. |
| Incident command | `docs/milestones/v0.91.7/review/runtime/CSM_POLIS_INCIDENT_COMMAND_RUNBOOKS_4922.md` | Operator flow and post-incident record schema. |

## Objective Classes

| Class | Scope | Target | Measurement method | Current status |
| --- | --- | --- | --- | --- |
| `local_checkpoint_rpo` | Running CSM daemon checkpoint cadence | RPO target: `<= 3s` between daemon partial checkpoints when daemon is healthy and configured with default cadence. | `RUNTIME_DAEMON_SUPERVISION_4885.md` proves default `--checkpoint-interval-secs 3` and retained `checkpoint_write` events. | target_defined_with_local_proof |
| `local_observability_rpo` | Retained operator/OTel events | RPO target: every significant local event class retained or explicitly non-claimed. | `no_sparrow_4909/proof_summary.json` and coverage matrix. | target_defined_with_local_proof |
| `capsule_restore_rpo` | Selected continuity capsule contents | RPO target: restored capsule reaches the captured point-in-time state with required manifest, stage, restore, and daemon fire-up checks passing; no continuous-write RPO claim beyond the selected capsule point. | `proof_summary.json` reports `artifact_count: 18`, retained roles, restored status, daemon fire-up pass, and negative missing/corrupt/path/credential cases. | measured_for_selected_capsule |
| `aws_restore_fireup_rto` | Restore/fire-up from retained capsule on Agent Logic EC2 Spot | RTO target: `<= 10m` for current bounded proof lane, excluding operator decision time and production traffic cutover. | `aws_remote_restore_fireup_summary.json` timings. | measured_pass |
| `runtime_api_readiness_rto` | Local API diagnosis after runtime artifacts exist | RTO target: operator can classify health/readiness from retained artifacts once `csm api serve` starts. | #4929 proves `/status`, `/health`, `/ready`, `/metrics`, `/events`. | target_defined_with_local_proof |
| `production_multi_region_rto_rpo` | Production hosted runtime, cloud failover, secret-bearing provider state | No target claimed in v0.91.7. | Requires future production topology, durable storage, provider-secret handling, and control-plane proof. | non_claim |

## Measured Recovery Scenario

Scenario: restore and fire up CSM continuity capsule on Agent Logic EC2 Spot.

Evidence:

- `docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/aws_remote_restore_fireup_summary.json`
- `docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json`

Measured RTO:

| Segment | Seconds |
| --- | ---: |
| EC2 launch | `9` |
| SSM ready | `15` |
| Remote restore/fire-up command | `275` |
| Operational restore/fire-up subtotal | `299` |
| Teardown | `91` |
| Wrapper/accounting overhead | `2` |
| Total retained wrapper time | `392` |

RTO interpretation:

- The measured recovery proof satisfies the bounded v0.91.7 target of
  `<= 10m` for restore/fire-up in the retained Agent Logic EC2 Spot lane.
- The operational restore/fire-up subtotal is `299s`; the full retained wrapper
  including teardown is `392s`.
- This is not a production traffic failover measurement.

Measured RPO:

| Scope | Measurement | Result |
| --- | --- | --- |
| Selected continuity capsule | Required artifact restoration relative to captured manifest. | Capture, stage, restore, and restored daemon fire-up are recorded as passed for the selected capsule; negative missing/corrupted/path/credential cases failed as expected. |
| Running daemon checkpoint cadence | Default partial checkpoint target. | `<= 3s` target from daemon supervision proof when the daemon is healthy and using default cadence. |
| Continuous production writes | Not measured. | non_claim |

RPO interpretation:

- For the selected capsule scenario, RPO is point-in-time: the restored runtime
  recovers the captured capsule contents and does not claim writes after capture.
- For a healthy running daemon, local checkpoint cadence defines a `<= 3s`
  checkpoint target, but this issue does not prove production-grade durable
  storage or cross-region replication.

## Observability Requirements

Every recovery measurement must retain:

- recovery start timestamp
- selected checkpoint or capsule ref
- daemon status ref
- runtime API `/health` and `/ready` sample when available
- replay or restore result
- OTel/operator event refs
- completion or failure classification
- non-claims and follow-up owners

Current evidence satisfies these requirements for the selected #4910 continuity
capsule restore/fire-up scenario through retained summary, proof, observability,
OTel, and negative-case artifacts. Future incidents should use the #4922
post-incident template.

## Gap Register

| Gap | Current disposition | Owner route |
| --- | --- | --- |
| Production multi-region RTO/RPO | non_claim | Future production topology / WP-08 cloud operations. |
| Provider-secret-bearing state export | non_claim; #4910 explicitly excludes provider tokens/secret material. | Security/provider storage owners. |
| 12-nines durable storage guarantee | scheduled/not proven by WP-07 packet. | #4913 / storage owner. |
| Per-agent snapshot/diff cadence | scheduled/not proven by this packet. | #4912 / Godel agent snapshot owner. |
| Safe-fail serialization maximization | scheduled/not proven by this packet. | #4911. |
| Restore fire-drill cadence | target consumer; not implemented here. | #4919. |
| Overload/backpressure RTO effect | target consumer; not implemented here. | #4921. |
| Final coherence gate | consumes this packet, not replaced by it. | #4906. |

## Non-Claims

- No production RTO/RPO is claimed from local-only or single EC2 Spot proof.
- No hosted telemetry backend RTO/RPO is claimed.
- No provider-secret export or secret-bearing restore is claimed.
- No multi-region disaster recovery, DNS/CloudFront cutover, or customer traffic
  failover is claimed.
- No unmeasured recovery target is treated as satisfied.

## Validation

Focused checks for this packet:

```sh
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/proof_summary.json >/dev/null
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910/aws_remote_restore_fireup_summary.json >/dev/null
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/proof_summary.json >/dev/null
python3 -m json.tool docs/milestones/v0.91.7/review/runtime/no_sparrow_4909/analysis/no_sparrow_coverage_matrix.json >/dev/null
test -f docs/milestones/v0.91.7/review/runtime/RUNTIME_DAEMON_SUPERVISION_4885.md
test -f docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md
test -f docs/milestones/v0.91.7/review/runtime/CSM_POLIS_INCIDENT_COMMAND_RUNBOOKS_4922.md
```

Expected result: retained #4910 and no-sparrow JSON evidence parses, and the
daemon, runtime API, and incident-command evidence packets are present.

Legacy wording note: older cited no-sparrow evidence uses the previous
portable-state terminology. This #4918 packet uses `continuity capsule` and
`portable continuity bundle` terminology for current claims and treats older
terminology in cited evidence as historical source text, not preferred current
operator language.

## Build Platform Timing

Comparable benchmark command:

```sh
bash adl/tools/run_build_platform_benchmark.sh --platform wuji --cache-posture local_warm_target --out .adl/local-artifacts/build-platform/4918-wuji-summary.json --artifact-dir .adl/local-artifacts/build-platform/4918-wuji
```

Current rows:

| Platform | Cache posture | Build | Test | Benchmark total | Wrapper wall | Status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `wuji` | `local_warm_target` | `105s` | `108s` | `213s` | `213s` | passed |
| `nessus` | `persistent_remote_target_cache` | `25s` | `<1s` | `25s` | `31s` | passed |
| `codebuild` | `fixed_builder_image_stable_local_target_cache_s3_sccache` | `43s` | `77s` | `120s` | `149s` | passed |
| `aws_spot` | `fixed_builder_image_warm_ebs_cache` | not_run | not_run | not_run | `<1s` | pre_launch_tooling_failure |

Observed speed note: the local benchmark again paid a cold worktree build/test
profile cost. Nessus is materially faster for this tiny lane (`31s` wrapper
wall versus `213s` local). CodeBuild passed with `120s` benchmark time and
`149s` submit-to-complete wall, backed by a 100% sccache hit rate in the
retained CodeBuild log. The Spot wrapper reproduced the same pre-launch tooling
problem observed on #4922: the Agent Logic AWS profile resolved, but the issue
worktree did not contain an executable repo-owned `adl-aws-remote-validation`
binary. Per operator instruction, this packet records the missing repo binary
instead of building it ad hoc.
