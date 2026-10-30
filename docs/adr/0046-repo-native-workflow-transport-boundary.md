# ADR 0046: Repo-Native Workflow Transport Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4622, #4806, #4960, #4989
- Related ADRs: ADR 0033, ADR 0037
- Source evidence:
  - `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md`
  - `docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md`
  - `docs/milestones/v0.91.7/review/V0917_TOOLS_SPRINT_4806_REVIEW_REMEDIATION_4961.md`

## Context

ADL issue and PR work repeatedly lost time when workflow helpers depended on
raw `gh`, shell fallback, or inconsistent GitHub metadata interpretation.

## Decision

ADL workflow issue/PR metadata and mutation should flow through repo-native
typed commands and the shared GitHub token resolver. Raw `gh` is not a normal
C-SDLC workflow backend. Remaining direct helper use is migration debt, not an
approved alternate path.

## Consequences

- Issue/PR behavior has one transport boundary.
- Missing repo-native coverage becomes a tooling bug.
- Token handling and observability can be tested in one place.

## Alternatives Considered

### Keep raw `gh` as the expected fallback

Rejected. It makes session behavior depend on host-local shell state.

## Validation Notes

Future changes should prove issue list/view/create/edit/comment, PR validation,
PR creation, readiness, merge, and closeout through repo-native paths.

## Non-Claims

- This ADR does not claim every legacy helper has been migrated.
- This ADR does not expose or commit credentials.
