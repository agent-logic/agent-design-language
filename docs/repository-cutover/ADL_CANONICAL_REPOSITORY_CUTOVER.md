# ADL Canonical Repository Cutover

This runbook promotes `agent-logic/agent-design-language` to canonical code and
push authority without deleting, transferring, privatizing, or rewriting
`danielbaustin/agent-design-language`. The legacy repository remains public and
retains historical issues, pull requests, releases, and active v0.92 issue
authority until those issues close.

## Current Decision

The repository copy is complete and exact at the recorded common revision, but
activation must wait until Sprint 1 is terminal. Current C-SDLC issue records
and publication contracts name the legacy repository; replacing `origin` first
would strand active work. This is an ordering gate, not a request for tooling
changes or lifecycle-record rewrites.

The retained inventory is **provisional** while Sprint 1 is active. Immediately
before activation, refresh the issue, pull-request, worktree, ref, automation,
integration, and current-reference manifests and rerun the focused static and
live verifiers. Snapshot counts in the present packet must not authorize the
cutover.

`asksifu` and `Horust` are excluded. They remain unchanged in the personal
account and must not be copied or created in the Agent Logic organization.

## Pre-Activation Gates

1. Finish Sprint 1 on `danielbaustin/agent-design-language`, including WP-02A.
2. Reconfirm exact source/destination branch and tag parity.
3. Confirm zero open source pull requests and disposition every open issue.
4. Preserve all worktrees, detached heads, dirty states, and local branches.
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
7. Verify authenticated canonical push authority, workflow status, badge links,
   active issue continuity, every registered worktree, and the rollback path.

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

## Compact Disposition Manifests

The main runbook stays bounded. Every snapshot entry has an explicit
disposition in these redacted manifests:

- Active issues: `.csdlc/evidence/5891/active-issue-dispositions.tsv`
- Active pull requests: `.csdlc/evidence/5891/active-pull-request-dispositions.tsv`
- Worktrees: `.csdlc/evidence/5891/worktrees.txt`
- Automations: `.csdlc/evidence/5891/automation-dispositions.tsv`
- Integrations: `.csdlc/evidence/5891/integration-dispositions.tsv`
- Current and preserved references: `.csdlc/evidence/5891/reference-dispositions.tsv`

Worktree rows intentionally retain only an opaque identifier, exact HEAD,
branch-or-detached mode, dirty state, and disposition. Machine-local paths are
not durable evidence.

## Rollback

Rollback is non-destructive. Restore the recorded personal URL as `origin` and
retain the Agent Logic URL under a noncanonical remote name. Never delete refs,
force-push, rewrite history, remove worktrees, or delete either repository.
Published content is reversed only through a separately reviewed pull request.

The complete redacted evidence and dispositions are recorded in
`ADL_CANONICAL_REPOSITORY_CUTOVER_INVENTORY.json` and `.csdlc/evidence/5891/`.
