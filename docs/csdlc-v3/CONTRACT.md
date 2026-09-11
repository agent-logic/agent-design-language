# C-SDLC v3 Contract — V3-A

Status: accepted construction history plus the post-cutover operational contract.

This contract began as the C-SDLC v3 construction boundary accepted by V3-A.
That pre-cutover boundary remains historical evidence. PR #591 merged the
operator-reviewed V3-F/#505 decision on 2026-09-07; v3 is the operational authority after the merged V3-F cutover when its canonical native selector,
receipt, terminal reconciliation, and Git-object checks all validate. Missing or
stale proof suspends authority rather than falling back to v2.

## Authority and compatibility

- v2 was the sole operational authority throughout V3-A through V3-E.
- v3 is the post-cutover operational authority; v2 is retained only for an
  explicitly authorized rollback or bounded transition remediation.
- v3 artifacts created by V3-A are design, schema, and construction-decision
  inputs only.
- v3 cannot bind worktrees, mutate issue state, publish pull requests, finish
  issues, or clean worktrees during V3-A.
- v3 compatibility means “can represent and check retained v2 invariants,” not
  “may bypass v2.”
- Unsupported or not-yet-proven behavior fails closed instead of falling back
  to v1 wrappers, raw GitHub mutation, hand-edited cards, or implicit local
  state.

## Retained predecessor contract

V3-A retains three predecessor inputs:

- #161 freezes the product contract, command tree, state/output shapes,
  capability matrix, reviewer-independence, publication linkage, finish,
  cleanup, migration, supported-platform, output-filtering, schema-evolution,
  and state-size constraints.
- #162 contributes measured Rust construction evidence, dependency limits,
  layer boundaries, parser/template/GitHub-client findings, in-process output
  filtering, commit-primitive recommendations, and the promote-or-discard
  disposition.
- #163 supplies the operator-approved platform commit matrix, durability
  posture, Windows fail-closed/read-only posture where mutation is not proven,
  and rollback implications required before transaction storage work.

Every retained requirement from #161 through #163 must have exactly one
disposition in `predecessor-coverage.json`: `retained`, `deferred`, or
rejected with reason. A missing or duplicated predecessor row blocks V3-B.

## Construction decision

V3 construction starts with one small Rust crate boundary:

- `csdlc-v3/src/lib.rs` exposes static contract-denominator checks only.
- The crate may validate contract shape, predecessor coverage, architecture
  boundaries, and proportional-lifecycle decisions.
- The crate must not call v2 owner binaries, GitHub APIs, filesystem mutation
  commands, process-control commands, provider APIs, or shell commands.
- Later implementation issues may add command, repository, lifecycle, storage,
  adapter, review, publication, and parity modules only inside their declared
  issue ownership.

The default construction posture is boring and explicit: typed inputs,
deterministic checks, no ambient repository authority, and no hidden process
state.

V3-A does not record satisfied #162 measurement evidence. The ten frozen
#162 thresholds and their absent observations are recorded in
`construction-decision.json` against the expected evidence artifact
`.csdlc/evidence/162/proof.json`; because that exact-revision evidence is
missing, the measured construction slice is not promoted. The only accepted
construction seed is the static one-binary/one-library, four-layer Rust shape
plus deterministic contract parsing that does not initialize repository,
credential, network, or child-task state. Any future measured value that
exceeds the recorded dependency, scope, or execution-threshold criteria remains
a stop-and-revise condition for later v3 issues rather than implicit approval
to widen the crate.

In short: exact-revision evidence is missing for the #162 measurements, so
V3-A records a fail-closed non-promotion rather than a construction go signal.

Decision evidence is bound to #163 and Decision 11: v3 mutation authority may
not proceed until the approved platform commit matrix, rollback posture, and
Windows fail-closed/read-only policy are satisfied by the relevant owner issue.
That means the V3-A crate can be used only as static construction evidence.
V3-F/#505 must still make the explicit operator-reviewed promote-or-discard
decision for live authority using actual #162 measurements, their exact
revision, and #163 approval evidence.

