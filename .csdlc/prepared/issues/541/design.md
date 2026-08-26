# Design: Reconcile Onboarding Workflow Authority

## Scope

Update contributor-facing documentation so current workflow guidance matches the
Gate 10D2 typed C-SDLC v2 operating contract.

The core stale surface is `docs/onboarding.md`, which still describes the older
`adl_pr_cycle` / `pr ready` route as the default workflow. The corrected docs
should point current lifecycle work at `.adl/bin/csdlc-v2/` and the typed
contracts under `csdlc-v2/operator/skills/`.

## Authority

- Current lifecycle authority: Gate 10D2 typed C-SDLC v2.
- Canonical repository: `agent-logic/agent-design-language`.
- Legacy provenance repository: `danielbaustin/agent-design-language`.
- Root checkout policy remains: keep primary checkout on clean `main`; execute
  issue work only in bound issue worktrees under `/Volumes/FastWork/adl-worktrees`.

## Non-Goals

- Runtime behavior changes.
- Lifecycle binary or skill changes.
- Git remote, label, milestone, release, or PR state changes.
- Worktree binding during initialization.

## Validation

Use focused text checks over edited docs plus `git diff --check`.
