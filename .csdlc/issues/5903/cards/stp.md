# Structured Task Prompt

Template: 1.0.0

Issue: 5903

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the Sprint 4 execution-readiness packet and generated lifecycle projections.

## Deliverables

- Path-only SPP ownership projections
- Claim-free Sprint 4 operator packet
- Ten passing typed doctor reports
- Focused readiness validation

## Acceptance

1. All nine child SPP affected_areas collections contain only repository-relative owned paths
2. Umbrella #5857 owns its Sprint Execution Packet and evidence paths
3. Typed doctor passes #5857 and all nine children with ready=true
4. The prompt and execution packet use typed branch/worktree binding and no retired claim/reacquire route
5. Live prerequisites #5817, #5818, #5819, and #5801 are closed
6. No Sprint 4 product implementation or child binding occurs
7. Independent exact-head review has no unresolved actionable findings

## Dependencies

- Issue #5901 and canonical PR #4 merged
- Issues #5817, #5818, #5819, and #5801 closed
- Approved Sprint 4 child designs and diagrams

## Inputs

- AGENTS.md
- .adl/docs/TBD/V092_SPRINT_5857_BIRTHDAY_CORE_SESSION_PROMPT.md
- .csdlc/prepared/issues/5857/sprint-execution-packet.md
- .csdlc/prepared/issues/5857/sprint-execution-packet.yaml

## Non Goals

- Sprint 4 product implementation
- Closing umbrella #5857
- Changing the approved birthday architecture
- Binding a child worktree
