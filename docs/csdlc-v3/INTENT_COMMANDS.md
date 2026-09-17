# Issue intent commands

This page describes the current issue interface in
`csdlc-v3/src/application/intent/`. Exact-head review, complete required
validation, installation provenance, and release acceptance remain separate
evidence. Check the selected executable's help and descriptors before using
these forms.

The native command owners remain responsible for lifecycle writes,
authenticated remote operations, proof, terminal records and cleanup behind
the semantic transaction owner. Direct writer `--request` forms for `issue`,
`bind`, `edit`, `proof`, `github`, `github-issue`, `github-pr`, `review`,
`publish`, `finish`, and `clean` are retired. Their command names remain
available through `--help` and `--describe` for migration discovery. Advanced
execution uses an exact generated `--intent-request`. Explicit `local` remains
a historical, non-operational route.

## Ordinary operation

Use a positive issue number. The interface resolves the repository, registered
issue checkout, current template registry, native issue generation/digest,
branch, HEAD and canonical authority. Primary preparation uses resolved Git
metadata; issue execution uses the registered bound checkout. A missing or
ambiguous identity fails closed. `--repo-root ROOT` selects repository discovery;
it does not override a bound issue's ownership.

| Intent | Ordinary form | Owner effect |
| --- | --- | --- |
| Status | `csdlc status ISSUE [--decisions DECISIONS.json]` | Observe doctor/readiness and proof currency; do not run validators. |
| Prepare | `csdlc prepare ISSUE --plan PLAN.json` | Authenticate the existing source issue and initialize its six-card plan through the local owner. |
| Bind | `csdlc bind ISSUE` | Bind the prepared issue through the registered worktree owner. |
| Edit | `csdlc edit ISSUE --changes CHANGES.json` | Apply semantic card values through the typed editor and its transaction guards. |
| Rebuild | `csdlc rebuild ISSUE` | Explicitly regenerate all six card projections from the current semantic record and active registry. |
| Validate | `csdlc validate ISSUE` | Inspect native card/schema validation without mutation. |
| Proof | `csdlc proof ISSUE` | Run the declared admitted validators and retain their actual outcomes. |
| Review | `csdlc review ISSUE --evidence REVIEW.json` | Validate and retain externally supplied independent exact-head review. |
| Publish | `csdlc publish ISSUE` | Create or update the canonical PR through the authenticated remote owner and observe publication admission. |
| Finish | `csdlc finish ISSUE [--disposition DISPOSITION.json]` | Observe merged delivery or explicit no-PR disposition, then persist native terminal records. |
| Clean | `csdlc clean ISSUE` | Preview the exact terminal worktree; removal requires the returned token and `--execute`. |
| Recover | `csdlc recover ISSUE [--disposition DISPOSITION.json]` | Inspect one retained local transaction or remote operation; execution requires the exact returned digest. An indeterminate proof attempt also requires the typed abandonment disposition returned by preview. |

`--json` is accepted on ordinary intent forms; machine output is already JSON.
It is not a global option for every retained command. `--help` and `--describe`
remain discovery, not execution. No ordinary command requires an operator to
assemble internal registry, registration, adapter or lifecycle receipt inputs.
External plan, edits, independent review and explicit operation content remain
intentional user inputs.

`status` and `validate` only diagnose semantic projection health. They report
healthy, missing, altered or interrupted projections without repairing files or
advancing semantic state. `rebuild` is the separate guarded mutation. It derives
all six values and rendered cards from the current semantic record and active
registry, writes the projection manifest last, and acknowledges the resulting
projection through the transaction owner. Repeating it with unchanged inputs is
an expected no-op with identical bytes and digests. Rebuild preserves evidence
and lifecycle facts; it cannot create proof, review, publication, approval or
terminal truth. Pending semantic work, a stale semantic request version,
corrupted retained state, or a mismatched bound checkout fails closed and must
use the explicit recovery path where applicable.

## Administrative authority operations

The guarded administrative owners also use the semantic issue interface:

| Intent | Preview form | Executing form |
| --- | --- | --- |
| Install | `csdlc install ISSUE --operation OPERATION.json` | `csdlc install ISSUE --operation OPERATION.json --execute` |
| Cutover | `csdlc cutover 505 --operation OPERATION.json` | `csdlc cutover 505 --operation OPERATION.json --execute` |
| Rollback | `csdlc rollback 505 --operation OPERATION.json` | `csdlc rollback 505 --operation OPERATION.json --execute` |

