# Build Platform Benchmarks

`adl/tools/run_build_platform_benchmark.sh` runs the same small build/test
workload on each build platform so WP-06 timing comparisons use one shape.

The workload is:

```text
cargo build --manifest-path <repo>/adl/Cargo.toml --locked --bin adl-pr-doctor
cargo test --manifest-path <repo>/adl/Cargo.toml --locked --lib provider_communication -- --nocapture
```

The helper writes a JSON summary and logs under the selected artifact directory,
then prints one summary line:

```text
ADL_BUILD_PLATFORM_BENCHMARK platform=<name> build_seconds=<n> test_seconds=<n> total_seconds=<n> status=passed
```

## Local And Remote Platforms

Wuji:

```bash
bash adl/tools/run_build_platform_benchmark.sh \
  --platform wuji \
  --cache-posture linked_target_cache_warm \
  --out .adl/tmp/build-platform-benchmark/wuji/summary.json \
  --artifact-dir .adl/tmp/build-platform-benchmark/wuji
```

Nessus:

```bash
bash adl/tools/run_nessus_remote_validation.sh \
  --run-id <run-id> \
  --git-ref <branch-or-ref> \
  --local-artifact-dir .adl/tmp/build-platform-benchmark/nessus \
  --command 'if ! command -v clang >/dev/null 2>&1; then sudo apt-get update && sudo apt-get install -y clang; fi; export CC=clang; bash adl/tools/run_build_platform_benchmark.sh --platform nessus --cache-posture remote_target_sccache_warm --out .adl/tmp/build-platform-benchmark/nessus/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/nessus'
```

AWS Spot:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --check-account \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture fixed_builder_image_warm_ebs_cache --out .adl/tmp/build-platform-benchmark/aws-spot-ebs/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/aws-spot-ebs' \
  --git-ref <branch-or-ref> \
  --out .adl/tmp/aws-spot-remote-validation/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-spot-remote-validation/<run-id>/artifacts \
  --instance-type m7a.2xlarge \
  --json
```

CodeBuild:

```bash
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --wait \
  --project-name adl-codefriend-build \
  --source-version <branch-or-ref> \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_build_platform_benchmark.sh --platform codebuild --cache-posture fixed_builder_image_stable_local_target_cache_s3_sccache --out .adl/tmp/build-platform-benchmark/codebuild-xlarge/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/codebuild-xlarge' \
  --out .adl/tmp/aws-codefriend-build/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build/<run-id>
```

## Cache Postures

- `wuji`: linked local target cache.
- `nessus`: remote target cache plus `sccache`; use `CC=clang` for the current
  Linux AWS LC build surface.
- `aws_spot`: retained warm EBS cache mounted at `/mnt/adl-cache`; this volume
  has a standing AWS storage cost and the run summary must show
  `cache_volume.attachment_state: "attached"`.
- `codebuild`: fixed ECR builder image, stable `/codebuild/adl-source` and
  `/codebuild/adl-target` paths, CodeBuild local target cache, and S3
  `sccache`.
- `wuji`: ARM64; do not claim image-backed parity until an arm64 or multi-arch
  builder image is published and proven.

## Current Comparison Snapshot

These are WP-06 working measurements, not universal performance claims:

| Platform | Cache posture | Build | Test | Total | Notes |
| --- | --- | ---: | ---: | ---: | --- |
| AWS Spot | `fixed_builder_image_warm_ebs_cache` | 24s | 49s | 73s | Fastest retained AWS/EC2 row; run summary recorded retained EBS cache attached and instance cleanup terminated. |
| CodeBuild XLARGE | `fixed_builder_image_stable_local_target_cache_s3_sccache` | 43-45s | 77-79s | 120-124s | Two repeated live runs completed with stable local target cache and 100% Rust `sccache` hit rate. |
| Nessus | `fixed_builder_image_warm_target_cache` | 34.08s | 0.39s | 34.476s | Millisecond-timed warm image-backed row after disk cleanup; no cloud compute cost. |
| Wuji | `linked_target_cache_warm_arm64` | not_claimed | not_claimed | not_claimed | ARM64 host needs an arm64 or multi-arch builder image before image-backed parity can be claimed. |
| AWS Spot baseline | `no_explicit_ebs_cache` | 221s | 190s | 411s | Historical baseline run completed and cleaned up; not accepted as warm-EBS proof. |

Refresh this table only from retained summaries or logs. Do not infer a cache
posture from command labels alone.
