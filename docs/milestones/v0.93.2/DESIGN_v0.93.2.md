# v0.93.2 Design

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Planned and unopened. The operator approved the scope split; this package does not approve execution, provider spend, publication or release.

## Purpose

Preserve the accepted split and product contracts while evolving Runtime and governance.

## Problem Statement

The original combined plan coupled Beta 1 launch to an unrelated full Runtime upgrade. The approved split removes that scheduling dependency without reducing platform scope.

## Goals

Complete recoverable plugins, enforce governance/security contracts and prove citizen migration and reproduction.

## Non-Goals

No repository re-extraction, Beta 1 launch redo, template foundation duplication, public private-source release or unbounded live operation.

## Scope

53 planned candidates: 40 inherited implementation/demo tasks plus 13 version-specific opening, integration, qualification and release-tail tasks. Existing #875 and #671 remain separately gated sidecars and are not counted in 53.

## Requirements

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated. Runtime v4 must retain compatible CodeFriend review, artifact, deployment and recovery contracts; incompatible changes require explicit consumer migration before upgrade.

## Proposed Design

RV design acceptance precedes implementation; native lifecycle and state migration precede process/WASM adapters. Governance builds on qualified Runtime; enterprise security consumes accepted actor/authority contracts; citizen continuity integrates both.

## Interfaces And Contracts

Pin repository commits, installed artifacts and contract versions. Public ADL stays free of private build dependencies. Private Runtime consumes public contracts; security owns its private enforcement producer. Upgrade and rollback preserve user data and launched consumer behavior.

## Validation Plan

Use component negatives, authentic interruption/recovery journeys and installed consumer proof. INTEGRATE assembles exact versions; QUALIFY independently runs the admitted denominator, including CodeFriend compatibility/rollback.

## Exit Criteria

All mandatory outcomes have accepted evidence or explicit operator disposition; review and release authorization remain distinct.
