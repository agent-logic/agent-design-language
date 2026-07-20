# Structured Review Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
adl/tools/test_ci_runtime_contracts.sh
.csdlc/issues/5602
.csdlc/prepared/issues/5602

## Prompts

- Does each workspace source cargo llvm-cov show-env before executing its partitioned cargo nextest runs?
- Does each workspace emit exactly one explicit cargo llvm-cov report after all partition profiles are collected?
- Do the focused contracts prove instrumentation export, partition failure propagation, run isolation, and unchanged coverage gates?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The hosted exact-head CI run remains the final integration proof after lifecycle metadata is pushed.

## Review Result

Revision: Some("git-blake3:628028525be21d9e03bb6a216066422c1757e084:92882ea809c8013868e68be36eb19758a0ba588377d7c1d250fa7fbb0ae72e1a")

Reviewer: Some("codex-task:019f81ab-eadd-7430-be8a-b92f560b7f41")

Result: pass
