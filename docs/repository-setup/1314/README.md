# Private atec repository setup — ADL #1314

Created [agent-logic/atec](https://github.com/agent-logic/atec) privately and
initialized main at `a3f7976a1769427f031774afd0d0051ee879e066`.
`readiness.json` records observed identity, settings, checks and exact-head CI.
`scaffold/` retains the independently reviewed nine-file initial payload.
The authoritative ongoing source is the atec repository.

The local checkout is `/Volumes/FastWork/atec`; future worktrees belong under
`/Volumes/FastWork/atec-worktrees`. These exact host paths are intentional setup
outputs. `.build/` and `.scratch/` keep disposable outputs on that volume.

The installed native `prepare` and `bind` routes succeeded for ADL #1314.
The selected `github-admin-api` contract allows exact existing repository
observations and bounded issue/PR operations, but excludes organization repository
creation and repository settings updates. `project-adopt` is retired and
`foundation` is observational. Therefore the explicitly authorized bootstrap used
`gh repo create --private`, `gh repo edit`, and an initial Git push after independent
review. No raw issue/PR lifecycle mutation was used. Native ADL publication and
terminal reconciliation remain separate from repository initialization.

The native quota lock required access to its existing shared directory outside
the default sandbox; escalation allowed the same scheduler to proceed. No shared
binary, quota reset, organization permission, cloud or deployment change occurred.

Issues are enabled, wiki disabled, merge and squash enabled, rebase disabled,
and merged branches deleted automatically. Review and passing checks are documented
contributor requirements; this bootstrap does not claim server-enforced branch
protection or product security qualification. Product scope, implementation stack,
and licensing remain explicit future decisions. No application tests or build
are claimed by the successful repository hygiene CI.
