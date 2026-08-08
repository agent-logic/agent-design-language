# Structured Review Prompt

Template: 1.0.0

Issue: 22

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/docker/adl-builder/Dockerfile
adl/tools/run_aws_spot_builder_image_validation.sh
adl/tools/test_adl_builder_image.sh
adl/tools/test_run_aws_spot_builder_image_validation.sh

## Prompts

- Is Ruby version and source digest provenance explicit and verified?
- Can a missing Ruby runtime reach the requested validation command?
- Are all existing builder checks preserved?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The actual image build and digest publication remain a separate operational action; this issue proves immutable pinning and preflight behavior.

## Review Result

Revision: Some("git-blake3:8db86f760f48630a9739de4cfb2094e232be1b0e:16a75e84e9370fed6fab0a1c032e8448a9e3864e49b228bfbb4173bfb6a0a432")

Reviewer: Some("subagent-Copernicus")

Result: pass
