# Terminal cache conflict: #570/#571 stale prep worktrees

Date: 2026-08-31

Actor: Worker #6

Scope: Sprint 6 cutover readiness, #570/#571 gate reconciliation.

## Observation

Live GitHub readback shows both gate issues are terminal:

- #570 / PR #584: issue closed as completed, PR merged at head
  `ee8b32201b8efaaed6040eacf3b017193f43d110`, merge commit
  `989d536af37455f1657be88af5d4d8c82d21b0b1`.
- #571 / PR #585: issue closed as completed, PR merged at head
  `2155f3fbb84598f19b65fcec855c6e41790b1c71`, merge commit
  `756da2103e99afc0a030ffd9c76290b7dc79ccad`.

The registered FastWork worktrees available for #570/#571 are prep-state
worktrees:

- `/Volumes/FastWork/adl-worktrees/adl-issue-570-v3-g-docs-skill-cutover-readiness`
  has `.csdlc/issues/570/index.json` at phase `ready`, generation `4`,
  digest `380377ef9b0faa7b40949706c0c5980837933c2487d3bb97e62e68758b6f315f`.
- `/Volumes/FastWork/adl-worktrees/adl-issue-571-v3-a-followup-predecessor-proof-lifecycle-gates`
  has `.csdlc/issues/571/index.json` at phase `ready`, generation `3`,
  digest `e6e59bf21a4ef464447c24fe2d3d26b1f86095acefde1a36412fa7ff39d41b9d`.

The common Git terminal cache already contains terminal envelopes for later
canonical generations:

- #570 derived terminal: generation `26`, digest
  `3bc9f5e4993acaa52d91b6bc684688ad3cde32893330e96a845706d440eb343a`.
- #571 derived terminal: generation `13`, digest
  `726dd72f563c5fc8b95c7fa44c7c6a86c7e7da0eff128ac55609e90d14172c3e`.

## Commands exercised

The typed v2 historical finish path was exercised against both available
worktrees using exact live issue/PR identity. Both failed closed:

```text
{"code":"reconciliation_required","message":"derived terminal cache conflicts with retained immutable authority","schema":"csdlc.error.v1"}
```

The typed cached-terminal validation path also failed closed for both available
worktrees:

```text
{"code":"reconciliation_required","message":"derived terminal envelope does not match canonical issue truth","schema":"csdlc.error.v1"}
```

## Defect captured

The current v2 terminal cache is keyed by issue in the shared Git common
directory. When an older prep-state worktree for the same issue remains
registered after a later execution worktree has produced terminal authority,
the prep worktree cannot validate or reconcile terminal truth through the
existing typed finish/cache commands.

This is correct fail-closed behavior for immutable receipts, but it is a
cutover-readiness defect because an operator cannot run one typed command from
the visible registered #570/#571 worktrees and get an immediately actionable
"this worktree is stale; terminal authority already belongs to generation N in
the execution worktree/cache" result.

## Required follow-up before cutover

Add a typed, non-mutating terminal reconciliation diagnostic that:

1. compares the current local projection with any retained common terminal
   envelope for the same issue;
2. classifies `same_projection_terminal`, `stale_projection_terminal_exists`,
   `conflicting_terminal_authority`, and `missing_terminal_authority`
   distinctly;
3. reports the canonical generation/digest of both the local projection and the
   retained terminal envelope;
4. refuses to overwrite immutable terminal receipts; and
5. gives an explicit next action for stale prep worktrees without requiring raw
   GitHub or manual JSON inspection.

Until that exists, #570/#571 should be treated as live-remote terminal gates
with retained common terminal receipts, but their stale prep worktrees should
not be pruned or used as proof that local closeout completed from the registered
prep directories.
