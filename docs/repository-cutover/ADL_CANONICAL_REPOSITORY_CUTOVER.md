# ADL Canonical Repository Cutover

This runbook promotes `agent-logic/agent-design-language` to canonical code and
push authority without deleting, transferring, privatizing, or rewriting
`danielbaustin/agent-design-language`. The legacy repository remains public and
retains historical issues, pull requests, releases, and active v0.92 issue
authority until those issues close.

## Current Decision

The repository copy is complete and exact at the recorded common revision.
Activation proceeds after WP-02A and the Freedom Gate quality prerequisite are
terminal. Current C-SDLC issue records and publication contracts continue to
name the legacy issue tracker while code and push authority move to the Agent
Logic repository, so active work remains reachable without lifecycle-record
rewrites.

The retained inventory is **provisional** until activation. Immediately before
activation, refresh the issue, pull-request, worktree, ref, automation,
integration, and current-reference manifests and rerun the focused static and
live verifiers. Snapshot counts alone must not authorize the cutover.

`asksifu` and `Horust` are excluded. They remain unchanged in the personal
account and must not be copied or created in the Agent Logic organization.

## Pre-Activation Gates

1. Confirm WP-02A and the Freedom Gate quality prerequisite are terminal.
2. Reconfirm exact source/destination full branch and tag manifests, including
   annotated-tag peeled refs.
3. Confirm zero open source pull requests and disposition every open issue.
4. Preserve every registered worktree, branch or detached binding, and all
   existing work. Active sessions may advance their own HEAD or dirty state.
5. Recreate reviewed destination variables and the `adl-spot-ci` environment.
6. Keep secret-, OIDC-, CodeBuild-, package-, App-, and runner-dependent lanes
   disabled until their authority is independently proven.
7. Prevent simultaneous scheduled workflows across both repositories.
8. Enable destination Actions, prove the canonical checks, then recreate branch
   protection using checks that have actually reported on the destination.

## Activation Window

1. Pause new issue execution and record a final ref and active-work inventory.
2. Push the independently reviewed canonical-reference change to the Agent
   Logic repository and prove its required checks.
3. Merge the destination pull request.
4. Rename the existing shared `origin` remote to `legacy-origin` and add
   `https://github.com/agent-logic/agent-design-language.git` as the new
   `origin`. Do not recreate the checkout or any worktree.
5. Retarget only `main` initially. Existing branches retain their recorded
   legacy relationship until intentionally reconciled.
6. Open and merge a source-only README notice pointing contributors to the
   canonical repository. No other personal-repository mutation is authorized.
7. Verify authenticated canonical push authority, workflow status, both README
   badge surfaces, active issue continuity, every registered worktree and its
   branch or detached binding, and the rollback path through a non-destructive
   local remote-renaming drill.
8. Generate distinct post-cutover full-ref manifests for both repositories,
   review those terminal manifests independently, and only then close #5891.

The source notice must be this exact one-line paragraph prepended to the
unchanged pre-cutover README:

```text
> **Canonical development has moved:** New code, branches, and pull requests belong in [agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language).
```

Existing issues are closed from canonical pull requests only with qualified
references such as `Closes danielbaustin/agent-design-language#5801`. Historical
links remain on the legacy repository because that history was not copied.

## Integration Disposition

- Core CI: activate and prove on the destination before canonical protection.
- Scheduled CI: keep one authority at a time; never run duplicate schedules.
- Codecov: prove canonical upload and badge state before making it required.
- AWS OIDC and CodeBuild: update the Agent Logic business-account trust and
  source URLs separately; dependent workflows fail closed until verified.
- Repository secrets: recreate out of band by name; never print or retain values.
- Packages, organization runners, and GitHub Apps: activation stop condition
  until inventoried with sufficient read-only authority or proven unused.

The post verifier derives canonical integration states from live APIs and the
retained operator-authenticated GitHub UI inventory rather than accepting
free-form dispositions: `N_names` for variables and secrets, `present` for the
environment, disabled dependent AWS workflows, the observed Codecov badge
state, and exact package, organization-runner, GitHub-App, and webhook counts.
The final manifest must record those exact observed tokens and receive
independent review. Package and runner counts may use the retained signed-in UI
evidence when the CLI token lacks the two read-only organization scopes.

## Compact Disposition Manifests

The main runbook stays bounded. Every snapshot entry has an explicit
disposition in these redacted manifests:

- Active issues: `.csdlc/evidence/5891/active-issue-dispositions.tsv`
- Active pull requests: `.csdlc/evidence/5891/active-pull-request-dispositions.tsv`
- Worktrees: `.csdlc/evidence/5891/worktrees.txt`
- Automations: `.csdlc/evidence/5891/automation-dispositions.tsv`
- Integrations: `.csdlc/evidence/5891/integration-dispositions.tsv`
- Current and preserved references: `.csdlc/evidence/5891/reference-dispositions.tsv`

Worktree rows intentionally retain only an opaque identifier, snapshot HEAD,
branch-or-detached mode, snapshot dirty state, and disposition. Activation
requires registration and branch-or-detached continuity; active sessions may
advance their own HEAD or dirty state. Machine-local paths are not durable
evidence.

## Rollback

Rollback is non-destructive. Restore the recorded personal URL as `origin` and
retain the Agent Logic URL under a noncanonical remote name. Never delete refs,
force-push, rewrite history, remove worktrees, or delete either repository.
Published content is reversed only through a separately reviewed pull request.

The complete redacted evidence and dispositions are recorded in
`ADL_CANONICAL_REPOSITORY_CUTOVER_INVENTORY.json` and `.csdlc/evidence/5891/`.
