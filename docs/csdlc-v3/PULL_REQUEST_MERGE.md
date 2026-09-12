# Native pull-request merge

Issue #844 supplies the missing merge transport between reviewed publication
and terminal observation. Only native `github-pr` owns `pull_request_merge`.
`github`, `github-issue`, `publish`, and `review` reject this mutation family.
Use the installed native v3 binary; raw `gh pr merge` remains prohibited.

## Request and execution

Run from the issue's bound worktree at the exact reviewed head, after explicit
operator authorization to merge this repository and numeric PR:

```sh
.adl/bin/native-v3/csdlc github-pr --execute --request merge-request.json
```

The JSON shape is:

```json
{
  "expected_lifecycle_digest": "<canonical authority-selector digest>",
  "exact_review_sha": "<40-character reviewed HEAD>",
  "operation": {
    "kind": "github_mutation",
    "request": {
      "repository": "agent-logic/agent-design-language",
      "issue": 844,
      "pull_request": 1234,
      "expected_head_sha": "<same reviewed HEAD>",
      "operator_approval": "<explicit authorization reference for this PR merge>",
      "credential_names": ["GITHUB_TOKEN"],
      "mutation": {
        "action": "pull_request_merge",
        "base": "main",
        "method": "merge",
        "review_receipt_path": ".csdlc/evidence/844/typed-review.json",
        "review_receipt_digest": "<typed review receipt payload digest>"
      }
    }
  }
}
```

The PR number is illustrative, not an authorized target. Resolve the actual
number, head, base and authorization before execution. The review is the existing
`csdlc.v3.typed_review_receipt.v1` contract: matching repository, issue, reviewed
revision/current head and payload digest, nonempty evidence and separate
implementer/reviewer identities. Issue #849 adds required merge linkage to that
review receipt (publication-only legacy receipts remain readable):

```json
"publication_linkage": {
  "repository": "agent-logic/agent-design-language",
  "issue": 844,
  "mode": "closing"
}
```

The issue repository is explicit even when it equals the PR repository. It may
differ for split repositories. The existing review payload digest remains
unchanged when linkage is absent; with linkage it is the stable NUL-delimited
digest of that legacy digest, issue repository, decimal issue number and mode.
Use the `operator_manual review-digest` helper; do not reuse an old digest or
invent a review. Merge refuses absent or mismatched linkage. This change does not introduce a stronger
review-signature authority. The canonical native selector/receipt and current
Git head are also checked. The approval string records authorization; it does
not create authorization on the operator's behalf.

Only explicit `method: merge` is supported. Squash and rebase fail deserialization;
there is no default or method substitution. Two ordered parents make a completed
merge's base/head identity independently inspectable. Disabled merge commits,
linear-history rules, mandatory queues, and unsupported active rule types fail
closed rather than using a bypass or silently choosing another method.

## Admission and recovery

Merge authenticates the PR body, complete `closingIssuesReferences`, and the
qualified issue URL/repository/number/state. The body must contain exactly one
canonical whole-line closing keyword (for example `Closes owner/repo#844`) or
`Part of owner/repo#844` / `Part-Of owner/repo#844`, matching the reviewed mode.
Same-repository `#844` is accepted; cross-repository short references are not.
Ambiguous, mixed, duplicate or wrong-target directives fail closed. Ordinary
prose such as “fix formatting” is not a closing directive. Closing mode requires
exactly the target issue in the complete GitHub relation set; PartOf requires
an empty closing relation set, including manually linked issues. Missing or
partial readback never substitutes for an empty relation. Both modes require
the qualified issue OPEN before dispatch.

The generated read-only `pull-request-merge-linkage` query admits only a numeric
PR and qualified issue target, never caller GraphQL. It observes the issue in
the same authenticated response using a separately qualified repository alias.

Authenticated GraphQL observation binds repository, PR URL/number, exact head,
base name and commit, non-draft OPEN state, MERGEABLE/CLEAN status, review
outcome, all review threads and latest reviews, and head-commit check results.
Legacy branch protection and active branch rulesets supply the required context
and app denominator. Every required check must actually succeed; omitted contexts,
wrong apps, skipped required checks, missing policy fields, GraphQL errors and
partial pages are rejected. A nullable review decision is accepted only when
observed policy does not require approval. Unresolved threads or changes-requested
reviews block this bounded route even if server policy would allow bypass.

