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

`csdlc-conversion-rehearsal` is a separate executable, pinned to the same source
revision as the candidate. The candidate's `csdlc` checksum does not identify it.
The following is a **future isolated-use procedure**, not a live deployment
procedure or an instruction to rerun the waived rehearsal now.

The operator supplies only two paths: `ISOLATED_ROOT`, a new absolute directory
outside the source checkout, and `PRIOR_BINARY`, the retained SIM06 executable
from source `6425ba9bbce4cc1f46789c2e3ac019adf86238b8`. The builder authenticates
that file against its retained SHA256 and supplies its corresponding BLAKE3.
Run from this issue worktree's root. The commands create a genuine independent
repository and registered linked worktree; historical machine paths are not reused.

```sh
set -eu
: "${ISOLATED_ROOT:?Set a new absolute isolated directory}"
: "${PRIOR_BINARY:?Set the retained SIM06 executable path}"
case "$ISOLATED_ROOT" in /*) ;; *) exit 2 ;; esac
test ! -e "$ISOLATED_ROOT"
SOURCE_REPOSITORY=$(git rev-parse --show-toplevel)
PACKET="$SOURCE_REPOSITORY/.csdlc/evidence/874/sim-08"
REVISION=067cb99bf5c6220f64c9faadd7da6abdca34bcc4
mkdir -p "$ISOLATED_ROOT/requests" "$ISOLATED_ROOT/bin"
printf '%s\n' '{"isolated":true}' > "$ISOLATED_ROOT/.csdlc-conversion-rehearsal.json"
git clone --no-hardlinks --no-checkout "$SOURCE_REPOSITORY" "$ISOLATED_ROOT/primary"
git -C "$ISOLATED_ROOT/primary" worktree add -b sim08-isolated "$ISOLATED_ROOT/linked" "$REVISION"
cp "$PRIOR_BINARY" "$ISOLATED_ROOT/bin/prior-csdlc"
cargo build --locked --offline --manifest-path "$ISOLATED_ROOT/linked/csdlc-v3/Cargo.toml" --target-dir "$ISOLATED_ROOT/build" --bin csdlc-conversion-rehearsal
cp "$ISOLATED_ROOT/build/debug/csdlc-conversion-rehearsal" "$ISOLATED_ROOT/bin/csdlc-conversion-rehearsal"
CONVERTER="$ISOLATED_ROOT/bin/csdlc-conversion-rehearsal"
REQUEST="$ISOLATED_ROOT/requests/conversion.json"
shasum -a 256 "$CONVERTER" "$ISOLATED_ROOT/linked/csdlc-v3/Cargo.lock" > "$ISOLATED_ROOT/requests/build.sha256"
rustc --version --verbose > "$ISOLATED_ROOT/requests/toolchain.txt"
python3 "$PACKET/conversion_request.py" --isolated-root "$ISOLATED_ROOT" --linked-worktree "$ISOLATED_ROOT/linked" --source-root "$ISOLATED_ROOT/linked/.csdlc/evidence/872/conversion-rehearsal/snapshots/source" --prior-executable "$ISOLATED_ROOT/bin/prior-csdlc" --operation-id sim08-copied-records --writer-probe-issue 868 --output "$REQUEST"
```

`conversion_request.py` writes only that new request. It derives Git common,
linked branch/HEAD, registry/authority paths and the exact seven retained source
roles. It rejects source identity drift, escaped paths, a primary-only checkout,
the live repository's Git common directory and changed prior executable bytes.
It neither invokes the converter nor claims native admission. No guardian request,
fence marker, semantic state or approval is manufactured. The full request field
contract is in `conversion-command-contract.json`.

After isolated execution is authorized, use this exact frozen request:

```sh
set +e
"$CONVERTER" convert --request "$REQUEST" > "$ISOLATED_ROOT/requests/convert.stdout.json" 2> "$ISOLATED_ROOT/requests/convert.stderr"
CONVERT_EXIT=$?
printf '%s\n' "$CONVERT_EXIT" > "$ISOLATED_ROOT/requests/convert.exit"
set -e
"$CONVERTER" operation-evidence --request "$REQUEST" > "$ISOLATED_ROOT/requests/operation-evidence.json"
```

`convert` acquires the exact native issue locks through its internal
`writer-fence-guardian`. Require exit 0 and `status:completed`; retain the result,
request bytes, executable hashes and journal/effect paths. The builder sets
`writer_fence_probe:false`: this standalone sequence does not claim old-writer
probe proof. The retained SIM06 driver owns the two probe handshakes; do not set
that flag without the driver. A failed conversion is not a successful no-op:
inspect `operation-evidence` before deciding recovery. An admitted restart uses
`convert` with identical request bytes and operation identity.

**Restore is a conditional recovery branch, never the next success step.** If
inspection reports zero semantic and remote effects, invoke:

```sh
"$CONVERTER" restore-pre-effect --request "$REQUEST" > "$ISOLATED_ROOT/requests/restore-result.json"
```

Require exit 0 and `allowed:true`. The owner rechecks effects and unchanged source
hashes before releasing the fence. Exit 2 with `allowed:false` prohibits restore;
retain the operation for reconciliation. In particular, a completed conversion
has effects and must not be followed by this restore command.

This owner activates its own executable in a private slot, records a synthetic
remote acknowledgment and restores that private slot internally. It **does not
install the qualified live candidate or restore live snapshots**. There is no
live request to construct for it. The original guide's implied live conversion
and restore capability was incorrect; that part of P2 remains open.

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
