# Structured Task Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #415 builder diagnostics prerequisite only.

## Deliverables

- Individually labeled builder preflight checks.
- Atomic redacted retained builder-toolchain log.
- Exact failed-check diagnostic with exit status.
- Simulated missing-tool regression and runner retention contract.

## Acceptance

1. AC-1: Every builder preflight check has a stable label and records its exit status.
2. AC-2: Early failure retains exact redacted stdout, stderr, and toolchain diagnostics in portable artifacts while removing raw temporary captures.
3. AC-3: The owner summary identifies the precise failed check and missing executable rather than only stage plus exit 127.
4. AC-4: Existing exact-owner instance, IAM, security-group, and zero-resource cleanup semantics remain unchanged.
5. AC-5: Focused tests simulate a missing executable, dynamically prove normal-path runner emission and raw-capture removal, and prove missing diagnostic emission cannot block summary or cleanup compatibility.
6. AC-6: Fresh exact-head review, required CI, merge, typed finish, terminal cache, and ancestry complete before #268 retries.

## Dependencies

- Blocks #268 paid retry.
- Consumes the completed #268 failure observation without claiming which executable was previously missing.

## Inputs

- GitHub issue #415
- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh
- tools/aws_remote_validation/scripts/remote_validation_runner.sh

## Non Goals

- Running or retrying #268.
- Executing #269.
- Changing provider, instance, image, or cleanup policy.
- Speculating about the executable missing in the completed paid attempt.
