# Structured Task Prompt

Template: 1.0.0

Issue: 22

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue 22 only: builder Ruby availability and fail-closed preflight.

## Deliverables

- Pinned Ruby runtime in adl/docker/adl-builder/Dockerfile
- Ruby provenance in /usr/local/share/adl-builder-toolchain.txt
- Ruby preflight and repository validator smoke
- Focused positive and missing-Ruby regression coverage

## Acceptance

1. The builder image pins Ruby to an explicit version and verifies the official source archive SHA-256
2. Ruby version and provenance are recorded in the immutable toolchain manifest
3. Builder preflight executes ruby --version, a minimal Ruby expression, and one repository Ruby validator smoke before the requested validation command
4. A missing or unusable Ruby runtime fails builder preflight and prevents the requested validation command
5. Existing Rust, nextest, sccache, LLD, AWS CLI, architecture, cache, and immutable image checks remain intact
6. Focused shell contracts, diff hygiene, independent review, and required GitHub checks pass

## Dependencies

- WP-03 AWS Spot builder path is present on current main

## Inputs

- adl/docker/adl-builder/Dockerfile
- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_adl_builder_image.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh

## Non Goals

- No dynamic Ruby installation on Spot hosts
- No AWS execution or image publication in this issue
- No changes to unrelated CI, lifecycle tooling, or other sprints
