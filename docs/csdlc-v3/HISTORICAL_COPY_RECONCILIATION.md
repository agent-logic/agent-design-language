# Preserved historical copies in the transition census

The transition census refuses unregistered or changed lifecycle residue. For a
closed issue whose old checkout contains an unbound preparation bundle or a stale native binding with retained terminal proof,
`csdlc-transition reconcile-history` supplies an explicit reconciliation path.
It preserves the original files and snapshots them under primary Git metadata.
It does not create binding, publication or terminal authority and does not
remove the checkout or count a pilot journey.

Use only a reviewed exact transition-owner build installed through
`adl/tools/install_owner_binaries.sh` into an isolated stable directory. Retain
its source revision, binary hash and installer provenance. A historical
reconciliation does not replace the shared lifecycle binary.

The spec contains these required fields:

```json
{
  "primary": "/absolute/canonical/primary",
  "checkout": "/absolute/registered/historical/checkout",
  "expected_head": "exact 40-character checkout HEAD",
  "issues": [482],
  "operator": "accountable operator",
  "approval_reference": "actual instruction to reconcile these preserved copies"
}
```

Supply the actual repository paths, exact HEAD, bounded issue list and real
operator decision. The primary remains this repository. An optional `source_repository` may name
`danielbaustin/agent-design-language` for a copy from that historical repository;
it must match the source index and authenticated GitHub identity exactly.
Omitting it selects `agent-logic/agent-design-language`. The command authenticates
GitHub through the shared native credential adapter. Set the approved
`ADL_GITHUB_TOKEN_FILE` source when required; never copy its contents into a
spec or receipt.

```sh
"$TRANSITION" reconcile-history --spec "$SPEC" > "$INVOCATION/history-preview.json"
"$TRANSITION" reconcile-history --spec "$SPEC" --execute --preview "$PREVIEW_DIGEST" > "$INVOCATION/history-result.json"
```

`PREVIEW_DIGEST` must be the exact digest returned by a successful preview.
Preview reads source, topology and authenticated GitHub state without writing.
Review its exact records before execution. Execution rereads the same inputs,
refuses changed previews, takes nonblocking native issue locks and rejects a
sealed native namespace. It creates verified snapshots and immutable dispositions
under `.git/csdlc-v3/local/historical-copies/`; originals remain unchanged.
Retain these records for as long as the copies remain present. Interrupted
execution may leave partial snapshot evidence. Repeat the identical approved
spec and digest only when current preview still matches; changed bytes refuse
rather than overwrite an earlier snapshot or disposition.

Admission requires all of the following:

- Exact registered checkout and HEAD, in the canonical repository.
- No native issue, semantic issue, binding or pending native transaction.
- An initialized or ready source index for the exact repository/issue, with no
  branch, worktree, publication or terminal record. Initialized copies have no
  transitions; ready copies have exactly the initialized-to-ready transition.
- Alternatively, a native bound copy whose matching binding points to an absent,
  unregistered execution target. This requires the exact retained native terminal
  receipt and fresh authenticated confirmation that its PR merged with the
  recorded head in this repository. The receipt and filtered PR readback are
  included in the preview and rechecked during every census.
- Exactly the index, audit (or native binding) and six Markdown/value card pairs.
  Legacy terminal copies may also contain retained design/diagram files, only
  when their exact bytes match the validated receipt. Unbound preparation may
  include `authored/design.md` and `authored/diagram.mmd` only when explicitly
  referenced by that issue index; their bytes are snapshotted as preparation,
  without asserting they match a later terminal delivery. Missing, additional and
  symlinked files refuse admission. Tracked copies retain their exact checkout
  HEAD, Git index/staged-diff digest and working-file snapshot; the command
  never stages, restores, deletes or commits their source files.
- Legacy copies with claims or later lifecycle phases require the retained
  `csdlc-v2/closeout/ISSUE.json` receipt. Its receipt and record digests,
  repository/issue/initialization identity, terminal/publication linkage and
  source topology must agree. This is read-only historical evidence validation,
  not a revival of the retired v2 lifecycle. Its exact merged PR/head is
  authenticated in the source repository. Missing or inconsistent evidence
  remains a refusal.
- Fresh authenticated readback of the exact closed GitHub issue, not a PR.
- Equal source and snapshot fingerprints before the disposition is recorded.

Every later census checks retained dispositions before ordinary tracked-history
classification. It verifies the source, snapshot, checkout identity and fresh
closed readback. Reopening/reclosing the issue, changing the source or snapshot,
changing the Git index, committing the bundle, or deleting its directory refuses
the census. A new live
native/semantic/binding record stays in the operational denominator. Unknown
residue without a disposition still refuses classification. This command does
not waive complete census, owner acknowledgments, current qualification, an
explicit pause window or the fresh post-verification resume decision.

## Validation classification

PVF tooling; deterministic same-host fixtures with mocked authenticated GitHub
readback; required SIM-09 preflight guard proof. Run:

```sh
cargo test --locked --offline --manifest-path csdlc-v3/Cargo.toml --lib transition::
```

History fixtures prove preservation/replay, stale preview refusal, exact closed
identity, reopened/reclosed issues, active/pending records, extra/symlinked files,
changed topology, changed/deleted originals or snapshots, retained-disposition
precedence, busy writers and sealed namespaces. Existing census and transition
fixtures remain required. Fixture results do not establish live conversion or
30-journey exposure. Record actual read-only live preview and any authorized
reconciliation separately from fixture proof.
