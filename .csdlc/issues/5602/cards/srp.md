# Structured Review Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

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

- The hosted instrumented workspace remains the final integration proof for the complete llvm-cov renderer and profile set.

## Review Result

Revision: Some("git-blake3:72656435797693a0aeca1ad9136474626abae6e1:dd473ba812bdcd41213f7d44270894bb323137c3b729cc2c548ebe5322b8e9a3")

Reviewer: Some("codex-task:019f81ab-eadd-7430-be8a-b92f560b7f41")

Result: pass
