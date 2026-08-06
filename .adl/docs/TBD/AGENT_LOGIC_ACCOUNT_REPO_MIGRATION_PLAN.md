# Agent Logic GitHub Repository Copy Plan

## Status

Final execution plan for v0.92 WP-02. This document defines the non-destructive
copy sequence and proof required to create company-owned mirrors under the
existing `agent-logic` organization while preserving every source repository
under `danielbaustin` unchanged.

This plan does not authorize a copy window by itself. The operator must start
each named window after its preflight passes. Source deletion, transfer,
rename, archival, visibility change, settings mutation, issue movement, and
history rewrite are forbidden throughout WP-02.

## Decision

Create these five destination repositories as independent copies:

| Order | Read-only source | Destination | Required visibility |
| --- | --- | --- | --- |
| 1 | `danielbaustin/cognitive-sdlc-paper` | `agent-logic/cognitive-sdlc-paper` | Private |
| 2 | `danielbaustin/godel-hadamard-bayes-paper` | `agent-logic/godel-hadamard-bayes-paper` | Private |
| 3 | `danielbaustin/general-intelligence-paper-private` | `agent-logic/general-intelligence-paper-private` | Private |
| 4 | `danielbaustin/universal-tool-schema` | `agent-logic/universal-tool-schema` | Private |
| 5 | `danielbaustin/agent-design-language` | `agent-logic/agent-design-language` | Public |

The full denominator is seven source repositories. These two are immutable
negative controls and receive no destination copy:

- `danielbaustin/asksifu`
- `danielbaustin/Horust`

Repositories already owned by `agent-logic` are read-only integration inputs:

- `agent-logic/agent-logic.ai`
- `agent-logic/codefriend.ai`
- `agent-logic/strategic-cognitive-reserve`

## Copy Semantics

GitHub repository duplication copies Git objects and copyable refs. For WP-02,
the exact ref denominator is branches (`refs/heads/*`), tags (`refs/tags/*`),
and supported Git notes (`refs/notes/*`). GitHub-owned pull-request refs under
`refs/pull/*` are source-authoritative collaboration metadata and are excluded
from destination ref parity. Git LFS objects need
their own fetch and push. GitHub's ordinary duplication and import paths do not
copy collaboration history or repository settings such as issues, pull
requests, discussions, collaborators, rulesets, environments, secrets,
webhooks, Apps, Pages, packages, or security configuration.

Therefore WP-02 has two distinct outputs:

1. an exact Git and Git LFS copy at the destination; and
2. an explicitly reconstructed destination configuration, with every
   non-copyable source surface either recreated, linked back to the retained
   source, or dispositioned as intentionally source-authoritative.

No evidence may describe an issue, pull request, setting, package, or
integration as copied unless live destination inspection proves it.

## Success Criteria

WP-02 is complete only when:

1. all five destinations exist in order with the visibility matrix above;
2. every destination has the exact approved source refs, object graph, default
   branch, tags, and Git LFS objects;
3. destination settings and integrations are reconstructed or have an explicit
   reviewed disposition;
4. the five source repositories still exist under `danielbaustin` with their
   pre-copy visibility, default branch, refs, HEAD, settings inventory, and
   collaboration inventory unchanged;
5. `asksifu` and `Horust` remain unchanged and have no destination copy;
6. issue #5888 is open and bound to the verified public ADL destination for
   the later website-reference cutover; and
7. no secret values or unexplained drift appear in retained evidence.

## Safety Invariants

- Source remotes are read-only inputs. No command may push to, edit, transfer,
  rename, archive, delete, or change settings on a `danielbaustin` repository.
- Destination creation is the first mutation in each copy window.
- Every write command must name `agent-logic/<repository>` explicitly.
- Before a mirror push, verify the push URL owner is exactly `agent-logic` and
  the repository name is the expected current item.
- Never use `git push --mirror`; push only the approved destination refspecs.
- Work one repository at a time and stop on any failed gate.
- Preserve names and the required visibility matrix.
- Record secret and variable names and scopes only, never values.
- Do not rely on source deletion, ownership transfer, or redirect behavior.
- Do not use `/private/tmp`; use the issue worktree or another approved durable
  workspace for temporary mirror data.

## Gate 0: Organization Readiness

Before the first copy window:

1. Confirm `agent-logic` is the intended company destination.
2. Confirm organization owners, billing owner, recovery contact, and required
   2FA.
