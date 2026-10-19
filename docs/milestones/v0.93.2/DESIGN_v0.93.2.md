# v0.93.2 Design

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## Problem Statement

The previous allocation coupled a nearly absent Beta 1 improvement backlog to Runtime v4 and GHB. The operator approved a smaller product milestone.

## Goals

CodeFriend Beta 1 launch and enterprise security

## Non-Goals

Python reduction, affect and humour remain deferred outside both allocations. Existing historical evidence is retained. No repository split is repeated; no sibling checkout is required to validate public ADL.

## Scope

31 planned identities across five dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 21 retained identities plus five explicit Beta 1 product packages and four existing C-SDLC improvements plus independent CI #205; 46 platform identities move to v0.93.3.

## Requirements

[Execution graph](EXECUTION_PLAN_v0.93.2.json), [specifications](WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml), and [issue wave](WP_ISSUE_WAVE_v0.93.2.yaml) carry exact acceptance and dependencies.

## Proposed Design

CF-B1-01 measures remaining gaps first. Product and security delivery may run in parallel after their contract dependencies are accepted. Integration and independent qualification precede CF-92 launch.

## Interfaces And Contracts

Supported Runtime supplies existing enforcement primitives. Enterprise security owns policy, authorization, key lifecycle, incident evidence and data isolation. CodeFriend consumes versioned interfaces. Any missing Beta 1 primitive must be scoped explicitly; the entire future platform cannot become an implicit prerequisite.

## Validation Plan

Focused planning checks prove allocation only. Each product package requires its named installed behavior and adverse cases. Reuse prior proof only when the exact consumer/version/configuration remains applicable.

## Exit Criteria

All consumer contracts and required product scenarios accepted, with exact lockset and no unsupported platform claims.