## Proportional lifecycle contract

C-SDLC v3 must simplify the lifecycle itself, not merely automate the v2
ceremony. The default path keeps only gates that materially reduce delivery
risk and names the concrete hazard for every retained gate.

Default V3 path. The normative default retains the safety-critical bind,
publication, finish, and cleanup transitions from the retained-gate matrix:

1. one meaningful design gate;
2. bind to exact repository, branch, and worktree identity;
3. focused validation proportional to the changed surface;
4. one independent implementation review at exact revision;
5. publication with exact GitHub repository, base, head, and linkage truth;
6. finish from live GitHub merge and issue-closure evidence;
7. cleanup of only the exact registered worktree after terminal evidence.

Intermediate projections, repeated generation/digest handoffs, duplicate
readiness reviews, and umbrella reviews that merely repeat child proof are
derived, collapsed, or removed by default. Additional gates are risk-triggered:
each retained gate must name a concrete hazard, not process completeness alone.

A routine three-issue sprint must be mechanically prepared and made ready in
three minutes or less, not hours. Hand-authored lifecycle JSON and repeated
digest choreography are not acceptable as the default operator experience.

## Simple issue operations

For ordinary issue creation, use the GitHub-like form:

```sh
csdlc github-issue create \
  --repo agent-logic/agent-design-language \
  --title "One bounded outcome" \
  --body-file issue.md \
  --label type:task \
  --milestone 1 \
  --expected-head <exact-reviewed-40-hex-sha> \
  --execute
```

`--body` may replace `--body-file`; using both or neither fails before any
remote action. `--label` and `--assignee` are repeatable. The credential name
defaults to `GITHUB_TOKEN` and may be selected with `--credential-name`; a
credential value is never accepted as an argument.

The simple form builds the same typed operational dispatch used by the
request-file interface. It therefore retains the canonical v3 authority gate,
durable intent and operation marker, authenticated readback, assigned issue
number, mutation receipt, and idempotent reconciliation. Omitting `--execute`
prints the typed request without mutation. Execution fails closed unless the
repository's canonical selector grants v3 operational authority at the exact
reviewed head.

The request-file form remains the advanced and audit-oriented interface:

```sh
csdlc github-issue --request issue-create-dispatch.json --execute
```

For explicit duplicate, superseded, or no-op issue closure, use:

```sh
csdlc github-issue close \
  --repo agent-logic/agent-design-language \
  --issue 792 \
  --disposition duplicate \
  --duplicate-of 791 \
  --rationale "accidental retry duplicate of #791" \
  --body-file issue-792-current-body.md \
  --expected-head <exact-reviewed-40-hex-sha> \
  --execute
```

`--disposition` accepts `duplicate`, `superseded`, or `no-op`. Duplicate
closure requires `--duplicate-of`; all closure requires a non-empty rationale.
`--body` or `--body-file` must provide the authenticated current issue body so
the route can append close truth without clobbering existing issue provenance.
The typed close route patches the issue to GitHub `state=closed` with
`state_reason=not_planned`, appends the C-SDLC operation marker and close
section to the issue body, and then authenticates readback of the same closed
issue before recording a durable receipt. It intentionally rejects `completed`
state reasons so this route cannot masquerade as implementation completion;
merged implementation finish remains owned by the `finish` route.

## Rollback and fail-closed behavior

- v2 is the retained rollback target after V3-F; rollback requires explicit
  operator authorization and changes the native selector to `rollback`.
- If native authority proof is missing, stale, or malformed, v3 reports
  suspended authority without silently activating v2.
- Rollback must preserve exact revision identity, audit provenance, publication
  linkage, terminal finish truth, and cleanup safety.
- macOS and Linux are the required #505 cutover platforms. Windows operational
  portability is deferred to #710 and is not part of the #505 proof denominator;
  unsupported Windows mutation remains fail-closed where the current binary
  exposes a mutation fence.
