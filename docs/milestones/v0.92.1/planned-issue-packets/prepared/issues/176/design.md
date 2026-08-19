# V3-13 Design

Issue: #176

## Objective

Establish one typed, mockable GitHub boundary and complete read-only issue, PR, check, review, mergeability, and repository observation.

## Scope

Octocrab client construction, Rustls, authentication, repository and authenticated human-reviewer identity observation, REST/GraphQL endpoint wrappers, pagination, rate-limit and retry classification, response normalization, fake transport registry, and `pr status`.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Narrow GitHub trait, Octocrab adapter, concrete V3-04 `ReviewerIdentityResolver` implementation, normalized observation types, unexpected/unconsumed HTTP fixtures, pagination/retry policy, and read-only status commands.

## Owned Paths

- `csdlc-v3/src/adapters/github/read/**`
- `csdlc-v3/tests/github/read/**`
- `.csdlc/issues/176/**`
- `.csdlc/prepared/issues/176/**`
- `.csdlc/prepared/issues/176/validate-outcome.rb`
- `.csdlc/evidence/176/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Domain modules depend only on normalized GitHub observations.
2. Pagination, rate limits, authentication, missing resources, and unknown mergeability remain distinct.
3. Required checks bind to exact head SHA and terminal conclusions.
4. `IssueObservation` is populated from the typed REST issue endpoint and preserves qualified identity, `state`, `state_reason`, `updated_at`, and observation time; missing or ambiguous fields cannot be normalized to open.
5. REST fixtures separately prove `state: null`, HTTP 404, and `state: closed` with `state_reason: completed`; none can normalize to an open checkpoint target.
6. Every raw-request endpoint names its GitHub API reference and has typed request/response structures plus transport-level fixtures.
7. Read-only commands perform no remote or local lifecycle mutation.
8. Authenticated human-principal observation is typed and activates no publication authority until V3-12 independently evaluates it.

## PVF Lanes

- `v3-13-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/176/validate-outcome.rb`.
- `v3-13-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-13-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Unexpected/unconsumed fixture checks, pagination matrices, rate-limit and retry tests, exact-head check fixtures, authentication/redaction tests, and bounded live read-only canary observation.

## Authority Boundary

- Issue V3-13 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- GitHub mutation, publication, foreground watch, merge, finish, cleanup, lifecycle transitions, or raw `gh`/shell fallback.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- An endpoint requires raw shell/`gh`, response ambiguity is collapsed into success, credentials enter URLs/logs, or observation mutates state implicitly.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-13`
