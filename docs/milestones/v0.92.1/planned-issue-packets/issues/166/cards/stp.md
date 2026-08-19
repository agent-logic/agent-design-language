# Structured Task Prompt

Template: 1.0.0

Issue: 166

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-05 within its exact owned paths and authority boundary.

## Deliverables

- Repository and issue context types, discovery adapter, read-only importer, compatibility report, representative v2 fixture corpus, and normalized parity output.

## Acceptance

1. Resolution precedence is explicit and produces one canonical identity.
2. Symlink, path escape, ambiguous remote, and ambiguous issue cases fail closed.
3. Every unsupported v2 field is reported with record and field identity.
4. Unsupported fields produce `ImportStatus::BlockedUnsupportedFields`; the record cannot enter a v3 mutation path until every field has a reviewed preserve, map, or explicit operator disposition.
5. Import never writes v2 or v3 state and does not infer missing authority.

## Dependencies

- V3-01: issue #161
- V3-03: issue #164
- V3-04: issue #165

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-05
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- V3 state writes, binding, lifecycle transitions, GitHub mutation, or automatic conversion of v2 records.
