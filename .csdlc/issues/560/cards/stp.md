# Structured Task Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded workspace ci-coverage stabilization for the seven runtime_v2 unified-kernel tests plus the context-mirror milestone-input portability defect exposed by the exact hosted lane; no unrelated product behavior.

## Deliverables

- adl/.config/nextest.toml
- adl/src/adl_gws_context_mirror.rs
- adl/src/bin/demo_adl_gws_context_mirror.rs
- docs/planning/ADL_FEATURE_LIST.md
- .csdlc/prepared/issues/560/validate-focused-proof.sh
- .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

## Acceptance

1. AC-1: The ci-coverage profile applies an exact bounded timeout override to seven runtime_v2 unified-kernel tests without weakening unrelated tests.
2. AC-2: Runtime v2 product semantics and all seven tests' semantic assertions remain unchanged.
3. AC-3: Focused local proof selects and passes exactly seven unified-kernel tests and the previously failing context-mirror binary test.
4. AC-4: The context-mirror fixture supplies canonical inputs and milestone detection accepts only one explicit active-status marker, rejecting future, completed, or conflicting markers.
5. AC-5: Independent exact-head review passes before republication.
6. AC-6: Required hosted checks, including workspace coverage, are green before merge.

## Dependencies

- #558 is merged/closed and repaired the previous adl-runtime coverage blocker.
- #514 remains separate downstream work and is not modified here.
- #499 remains separate downstream work and is not modified here.

## Inputs

- GitHub issue #560
- GitHub Actions runs 33017588921 and 33025738363
- adl/.config/nextest.toml
- adl/tools/run_authoritative_coverage_lane.sh
- adl/src/adl_gws_context_mirror.rs
- adl/src/bin/demo_adl_gws_context_mirror.rs
- docs/planning/ADL_FEATURE_LIST.md

## Non Goals

- No Runtime v2 product semantic changes.
- No Runtime v4 work.
- No #483, Sprint 2, or Sprint 3 edits.
- No broad workspace timeout increase unless exact-test override proves impossible.
