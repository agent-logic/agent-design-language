# Closed issue reconciliation without a PR

An issue closed as retired, duplicate, superseded, absorbed, completed coordination,
or historical disposition still requires durable closeout. These outcomes are not
merged implementation proof and must not be represented by a fabricated PR.

Use native `csdlc finish --request <request.json> --observe-github`. The request
sets `pull_request` and `mode` to null and includes `no_pr_closeout`:

```json
{
  "disposition": "retired_without_execution",
  "operator": "authorized operator identity",
  "rationale": "Issue retired before execution; scope is retained by its successor.",
  "evidence_refs": ["https://github.com/owner/repo/issues/123#issuecomment-456"],
  "expected_issue_updated_at": "2026-09-09T00:00:00Z",
  "expected_issue_closed_at": "2026-09-08T00:00:00Z"
}
```

Supported dispositions are `retired_without_execution`, `duplicate`, `superseded`,
`absorbed`, `coordination_completed`, and `historical_disposition`. The operator
must authorize the disposition based on the cited evidence. The tool authenticates
GitHub closure and exact timestamps; it does not independently evaluate whether
all implementation or coordination acceptance criteria were fulfilled. A closed
issue alone is insufficient to author the rationale.

The enclosing request retains repository, issue, exact `expected_head_sha`,
credential name, and canonical `terminal_state` output paths. The head identifies
the local reconciliation context, not a merged implementation. Fresh authenticated
GitHub observation must match the approved timestamps. Open issues, PR objects,
missing rationale/evidence/operator, mixed PR/no-PR requests, and stale snapshots
fail closed. Repeated identical reconciliation is idempotent; conflicting receipts
preserve existing state and require explicit investigation.

Receipts retain `pull_request: null` and the complete `no_pr_closeout` disposition.
Existing merged-PR receipt serialization is unchanged. Cleanup requires the same
no-PR disposition as the receipt, plus the existing exact-head, clean-worktree,
registered-path and preview guards. No-PR closeout does not authorize forced cleanup,
public launch, release approval, reopening issues, or changing old PR bodies.

A merged checkpoint whose current body says `Part-Of` remains nonterminal in the
merged-PR route. If later operator adjudication closes the issue, record that
historical disposition explicitly rather than converting the checkpoint into
implementation acceptance.

Validation: the focused `terminal_cleanup_cutover_commands` integration target
covers authenticated no-PR observation, missing/ambiguous request failures, stale
closure timestamps, idempotent persistence, conflict preservation and cleanup
identity. These are deterministic offline tooling tests; live issue reconciliation
is separate operational evidence.
