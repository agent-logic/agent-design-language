# Session Coordination And Root Checkout Policy

## Purpose

ADL sessions often run in parallel. They share the same Git repository, issue
queue, local worktrees, and operator attention, but they do not automatically
share chat history or intent. This policy makes the root checkout and session
handoff rules explicit so one session does not strand another on the wrong
branch, overwrite active work, or hide important workflow state in memory.

## Current Authority

This document clarifies the existing workflow rules in:

- `AGENTS.md`
- `docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md`
- `docs/onboarding.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- the typed v2 doctor/bind contracts

If this document conflicts with `AGENTS.md`, `AGENTS.md` wins until both files
are updated together.

## Root Checkout Ownership

The primary checkout is the repository root, for example:

```text
/Users/daniel/git/agent-design-language
```

The primary checkout must normally stay on clean `main`.

Allowed primary-checkout uses:

- read-only inspection
- issue creation/bootstrap
- typed `csdlc-doctor` readiness inspection
- prep-scout issue inspection/readiness checks for a separate next-issue lane
  while the current issue is in a truthful wait state
- typed `csdlc-bind` when `main` is clean
- fast-forwarding `main`
- checking root/worktree state before routing

Disallowed primary-checkout uses:

- tracked implementation edits
- janitor repairs to PR branches
- finish staging for an issue branch
- leaving the root checkout on a feature branch
- parking untracked issue artifacts in root when a bound worktree exists

After an issue is bound, tracked edits happen in the bound issue worktree.
The bound worktree's local `.adl/<version>/tasks/...` bundle is the active
issue-local execution surface for normal issue work. Materialized
`.adl/<version>/sprints/...` packet copies in the worktree are convenience
mirrors for local context, not silent replacements for the primary checkout's
canonical sprint record. If the worktree-local issue identity disagrees with
the primary checkout for the same issue, stop and repair the mismatch instead
of guessing which copy is right. Root-only `.adl` state remains bootstrap,
coordination, and sprint-truth context rather than a hidden per-issue live
authority during execution.

Prep-scout exception:

- a prep scout may use the root checkout for read-only next-issue inspection
  and readiness classification while another issue is waiting
- the prep scout must not convert that root-checkout preparation pass into
  tracked implementation or hidden candidate-issue mutation on `main`
- if a candidate would require mutation before it can be called ready and there
  is no proven prep-only repo-native bind surface, stop as `needs_operator` and
  record the tooling gap instead of improvising a manual fallback
- if the candidate issue is promoted into execution, leave prep-scout mode,
  resolve `v2` with `csdlc-install`, and use the typed `csdlc-bind` request
  before any tracked edits occur

## Required Startup Check

Before starting or resuming tracked issue work, a session must check:

```bash
git status --short --branch
git worktree list --porcelain
```

Expected root state:

```text
## main...origin/main
```

If the primary checkout is not on `main`, has tracked changes, or is occupied by
an issue branch, stop before implementation. Route the recovery through typed
`csdlc-doctor` and `csdlc-bind` evidence when the issue/worktree can be
identified. Use only the narrowest manual fallback needed
to preserve work in an issue worktree, restore the primary checkout to clean
`main`, and record what moved where.

If a broad process check is needed, use the permission-safe process helper from
`docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md`; do not use broad `ps`,
`pgrep`, or `lsof` scans as workflow control.

## Git Topology Coordination

Current issue ownership is derived from observable repository state:

- the live GitHub issue and PR
- the issue branch
- the registered worktree
- the branch/worktree relationship reported by Git

Claims, leases, heartbeats, protected-path ledgers, and machine-local lock
records are not lifecycle authority. Sessions should inspect the shared Git
topology before binding, avoid a branch or worktree already in active use, and
record concise handoff notes on the issue or PR when responsibility changes.
`csdlc-doctor` and `csdlc-bind` should report this topology directly.

## Broadcast Notes

When a session changes shared workflow state, it must leave a short durable note
in the relevant issue, sprint packet, PR, or closeout record. Examples:

- root checkout repaired
- feature branch moved from root into `.worktrees/...`
- issue is active in another session
- issue is waiting on CI, review, or watcher
- raw GitHub fallback was used because repo-native tooling failed
- lifecycle wrapper stalled or failed and a bounded fallback was used

Broadcast notes should be factual, brief, and free of secrets. They should name
the issue, branch, worktree, and next expected owner/action.

For v0.91.6 rescue-sprint work, broadcast notes should also name whether the
state is active execution, watcher-owned wait, janitor repair, prep-scout
handoff, or closeout. This keeps resumed sessions from guessing whether a draft
PR is healthy, blocked, or abandoned.

## Collision Handling

When another session appears to own an issue or worktree:

1. Do not start duplicate implementation work.
2. Inspect the issue, PR, branch, and worktree state.
3. If the state is healthy, leave it alone or watch it.
4. If the state is stale or broken, record the evidence and use the typed
   doctor, shepherd, or closeout binary as appropriate.
5. If root is occupied by that work, use typed v2 doctor/bind evidence first.
   Use manual preservation only as a
   bounded fallback to move the work into an issue worktree before restoring
   root to clean `main`.

Prep-scout-specific collision rule:

- if the candidate issue already has an open PR or bound
  worktree owned by another session, classify the handoff as `collision`
  instead of beginning duplicate preparation

## Tooling Failure Handling

If a repo-native lifecycle command fails or hangs:

- stop the command rather than waiting indefinitely
- verify whether it partially created an issue, worktree, PR, or local bundle
- record the failure in the issue or a remediation issue
- use the narrowest fallback needed to preserve root checkout safety
- do not normalize the fallback into the preferred workflow

This rule exists so emergency cleanup does not become a second, undocumented
workflow.

### Audited raw-`gh` break-glass transport

Typed owners remain the normal and final authority. Raw `gh` may transport a
covered remote write only when every prerequisite in `AGENTS.md` is satisfied:
the applicable typed owner has a confirmed reproducible regression, a durable
tooling issue contains safe reproduction evidence, and the operator has
explicitly authorized this exact repository, numeric issue or pull request (or
exact PR-creation head), and operation after seeing the blocker.

The only allowed canonical argv shapes are shown below. Argument order is part
of the contract; angle-bracket values are single arguments, not shell
expressions.

```text
gh issue comment <number> --repo <owner/name> --body-file <safe-relative-path>
gh issue edit <number> --repo <owner/name> --body-file <safe-relative-path>
gh pr create --repo <owner/name> --base <branch> --head <branch> --title <text> --body-file <safe-relative-path>
gh pr create --repo <owner/name> --base <branch> --head <branch> --title <text> --body-file <safe-relative-path> --draft
gh pr edit <number> --repo <owner/name> --body-file <safe-relative-path>
gh pr ready <number> --repo <owner/name>
gh pr comment <number> --repo <owner/name> --body-file <safe-relative-path>
```

Run the command from the primary checkout so the body path is exactly
`.git/csdlc-v2/break-glass/<invocation-id>/body.md`. The invocation ID must be
a non-empty ASCII alphanumeric-led component containing only ASCII
alphanumerics, dot, underscore, or hyphen, and must not be `.` or `..`. The
body must be a current-user-owned regular final-component non-symlink with mode
0600. The title must be non-sensitive. No other flags, flag order, repeated
flags, stdin body, absolute or traversing body path, shell expansion, alias,
extension, `gh api`, target alias, or bulk selector is allowed.

The denylist is structural, not advisory. It includes issue creation or close;
PR close, merge, or base mutation; labels, milestones, projects, assignees, or
reviewers; finish or cleanup; release, repository, workflow, secret, variable,
administrative, deletion, force, and bulk operations. Additional operator
wording cannot widen this list; a policy change requires its own review and
merge.

#### Break-glass receipt protocol

Create one unique directory with mode 0700 beneath
`.git/csdlc-v2/break-glass/`. Create each event with exclusive create-new
semantics. `intent.json`, `result.json`, and `reconciliation.json` must never be overwritten,
truncated, replaced, or amended. The body file is transient input, not a
receipt; remove it after the result event is durably created without changing
the event files.

`intent.json` is created before the command and records:

- `schema`, `timestamp_utc`, `actor`, `regression_issue`, and
  `authorization_reference`;
- `repository`, numeric `issue` or `pull_request`, `operation_class`, and
  `redacted_argv` (replace title/body values with digests or fixed redactions);
- `bound_worktree`, `branch`, `exact_head`, `typed_generation`, and
  `typed_digest`;
- `remote_pre_state` or, for PR creation, the exact head plus proof that no PR
  currently exists for it; and
- the planned typed reconciliation owner and operation.

`result.json` is created after the command and binds the SHA-256 digest of
`intent.json`, completion timestamp, exit classification, and only safe remote
identity fields such as issue or PR number, URL, state, and exact head. It must
not contain request/response bodies.

`reconciliation.json` is created only after the typed owner successfully
observes and reconciles the exact remote post-state. It binds the SHA-256
digests of both prior events, timestamp, reconciliation owner and operation,
exact remote post-state, resulting `typed_generation`, resulting
`typed_digest`, and `reconciliation_status: succeeded`.

All three events must declare that they contain no credentials, token values, token-file contents, environment dumps, sensitive request bodies, or raw response bodies.
The affected lifecycle enters a freeze after the intent event. A failed or
missing result does not erase the freeze. Only a successful typed operation and
matching create-only reconciliation event release it; until then, readiness,
review, publication, merge-ready, terminal, and finish claims remain denied.

## Non-Goals

This policy does not:

- replace typed v2 lifecycle routing
- replace issue cards or closeout truth
- permit tracked work on `main`
- make chat memory authoritative
- solve all future polis governance or non-software guild occupancy rules