3. Confirm private-repository, Actions, Pages, package, LFS, and security
   capabilities and budgets.
4. Confirm the operator can create repositories and administer destination
   settings without mutating source settings.
5. Confirm all five destination names are free.
6. Confirm the exact seven-repository denominator, five-copy allowlist, two
   untouched controls, and visibility matrix.

Exit condition: organization readiness and the exact copy-only scope are
recorded. Transfer authority is neither required nor exercised.

## Gate 1: Read-Only Source Manifest

Capture a redacted source manifest without changing the repository:

- owner/name, visibility, default branch, exact HEAD, all refs, tags, releases,
  object counts, and Git LFS inventory;
- issues, pull requests, milestones, projects, discussions, wiki, assignees,
  collaborators, and outside collaborators;
- rulesets, protections, required checks, CODEOWNERS, environments, approvals,
  workflows, schedules, runner labels, variables, and secret names;
- packages, Pages, custom domains, OIDC subjects, webhooks, deploy keys, Apps,
  OAuth integrations, callbacks, forks, submodules, and downstream consumers;
- security-feature state and current operational URLs.

Record both machine-readable data and unsupported/manual dispositions. Secret
values are never read or retained. Compute `refs_sha256` from the canonical
sorted array of `[fully-qualified-ref, object-sha]` pairs for the approved
branches, tags, and notes; the live verifier recomputes this exact form.

Exit condition: the source manifest is timestamped, redacted, reviewed, and
bound to the exact source HEAD and refs immediately before the copy.

## Gate 2: Destination Plan

For the active repository:

1. Confirm the destination name is still free.
2. Decide the exact destination visibility from the fixed matrix.
3. Classify each non-Git source surface as:
   - recreate at destination;
   - retain at source and link explicitly;
   - intentionally omit with rationale; or
   - operator action required because GitHub does not expose the value.
4. Prepare destination teams, rulesets, Actions policy, environments, secret
   names/scopes, OIDC trust, packages, Pages, webhooks, Apps, and consumers.
5. For open issues and pull requests, retain the source as historical authority
   unless a separately reviewed copy mechanism can preserve attribution and
   links without modifying the source.
6. Prepare a rollback that deletes or quarantines only the new destination;
   source recovery must never be necessary because the source is immutable.

Exit condition: every non-Git surface has an owner, disposition, validation,
and destination-only recovery action.

## Gate 3: Copy Window

For the active repository:

1. Re-read source HEAD, refs, visibility, and default branch; stop on drift.
2. Create the empty destination with the exact required visibility and without
   an initialized README, license, or `.gitignore`.
3. Disable GitHub Actions on the empty destination and verify it is disabled
   before pushing any ref. This prevents copied push and scheduled workflows
   from running with incomplete destination configuration.
4. Create a local mirror from the read-only source.
5. Fetch all Git LFS objects when LFS is present.
6. Set and inspect a separate push URL that names only the expected
   `agent-logic/<repository>` destination.
7. Push the approved branches, tags, and supported notes to the destination
   with explicit refspecs: `+refs/heads/*:refs/heads/*`,
   `+refs/tags/*:refs/tags/*`, and `+refs/notes/*:refs/notes/*`. Do not use
   `git push --mirror`, and do not treat GitHub-owned `refs/pull/*` as copyable
   refs.
8. Push all Git LFS objects to the destination when applicable.
9. Set the destination default branch and reconstruct approved destination
   configuration.
10. Re-enable Actions only after required destination secrets, variables,
    environments, OIDC trust, rulesets, permissions, and workflow dispositions
    are verified. Keep Actions disabled for a cold mirror.
11. Do not change any source repository or source-side configuration.

The next repository may not start until Gate 4 passes.

## Gate 4: Destination And Source Verification

Verify the destination:

- exact refs and object identity against the approved source snapshot;
- exact HEAD, default branch, tags, releases disposition, and LFS object proof;
- required visibility;
- reconstructed rulesets, checks, teams, collaborators, environments, Actions,
  Pages, packages, OIDC, webhooks, Apps, security, and consumer configuration;
- explicit issue/PR/collaboration-history disposition; and
- a bounded repository-appropriate smoke check.

Then re-read the source and compare it with its before-manifest. The source must
still have the same owner, visibility, default branch, refs, HEAD, and settings
inventory. Any unexplained source difference is a blocker even if the
destination is correct.

