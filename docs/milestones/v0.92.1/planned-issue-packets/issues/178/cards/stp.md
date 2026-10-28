# Structured Task Prompt

Template: 1.0.0

Issue: 178

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-15 within its exact owned paths and authority boundary.

## Deliverables

- Finish reconciler, exact linkage-aware terminal truth table, checkpoint receipt schema that cannot imply parent closure, terminal receipt schema, typed `ExternalParentClose` disposition, cleanup classifier and remover, preview output, canonical path policy, and safety fixtures.

## Acceptance

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

## Dependencies

- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175
- V3-13: issue #176
- V3-14: issue #177

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-15
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- PR publication, foreground watch, merge, broad cache removal, remote rollback, or deletion before terminal reconciliation. V3-15 is scoped to the no-merge command surface. If operator Decision 10 later authorizes `finish --merge`, that path requires a separately reviewed scope and contract revision before implementation; it cannot enter this issue by interpretation.
