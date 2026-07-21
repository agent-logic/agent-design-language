# Structured Intent Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make hosted authoritative coverage collect profiles per partition and render only the intended combined reports.

## Required Outcome

Hosted coverage no longer fails because a redundant partition-local llvm-cov report crashes, while test scope and coverage gates remain unchanged.

## Scope

- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh

## Authority

- Issue #5602 owns only authoritative coverage orchestration
- Issue #5336 and PR #5599 own Runtime v3 planning content
- No AWS execution is authorized

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and ADL owner binaries only
- Never use raw gh or AWS
- Never weaken or bypass the coverage gate
