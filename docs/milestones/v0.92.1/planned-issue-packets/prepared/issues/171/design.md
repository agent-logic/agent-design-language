# V3-10A Design

Issue: #171

## Objective

Deliver issue initialization, observation, and topology-bound execution context over the kernel and transaction store.

## Scope

`issue init/show/status`, `bind`, repository and issue selection, topology collision checks, typed request/result schemas, and human/JSON presentation.

## Dependencies

- V3-05: issue #166
- V3-06: issue #167
- V3-07: issue #168
- V3-08: issue #169
- V3-09: issue #170

## Architecture Decisions

- `V3-D07`

## Deliverables

- Issue and bind command modules, direct-flag and `--input` contracts, topology proof, collision taxonomy, and end-to-end local fixtures.

## Owned Paths

- `csdlc-v3/src/commands/issue/**`
- `csdlc-v3/src/commands/bind/**`
- `csdlc-v3/tests/commands/issue/**`
- `csdlc-v3/tests/commands/bind/**`
- `.csdlc/issues/171/**`
- `.csdlc/prepared/issues/171/**`
- `.csdlc/prepared/issues/171/validate-outcome.rb`
- `.csdlc/evidence/171/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Common paths use direct flags while `--input` provides typed automation.
2. Bind verifies actual canonical branch/worktree topology and rejects every same-issue, cross-issue, main-branch, missing, dirty-policy, and drift case.
3. Issue commands remain idempotent and never infer ownership from branch names alone.
4. Human and JSON results preserve the same typed outcome.

## PVF Lanes

- `v3-10a-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/171/validate-outcome.rb`.
- `v3-10a-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-10a-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Parser/run tests, temporary-repository journeys, complete topology collision matrix, idempotency tests, human/JSON snapshots, and v2 normalized parity.

## Authority Boundary

- Issue V3-10A owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Card editing, doctor repair guidance, PVF execution, formal review, GitHub mutation, finish, cleanup, or cutover.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Binding trusts requested rather than observed topology, repository identity is ambiguous, or common use still requires request files.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-10a`
