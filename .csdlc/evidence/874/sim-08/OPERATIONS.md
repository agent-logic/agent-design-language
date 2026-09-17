# SIM-08 operations

Issue #874 → pilot #875 → sprint #866. No new rehearsal or review is claimed;
see `operator-disposition.json`. Publication does not authorize a writer pause,
installation, conversion or resume.

## Use the existing native command

The qualified executable is `csdlc`, source
`067cb99bf5c6220f64c9faadd7da6abdca34bcc4`, SHA256
`6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9`.
`candidate.json` binds its qualification to merged PR #1060. Resolve `CSDLC` to
those verified executable bytes; do not substitute the currently installed binary
without checking its hash. Actual discovery is retained in `candidate-help.txt`
and `candidate-contract.json`.

`transition.py` now constructs this request without hand-editing state. From this
packet directory, use the actual issue, checkout, verified candidate, retained
intent plan and a new output file:

```sh
python3 transition.py prepare --issue "$ISSUE" --checkout "$CHECKOUT" --candidate "$CSDLC" --retained-plan "$RETAINED_PLAN" --output-plan "$PLAN"
```

This prints the exact native argv and writes only the new plan file. It verifies
candidate bytes, native/card identity, exact slug, supported phase, bound checkout
and unchanged retained plan. Add `--execute` only when this issue's adoption is
authorized; the native owner still performs all lifecycle admission. `status`,
`validate` and `recover` use the same issue/checkout/candidate arguments. A
recovery execution additionally requires `--preview` from fresh native inspection.
The helper does not implement global cutover or snapshot restoration.

For supported ready/bound legacy issue adoption, **the existing live entrypoint
is `prepare`**, not the copied-record converter:

```sh
"$CSDLC" status "$ISSUE" --repo-root "$CHECKOUT" --json
"$CSDLC" prepare "$ISSUE" --plan "$PLAN" --repo-root "$CHECKOUT" --json
"$CSDLC" validate "$ISSUE" --repo-root "$CHECKOUT" --json
```

These variables are required operator inputs: `ISSUE` is the actual numeric issue;
`CHECKOUT` is its bound worktree for bound adoption, or primary for ready adoption;
`PLAN` is an absolute path to its retained native intent plan. Construct that JSON
with schema `csdlc.v3.intent_plan.v1`, the **existing exact slug**, all six current
card-value objects under `cards`, existing validator declarations, and publication
metadata (`base`, `title`, `body`, `draft`). Copy current semantic inputs rather
than reconstructing approvals, validation or publication history. Resolve native
records from `git rev-parse --path-format=absolute --git-common-dir`; prepared
card values live beneath `csdlc-v3/local/issues/ISSUE/cards`. For bound records,
use the bound issue's current card values and verify the matching native binding.
Do not treat an arbitrary changed plan as preservation of the old issue contract.

`prepare` resolves authority, observes GitHub identity, checks the compatibility
census and handles its own native writer locking. Bound legacy adoption owns its
persistent fence through projection completion. Do not manually launch a guardian
or write fence markers. Require `status:completed`, then matching issue/binding
identity and successful validation. `LegacyMigrationRequired`, unknown history,
pending remote effects or unsupported phase means **no adoption**, not permission
to delete evidence or force the request. This per-issue command is not a global
cutover, installation or snapshot-restore command.

For interrupted native operations, inspect the existing recovery identity:

```sh
"$CSDLC" recover "$ISSUE" --repo-root "$CHECKOUT" --json
```

Only when that result supplies an executable classified recovery and its fresh
preview digest, use the same issue and checkout:

```sh
"$CSDLC" recover "$ISSUE" --execute --preview "$PREVIEW" --repo-root "$CHECKOUT" --json
```

`PREVIEW` comes from that observation; do not invent it or replay an uncertain
remote mutation. A refusal remains a refusal. Recovery is not snapshot rollback.

## Separate copied-record converter

