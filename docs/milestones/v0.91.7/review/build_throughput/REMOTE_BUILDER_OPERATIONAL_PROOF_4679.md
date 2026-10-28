# Remote Builder Operational Proof for `#4679`

Status: `satisfied_by_merged_child_lanes_with_operational_runbooks`
Issue: `#4679`
Date: 2026-07-06

## Scope

Issue `#4679` asked WP-06 to prove EC2 Spot or an alternate remote builder as a
concrete validation/build path. The work was intentionally split into bounded
child issues, then reconciled here:

- `#4837`: AWS Spot EC2 remote validation lane with retained warm EBS cache.
- `#4838`: GitHub Actions plus AWS CodeFriend CodeBuild lane.
- `#4879`: reusable ADL builder image for CodeBuild, Spot, Nessus, and local
  Docker-compatible runners.
- `#4680`: shared Rust cache/linker/target-dir setup for local and remote
  validation surfaces.

This packet records that `#4679` is satisfied by those merged child lanes and
the retained proof surfaces below. It does not add a fifth remote-builder lane
or rerun paid AWS jobs.

## Operational Entry Points

The merged repo now has first-class operator entry points:

- AWS Spot: `adl/tools/run_aws_spot_remote_validation_lane.sh`
- CodeBuild: `adl/tools/run_aws_codefriend_build_lane.sh`
- CodeBuild setup: `adl/tools/setup_aws_codefriend_build_resources.sh`
- Builder image setup/import: `adl/tools/setup_adl_builder_image.sh` and
  `adl/tools/import_adl_builder_image_from_s3_to_ecr.sh`
- Nessus remote lane: `adl/tools/run_nessus_remote_validation.sh`
- Shared benchmark: `adl/tools/run_build_platform_benchmark.sh`
- Cache/linker helper: `adl/tools/rust_cache_env.sh`
- Platform routing: `adl/tools/validation_manager.py`

Operator docs:

- `docs/tooling/AWS_SPOT_REMOTE_VALIDATION_LANE.md`
- `docs/tooling/AWS_CODEFRIEND_BUILD_LANE.md`
- `docs/tooling/ADL_BUILDER_IMAGE.md`
- `docs/tooling/BUILD_PLATFORM_BENCHMARKS.md`

## Retained Proof Surfaces

- Spot proof: `docs/milestones/v0.91.7/review/build_throughput/AWS_SPOT_REMOTE_VALIDATION_LANE_4837.md`
- CodeBuild proof: `docs/milestones/v0.91.7/review/build_throughput/AWS_CODEFRIEND_BUILD_LANE_4838.md`
- Builder image proof: `docs/milestones/v0.91.7/review/build_throughput/ADL_BUILDER_IMAGE_4879.md`
- CodeBuild xlarge native S3 cache proof:
  `docs/milestones/v0.91.7/review/build_throughput/codebuild-xlarge-native-sccache-s3-repeat-20260704.md`
- Retained Spot hot-cache summary:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json`

## Current Platform Results

These are retained WP-06 benchmark rows for the shared workload in
`adl/tools/run_build_platform_benchmark.sh`.

| Platform | Current proof posture | Build | Test | Total | Operational note |
| --- | --- | ---: | ---: | ---: | --- |
| AWS Spot EC2 | fixed builder image plus retained warm EBS cache | 24s | 49s | 73s | Fastest retained AWS/EC2 row; AWS summary recorded cache volume attached and cleanup terminated. |
| CodeBuild XLARGE | fixed builder image plus stable local target cache and S3 `sccache` | 43-45s | 77-79s | 120-124s | Two repeated live runs completed without manual intervention. |
| Nessus | fixed builder image plus warm target/cache after disk cleanup | 34.08s | 0.39s | 34.476s | Operator host, no cloud compute cost; not a fresh cold-cache result. |
| Wuji | no valid image-backed row yet | not_claimed | not_claimed | not_claimed | Wuji is ARM64 and needs an arm64 or multi-arch builder image before parity can be claimed. |

Historical non-image local rows remain useful diagnostics, but current
remote-builder planning should use the image-backed rows above.

## Routing Truth

`adl/tools/validation_manager.py` now exposes platform candidates for:

- `nessus`, with `remote_target_sccache_warm`
- `aws_spot`, with `warm_ebs_cache:/mnt/adl-cache`
- `codebuild`, with `stable_local_target_cache_plus_s3_sccache`
- `wuji`, rejected until an arm64 builder image exists

The validation manager deliberately dry-runs paid AWS launch surfaces. Live
Spot and CodeBuild execution still requires explicit operator launch through
the lane wrappers or manual GitHub workflow dispatch.

## Cost And Safety Boundaries

- AWS work defaults to the Agent Logic business profile `agent-logic-admin`.
- Spot uses the retained EBS cache volume
  `adl-aws-remote-validation-cache-volume`; that volume has a standing storage
  cost while retained.
- Spot live runs serialize around the retained EBS cache volume and fail closed
  when the volume is already in use.
- CodeBuild uses the `adl-codefriend-build` project, the fixed builder image,
  stable `/codebuild/adl-source` and `/codebuild/adl-target` paths, CodeBuild
  local target cache, and S3 `sccache`.
- GitHub Actions paths are manual `workflow_dispatch` only; they do not run on
  every PR or push.

## Result Truth

`#4679` is satisfied as an umbrella proof by the merged child lanes. The
repository now has operational remote-builder paths for Spot, CodeBuild, and
Nessus, plus truthful routing that rejects Wuji image parity until an ARM64
builder image exists.

No fresh live AWS job was launched for this reconciliation issue. The retained
proof packets above are the authority for live timings, cache posture, cleanup,
and cost caveats.
