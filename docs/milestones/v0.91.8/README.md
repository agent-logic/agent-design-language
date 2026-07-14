# v0.91.8 ADL Core Rearchitecture

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers
- Status: planned; setup issue `#5335`

## Status

Current status: planned and not yet release-approved.

- Planning: setup package in review
- Execution: not started
- Validation: planned
- Release readiness: not claimed

## Purpose

Replace the monolithic `adl/` product with a small clean-room implementation,
using the method already proven by Runtime v3 and C-SDLC v2. The target is 90%
incumbent deletion, with 80% as the minimum acceptable result.

## Milestone Role

v0.91.8 is a platform bridge between the v0.91.7 runtime/control-plane
rearchitecture and the v0.92 first-birthday product milestone. It owns ADL
language/compiler/portable-engine simplification; it does not redefine v0.92.

Expected outcomes:

- a typed six-primitives language and schema core;
- a pure deterministic compiler to a versioned execution plan;
- a bounded portable engine with narrow provider/tool ports;
- a thin CLI and independently owned adapters;
- normalized parity, reversible cutover, and approval-gated legacy deletion;
- full acceptance and deployment of ADL v2, Runtime v3, and C-SDLC v2;
- Unity Observatory and Adaptive Learning DAG readiness disposition against the deployed stack.

## Dependency Boundary

This milestone consumes:

- Runtime v3 component-kernel, parity, and cutover evidence;
- C-SDLC v2 clean-room, selector, and deletion-gate evidence;
- incumbent `adl/` behavior as characterization evidence only.

Runtime supervision and cognitive services remain owned by Runtime v3.
C-SDLC lifecycle behavior remains owned by C-SDLC v2.

## Scope Summary

In scope:

- language, compiler, engine, contracts, CLI, and adapter boundaries;
- authoritative baseline and normalized parity corpus;
- selector, soak, rollback, cutover, and legacy deletion;
- stable owner-binary installation, service deployment, operational readiness,
  recovery, publication, closeout, and consumer proof for all three products;
- build, binary-size, test-count, and validation-time budgets.

Out of scope:

- new cognitive features or birthday semantics;
- redesigning Runtime v3 or C-SDLC v2;
- production provider expansion unrelated to parity;
- counting code movement as deletion.

## Consumption Rules

v0.92 may consume the new default ADL product only after the v0.91.8 cutover
and deletion gates pass. If v0.91.8 is incomplete, v0.92 must record a blocker
or an explicit bounded compatibility decision rather than absorb this work.

## Document Map

- [Vision](VISION_v0.91.8.md)
- [Design](DESIGN_v0.91.8.md)
- [Decisions](DECISIONS_v0.91.8.md)
- [WBS](WBS_v0.91.8.md)
- [Sprint](SPRINT_v0.91.8.md)
- [Issue wave](WP_ISSUE_WAVE_v0.91.8.yaml)
- [Execution readiness](WP_EXECUTION_READINESS_v0.91.8.md)
- [Quality gate](QUALITY_GATE_v0.91.8.md)
- [Feature proof coverage](FEATURE_PROOF_COVERAGE_v0.91.8.md)
- [Canonical feature-doc index](FEATURE_DOCS_v0.91.8.md)
- [Feature docs](features/README.md)
- [Demo matrix](DEMO_MATRIX_v0.91.8.md)
- [Checklist](MILESTONE_CHECKLIST_v0.91.8.md)
- [Release plan](RELEASE_PLAN_v0.91.8.md)
- [Draft release notes](RELEASE_NOTES_v0.91.8.md)
- [ADR plan](ADR_PLAN_v0.91.8.md)
- [Platform acceptance and deployment](features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md)
- [v0.92 handoff](NEXT_MILESTONE_HANDOFF_v0.91.8.md)

## Exit Criteria

- Normalized parity and rollback proof pass.
- The new product is the reviewed default.
- ADL v2, Runtime v3, and C-SDLC v2 are installed, deployed, accepted, and operationally proven.
- At least 80% of the pinned incumbent denominator is deleted.
- All retained surfaces are enumerated with owner and justification.
- v0.92 handoff names the exact consumable contracts and residual risks.
