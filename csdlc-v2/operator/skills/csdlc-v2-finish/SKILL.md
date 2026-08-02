---
name: csdlc-v2-finish
description: Finish one reviewed and published C-SDLC v2 issue by merging its exact green reviewed PR head, or by recognizing an already-terminal GitHub state, without a second closeout PR.
---

# C-SDLC v2 Finish

Use `csdlc-install resolve` to locate the active `csdlc-finish` binary, then pass
one typed finish request with `--root` and `--request`.

The command validates canonical pre-merge evidence and live GitHub identity. An
open PR is merged only at the declared SHA after its required checks and exact-
head review pass. A merged, closed-unmerged, or explicitly approved closed-no-PR
issue is recognized idempotently.

The result is a minimal derived terminal envelope retained beneath the Git
common directory. It is a rebuildable cache, not a new lifecycle record. The
command never edits tracked cards after merge, opens a closeout PR, or removes a
worktree.

Run cleanup separately. Legacy `csdlc-closeout` remains available only for
records that still require the old migration/repair path.
