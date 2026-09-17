# Coordination-only completion

`csdlc github-issue ISSUE --operation FILE --execute` supports the explicit
`issue_complete_coordination` action. It closes a coordination umbrella with
GitHub's `completed` reason after authenticated delivery checks. It does not
change administrative `issue_close`: duplicate, superseded and no-op closure
continue to reject a `completed` reason. Ordinary implementation delivery still
uses reviewed publication, merge and terminal reconciliation.

## Declare the coordination contract

The operator must explicitly approve that this issue has **only coordination
acceptance work**, identify the complete child denominator, and review the
completion evidence. Do not infer that classification from an issue title,
label, closed child count, or an agent's report. An issue with its own unfinished
implementation requirements is ineligible. The native owner verifies this
explicit declaration and actual child deliveries; it does not interpret arbitrary
natural-language acceptance criteria or turn evidence bytes into an independent
review verdict.

Preserve the existing issue body and append exactly one standalone contract line:

```text
<!-- csdlc-coordination:v1 {"repository":"agent-logic/agent-design-language","issue":929,"kind":"coordination_only","children":[{"issue":887,"pull_request":989,"head_sha":"<exact 40-hex merged PR head>"}]} -->
```

The example child set is illustrative, not the complete #929 denominator. Supply
all actual children. The contract requires 1–100 distinct issue and PR numbers,
all in the parent repository, with exact child PR heads. Unknown contract fields,
multiple markers, wrong identities and missing children are not silently repaired.
The denominator is the explicit operator-approved contract in the live issue body;
GitHub sub-issue membership and free-form Markdown lists are not inferred authority.

For a current semantic issue, use the ordinary typed `issue_edit` operation to
append that operator-approved contract while preserving all existing text. Read
back the full resulting body, including the edit operation marker, and its current
`updated_at`. Completion preserves those bytes and appends its own operation marker.

A legacy umbrella without semantic state cannot use ordinary `issue_edit`. For
that bounded compatibility case, include `install_contract` in the completion
object. The owner authenticates the exact marker-free current body and timestamp,
validates the structured contract and every ordinary completion guard, and appends
the canonical marker after the exact authenticated body bytes in the same PATCH
that closes the issue. The complete outbound body, including the contract and
operation markers, must fit GitHub's body limit. It rejects an
existing or malformed marker, extra contract fields, wrong identity or child
set, stale pre-state, missing approval, and a changed replay. This does not enable
any other legacy issue edit or synthesize semantic lifecycle state.

## Complete, then finish

Save nonempty local review/completion evidence beneath `.csdlc/evidence/` in the
bound worktree or the resolved Git `csdlc-v3` receipt directory. Pin each file's
exact BLAKE3 digest; references must resolve within these durable repository
surfaces. There may be at most 32 files, each at most 4 MiB. Evidence must describe
the actual acceptance outcome and limitations; a digest proves byte identity,
not the truth of arbitrary claims in those bytes.

```json
{
  "action": "issue_complete_coordination",
  "operator_approval": "Explicit operator decision reference for this exact issue, child set and evidence",
  "completion": {
    "current_body": "<exact authenticated full current body, including contract>",
    "expected_updated_at": "<exact authenticated updated_at>",
    "rationale": "All declared coordination requirements and child deliveries accepted",
    "evidence": [
      {"path": ".csdlc/evidence/929/completion-review.json", "digest": "<64-hex BLAKE3>"}
    ]
  }
}
```

For a marker-free legacy umbrella, the completion object additionally contains:

```json
"install_contract": {
  "repository": "agent-logic/agent-design-language",
  "issue": 929,
  "kind": "coordination_only",
  "children": [
    {"issue": 887, "pull_request": 989, "head_sha": "<exact 40-hex merged PR head>"}
  ]
}
```

`current_body` remains the exact authenticated pre-mutation body and therefore
does not contain the marker in this form. Existing marked umbrellas must omit
`install_contract`.

Before dispatch the owner verifies the exact open parent issue, body and timestamp;
each child must be authentically `closed` with an explicit `completed` reason.
Authenticated PR linkage must identify the same repository, PR and approved head,
report a merged commit, and contain exactly the corresponding closing issue link.
A child body may also contain non-closing parent references, for example
`Closes #887` followed by `Part of #505`. These references do not replace the
child's required closing directive or authorize any extra closing issue. The
pre-merge reviewed-publication linkage policy remains unchanged.
Incomplete linkage pagination, partial GraphQL errors, unmerged PRs and stale or
missing evidence fail closed. The parent snapshot and evidence bytes are rechecked
after child observations. GitHub does not offer a transaction across these reads
and the final PATCH; these are immediate authenticated observations, not a claim
of atomic multi-issue locking.

The owner retains a digest-bound readiness receipt under resolved Git metadata
at `csdlc-v3/coordination-readiness/`, then uses the existing immutable mutation
intent and authenticated reconciliation machinery. Reconciliation requires exact
`closed`/`completed`, issue identity, preserved body and operation marker. An
identical successful replay does not issue a second PATCH. Explicit recovery
after authenticated absence runs the full readiness guard again before any retry.

After successful closure, run ordinary native finish with a disposition file:

```json
{
  "disposition": "coordination_completed",
  "operator": "<operator decision identity>",
  "rationale": "<accepted coordination outcome and limitations>",
  "evidence_refs": [".csdlc/evidence/929/completion-review.json"]
}
```

```sh
csdlc finish 929 --disposition coordination-disposition.json
```

Finish performs its own authenticated terminal observation. Then preview and run
native `clean` separately. Completion does not itself finish, clean, publish,
merge, or rewrite cards after closure.

## Validation classification

The new owner tests in `commands/remote/coordination/tests.rs` and installed
`installed_coordination_completion` target for #1006 and #1061 are PVF **tooling**, deterministic
synthetic-transport integration proof, CPU/local-filesystem only, no live GitHub
writes or provider calls. They are required issue proof, not independent evidence
that any real coordination issue has completed. Installed tests copy the actual
candidate binary outside Cargo output and execute the ordinary command path.
Existing administrative closure and no-PR finish regressions remain required.
