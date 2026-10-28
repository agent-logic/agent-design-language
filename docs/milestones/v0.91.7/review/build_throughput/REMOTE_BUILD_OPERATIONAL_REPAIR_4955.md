# Remote Build Operational Repair `#4955`

Status: `implemented_with_live_nessus_probe`
Date: 2026-07-06

## Scope

Issue `#4955` repairs the remote-build path before WP-08 starts. It consumes the
local #4911 retained problem note that showed:

- Nessus raw-host PR-fast validation can fail before proof when
  `cargo-nextest` is missing.
- Nessus builder-image mode could try to pull a local-only tag.
- AWS Spot account/cache posture should stay explicit before paid runs.

## Changes

- `adl/docker/adl-builder/Dockerfile` now installs and verifies
  `cargo-nextest` from the upstream prebuilt release tarball.
- `adl/tools/run_nessus_remote_validation.sh` now supports
  `--builder-pull-policy missing|always|never`.
- Nessus summaries record builder pull policy, local image presence, and whether
  a pull was attempted.
- Raw-host Nessus commands that require nextest fail closed when
  `cargo nextest --version` is unavailable and no builder image is configured.
- Spot and Nessus docs now distinguish pullable registry images, local/preloaded
  image mode, warm EBS cache, and fixed-builder-image proof.

## Local Contract Proof

Passed:

```sh
bash adl/tools/test_adl_builder_image.sh
bash adl/tools/test_run_nessus_remote_validation.sh
bash adl/tools/test_run_validation_manager_nessus_lane.sh
bash adl/tools/test_run_aws_spot_remote_validation_lane.sh
git diff --check
```

These tests cover the Dockerfile nextest requirement, Nessus preloaded-image
mode without pull, raw-host nextest fail-closed behavior, validation-manager
Nessus routing, and AWS Spot wrapper account/cache command rendering.

## Live Nessus Probe

The corrected `linux/amd64` image was built once locally, tagged
`adl-builder:v0.91.7-fixed`, streamed to Nessus with `docker load`, and then
used through the repo wrapper with pull disabled.

Command shape:

```sh
bash adl/tools/run_nessus_remote_validation.sh \
  --builder-image adl-builder:v0.91.7-fixed \
  --builder-pull-policy never \
  --git-ref origin/main \
  --command 'cargo nextest --version' \
  --local-artifact-dir .adl/local-artifacts/remote-build/4955-nessus-nextest-probe-fixed
```

Result:

- status: `passed`
- elapsed seconds: `6`
- resolved builder runtime: `docker`
- builder image local present: `true`
- builder pull attempted: `false`
- command: `cargo nextest --version`

This proves the Nessus image-backed nextest path is operational after the image
is present on the host. It is intentionally a toolchain probe, not a full
PR-fast validation run.

## AWS Spot Dry-Run Proof

No EC2 resources were launched for this issue. The no-cost account/cache
boundary was verified with:

```sh
AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --check-account \
  --git-ref codex/4955-remote-build-operational-builder-image-paths \
  --command 'cargo nextest --version' \
  --out .adl/local-artifacts/remote-build/4955-aws-spot-dry-run-summary.json \
  --artifact-dir .adl/local-artifacts/remote-build/4955-aws-spot-dry-run \
  --instance-type m7a.2xlarge \
  --print-command \
  --json
```

Result:

- Agent Logic profile account check: `PASS`
- retained EBS cache volume: `adl-aws-remote-validation-cache-volume`
- retained cache mount: `/mnt/adl-cache`
- SSH tail posture: enabled by wrapper defaults
- EC2 launch: `not_run`; dry-run only

## Operational Notes

- The preloaded Nessus tag is local host state. Use a pullable registry URI for
  repeatable multi-host runs, or set `ADL_NESSUS_BUILDER_PULL_POLICY=never`
  when intentionally relying on a preloaded local tag.
- Do not claim a Spot run used the fixed builder image merely because the warm
  EBS cache was attached. The remote command or retained summary must prove
  image execution.
- The #4911 raw-host failures are remote-lane setup failures, not evidence that
  #4911 code behavior failed.

## Non-Claims

- This issue does not start WP-08.
- This issue does not publish a new ECR image tag.
- This issue does not run a paid AWS Spot validation.
- This issue does not resolve the unrelated `pr watch closeout_needed`
  classifier tracked by `#4950`.
