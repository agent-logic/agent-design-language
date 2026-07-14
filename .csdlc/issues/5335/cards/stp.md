# Structured Task Prompt

Template: 1.0.0

Issue: 5335

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Create and validate the v0.91.8 planning package and approved issue wave, then publish a draft setup PR and stop before merge.

## Deliverables

- complete v0.91.8 milestone documentation package
- architecture and ownership map
- pinned LoC baseline and deletion denominator plan
- machine-useful WP issue wave and sprint plan
- feature, validation, demo, review, ADR, release, and handoff plans
- GitHub label and concrete child issues
- source and downstream milestone routing updates

## Acceptance

1. AC-1 mandatory milestone planning files exist in planned posture
2. AC-2 architecture assigns one owner per retained capability and excludes CSM and C-SDLC from ADL core
3. AC-3 budgets target 90 percent deletion and fail below 80 percent
4. AC-4 issue wave covers characterization through cutover deletion review and closeout
5. AC-5 v0.91.7 and v0.92 routing truth consumes v0.91.8 without scope absorption
6. AC-6 focused docs YAML link placeholder and diff validation passes
7. AC-7 bounded subagent review findings are resolved before publication

## Dependencies

- Runtime v3 architecture and cutover evidence on main
- C-SDLC v2 architecture and cutover evidence on main

## Inputs

- AGENTS.md
- docs/architecture/CSDLC_V2_CLEAN_ROOM_ARCHITECTURE.md
- docs/architecture/csdlc-v2/CSDLC_V1_BASELINE_AND_V2_BUDGETS.md
- docs/architecture/ADL_RUNTIME_KERNEL_PROOF.md
- docs/architecture/runtime_v3_parity_matrix.v1.json
- docs/milestones/v0.91.7/review/runtime/csm_runtime_rearchitecture_5068.md
- adl/src
- adl/Cargo.toml
- docs/milestones/v0.91.7
- docs/milestones/v0.92

## Non Goals

- implement the replacement ADL product
- delete incumbent code
- change Runtime v3 or C-SDLC v2
- redefine v0.92 birthday semantics
- run broad Rust validation for docs-only planning
- claim parity cutover or release approval
