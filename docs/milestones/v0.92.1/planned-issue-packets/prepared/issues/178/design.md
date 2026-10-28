# V3-15 Design

Issue: #178

## Objective

Reconcile terminal GitHub truth and provide a separate, path-exact, fail-closed cleanup operation.

## Scope

`finish`, linkage-aware PR selection, merged/closed/checkpoint/no-PR outcomes, terminal receipts, projection reconciliation, `clean` classify/preview/remove, canonical worktree identity, dirty/live/drift predicates, and retained evidence.

## Dependencies

- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175
- V3-13: issue #176
- V3-14: issue #177

## Architecture Decisions

- `V3-D10`

## Deliverables

- Finish reconciler, exact linkage-aware terminal truth table, checkpoint receipt schema that cannot imply parent closure, terminal receipt schema, typed `ExternalParentClose` disposition, cleanup classifier and remover, preview output, canonical path policy, and safety fixtures.

## Owned Paths

- `csdlc-v3/src/commands/finish/**`
- `csdlc-v3/src/commands/clean/**`
- `csdlc-v3/tests/terminal/**`
- `.csdlc/issues/178/**`
- `.csdlc/prepared/issues/178/**`
- `.csdlc/prepared/issues/178/validate-outcome.rb`
- `.csdlc/evidence/178/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Finish derives terminal truth from exact GitHub state and never creates or selects an ambiguous second PR.
2. A merged `part_of` publication records checkpoint completion without closing or terminally completing the parent issue; only a matching `closing` publication or explicit no-PR outcome can do so.
3. Successful checkpoint finish transitions `published | merge_ready` through `checkpoint_completed` to `implemented`, retains checkpoint evidence, and invalidates the prior review/publication authorization before another slice.
4. A complete acceptance journey merges multiple `part_of` checkpoints for one issue, preserves the open parent after each, then processes a later independently reviewed `closing` publication through finish and closes that exact parent without selecting any checkpoint PR as terminal authority.
5. A merged `part_of` checkpoint whose parent later closes returns `operator_required`; the separately authorized external-parent-close disposition records distinct causes and reaches terminal truth without crediting the checkpoint PR or requiring remote rollback.
6. Cleanup is a separate command after finish and defaults to preview.
7. Cleanup requires canonical candidate-path equality with the verified Git worktree root; prefix and relative matches are rejected.
8. Live, dirty, mismatched, absent, unregistered, and already-removed worktrees have distinct outcomes.
9. Build/cache directories from any other worktree are never deletion targets.
10. Cleanup requires committed `closed_out` state and its terminal receipt; a GitHub merge without local terminal reconciliation remains ineligible.

## PVF Lanes

- `v3-15-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/178/validate-outcome.rb`.
- `v3-15-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-15-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Linkage-aware terminal outcome matrix, multiple checkpoint then closing journeys, close-between-merge-and-finish race and disposition tests, ambiguous/mixed-PR negatives, checkpoint/terminal receipt tamper tests, canonical/symlink/path-escape fixtures, dirty/live/drift cleanup matrix, exact deletion-list proof, and bounded end-to-end canary closeout.

## Authority Boundary

- Issue V3-15 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- PR publication, foreground watch, merge, broad cache removal, remote rollback, or deletion before terminal reconciliation. V3-15 is scoped to the no-merge command surface. If operator Decision 10 later authorizes `finish --merge`, that path requires a separately reviewed scope and contract revision before implementation; it cannot enter this issue by interpretation.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Finish trusts local prose over GitHub, PR selection is ambiguous, a `part_of` PR can close or terminally complete its parent, cleanup cannot prove exact path identity, or deletion scope includes another live/open worktree.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-15`
