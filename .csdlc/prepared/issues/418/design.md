# Issue #418 Design — Audited `gh` Break-Glass Policy

## Decision

Typed C-SDLC v2 remains the sole default lifecycle authority. A raw `gh`
write is permitted only as a narrow, operator-authorized emergency transport
when a confirmed and durably tracked regression in the typed owner blocks the
same remote operation.

## Preconditions

All conditions are mandatory:

1. The applicable typed owner was invoked against the exact repository and
   target and failed reproducibly.
2. A durable tooling-regression issue records safe reproduction evidence.
3. The operator explicitly authorizes one repository, target, and command
   class after seeing the blocker.
4. The bound worktree, branch, exact HEAD, typed generation/digest, and remote
   pre-state are captured before mutation.
5. The requested command is in the allowlist and uses the smallest possible
   target-specific arguments.

## Allowlist

The exception may transport only these canonical argv shapes, with one
separate authorization per invocation:

- `gh issue comment <number> --repo <owner/name> --body-file <safe-relative-path>`
- `gh issue edit <number> --repo <owner/name> --body-file <safe-relative-path>`
- `gh pr create --repo <owner/name> --base <branch> --head <branch> --title <text> --body-file <safe-relative-path>` with an optional final `--draft`
- `gh pr edit <number> --repo <owner/name> --body-file <safe-relative-path>`
- `gh pr ready <number> --repo <owner/name>`
- `gh pr comment <number> --repo <owner/name> --body-file <safe-relative-path>`

The repository and numeric target must be explicit where a target already
exists. PR creation instead binds the exact branch and HEAD plus a remote
pre-state proving no PR exists for that head. Body content is supplied only by
a mode-0600 file inside the invocation's local receipt directory; it never
appears in argv. Reads remain unrestricted by this policy.

No other flags, reordered or repeated flags, aliases, extensions, `gh api`,
stdin body input, absolute/traversing body paths, bulk selectors, or shell
expansion are allowed. In particular, `issue edit --state`, `pr edit --base`,
label/milestone/project/assignee/reviewer mutations, and target aliases are
denied.

Merge, issue close, finish, cleanup, deletion, release publication,
administration, secrets/variables, workflow mutation or dispatch, force
operations, and broad/bulk mutations are denied. They remain denied unless a
later reviewed policy change names a safely reconcilable operation; ad-hoc
operator wording does not silently widen this policy.

## Receipt

Before the command, create a new `intent.json` beneath a unique invocation
directory in `.git/csdlc-v2/break-glass/`. After it returns, create a separate
`result.json` referring to the intent digest. After typed reconciliation,
create a third `reconciliation.json` that binds the intent and result digests,
exact reconciled typed generation/digest, exact remote post-state,
reconciliation owner/operation/result, and timestamp. Use create-new semantics;
never overwrite any event file. The three records collectively contain:

- UTC timestamp, actor, regression issue, and authorization reference;
- repository, issue or PR, operation class, redacted argv, and remote pre-state;
- bound worktree, branch, exact HEAD, typed generation and digest;
- exit classification and redacted remote result identity;
- a declaration that no credential, token, sensitive body, or response payload
  was retained; and
- the required typed reconciliation operation and eventual status.

The token value, token-file contents, environment dump, issue/PR bodies that
may contain sensitive content, and raw response bodies are never recorded.

## Reconciliation

The raw mutation is transport evidence, not typed lifecycle authority. The
affected issue is frozen from any later readiness, review, publication,
merge-ready, terminal, or finish claim until a supported typed observation or
recovery operation reconciles exact remote identity and state into the issue
record and the create-only reconciliation event records success. If
reconciliation is unavailable, the lifecycle remains blocked and a tooling
repair is required.

## Failure Policy

Fail closed when any precondition, allowlist match, receipt, redaction, exact
identity, or reconciliation route is missing. Never retry with broader raw
GitHub authority.
