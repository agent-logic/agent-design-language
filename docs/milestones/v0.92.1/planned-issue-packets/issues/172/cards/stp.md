# Structured Task Prompt

Template: 1.0.0

Issue: 172

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-10B within its exact owned paths and authority boundary.

## Deliverables

- Card command modules and semantic edit operations generated from or mechanically checked against the V3-01 capability matrix, doctor finding registry, read-only repair recommendations, stranded-state detector, projection repair fixtures, and typed result schemas.

## Acceptance

1. Card edits mutate semantic values and regenerate all affected projections.
2. Rendered Markdown and stale projections never become input authority.
3. Doctor is read-only, specific, and identifies the next valid operation.
4. Doctor reports a dedicated invariant failure when a wrong or stale acceptance-bearing field has no authorized correction path; ordinary healthy states always receive a capability-derived next operation.
5. Projection drift, invalid schema, unsupported import fields, and topology blockers remain distinguishable.
6. `card show`, `card edit`, and doctor enforce the V3-06 per-phase required and optional field table and its one declared placeholder.

## Dependencies

- V3-05: issue #166
- V3-06: issue #167
- V3-07: issue #168
- V3-08: issue #169
- V3-09: issue #170

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-10b
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Binding, PVF execution, formal review, GitHub mutation, automatic repair without typed edit authority, finish, cleanup, or cutover.