These routes are for explicit installation or authority administration, not
ordinary issue lifecycle work. `install` accepts an `InstallPlanInput` as its
operation content. `cutover` and `rollback` accept a complete
`TerminalRouteRequest`; the nested cutover operation, repository, issue,
primary repository root and execute value must match the semantic invocation.
Cutover and rollback remain restricted to issue 505 and require their existing
approval and proof inputs. Omitting `--execute` is read-only preview; adding it
requests the guarded effect. The legacy administrative `--request` and
`--observe-github` forms remain visible only for migration discovery and are
retired for execution. Exact generated `--intent-request` snapshots are the
advanced interface.

Status runs the native doctor, eligibility, schedule and shepherd observation
owners. It does not infer dependency completion, budget acceptance or permission
to start a later sprint child. Optional `DECISIONS.json` is a strict object with
optional `design_ready`, `dependencies_ready` and `budget_available` booleans,
plus `retryable_failure` and `operator_decision_needed` booleans defaulting to
false. Absent design/dependency/budget decisions remain explicitly required.
These are caller-supplied operator decisions, not observed dependency execution
proof. A `ready` status is not execution or terminal delivery.

## Plan and edit content

`PLAN.json` is a strict `csdlc.v3.intent_plan.v1` object with:

- `schema` and a lowercase, hyphenated `slug` of at most 100 characters;
- `cards`, containing exactly `sip`, `stp`, `spp`, `vpp`, `srp` and `sor` values
  objects prepared from the active versioned templates;
- `validators`, an array of `{id, program, args, success_marker}` declarations
  with optional `timeout_seconds` (default 300, admitted range 1 through 300);
- `publication`, containing `{base, title, body, draft}`.

Issue identity, authenticated title, repository, branch and worktree are resolved
by the owner. Card prose cannot override that identity. Publication must carry
`Closes #ISSUE` for the tracked issue without closing unrelated issues. The
current intent publication contract uses closing mode; it does not silently
convert a checkpoint into terminal delivery.

A bounded validator declaration has this shape:

```json
{
  "id": "owner-contract",
  "program": "cargo",
  "args": ["test", "--offline", "--manifest-path", "csdlc-v3/Cargo.toml", "--test", "command_manifest"],
  "success_marker": "test result: ok."
}
```

The proof owner admits bounded Cargo test arguments, verifies issue/worktree
ownership and requires successful output with a nonzero passed-test count.
An empty validator set, an exit-zero marker-only process or stale candidate
bytes does not establish proof. Cargo targets selected by the validator and
declaring `harness = false` are not admitted because their arbitrary stdout
cannot establish Rust test-harness execution. The owner bounds output and runtime and records actual results. Each validator
runs in its own Unix process group. Nonblocking output capture remains bounded
when descendants retain output handles after Cargo exits. Timeout or scoped
SIGINT/SIGTERM cancellation terminates that owned group, with at most two seconds
of additional reap/drain polling. Records distinguish `timed_out`, `cancelled`
and `cleanup_complete`; elapsed time includes supervision and cleanup. Previous
signal handlers are restored after the serialized validator invocation.
Non-Unix validator execution is explicitly unsupported. Current proof refuses tracked changes and unsupported untracked
candidate inputs, including ignored files outside native `.csdlc/` artifacts and
the explicit disposable cache roots `target/`, `csdlc-v3/target/`, and
`adl/target/`; commit the candidate before its proving run. Those artifact
exemptions do not admit executable source: manifests, declared target files and
repository path dependencies must be tracked. Ancestor workspace manifests, inherited
dependencies and patch paths receive the same admission. External local or unknown
compiler packages are refused; only Cargo-identified external registry packages
remain within the disclosed cached-dependency limitation. Compiler dependency records for
repository crates also reject untracked `include!`, `include_bytes!`, `include_str!`
and module-path inputs, including inputs beneath exempt directories. Unsupported
generated repository source is refused rather than counted as tracked proof.
Repository tracked bytes and declared validators enter the input digest. Cargo still uses the
operator environment's PATH and HOME: ambient HOME Cargo configuration, external
toolchain binaries and cached dependencies are not fully fingerprinted by this
receipt, so it does not establish a hermetic build. See
`csdlc-v3/src/commands/proof/intent.rs` for the exact admission policy.

Validator records retain exact captured-stream digests and bounded diagnostic
excerpts. Excerpts replace repository/HOME paths, omit sensitive-key lines and
are explicitly non-lossless. Input drift or a later execution failure retains
the earlier actual outcomes in a failed receipt. If the native evidence guard
refuses persistence, the result reports `evidence_persisted: false` and the
execution evidence on stdout; it does not bypass the guard or establish proof.

