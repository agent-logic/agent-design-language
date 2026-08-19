# Issue 432 Design: Local-Only `.adl` Boundary

## Outcome

Make `.adl/**` exclusively operator-local. Remove the exact 27 tracked paths,
relocate the worktree placement policy to `config/worktree-policy.json`, update
active consumers, and add a deterministic guard against tracked `.adl` content
or authoritative tracked dependencies on it.

## Classification

- The worktree policy is active authority and moves atomically.
- Publicly useful planning/review documents move to reviewed tracked locations
  outside `.adl` only when still required; otherwise they leave the index and
  remain available in Git history and local ignored state.
- Raw logs, provider output, handoffs, and local evidence are never promoted.
- Historical immutable records may mention `.adl` as provenance. They must not
  be executable or fallback authority.

## Implementation

1. Retain an exact pre-change inventory of tracked `.adl` paths and active
   tracked references with one disposition each.
2. Move `.adl/worktree-policy.json` to `config/worktree-policy.json` and update
   lifecycle code, tests, AGENTS guidance, and any active policy consumers.
3. Remove the remaining tracked `.adl` files from the index without deleting
   unrelated operator-local ignored files.
4. Add a tracked guard that checks `git ls-files .adl` is empty and scans active
   source, scripts, tests, CI, schemas, manifests, policy, and current guidance
   for forbidden authoritative `.adl` dependencies while excluding immutable
   lifecycle/history evidence.
5. Prove allowed FastWork and rejected non-FastWork binding behavior in a fresh
   checkout.

## Invariants

- `.csdlc/**` remains canonical lifecycle authority.
- The allowed worktree parent remains `/Volumes/FastWork/adl-worktrees`.
- No local `.adl` file becomes fallback authority.
- No credential, raw provider output, log, machine path, or private evidence is
  promoted to a tracked replacement.
- Historical Git evidence remains available without remaining in the current
  tracked `.adl` tree.

## Validation

Run the issue-owned migration guard, focused C-SDLC path-policy tests, fresh
checkout positive/negative bind proof, secret/path hygiene, and `git diff
--check`. Require an exact zero tracked-path denominator.

## Rollback

Restore the prior tracked paths and policy consumer atomically from the parent
revision. Never reconstruct removed local-only material from operator state.

