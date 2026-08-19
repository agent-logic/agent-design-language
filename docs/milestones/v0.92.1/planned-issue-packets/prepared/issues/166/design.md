# V3-05 Design

Issue: #166

## Objective

Resolve repository and issue context deterministically and import v2 records without granting v3 mutation authority.

## Scope

Root discovery, canonical repository identity, remote resolution, branch/worktree observation, issue selection precedence, symlink-safe paths, v2 record/card parsing, unsupported-field reporting, and normalized read-only projections.

## Dependencies

- V3-01: issue #161
- V3-03: issue #164
- V3-04: issue #165

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Repository and issue context types, discovery adapter, read-only importer, compatibility report, representative v2 fixture corpus, and normalized parity output.

## Owned Paths

- `csdlc-v3/src/repository/**`
- `csdlc-v3/src/import/**`
- `csdlc-v3/tests/import/**`
- `.csdlc/issues/166/**`
- `.csdlc/prepared/issues/166/**`
- `.csdlc/prepared/issues/166/validate-outcome.rb`
- `.csdlc/evidence/166/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Resolution precedence is explicit and produces one canonical identity.
2. Symlink, path escape, ambiguous remote, and ambiguous issue cases fail closed.
3. Every unsupported v2 field is reported with record and field identity.
4. Unsupported fields produce `ImportStatus::BlockedUnsupportedFields`; the record cannot enter a v3 mutation path until every field has a reviewed preserve, map, or explicit operator disposition.
5. Import never writes v2 or v3 state and does not infer missing authority.

## PVF Lanes

- `v3-05-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/166/validate-outcome.rb`.
- `v3-05-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-05-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Temporary-repository matrix, malicious path fixtures, remote/branch/worktree ambiguity tests, full representative importer corpus, and no-write filesystem assertions.

## Authority Boundary

- Issue V3-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- V3 state writes, binding, lifecycle transitions, GitHub mutation, or automatic conversion of v2 records.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Context depends on process-global current directory, unsupported fields are dropped silently, or importer execution can mutate either generation.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-05`
