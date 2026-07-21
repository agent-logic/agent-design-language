# Structured Review Prompt

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/5615
.csdlc/issues/5615
.csdlc/prepared/issues/5615
.github/workflows/ci.yaml
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/ci_path_policy.sh
adl/tools/run_cargo_validation.sh
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_run_cargo_validation.sh
adl/tools/test_select_validation_lanes.sh

## Prompts

- Can a C-SDLC v2 source/test diff report green if its standalone job is absent or skipped?
- Can metadata-only or C-SDLC-only changes launch ADL workspace or Runtime coverage?
- Do mixed diffs retain every stronger pre-existing proof requirement?
- Does the wrapper fail closed without a declared or writable external root?
- Are stable aggregate names and semantics unchanged?

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