Exit condition: destination proof passes and source immutability is proven.

## ADL And Website Cutover

`agent-design-language` copies last. The destination remains public. Only after
its destination passes Gate 4 may the dedicated `agent-logic.ai` change replace
the four current ADL links in:

- `site/index.html`, header and footer;
- `site/beta/index.html`, header and footer.

The source `danielbaustin/agent-design-language` repository remains public and
unchanged throughout WP-02. A later issue may add a prominent canonical-repo
notice, then optionally archive the source after the destination is proven.
That notice is not a native GitHub redirect, and retained historical evidence
does not need rewriting.

## Organization-Wide Final Verification

1. Confirm exactly five destinations and exactly seven source controls.
2. Confirm all seven source repositories remain under `danielbaustin`.
3. Confirm the five copied sources are unchanged from their before-manifests.
4. Confirm `asksifu` and `Horust` are unchanged and absent from `agent-logic`.
5. Confirm four private destinations and one public ADL destination.
6. Verify that #5888 owns the post-copy website reference update and is blocked
   until the public ADL destination passes Gate 4. The website cutover is not a
   WP-02 closeout requirement.
7. Record source/destination URLs, exact refs and HEADs, manifest digests,
   destination creation times, validation outcomes, repairs, and residual risk.

## Failure And Recovery

When a copy or verification fails:

1. stop the copy wave;
2. preserve the source and its manifests without mutation;
3. disable, quarantine, or delete only the incomplete destination with explicit
   operator authorization;
4. repair destination configuration or recreate the destination from the same
   approved source snapshot;
5. rerun complete destination and source-immutability verification.

There is no transfer-back path because ownership never moves.

## Evidence Package

Retain one compact, redacted package containing:

- organization-readiness receipt;
- exact five-copy and two-control inventory;
- fixed visibility matrix;
- source before/after immutability manifests and digests;
- destination Git/ref/LFS and configuration manifests;
- non-copyable GitHub-surface dispositions;
- destination creation timestamps, Actions-disabled observations, actual first
  push timestamps and transcripts, and canonical URLs;
- per-repository validation outcomes;
- #5888 handoff receipt bound to the verified public ADL destination;
- repairs, residual risks, and first post-copy review date.

Every one of the 37 named non-Git platform surfaces must carry one digest-bound proof:

- `live_api`, with separate source and destination response digests;
- `operator_confirmation`, with a redacted evidence artifact and digest; or
- `not_applicable`, only when the reviewed disposition is also not applicable.

The operator confirms organization readiness on #5819 with these exact lines:

```text
WP-02-ORG-READINESS: CONFIRMED
OWNERS: CONFIRMED
BILLING: CONFIRMED
RECOVERY: CONFIRMED
ACTIONS-POLICY: CONFIRMED
PACKAGES: CONFIRMED
GITHUB-APPS: CONFIRMED
```

After each repository passes Gate 4, the operator binds the evidence chain on
#5819 with these exact lines, replacing placeholders with retained digests:

```text
WP-02-REPOSITORY: <repository-name>
ACTIONS-DISABLED: <actions-disabled-receipt-sha256>
ACTIONS-BEFORE-FIRST-PUSH: <first-push-receipt-sha256>
LFS-PARITY: <lfs-receipt-sha256>
PLATFORM-DISPOSITIONS: <platform-packet-sha256>
SOURCE-IMMUTABILITY: <source-after-manifest-sha256>
```

The two negative controls require timestamped snapshots before the first copy
window and after the fifth copy window, plus final live verification that no
destination exists.

## Sources

- GitHub, `Duplicating a repository`:
  <https://docs.github.com/en/repositories/creating-and-managing-repositories/duplicating-a-repository>
- GitHub, `About source code imports using the command line`:
  <https://docs.github.com/en/migrations/importing-source-code/using-the-command-line-to-import-source-code/about-source-code-imports-using-the-command-line>
- GitHub, `Backing up a repository`:
  <https://docs.github.com/en/repositories/archiving-a-github-repository/backing-up-a-repository>
- Current live GitHub inventory for `danielbaustin` and `agent-logic`.
- Repository operating contract: `AGENTS.md`.

## Review Boundary

Independent review must verify the copy-only safety model, exact denominator,
visibility matrix, source immutability, destination-only writes, Git/LFS parity,
non-copyable metadata dispositions, and the #5888 handoff. Review does not
authorize destination creation, website cutover, or any source mutation.
