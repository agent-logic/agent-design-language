# C-SDLC v3 Tooling Changeover Notice

Status: pre-change notification for issue #505.

ADL is preparing a large C-SDLC tooling changeover from the current typed v2
owner-binary route to the v3 replacement route. This notice exists so operators,
reviewers, and future agents see the change before the default tooling switch
happens.

## Current authority before #505 lands

- C-SDLC v2 remains the live lifecycle authority.
- Current lifecycle writes still use `.adl/bin/csdlc-v2/*` and the typed owner
  contracts under `csdlc-v2/operator/skills/`.
- C-SDLC v3 remains construction and cutover evidence only. It must not bind
  worktrees, publish PRs, finish issues, clean worktrees, mutate GitHub, or
  retire v2 before the V3-F/#505 cutover decision is explicitly approved.
- Historical `adl_pr_cycle`, `pr.sh`, and `pr ready/run/finish/closeout`
  language is not current operator guidance.

## What will change after approved cutover

After #505 is reviewed, approved, merged, and terminally reconciled, the default
operator guidance may switch from the v2 owner-binary route to the v3 route.
That switch is expected to be materially different: fewer hand-authored lifecycle
steps, stronger typed authority boundaries, and a simpler path for getting an
issue from ready state into execution.

The changeover must update the same instruction surfaces together:

- root `AGENTS.md`
- `csdlc-v2/AGENTS.md`
- `csdlc-v3/AGENTS.md`
- `docs/default_workflow.md`
- `docs/onboarding.md`
- `docs/architecture/ADL_ARCHITECTURE.md`
- tracked operational skill docs under `adl/tools/skills/`
- the retained historical `adl_pr_cycle` guidance

## Operator notification

Before changing the default lifecycle route, the #505 implementer must send a
pre-change notification using the typed C-SDLC v2 GitHub issue owner. The
notification must identify:

- the exact issue and PR that propose the changeover;
- the fact that v2 remains live until the PR merges and terminal closeout is
  recorded;
- the instruction surfaces that were updated;
- the rollback target if v3 parity, canary, publication, finish, or cleanup
  proof fails;
- the expected operator behavior during the transition window.

The notification is informational only. It is not approval, terminal authority,
or permission to retire v2.

## Rollback and non-claims

If parity, migration canary, rollback, observation-window, publication, finish,
cleanup, or operator approval proof is incomplete, v2 remains the rollback and
live-authority target. Do not infer approval from a prepared branch, a green
validator, a merged predecessor issue, or this notice.
