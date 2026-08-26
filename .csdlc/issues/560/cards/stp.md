# Structured Task Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Coverage-profile or test-harness timeout bound for the three named runtime_v2 unified-runtime-kernel tests only.

## Deliverables

- adl/.config/nextest.toml
- .csdlc/prepared/issues/560/validate-focused-proof.sh
- .csdlc/prepared/issues/560/validate-lifecycle-evidence.sh

## Acceptance

1. AC-1: The fix is limited to workspace ci-coverage timeout/profile or harness bounds for the three exact runtime_v2 unified-runtime-kernel tests.
2. AC-2: Runtime v2 product semantics and the three tests' semantic assertions remain unchanged.
3. AC-3: Focused local coverage proof exercises the three affected tests under ci-coverage or the closest repo-supported focused equivalent.
4. AC-4: The PR closes #560 and documents run 33017588921 plus the #514 shared-gate role.
5. AC-5: Independent OpenAI Responses API exact-head review passes before publication/finish.
6. AC-6: Required hosted checks are green before typed C-SDLC finish merge.

## Dependencies

- #558 is merged/closed and repaired the previous adl-runtime coverage blocker.
- #514 remains separate downstream work and is not modified here.
- #499 remains separate downstream work and is not modified here.

## Inputs

- GitHub issue #560
- GitHub Actions run 33017588921
- PR #514 head 401a6b533bce34c2d1d3b580b36939a3392f3b78
- adl/.config/nextest.toml
- adl/tools/run_authoritative_coverage_lane.sh

## Non Goals

- No Runtime v2 product semantic changes.
- No Runtime v4 work.
- No #483, Sprint 2, or Sprint 3 edits.
- No broad workspace timeout increase unless exact-test override proves impossible.
