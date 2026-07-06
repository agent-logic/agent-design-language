# Remote Build How-To

Use this playbook to run ADL validation or repeated builds on Nessus, AWS Spot,
or AWS CodeBuild without rediscovering the setup. Keep runs issue-bound,
explicit, and proof-producing.

## 1. Standard Setup

Run from the issue worktree.

```sh
export ADL_WORKTREE=$(git rev-parse --show-toplevel)
export ADL_BIN=${ADL_BIN:-/Users/daniel/git/agent-design-language/adl/target/debug/adl}
export ADL_AWS_PROFILE=agent-logic-admin
export ADL_ARTIFACT_DIR=.adl/local-artifacts/remote-build
mkdir -p "$ADL_ARTIFACT_DIR"
```

Rules:

- Do not do tracked issue work on root `main`.
- Use `agent-logic-admin` for ADL AWS work.
- Keep `ADL_WORKTREE` pointed at the bound issue worktree. `ADL_BIN` may point
  at the approved repo-owned binary; do not rebuild it unless the issue requires
  a new binary.
- Keep AWS Spot and CodeBuild live runs explicit; both may cost money.
- Use the pre-published ADL builder image. Do not rebuild the image inside each
  build job.
- Put scratch outputs under ignored `.adl/local-artifacts/`.
- Record the platform, cache posture, benchmark line, and proof artifact path.

## 2. Choose A Lane

| Lane | Use when | Cache posture to prove |
| --- | --- | --- |
| Nessus | Fast no-cloud-cost remote validation on operator hardware | persistent remote target/cache; state cold or warm |
| AWS Spot | Fast AWS/EC2 validation | fixed builder image plus retained warm EBS cache |
| CodeBuild XLARGE | Scalable isolated CodeFriend-style repeated builds | fixed ECR image, stable `/codebuild` paths, local target cache, S3 `sccache` |
| Wuji/local | Local ARM work | no image-backed parity until ARM64 or multi-arch image exists |

For scheduler routing without launching paid work:

```sh
bash adl/tools/validation_manager.sh --platform-routing
```

## 3. Nessus

Remote host: `daniel@nessus.local`.

Select the lane first. Use committed refs for remote proof; the remote host
checks out the advertised git ref and cannot see local uncommitted worktree
changes.

```sh
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --base origin/main \
  --head HEAD \
  --remote-git-ref <branch-or-commit> \
  --remote-artifact-dir "$ADL_ARTIFACT_DIR/nessus"
```

Run it:

```sh
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --run \
  --base origin/main \
  --head HEAD \
  --remote-git-ref <branch-or-commit> \
  --remote-artifact-dir "$ADL_ARTIFACT_DIR/nessus"
```

Record:

- remote git ref
- fetched summary/log artifact paths
- cold image-backed or warm target-cache posture
- benchmark line if `run_build_platform_benchmark.sh` ran

## 4. AWS Spot

Dry-run/account check:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --check-account \
  --git-ref <branch-or-commit> \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture fixed_builder_image_warm_ebs_cache --out .adl/local-artifacts/build-platform/aws-spot-summary.json --artifact-dir .adl/local-artifacts/build-platform/aws-spot' \
  --out "$ADL_ARTIFACT_DIR/aws-spot-dry-run-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/aws-spot-dry-run"
```

Live run:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --check-account \
  --git-ref <branch-or-commit> \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture fixed_builder_image_warm_ebs_cache --out .adl/local-artifacts/build-platform/aws-spot-summary.json --artifact-dir .adl/local-artifacts/build-platform/aws-spot' \
  --out "$ADL_ARTIFACT_DIR/aws-spot-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/aws-spot"
```

Record:

- account check passed
- advertised git ref
- builder image tag
- retained EBS cache attached
- benchmark line
- cleanup/termination completed
- redacted logs/artifacts

Do not call a Spot row warm unless the retained EBS cache is attached in the
AWS-side summary.

## 5. CodeBuild / CodeFriend

Run the wrapper dry-run/account check first:

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --check-account \
  --profile agent-logic-admin \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --out "$ADL_ARTIFACT_DIR/codebuild-dry-run-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-dry-run"
```

Create or update the project only after the wrapper account check passes. This
step mutates AWS IAM, S3, and CodeBuild resources.

```sh
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2 \
  --compute-type BUILD_GENERAL1_XLARGE \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-setup"
```

Live run:

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --profile agent-logic-admin \
  --project-name adl-codefriend-build \
  --source-version <branch-or-commit> \
  --region us-west-2 \
  --env 'ADL_CODEFRIEND_BUILD_COMMAND=bash adl/tools/run_build_platform_benchmark.sh --platform codebuild --cache-posture fixed_builder_image_stable_local_target_cache_s3_sccache --out .adl/local-artifacts/build-platform/codebuild-summary.json --artifact-dir .adl/local-artifacts/build-platform/codebuild' \
  --out "$ADL_ARTIFACT_DIR/codebuild-live-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-live" \
  --wait \
  --poll-seconds 15 \
  --timeout-seconds 900
```

Record:

- project `adl-codefriend-build`
- compute type
- builder image tag
- stable `/codebuild/adl-source` and `/codebuild/adl-target` paths
- CodeBuild local target cache and S3 `sccache` posture
- benchmark line
- terminal CodeBuild status
- redacted logs/artifacts

Do not report nested Docker-in-CodeBuild, image-built-inside-job, or S3-only
diagnostic rows as the operational CodeBuild path.

## 6. Shared Benchmark Command

Use this inside any lane when comparing platforms:

```sh
bash adl/tools/run_build_platform_benchmark.sh \
  --platform <platform> \
  --cache-posture <cache-posture> \
  --out "$ADL_ARTIFACT_DIR/<platform>-benchmark-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/<platform>-benchmark"
```

Accepted current comparison rows live in
[Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md).

## 7. Proof Checklist

Every reported remote-build result should include:

- issue/worktree and git ref
- platform and cache posture
- command or wrapper used
- dry-run versus paid live run
- build seconds, test seconds, total seconds, status
- cache proof: warm EBS, stable CodeBuild target cache, S3 `sccache`, or Nessus
  warm target cache
- artifact path under `.adl/local-artifacts/`
- cleanup status for AWS resources
- explicit non-claims for any missing live proof

## 8. Troubleshooting

- Wrong AWS account: rerun the wrapper with `--check-account --profile agent-logic-admin`.
- CodeBuild too slow: verify fixed ECR image, stable `/codebuild` paths,
  local target cache, and S3 `sccache` hit rate.
- Spot too slow: verify retained EBS cache attachment and cleanup status.
- Nessus SSH asks for a password/passphrase: fix the operator SSH key before
  treating the lane as operational.
- `0s` timing looks suspicious: rerun with precise timing before reporting it.
- Wuji image parity is requested: stop until an ARM64 or multi-arch builder
  image exists.

## Related Docs

- [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md)
- [ADL Builder Image](ADL_BUILDER_IMAGE.md)
- [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md)
- [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md)
- [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md)
- [Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md)
