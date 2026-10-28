# Structured Task Prompt

Template: 1.0.0

Issue: 176

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-13 within its exact owned paths and authority boundary.

## Deliverables

- Narrow GitHub trait, Octocrab adapter, concrete V3-04 `ReviewerIdentityResolver` implementation, normalized observation types, unexpected/unconsumed HTTP fixtures, pagination/retry policy, and read-only status commands.

## Acceptance

1. Domain modules depend only on normalized GitHub observations.
2. Pagination, rate limits, authentication, missing resources, and unknown mergeability remain distinct.
3. Required checks bind to exact head SHA and terminal conclusions.
4. `IssueObservation` is populated from the typed REST issue endpoint and preserves qualified identity, `state`, `state_reason`, `updated_at`, and observation time; missing or ambiguous fields cannot be normalized to open.
5. REST fixtures separately prove `state: null`, HTTP 404, and `state: closed` with `state_reason: completed`; none can normalize to an open checkpoint target.
6. Every raw-request endpoint names its GitHub API reference and has typed request/response structures plus transport-level fixtures.
7. Read-only commands perform no remote or local lifecycle mutation.
8. Authenticated human-principal observation is typed and activates no publication authority until V3-12 independently evaluates it.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-13
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- GitHub mutation, publication, foreground watch, merge, finish, cleanup, lifecycle transitions, or raw `gh`/shell fallback.
