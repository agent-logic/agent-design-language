---
name: csdlc-v2-init
description: Create all six C-SDLC v2 cards and canonical issue state from typed input.
---
Invoke `csdlc-issue --root <repo> create --request <json>`. Do not edit
Markdown/state, invoke shell/Python lifecycle logic, bind a worktree, or infer
success from prose. The installed v2 generation remains the sole operational
authority.

## Pre-`code_repository` bound records

When a legacy `bound`, `implemented`, or `reviewed` record has no
`code_repository`, stop before publication and preserve its worktree. Do not
hand-edit `index.json` or cards, retarget remotes, or bypass repository checks.

After the issue worktree is clean and its registered branch, worktree, and all
effective `origin` fetch/push URLs identify the intended GitHub code repository,
run:

```text
csdlc-issue --root <bound-worktree> migrate-code-repository --request <json>
```

Use the request schema reported by `csdlc-edit schema`. It requires the issue,
exact `owner/repository` identity, current generation and digest, actor, and
reason. Stop on stale CAS, dirty state, missing or ambiguous topology, wrong
origin identity, unsupported phase, or any existing `code_repository`.
Successful migration preserves lifecycle and review truth; it does not grant
publication authority, so the normal `csdlc-publish` checks still apply.

## Bound issue identity/repository recovery

If an active bound lifecycle record was created against the wrong issue identity
and a canonical current-repo issue has been created through `csdlc-github-issue`,
do not hand-edit `.csdlc/issues/*`. Use the typed identity migration owner:

```text
csdlc-issue --root <bound-worktree> migrate-bound-issue-identity --request <json>
```

The request must bind the exact source generation/digest, source and target
issue/repository identities, actor/reason, and repo-local target issue evidence.
The operation refuses terminal records, conflicting target namespaces, stale
source truth, and dirty topology. Published or merge-ready records have
publication/readiness cleared so `csdlc-publish` must relink the PR to the
canonical target issue before `csdlc-finish`.
