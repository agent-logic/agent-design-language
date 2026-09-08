# Structured Task Prompt

Template: 1.0.0

Issue: 523

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Refresh and validate exactly one successor planning package; do not execute or open its planned work.

## Deliverables

- Updated v0.92.2 milestone planning package
- Updated ADL feature routing where required
- Retained denominator and deferred-routing validation evidence

## Acceptance

1. AC-1: The v0.92.2 scope and CodeFriend Beta 1 exit bar are explicit and internally consistent
2. AC-2: Every relevant v0.92.1 result or residual has a delivered, deferred, existing-issue, or planned disposition
3. AC-3: Deferred work remains deferred unless explicitly promoted
4. AC-4: The package identifies v0.95 integration intent without executing it
5. AC-5: No successor execution issues are created
6. AC-6: Focused planning validators pass and an independent review finds no unresolved actionable findings

## Dependencies

- Reviewed merge for #522 / TAIL-06
- Current canonical v0.92.1 milestone truth
- Existing v0.92.2 planning package

## Inputs

- docs/milestones/v0.92.1/**
- docs/milestones/v0.92.2/**
- docs/planning/ADL_FEATURE_LIST.md
- .adl/docs/TBD/**
- agent-logic/agent-design-language#523

## Non Goals

- Creating v0.92.2 issues
- Implementing CodeFriend
- Changing v0.92.1 product code
- Performing release ceremony
