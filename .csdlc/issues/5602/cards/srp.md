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

- The hosted instrumented workspace remains the integration proof for the complete llvm-cov renderer and profile set.

## Review Result

Revision: Some("git-blake3:e85087c8e14082990225d44198457e3d1bed7678:464b71a45afbb27e672c88bc61025b4bf381dcfd2bb26bd558dcd4578f8255ea")

Reviewer: Some("codex-task:019f81ab-eadd-7430-be8a-b92f560b7f41")

Result: pass
