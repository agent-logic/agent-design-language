# v0.93.1 Design

## Metadata

Planning template family 1.1.0; authoring issue #922. Scope split approved; Sprint 1 preparation opened by the operator on 2026-09-25. Split execution and release are not authorized. Named execution owners and resource limits remain to be assigned.

## Purpose

Separate release ownership without rebuilding the Runtime as a prerequisite for CodeFriend launch.

## Problem Statement

The original plan conflated Runtime v4 and Beta 1 co-scheduling with a technical dependency. Split qualification also must not require unimplemented launch features.

## Goals

Products that independently build, test, install and release without sibling checkouts or implicit Runtime/C-SDLC source coupling; explicit producer contracts; reliable review evidence; complete branded artifact catalog; proven launch recovery.

## Non-Goals

Runtime v4, expanded governance/security program, citizen reproduction/migration and Python reduction belong to v0.93.2. Existing privacy, authorization and isolation obligations remain launch requirements.

## Scope

WP-01/RD-01–RD-13; CF-01–CF-07; CT-01–CT-10; INTEGRATE/QUALIFY; TAIL-01–TAIL-10.

## Requirements

Existing #1148 (citation-grounded correctness) and #1149 (uncertain second-run recovery) are additional launch prerequisites. CF-05 reuses existing #1150 for independent qualification after scope alignment; do not seed a duplicate CF-05 issue. The 43 core work packages plus #1148/#1149 and operator-routed #875/#1145 map to 47 planned identities. Fourteen Sprint 1 core issues and CF-05 #1150 already exist; at most 28 core issues remain to create after split acceptance. The Sprint 1 umbrella and separately scoped supporting issues are outside this denominator. Preserve their identities and original #915 failures; do not create duplicate tasks or replay an uncertain request to manufacture a result. Both website modes and CLI remain in the qualification denominator; supported diagrams include all four architecture views plus scenarios.

## Proposed Design

RD-11 accepts frozen baseline portability and preserved known limitations. CF-01 then maps evidence and missing launch behavior. CF-02 pins compatible CodeFriend/Runtime contracts, independently installs the product and proves the required journey. Template content contracts and immutable style assets feed a single preview/export path. INTEGRATE accepts the pinned product/template/evidence lockset after CF-04 and CT-05. CF-05 reuses #1150 to independently qualify that candidate after #1148/#1149. QUALIFY audits the accepted #1150 and live CF-07 evidence without another full qualification campaign.

## Risks And Mitigations

Unknown request outcomes retain original identity and reservation; use explicit reconciliation, never blind replay. Export success does not prove review correctness. Separate private evidence from safe public manifests. Preserve rollback before source retirement.

## Alternatives Considered

Requiring all Runtime v4 was rejected because no launch dependency was demonstrated. Skipping templates or reducing the qualification denominator is not authorized.

## Validation Plan

Prove clean-checkout portability first, then installed product/template behavior and independent launch qualification. Test refusals, cancellation, interruption, rollback and deletion. Planning checks alone cannot establish product readiness.

## Exit Criteria

Exact source/artifact versions, execution denominators, uncertainty and authorization are retained for the accepted launch.

## Interfaces And Contracts

Each producer exposes a small versioned contract and distributable artifact. CodeFriend pins compatible Runtime interfaces, template schemas/assets and renderer versions; the website consumes the qualified product entry point. RD-11 accepts independent checkout portability and the baseline lockset. INTEGRATE pins the launch candidate before CF-05/#1150 qualification. No sibling source checkout or implicit Runtime/C-SDLC coupling is permitted.
