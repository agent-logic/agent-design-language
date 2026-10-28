# Structured Task Prompt

Template: 1.0.0

Issue: 170

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-09 within its exact owned paths and authority boundary.

## Deliverables

- Git and process traits, production adapters, fakes, V3-01 command-allowance enforcement, credential resolver, cancellation integration, and redaction tests.

## Acceptance

1. Every Git/process invocation is argv-based and typed.
2. Exit status, stdout, stderr, timeout, cancellation, and truncation remain distinguishable.
3. Credentials exist only in the child/provider process scope that needs them.
4. Branch-name observation alone never authorizes lifecycle work.

## Dependencies

- V3-01: issue #161
- V3-04: issue #165
- V3-05: issue #166

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-09
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Shell scripts as internal control flow, arbitrary command evaluation, GitHub API behavior, lifecycle decisions, or secret persistence.