`csdlc-conversion-rehearsal` is a separate executable from the same pinned source,
entrypoint `csdlc-v3/src/bin/csdlc_conversion_rehearsal.rs`. Its source hashes and
command contract are in `conversion-command-contract.json`; the `csdlc` checksum
above does not identify it. If later authorized for isolated use, build at that
exact source with:

```sh
cargo build --locked --offline --manifest-path csdlc-v3/Cargo.toml --bin csdlc-conversion-rehearsal
```

Resolve Cargo's target directory through `cargo metadata`, retain the resulting
`debug/csdlc-conversion-rehearsal` as operation-local `CONVERTER`, and record its
SHA256/toolchain/lockfile provenance. Do not replace the installed owner.

Its supported sequence is:

```sh
"$CONVERTER" convert --request "$REQUEST"
"$CONVERTER" operation-evidence --request "$REQUEST"
"$CONVERTER" restore-pre-effect --request "$REQUEST"
```

`REQUEST` is one frozen, absolute JSON path for an **isolated copied census**.
Construct it using the required-input table in `conversion-command-contract.json`.
The seven record roles must appear exactly in the declared order. Derive paths,
branch and HEAD from the actual registered isolated worktree, and the prior
executable BLAKE3 from its bytes. The historical request indexed in
`evidence-index.json` is an example, not a reusable live request.

`convert` owns the fence and starts `writer-fence-guardian` internally. Do not
hand-author guardian requests. `writer_fence_probe:true` requires the existing
SIM06 driver's two real old-writer probes and acknowledgments; use `false` for
standalone copied conversion and make no old-writer-denial claim. Retain stdout,
exit status and the journal/effect paths returned by `operation-evidence`.
Restart only the same classified operation using the same request bytes.
`restore-pre-effect` is conditional, not an unconditional third success step:
it refuses any semantic or remote effect with exit 2 and `allowed:false`.
With zero effects it verifies unchanged source hashes and releases the fence.

**This converter cannot deploy the live candidate.** It requires exactly seven
copied roles, activates its own executable in a private slot, writes a synthetic
remote acknowledgment, then restores that private slot. It has no arbitrary live
census, qualified installed-candidate, operator pause/resume or live rollback
request. Do not point it at production state. P2's live-cutover requirement is
still unresolved; native per-issue adoption above does not replace it.

## Live transition boundary and recovery

Before #875 can activate, its owner must retain the exact installed provenance,
registered worktrees/branches, generations/digests, pending transactions/remote
intents, receipt identities and actual legacy layouts. Preserve dirty work.
Obtain explicit scope/window/owner authorization; notify affected C-SDLC owners,
drain or classify every writer, retain a hash-bound snapshot and complete mapping.
Runtime/provider/cloud services remain outside this scope.

The planned global transition still needs an executable owner that keeps old
writers fenced through complete-census conversion, installed candidate readback,
and primary/genuine-linked-worktree verification before authorized resume.
Neither command above supplies that global lifecycle. **Do not start the pause
on the assumption that the missing live conversion/restore procedure exists.**

| State | Action |
| --- | --- |
| Missing authority, unsupported census, stale candidate or absent live owner | Do not start live conversion |
| Interrupted supported per-issue adoption | Inspect native `recover`; execute only its classified fresh preview |
| Copied operation with zero semantic/remote effects | `restore-pre-effect` with the identical request |
| New-format write, remote effect or uncertain effects | Preserve state; no automatic snapshot restore or remote replay; require authorized reconciliation |
| Live verification failure | Remain within the authorized pause/recovery decision; copied-record restore is not live rollback |

Once an executable live procedure exists and its complete readback passes,
resume only under the recorded operator decision. `PILOT.md` freezes the first
30 consecutive real journeys and all-attempt accounting. No pilot has run here.

`evidence-index.json` preserves SIM06's historical seven-role/114-file rehearsal
and SIM07 qualification separately. Static checks in `validation.json` are
integrity evidence only. No new rehearsal, review, installation or paid resource
use is performed by this correction.