- Any unsupported platform, unsupported output filter, missing predecessor
  disposition, or ambiguous authority boundary blocks later execution rather
  than silently weakening the contract.

## Review boundary

Review of this packet should ask only whether V3-A establishes a complete,
reviewable construction contract for #500. It must not approve V3 operational
cutover, V3-B/V3-C implementation, v2 retirement, or broad repository cleanup.

## Existing-issue metadata updates (#797)

Use `csdlc github-issue --request issue-edit-dispatch.json --execute` with the
usual canonical selector digest and exact-head operational envelope. Its
`operation.kind` is `github_mutation`; `operation.request` identifies the exact
repository, issue, expected head and credential name. The nested `mutation`
uses [the issue-edit schema](issue-edit.schema.json), for example:

```json
{
  "action": "issue_edit",
  "labels": {"operation": "add", "names": ["version:v0.92.1"]},
  "milestone": {"operation": "set", "number": 1}
}
```

- Omitted title, labels, assignees or milestone produce no corresponding PATCH
  field. Omitted body is fetched through authenticated readback and preserved,
  with the durable operation marker appended. Empty body explicitly clears its
  prose while retaining that marker.
- Labels support `add`, `remove` and `replace`. `replace` with `names: []`
  clears labels. Add/remove retain unrelated labels; name comparison is
  case-insensitive. Assignees accept an exact replacement list; `[]` clears it.
- Milestone `set` requires a positive numeric milestone number. `clear` emits
  an explicit API null. Bare null metadata and unknown operation fields are
  rejected; omission is never interpreted as a clear.
- Label deltas and an omitted body are resolved once from authenticated issue
  state before the durable intent is written. The intent retains both the
  original operation identity and the resolved target; its digest binds both.
  Readback requires exact requested label/assignee sets, milestone number/null,
  title when requested, and body including the operation marker. Missing or
  malformed readback fields are not evidence of a clear.
- After an uncertain response, rerun the identical request in the same bound
  checkout to reconcile read-only. Issue edits do not support automatic or
  `retry_after_authenticated_absence` write replay: a mismatch may represent
  later human changes. Retained intent remains for inspection; a separately
  reviewed new request is necessary if the original write did not apply.
  Successful receipt replay also rechecks the authenticated exact result.

PVF: the `issue_metadata_*` tests are required native-owner contract proof:
small deterministic local CPU/filesystem/Git fixtures and fake transports;
no live GitHub writes. Existing selector, credentials, durable input permissions
and stdout/stderr/redaction behavior remain unchanged. Tests cover request and
PATCH shape, exact readback, retained retry targets and drift rejection.

### Retained pull-request-ready intent recovery

A legacy ready intent may lack `resolved_ready_target`. An explicit
`retry_after_authenticated_absence` request first performs exact authenticated
readback. Only a draft result can enter recovery. The missing target is then
resolved through authenticated PR-number, node-ID, head and draft-state checks.
The original intent bytes and digest remain unchanged; the create-only recovery
receipt retains `resolved_ready_target` before mutation dispatch. This optional
receipt field preserves compatibility with older records.

If resolution observes an already-ready PR, exact read-only reconciliation
finishes the operation without consuming a mutation retry. A dispatched retry
consumes the existing one-shot recovery allowance even when transport or
readback is uncertain. A later invocation may reconcile a completed operation,
but cannot dispatch another recovery mutation. Invalid or unavailable target
resolution fails before mutation and does not consume the recovery allowance.

PVF: `legacy_ready_intent_*` are required native-owner recovery regression
checks using deterministic local Git/filesystem fixtures and fake transport.
They prove preservation of the original intent, target-bearing receipt before
dispatch, no-authorization and identity/lookup failures, already-ready races,
one-shot uncertain recovery, reconciliation replay and private input cleanup.
No live GitHub mutation or logging-channel change is involved.
