# Structured Review Prompt

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows/ci.yaml
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/ci_path_policy.sh
adl/tools/verify_coverage_producer_results.sh
adl/tools/test_verify_coverage_producer_results.sh
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
.csdlc/issues/13
.csdlc/prepared/issues/13

## Prompts

- Can an unselected coverage producer still acquire a runner or install tools?
- Do all selected and skipped producer combinations terminate the required aggregate correctly?
- Does Runtime-only routing keep both workspace producers skipped?
- Does full coverage retain Runtime plus both workspace shards?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
