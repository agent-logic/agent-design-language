# Structured Task Prompt

Template: 1.0.0

Issue: 5906

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Retain mergedAt evidence and select only the explicitly requested unique latest merged closing PR.

## Deliverables

- Merged-at closing PR identity
- Unique-latest precedence validation
- Focused rejection and compatibility tests
- Validated terminal reconciliation for 5818 and 5861

## Acceptance

1. Single merged candidate behavior remains compatible
2. Unique latest merged candidate is accepted only when explicitly requested
3. Wrong, missing-timestamp, and tied-timestamp candidates fail closed
4. Routine finish and review gates are unchanged
5. Issues 5818 and 5861 receive validated terminal truth

## Dependencies

- Issue 5905 historical finish compatibility path

## Inputs

- csdlc-v2/src/github.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate_finish.rs

## Non Goals

- Selecting by PR number
- Rewriting merged PR bodies
- Broad closeout redesign
- Product implementation
