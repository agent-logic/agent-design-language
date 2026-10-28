# V3-09 Design

Issue: #170

## Objective

Provide narrow, mockable effect boundaries without shell evaluation or credential leakage.

## Scope

Git repository/branch/worktree/status/diff operations, bounded process execution for declared PVF commands, environment construction, credential resolution, timeout/cancellation, output caps, and structured observations.

## Dependencies

- V3-01: issue #161
- V3-04: issue #165
- V3-05: issue #166

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Git and process traits, production adapters, fakes, V3-01 command-allowance enforcement, credential resolver, cancellation integration, and redaction tests.

## Owned Paths

- `csdlc-v3/src/adapters/git.rs`
- `csdlc-v3/src/adapters/process.rs`
- `csdlc-v3/src/adapters/credentials.rs`
- `csdlc-v3/tests/adapters/**`
- `.csdlc/issues/170/**`
- `.csdlc/prepared/issues/170/**`
- `.csdlc/prepared/issues/170/validate-outcome.rb`
- `.csdlc/evidence/170/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every Git/process invocation is argv-based and typed.
2. Exit status, stdout, stderr, timeout, cancellation, and truncation remain distinguishable.
3. Credentials exist only in the child/provider process scope that needs them.
4. Branch-name observation alone never authorizes lifecycle work.

## PVF Lanes

- `v3-09-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/170/validate-outcome.rb`.
- `v3-09-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-09-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Temporary Git repository journeys, hostile argv/path fixtures, timeout/cancellation tests, environment leakage tests, output-cap tests, and fake-adapter unexpected-call rejection.

## Authority Boundary

- Issue V3-09 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Shell scripts as internal control flow, arbitrary command evaluation, GitHub API behavior, lifecycle decisions, or secret persistence.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Any adapter invokes a shell, logs secrets, accepts ambiguous topology as authority, or cannot terminate and join a child process.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-09`
