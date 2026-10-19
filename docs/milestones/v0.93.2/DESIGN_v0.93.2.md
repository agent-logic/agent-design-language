# v0.93.2 Design

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Milestone opening authorized on 2026-10-19; [WP-01 #1296](https://github.com/agent-logic/agent-design-language/issues/1296) is in progress in Planning #12. See [opening record](OPENING_1296.md). Downstream execution, provider spend, deployment and release are not authorized by this opening.

## Purpose

Preserve the accepted split and product contracts while evolving Runtime and governance.

## Problem Statement

The original combined plan coupled Beta 1 launch to an unrelated full Runtime upgrade. The approved split removes that scheduling dependency without reducing platform scope.

## Goals

Complete recoverable plugins, enforce governance/security contracts and prove citizen migration and reproduction.

## Non-Goals

No repository re-extraction, Beta 1 launch redo, template foundation duplication, public private-source release or unbounded live operation.

## Scope

67 planned identities: the retained 52 platform candidates, existing CodeFriend #92, plus `GHB-SPRINT` and `GHB-WP00` through `GHB-WP12`. Existing #875 is a separately gated v0.93.1 sidecar and is not counted in 52. Podcast #671 is completed in v0.92.2 (PR #1174).

## Requirements

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated. Runtime v4 must retain compatible CodeFriend review, artifact, deployment and recovery contracts; incompatible changes require explicit consumer migration before upgrade.

## Proposed Design

RV design acceptance precedes implementation; native lifecycle and state migration precede process/WASM adapters. Governance builds on qualified Runtime; enterprise security consumes accepted actor/authority contracts; citizen continuity integrates both.

## Interfaces And Contracts

Pin repository commits, installed artifacts and contract versions. Public ADL stays free of private build dependencies. Private Runtime consumes public contracts; security owns its private enforcement producer. Upgrade and rollback preserve user data and ready-to-deploy consumer behavior.

## Validation Plan

Use component negatives, authentic interruption/recovery journeys and installed consumer proof. INTEGRATE assembles exact versions; QUALIFY independently runs the admitted denominator, including CodeFriend compatibility/rollback.

## Exit Criteria

All mandatory outcomes have accepted evidence or explicit operator disposition; review and release authorization remain distinct.

## v0.93.2 scope integration

The milestone retains the existing platform tracks and adds CodeFriend plus the approved full GHB program. CodeFriend public launch is routed through existing CodeFriend #92 and still requires exact deployment authorization. GHB requires Gate A, Gate B, and WP12 before milestone qualification; planning approval does not authorize experiments, provider spend, deployment, or issue creation. Runtime-v4 activation/generation and citizen-continuity/CM mechanisms are shared dependencies for GHB-WP07/WP08, not duplicate engines, schedulers, or transaction owners. ADL #1226 remains a deferred assessment.
