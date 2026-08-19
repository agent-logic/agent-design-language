# Structured Task Prompt

Template: 1.0.0

Issue: 167

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-06 within its exact owned paths and authority boundary.

## Deliverables

- State/schema module, embedded audit-event model and no-pruning initial policy, projection engine, card templates or AST builders, per-card/per-phase optionality table, digest profile, fixture corpus, and state/card compatibility report.

## Acceptance

1. `state.json` is the sole machine authority and every projection is reproducible from it plus declared immutable inputs.
2. Unknown schema versions and enum values fail explicitly.
3. All six cards preserve their distinct lifecycle semantics.
4. Missing required fields fail with a typed error; optional unset fields render only the declared placeholder at each lifecycle phase.
5. `audit.jsonl` is reproducible from embedded state events and has no separate mutation or integrity authority.
6. Projection drift is diagnosable and repair never treats Markdown as authority.

## Dependencies

- V3-04: issue #165
- V3-05: issue #166

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-06
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Lifecycle transition authorization, transaction recovery, GitHub observation, direct Markdown authority, or compatibility dual writes.