`CHANGES.json` uses this strict shape; values remain subject to native card
validation:

```json
{
  "schema": "csdlc.v3.intent_changes.v1",
  "amendment": {
    "class": "scope_acceptance",
    "transition_approved": true
  },
  "cards": {
    "sor": {"status": "IN_PROGRESS"}
  }
}
```

For card edits, the amendment declaration is mandatory and retained with the native effect.
`binding` is reserved for the topology-verified bind owner. An implementation
amendment also supplies the exact 40-character `implementation_revision`; the
owner requires it to match the bound effect head. `new_commit` records whether
the change itself requires renewed exact-head review currency.

Use the applicable card-editor skill to choose truthful fields. The command
adapter does not authorize arbitrary handwritten card structure. Unsupported
preview or execute flags on local preparation, editing, validation and proof
are rejected before dispatch.

Publication metadata uses a separate single-surface edit:

```json
{"schema":"csdlc.v3.intent_changes.v1","publication":{"base":"main","title":"Corrected title","body":"Closes #1046\n\nDescription","draft":true}}
```

The full publication object is required; base must equal the retained base.
Only ready, bound, implemented or reviewed issues admit this repair. Exact native
identity/version and pending-effect guards still apply. The native transaction
retains prior commits, advances semantic inputs, and invalidates downstream
proof, review and publication evidence. Refresh proof/review before publishing.
Cards, validators and publication must not be mixed in one edit. Preparation
and publication amendment require a line beginning `Closes #ISSUE` and reject
closing references to other issues. After publication, use `github-pr` updates.

## Amendment and evidence invalidation table

Every semantic amendment first requires the current source version, matching
issue checkout and intact retained evidence. Semantic transitions also require
explicit transition admission. The native `amendment_rule` and
`decide_amendment` functions expose and execute this table; their result records
the amendment class as the cause of each invalidation.

| Class | Admitted source states | Additional prerequisites | Resulting state | Invalidated evidence |
| --- | --- | --- | --- | --- |
| Scope or acceptance | Ready through Merge Ready | Approved semantic transition | Ready | Proof, readiness, review, publication, terminal, cleanup |
| Plan | Ready through Merge Ready | Approved semantic transition | Ready stays Ready; later states return to Bound | Proof, readiness, review, publication, terminal, cleanup |
| Proof or validator | Bound through Merge Ready | Approved semantic transition | Bound | Proof, readiness, review, publication, terminal, cleanup |
| Binding | Bound through Merge Ready | Approved semantic transition and current bound topology | Bound | Proof, readiness, review, publication, terminal, cleanup |
| Implementation | Bound through Merge Ready | Approved semantic transition and implementation revision | Bound | Proof, readiness, review, publication, terminal, cleanup |
| Review | Implemented through Merge Ready | Approved semantic transition, current proof and independent review | Implemented | Readiness, review, publication, terminal, cleanup |
| Display only | Ready through Closed Out | Actual projection change | Preserve | None |

A source state outside a row is inapplicable. Missing current-version,
checkout, evidence or class-specific facts is refused. A display request with no
projection change is inapplicable. Formatting-only projection drift therefore
does not invalidate semantic evidence. Any new Git commit still requires a
fresh exact-head review, including a display-only commit.

## Independent review and publication

`REVIEW.json` contains `receipt`, `receipt_digest`, `proof_path` and
`proof_digest`. `receipt` is the externally supplied native
`csdlc.v3.typed_review_receipt.v1` object: repository, issue, distinct implementer
and reviewer, exact `reviewed_revision` and `expected_head_sha`, nonempty
`evidence_digest`, and `publication_linkage` with repository, issue and
`mode: "closing"`. The reviewer supplies actual reviewed evidence; this command
does not manufacture an approval or replace independent review.

`receipt_digest` is the native `typed_review_receipt_payload_digest` of that
receipt. `proof_path` must be `.csdlc/evidence/ISSUE/intent-proof.json`;
`proof_digest` is BLAKE3 of its exact file bytes. The owner checks the proof's
canonical payload digest, successful nonempty validators, current input digest,
issue version and exact HEAD. A supplied hash is not permission to use stale
proof. Mutation freshness recomputes native card and intent-plan integrity;
changing plan bytes while leaving the index digest unchanged is refused.
Review records are create-only per exact head and reject conflicting
bytes or symlink traversal. Publication requires the retained external packet
and native review receipt to agree.

