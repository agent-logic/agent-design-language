# #632 V3-H.6 canary evidence index

Status: in progress; not cutover-ready.

## Current authority

C-SDLC v2 remains the only live lifecycle authority until #505 is explicitly
reviewed, approved, merged, and terminally reconciled. This packet is
construction/canary readiness evidence for v3, not authority cutover.

The current `csdlc-v3` CLI advertises only two commands: `foundation` and
`local`. The one-binary routes in `command-route-coverage.json` are target
surface names unless their `current_v3_cli_command` field names one of those
advertised commands. Planned routes without an advertised command are
cutover-blocking work, not usable operator routes.

## Live typed observations captured in this lane

- #632 GitHub issue read through `csdlc-github-issue`: pass; issue is open.
- #632 defect comment through `csdlc-github-issue`: pass; comment
  `5513915103` records DEFECT-019.
- #631 PR #644 unlinked PR-state through `csdlc-github-pr`: pass; PR is open,
  non-draft, clean, and CI-green on base
  `codex/627-v3-h1-command-denominator-r2`.
- #631 PR #644 linked PR-state through `csdlc-github-pr`: fail-closed with
  `reconciliation_required`; GitHub does not expose `Closes #631` as a closing
  relation for the stacked base.
- #632 local typed bootstrap through `csdlc-issue create`: pass after VPP lane
  coverage repair.
- #632 typed bind through `csdlc-bind`: pass; execution worktree is
  `/Volumes/FastWork/adl-worktrees/adl-issue-632-v3-h6-canaries-docs-readiness-exec`.
- #632 fresh worktree `csdlc-install install`: pass after one-line current-main
  import repair; the failed first run is retained as DEFECT-020.

## Explicit non-claims

- This packet does not claim #505 is ready to close.
- This packet does not claim v3 can finish or clean live issues before #505.
- This packet does not claim stacked PRs satisfy GitHub closing-linkage
  authority.
- This packet does not authorize merging #627 through #632.

## Next proof needed

Before #505 can close, every route in
`.csdlc/prepared/issues/632/command-route-coverage.json` must be either
exposed by the actual `csdlc` CLI and behaviorally proven through a real issue
canary, proven by an explicitly accepted deterministic fixture, or retained as a
named cutover blocker with operator-approved disposition.
