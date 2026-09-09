# Contributor Onboarding (Docs + Reports)

Use this page when you need to orient quickly in the ADL repo.

## Where to Add or Update Docs

- Project overview: `README.md`
- Tooling workflow docs: `adl/tools/README.md`
- Language docs: `adl-spec/`
- Contributor planning docs: `docs/`

## Where Reports Live

- `.adl/reports/burst/<timestamp_utc_z>/` (burst artifacts)
- `.adl/reports/pr-cycle/<issue>/<timestamp_utc_z>/` (per-issue cycle reports)
- `.adl/reports/INDEX.md` (report directory orientation)

## Workflow Context

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

Follow `docs/default_workflow.md` for current command discovery. The three-minute prepared-issue start target preserves typed validation, review and authority guards.

Canonical issue state lives under `.csdlc/issues/<issue>/`, with typed request
material normally prepared under `.csdlc/prepared/issues/<issue>/` or
Git-common invocation paths when the request is transient. Generated cards are
typed projections; do not edit their Markdown directly. Use the matching
native `edit`, `validate`, `review`, `publish`, `finish` and `clean` routes for lifecycle state changes.

The canonical repository is `agent-logic/agent-design-language`. The
`danielbaustin/agent-design-language` remote is legacy provenance unless a
bounded legacy task explicitly names it. In the primary checkout, `origin`
should identify the canonical repository; `legacy-origin`, when present, should
not be treated as the default target for current issue or PR work.

The primary checkout is inspection-only and should stay clean on `main`. Before
starting issue work, check `git status --short --branch` and
`git worktree list --porcelain`; if a feature branch or tracked changes are
sitting in the primary checkout, route the recovery through native v3
`doctor` / `bind` evidence when available. Use only the narrowest
manual fallback needed to preserve work in an issue worktree and restore root
to clean `main`. Native C-SDLC preparation and binding use `.adl/bin/native-v3/csdlc`
`issue` and `bind` with their declared typed inputs, followed by work in the
registered FastWork issue worktree. See
`docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` for the
cross-session handoff and broadcast-note rules.

After an issue is ready and bound, tracked implementation happens in the bound
issue worktree, not on root `main`. New ADL issue worktrees belong under
`/Volumes/FastWork/adl-worktrees` unless a typed migration/recovery route
explicitly records a different existing topology.

An initialized issue, green CI result, or published PR is not terminal proof by
itself. Keep review, publication, finish, and cleanup truth separate:
native `review` records exact-head review, native `publish` records publication,
native `finish` derives terminal authority from live GitHub state, and
native `clean` removes the exact registered worktree after truthful closeout.
Compression-safe finish validation is allowed only when the issue is low-risk
docs/static-tooling work and the SOR truthfully records focused local validation
instead of full local validation. CI remains required before merge.

## Reading Order

1. `README.md`
2. `adl/tools/README.md`
3. `.adl/reports/INDEX.md`
