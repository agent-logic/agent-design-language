# Structured Output Record

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Wired typed resident tool authority and actual long-lived Runtime provider outputs through UTS-to-ACC compilation, Freedom Gate evaluation, injected governed adapter dispatch, and lineage-bound terminal receipts.

## Artifacts

- commit:0bc8df3ff
- adl/src/resident_tool_execution.rs
- adl/src/long_lived_agent.rs
- adl-runtime/src/resident_agent.rs
- resident_tool_execution: 3 passed
- governed_executor: 25 passed
- resident_agent: 4 passed
- actual Runtime receipt integration: 1 passed
- cargo check: passed
- git diff --check: passed

## Execution

- Added typed resident authority id/ref/digest and sorted allowlist validation.
- Added a Runtime-owned resident proposal service using the real UTS-to-ACC compiler and Freedom Gate.
- Refactored governed execution to accept an injected adapter while keeping fixture dispatch test-only.
- Integrated provider StepOutput processing into long-lived Runtime cycles and fail-closed denial receipts.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
