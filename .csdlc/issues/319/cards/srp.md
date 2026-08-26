# Structured Review Prompt

Template: 1.0.0

Issue: 319

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/.config/nextest.toml
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh

## Prompts

- Does the gate prove reviewed-green merge ancestry without depending on typed finish or cleanup?
- Can stale, dirty, duplicate, partial, or conflicting release state pass?
- Are release claims and #268/v0.93 non-claims exact and truthful?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The replacement hosted coverage run must still demonstrate the bounded profile on GitHub before merge readiness.
- The post-merge clean-main ceremony receipt remains deferred until the reviewed PR merges; tag and release mutation remain unauthorized.

## Review Result

Revision: Some("git-blake3:96f1dd343d0954a133529924b3c9978282d84425:b4fcca1eb7de1dbd37671244e864fd095f70537f10d925ec911dbd4b3bd1111d")

Reviewer: Some("fresh-session:0568ed92-192f-4e8c-9553-928e0d85ef51")

Result: pass
