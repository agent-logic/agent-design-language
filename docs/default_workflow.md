# Default C-SDLC workflow

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

1. Read the issue, six cards and root `AGENTS.md`; inspect the primary checkout and existing worktrees.
2. Discover the native command and request shape with `.adl/bin/native-v3/csdlc --help` and the typed definitions in `csdlc-v3/src/commands/`. Commands require their declared request, registry and registration inputs; a subcommand name alone is not an invocation recipe.
3. Use native `issue`, `edit`, `validate` and `doctor` routes to prepare issue-specific cards, then `bind` to the exact branch and policy-compliant FastWork worktree.
4. Create the issue-bound session goal. Implement and run focused PVF proof in that worktree. Edit generated cards only through typed semantic requests.
5. Obtain independent review and record it through `review` before `publish`. Preserve exact-head review truth when the candidate changes.
6. Use typed `github-issue`, `github-pr` and `pr-state` routes for GitHub operations; `finish` owns terminal reconciliation and `clean` owns separate guarded cleanup.

The durable card sequence is `SIP -> STP -> SPP -> VPP -> SRP -> SOR`.
Canonical records live under `.csdlc/issues/<issue>/`; prepared inputs normally
live under `.csdlc/prepared/issues/<issue>/`. Scratch files are separate from
tracked issue evidence. A passing diagnostic is not product acceptance.

For an explicitly authorized v2 transition task, resolve the retained binary set
with `csdlc-install resolve --repo <repo> --issue <issue> --requested v2` and follow
`csdlc-v2/operator/skills/`. Record the operator authorization and exact scope;
do not change the default selector or weaken review, validation or terminal gates.
The retained v2 binding runbook is
`docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md`.

The former v1 workflow in `docs/legacy/DEFAULT_WORKFLOW_V1.md` is historical.
See `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` for shared
checkout policy and `docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md` for cutover history.
