# Structured Task Prompt

Template: 1.0.0

Issue: 627

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #627 only: denominator, manifest, one-binary command shell, help/coverage tests, and fail-closed behavior for not-yet-implemented live-authority routes.

## Deliverables

- Machine-readable v3 command manifest covering the full denominator
- One `csdlc` CLI shell exposing or reserving all replacement routes
- Focused tests for command coverage, help surface, and fail-closed stubs
- Issue-owned validator for denominator and no-v2-source-change guard
- Truthful handoff notes for #628 through #632

## Acceptance

1. AC-1: The 21-to-19 denominator is represented in a machine-readable manifest with route ownership and status.
2. AC-2: One `csdlc` binary exposes the replacement command surface without separate v3 helper binaries.
3. AC-3: Every unimplemented live-authority route fails closed with an explicit status and does not call v2, raw gh, or shell wrappers.
4. AC-4: Tests prove manifest/help coverage and reject denominator drift.
5. AC-5: No C-SDLC v2 source file is changed.

## Dependencies

- Sprint umbrella #625 must remain open.
- #505 must remain open and unmerged until sprint review and operator cutover decision.
- Current docs/csdlc-v3/full-replacement-denominator.json is source evidence, not final authority if it conflicts with this issue's reviewed output.

## Inputs

- agent-logic/agent-design-language#625
- agent-logic/agent-design-language#627
- agent-logic/agent-design-language#505
- docs/csdlc-v3/full-replacement-denominator.json
- docs/csdlc-v3/CONTRACT.md
- csdlc-v3/AGENTS.md
- AGENTS.md
- csdlc-v2/operator/generation-selector.json
- .adl/bin/csdlc-v2/

## Non Goals

- Full behavior implementation for all 19 routes
- GitHub lifecycle mutation through v3
- Publication, finish, cleanup, or cutover
- C-SDLC v2 source changes
- Independent sprint review
