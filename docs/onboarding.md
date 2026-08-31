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

Gate 10D2 is the current C-SDLC authority. Current issue lifecycle work uses
the independent typed Rust owner binaries under `.adl/bin/csdlc-v2/`, selected
through the typed contracts in `csdlc-v2/operator/skills/`, until an explicit
operator-reviewed V3-F/#505 cutover changes that contract. The old
`adl_pr_cycle`, `pr.sh`, prompt-template wrapper, and five-command compatibility
routes are historical surfaces, not current lifecycle authority.

C-SDLC v3 is the planned clean replacement line, but before V3-F it is only
construction and cutover-readiness evidence. Do not use v3 to bind worktrees,
mutate issue state, publish pull requests, finish issues, clean worktrees, or
retire v2. A prepared v3 issue should still be simple to start: once
dependencies are satisfied, inspection, typed bind, and first useful work should
take three minutes or less without bypassing v2 authority, review, validation,
publication, finish, or cleanup truth.

Canonical issue state lives under `.csdlc/issues/<issue>/`, with typed request
material normally prepared under `.csdlc/prepared/issues/<issue>/` or
Git-common invocation paths when the request is transient. Generated cards are
typed projections; do not edit their Markdown directly. Use the matching
`csdlc-edit`, `csdlc-validate`, `csdlc-review`, `csdlc-publish`,
`csdlc-finish`, and `csdlc-clean` routes for lifecycle state changes.

The canonical repository is `agent-logic/agent-design-language`. The
`danielbaustin/agent-design-language` remote is legacy provenance unless a
bounded legacy task explicitly names it. In the primary checkout, `origin`
should identify the canonical repository; `legacy-origin`, when present, should
not be treated as the default target for current issue or PR work.

The primary checkout is inspection-only and should stay clean on `main`. Before
starting issue work, check `git status --short --branch` and
`git worktree list --porcelain`; if a feature branch or tracked changes are
sitting in the primary checkout, route the recovery through typed v2
`csdlc-doctor` / `csdlc-bind` evidence when available. Use only the narrowest
manual fallback needed to preserve work in an issue worktree and restore root
to clean `main`. Native C-SDLC bootstrap for ADL issues runs from an isolated
staging checkout, then `csdlc-bind` moves approved work into the canonical
FastWork issue worktree. See
`docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` for the
cross-session handoff and broadcast-note rules.

After an issue is ready and bound, tracked implementation happens in the bound
issue worktree, not on root `main`. New ADL issue worktrees belong under
`/Volumes/FastWork/adl-worktrees` unless a typed migration/recovery route
explicitly records a different existing topology.

An initialized issue, green CI result, or published PR is not terminal proof by
itself. Keep review, publication, finish, and cleanup truth separate:
`csdlc-review` records exact-head review, `csdlc-publish` records publication,
`csdlc-finish` derives terminal authority from live GitHub state, and
`csdlc-clean` removes the exact registered worktree after truthful closeout.
Compression-safe finish validation is allowed only when the issue is low-risk
docs/static-tooling work and the SOR truthfully records focused local validation
instead of full local validation. CI remains required before merge.

## Reading Order

1. `README.md`
2. `adl/tools/README.md`
3. `.adl/reports/INDEX.md`