`review --preview plan`, `publish --preview plan` and `finish --preview plan`
perform their supported read-only admission paths. Ordinary review and publish
without preview execute their documented owner effects. Preview does not waive
proof or authentication requirements.

Publication identity comes from native remote mutation intents and authenticated
receipts. It is not supplied as a second PR-number file. Ambiguous targets,
changed uncertain create inputs and unresolved earlier candidate identities
fail closed. Existing PR updates and ready operations authenticate current PR
head, branch, base, open state and closing linkage before mutation, and repeat
that admission at the remote owner boundary. The existing GitHub owner expects the intended branch to be
available remotely; this adapter does not add a hidden Git push. A successful
remote mutation followed by failed authenticated readback remains visible as
recovery-required with its effects preserved.

## Rebind after a scope amendment

A `scope_acceptance` amendment returns semantic state to Ready while retaining
its registered checkout. Run `csdlc bind ISSUE` again to revalidate that exact
checkout and return to Bound. Rebind does not create another worktree or execute
validators. If HEAD changed, the native binding amendment records the new exact
revision and invalidates prior proof, review, publication and terminal evidence.
A Ready issue stays Ready during that head refresh until the explicit bind
transition succeeds.

This is also the recovery path when a retained validator is no longer admitted:
run `bind ISSUE`, then `edit ISSUE --changes FILE` with the replacement
`validators` declaration, then `proof ISSUE`. The replacement validators must
pass current admission. The old validator is never executed by bind or edit.
Unchanged active bindings return an observational no-op. Stale generated intent
requests, changed branch/worktree registration, authority drift and pending
recovery remain errors; obtain a fresh request or resolve the named recovery
instead of editing state files.

## Explicit GitHub operations

`csdlc github-issue ISSUE --operation OPERATION.json` and
`csdlc github-pr ISSUE --operation OPERATION.json` preview an explicit operation.
Add `--execute` to invoke its native owner. The issue family supports issue
create, comment, edit and explicit administrative close; the PR family supports
create, update, ready and merge. Issue creation uses the selected issue as the
coordination context while the native create request uses issue zero because
GitHub assigns the new number. It does not rebind the coordinating issue.

The strict content is the existing `GithubMutation` action object, except that
ordinary merge omits internal receipt identities:

```json
{
  "action": "pull_request_merge",
  "base": "main",
  "method": "merge",
  "operator_approval": "explicit authorization reference for this exact target"
}
```

The caller must supply a real nonempty authorization reference. The current
native owner accepts only `method: "merge"` with a provable two-parent merge
commit. `--execute` alone does not invent merge authorization. The owner derives the canonical
review receipt and preserves #849 publication linkage, exact-head, base, review,
check, branch-rule and authenticated reconciliation guards. Caller-supplied
internal review paths/digests are rejected by the ordinary merge parser.
PR creation must agree with the retained publication plan. Explicit PR update
content may change title or body under current target/head/base and review
admission; a replacement body must retain the canonical closing relation and
must not close unrelated issues. Omitted fields remain unchanged. This explicit
operation does not rewrite the retained plan used by ordinary `publish`.

`csdlc pr-state ISSUE` observes the canonical PR through authenticated transport.
For remote uncertainty, repeat the identical operation to request native
reconciliation. Where the existing owner explicitly permits authenticated
absence retry, its strict content is:

```json
{
  "operation": {"action": "pull_request_ready"},
  "recovery": "retry_after_authenticated_absence"
}
```

This does not enable retries for operations whose owner disallows them. Merge
retry remains reconciliation only. Never change the retained operation, delete
its intent or use another transport to bypass uncertainty.

## Explicit no-PR terminal disposition

For a genuinely non-implementation disposition, use `finish ISSUE --disposition
DISPOSITION.json`. The strict object contains `disposition`, `operator`,
`rationale` and `evidence_refs`. Dispositions are the native enum:
`retired_without_execution`, `duplicate`, `superseded`, `absorbed`,
`coordination_completed` or `historical_disposition`. Supply a real operator,
nonempty rationale and evidence references. This is administrative truth, not
merged implementation proof.

The adapter authenticates the exact closed issue and resolves its observed
closure/update timestamps internally. The terminal owner re-observes that same
identity and those timestamps before persistence. Caller-supplied timestamps,
internal receipts or fabricated authentication fields are not ordinary inputs.
The `--preview plan` form observes without terminal persistence.

## Preview tokens and cleanup

