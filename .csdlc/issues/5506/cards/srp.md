# Structured Review Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/check_coverage_impact.sh
adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Can auth-only routing skip required ADL tests for a mixed change?
- Does the Runtime v3 expression select the intended tests?
- Did any runtime source enter the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The shell routing contract uses fake cargo; actual llvm-cov interoperability remains hosted CI proof.

## Review Result

Revision: Some("git-blake3:1f30a6046adc24029da4bbd5c3bc359464f35001:da9e6998ebc1aedfbb800c694f51a7eba6d26307d5553fcc53512e20f5304dd5")

Reviewer: Some("subagent:019f74a4-d2d6-7e51-b69d-e92676e69394")

Result: pass