This bounded implementation admits at most 100 observed check/review entries
only when `hasNextPage` is false, and fewer than 100 effective branch rules.
Larger or unavailable observations fail closed; the first page never silently
represents the complete denominator. The read-only query is generated by the
adapter, not accepted as caller GraphQL text.

A per-repository/PR file lock excludes concurrent local dispatch. A durable
target guard also binds the PR to its original attempt, so an alternate review
receipt path or other changed request cannot bypass an uncertain intent. Replay
the original request; do not remove the guard. A create-only,
fsynced intent binds request, selector, review digest, qualified publication
linkage/mode, observed policy and base
before the narrow REST `PUT /repos/{owner}/{repo}/pulls/{number}/merge` executes.
New receipt directories and every linking ancestor through Git control are
synced before dispatch. Policy and PR eligibility are read again immediately
before dispatch, and that fresh pre-state is retained. The REST
body contains only the reviewed `sha` and explicit `merge_method`. Credentials
use the shared resolver and never enter argv, receipts or logs.

GitHub's REST SHA condition protects the head, **not an atomic snapshot of body, base
or policy**. Server protections remain required. A base change after the last
read can be detected only after the irreversible operation: reconciliation
rejects any first-parent mismatch and must not claim success. This route neither
claims body/base/policy compare-and-swap nor attempts to undo a remote merge.

Every retry after durable intent creation is observation-only, including an
intent whose process died before dispatch. An uncertain or still-open remote
result stays blocked; no automatic second PUT or generic absence retry is
allowed. Do not delete or rewrite intent to bypass this boundary. A matching
already-merged PR succeeds without mutation, provided its authenticated merge
commit has exactly two ordered parents, the second is the reviewed head, and
any retained pre-state's first parent matches. A changed merge identity cannot
replace a retained reconciliation receipt. Post-merge linkage and issue state
are revalidated: Closing requires CLOSED and PartOf requires OPEN. Late body
or issue-state drift blocks success; preflight cannot prevent every remote race.
Pre-#849 intents or merge reconciliations lacking linkage cannot be silently
upgraded or replayed as new proof; preserve them and seek a reviewed recovery.

Intent and reconciliation live under resolved Git metadata at
`csdlc-v3/remote/merges/`; the common mutation receipt remains under
`csdlc-v3/remote/mutations/`. The returned `reconciliation.merge` identifies
repository, PR, exact head, base name/commit, method, resulting merge commit,
qualified `publication_linkage` and authenticated `issue_state`. This distinguishes
a PartOf checkpoint from a Closing result without granting terminal authority.
No PR body/comment marker mutation is needed to identify a merge.

## Finish and validation

After successful authenticated merge reconciliation, run the existing native
`finish --request finish-request.json --observe-github`. Finish must independently
observe the merged PR, matching head and relation, and closed issue for terminal
Closing completion (or the open parent for a PartOf checkpoint); a merge
receipt alone does not manufacture terminal truth. Cleanup remains a separate
native `clean` operation for the registered worktree. No terminal schema or
second remote mutation is added by this change.

Focused proof and the encompassing native owner checks:

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests::merge_cases -- --test-threads=1
cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands executable_merge_is_owned_only_by_github_pr
cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1
cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check
```

PVF: required native C-SDLC owner contract; deterministic fake network and local
Git/filesystem; small CPU, no paid cloud/provider workload and no live merge.
Positive proof, eligibility negatives, method/review/route rejection, lock
exclusion, before/after base drift, uncertain replay and finish observation are
covered. CLI failures retain empty stdout and structured diagnostic stderr;
success remains JSON stdout. No new compatibility logging mode is claimed.
The legacy shared `csdlc` lane runner currently selects retained v2 Gate10A;
the native v3 all-target suite above is the matching owner validation here.

API sources: [GitHub merge endpoint](https://docs.github.com/en/rest/pulls/pulls#merge-a-pull-request),
[GraphQL PR/check semantics](https://docs.github.com/en/graphql/reference/pulls),
[branch protection](https://docs.github.com/en/graphql/reference/branches), and
[effective active branch rules](https://docs.github.com/en/rest/repos/rules#get-rules-for-a-branch).

Issue #849 release-criterion correction mapping is recorded separately in
[MERGE_LINKAGE_849.md](../milestones/v0.92.2/evidence/MERGE_LINKAGE_849.md).
Earlier #835/#844 and #522/#833 failed evidence remains historical evidence.
