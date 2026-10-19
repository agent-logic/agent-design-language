# v0.93.2 Decision Log

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Milestone opening authorized on 2026-10-19; [WP-01 #1296](https://github.com/agent-logic/agent-design-language/issues/1296) is in progress in Planning #12. See [opening record](OPENING_1296.md). Downstream execution, provider spend, deployment and release are not authorized by this opening.

## Purpose

Record approved routing and unresolved activation choices.

## How To Use

Scheduling decisions do not prove execution or accept architecture proposals.

## Decision Log

| Decision | Status | Consequence |
|---|---|---|
| Split original v0.93 under #922 | Operator approved | v0.93.1 qualifies Beta 1 as ready to deploy; v0.93.2 owns remaining platform scope. |
| Move Runtime v4 to v0.93.2 | Operator approved | No blanket Runtime v4 prerequisite for Beta 1; real launch blockers stay in v0.93.1. |
| Put complete template scope in v0.93.1 | Operator approved | Do not duplicate CT implementation here. |
| Preserve full platform scope | Planned allocation | 40 inherited results and 13 milestone-specific gates. |
| Four delivery sprints | Proposed | Capacity and named owners unresolved. |

## Open Questions

Resolve named owners, capacity, accepted predecessor residuals, exact version lockset, registry/access availability, successor version and sidecar dispositions before their affected gates. RV-01 must still resolve architecture review findings.

## Exit Criteria

Each unresolved decision has an accountable owner and gate; no inferred approval substitutes for evidence.

| Integrate GHB Gate A and Gate B | Operator approved 2026-10-10 | Adds GHB-SPRINT and GHB-WP00..12; scope approval does not authorize execution, spend, deployment, or issue creation. |
| Preserve v0.93.1 truth boundary | Live evidence refresh | v0.93.1 established ready-to-deploy qualification; public CodeFriend launch remains routed through existing CodeFriend #92. |