`recover ISSUE` returns `preview_digest` and the observed pending local
transaction or original native remote operation. Preview has no mutation effects
and writes no token file. `recover ISSUE --execute --preview DIGEST` requires
that exact current digest. If an interrupted proof has no retained outcome,
preview reports `abandon_indeterminate_proof` and the required disposition
schema. Execute it with `--disposition DISPOSITION.json`; the strict object names
that action and exact operation ID plus a nonempty rationale. Recovery records
the attempt as failed with unknown effects, does not rerun validators, and leaves
a later explicit `proof` command to start a distinct attempt. Local recovery compares the transaction under the
native issue lock. Remote recovery binds the issue snapshot and complete
retained intent bytes, then reconciles the original request rather than
recomputing its operation. Only the existing native owner's supported
one-shot authenticated-absence retry may dispatch another write; merge and
unsupported mutation retries remain reconciliation-only. Multiple pending
operations or simultaneous local and remote uncertainty are refused explicitly.
With no pending operation the owner reports the appropriate no-op outcome.

Native issue creation uses target issue zero and does not retain its coordinating
issue. These unattributable operations are excluded from `recover ISSUE` and
issue-local pending status. Reconcile an uncertain creation by explicitly
repeating its identical `github-issue ISSUE --operation FILE --execute` request;
shared checkout HEAD identity is not sufficient to assign it to an issue.

`clean ISSUE` returns `preview_token`. Execute with
`clean ISSUE --execute --preview TOKEN`. The token binds the current issue
snapshot, terminal receipt digest, native cleanup preview, registered topology,
policy and exact target. Changed issue version, HEAD, receipt, registration or
policy invalidates it. The native owner rechecks dirtiness, liveness, canonical
receipt and exact target before removal. Tokens are returned on stdout; preview
does not create a token file. The candidate intent cleanup bridge can archive
only the current issue's generated, untracked cards/evidence/completed transaction
records and lock. It verifies file bytes, permissions and symlink text in durable
Git-metadata storage before removing originals, with the issue index last.
Tracked changes and unrelated untracked files remain refused. The retained
request-based cleanup keeps its original clean-worktree contract. Archive and
partial-interruption behavior remain subject to focused validation; resolve
unclassified or live state through its actual owner.

Finish writes canonical terminal state and receipt in primary Git metadata so
cleanup does not remove the only terminal evidence. A repeated clean with no
registered issue binding checks the retained native receipt and terminal state
before reporting an observational no-op. It also checks the retained native
binding: a still-present or registered target after partial archival remains
recovery-required. A terminal receipt alone is not a reason to delete a different
checkout.

## The advanced request is the same intent

Use `--emit-request` on an ordinary form to obtain
`csdlc.v3.generated_intent_request.v1`. The complete emitted wrapper or its
strict `csdlc.v3.intent_request.v1` member is accepted by:

```sh
csdlc COMMAND --intent-request REQUEST.json
```

Save the emitted output directly; no manual extraction is required. The inner
request contains `schema`, `command`, `content`, `execute`, `preview` and `snapshot`.
The snapshot separates repository/issue, executing platform, authority selector
digest, issue generation/digest, and checkout root/branch/head. Both input forms
reach the same intent implementation. An advanced request must match current
identity; the adapter does not silently refresh it to make execution succeed.
Do not mix positional issue/content/execute/preview inputs with the advanced
request. Old internal `--request` JSON is not an intent snapshot.

Foreign-platform snapshots are refused explicitly. This guard does not prove
operation on a non-Unix host or cross-platform equivalence. Actual macOS/Linux
results and any untested host lanes belong in the issue's validation evidence.

## Evidence and boundaries

Machine results retain native outcome fields and the common versioned envelope.
Read status, authority, effects and findings separately. Unknown effects after
remote dispatch or a partial durable write remain unknown; a draft PR is not a
merge, and a merged PR is not native terminal closeout. Ordinary review has an
explicit guarded local-write effect because it retains external evidence;
retained legacy review admission remains observational.

Source anchors: `csdlc-v3/src/application/intent/{mod,context,local,remote,terminal}.rs`,
`csdlc-v3/src/commands/{local,proof,remote}/intent.rs`, and the existing remote and
terminal owners. Focused installed fixtures exercise isolated candidate binaries
and synthetic authenticated transport. Required exact-head review, full coverage
and CI acceptance must be checked in the issue evidence before publication;
this document makes no live delivery, installed upgrade or activation claim.

Coordination-only completion uses the distinct `issue_complete_coordination`
operation, with an explicit live parent contract, operator approval, durable
evidence digests and authenticated child deliveries. See
[Coordination completion](COORDINATION_COMPLETION.md) for the complete
ordinary edit, completion, reconciliation and finish sequence.
