# SIM-08 operations

Issue #874 → pilot #875 → sprint #866. This packet supplies the executable
procedure. Publication does not authorize its live execution. The operator's
no-new-review/no-new-rehearsal exception remains in `operator-disposition.json`.
Focused implementation tests are recorded separately in `validation.json`.

## Executables and preparation

Two executables have distinct roles:

- `csdlc`: the qualified lifecycle candidate, source
  `067cb99bf5c6220f64c9faadd7da6abdca34bcc4`, SHA256
  `6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9`,
  BLAKE3 `764a2f43b4f752cae97680b3294f5d26bed7d2538821c3d1fb7d8e57691ac44e`.
  `candidate.json` retains SIM07 qualification. Do not rebuild and assume the
  resulting bytes have this identity.
- `csdlc-transition`: this PR's separate native transition owner, declared in
  `csdlc-v3/Cargo.toml`, implemented by `src/bin/csdlc_transition.rs` and
  `src/transition.rs` in that component. It constructs live requests, holds the
  pause, stages conversion, installs the supplied qualified candidate, verifies,
  restores and releases writers. Its tests do not extend SIM07 qualification.

From the exact accepted #874 worktree revision, install only the transition
owner into its own generated stable directory. This does not replace shared
`csdlc` or start a pause:

```sh
set -eu
OPERATIONS_ROOT=$(git rev-parse --show-toplevel)
TRANSITION_DIR="$OPERATIONS_ROOT/.adl/bin/sim08-transition"
bash adl/tools/install_owner_binaries.sh --bin csdlc-transition --stable-bin-dir "$TRANSITION_DIR"
TRANSITION="$TRANSITION_DIR/csdlc-transition"
"$TRANSITION" --help
shasum -a 256 "$TRANSITION"
```

Retain the installer provenance, executable hash, source revision and help
alongside the invocation. Keep these bytes in place until terminal recovery or
resume; the request freezes the transition executable's BLAKE3 too. Do not use
Cargo target output as the operational executable.

The operator supplies `PRIMARY` (canonical primary checkout), `CANDIDATE`
(retained qualified executable above), and `INVOCATION` (a new absolute operation
directory beneath primary Git metadata, outside tracked source). Resolve metadata
with `git -C "$PRIMARY" rev-parse --path-format=absolute --git-common-dir`.
Before authorization, inspect without starting a pause:

```sh
mkdir -p "$INVOCATION"
"$TRANSITION" inventory --repo-root "$PRIMARY" > "$INVOCATION/inventory.json"
```

Keep the whole inventory. For every current semantic record, retain its exact
`issue`, `checkout` and `plan:null`. For every legacy record, retain its issue and
checkout and supply the absolute path to its original accepted native intent
plan (`csdlc.v3.intent_plan.v1`). That plan includes the exact existing `slug`,
all six `cards` value objects, `validators` and `publication` metadata. Its card
values must match `cards/{sip,stp,spp,vpp,srp,sor}.values.json` under the source:
primary `.git/csdlc-v3/local/issues/ISSUE`, or bound
`CHECKOUT/.csdlc/issues/ISSUE`. Preserve the accepted values; do not synthesize
review, receipt or publication history. The owner checks equality and native
compatibility admission. Missing plans, unsupported phases/history, unhealthy
current records or pending recovery must be reconciled through native lifecycle
commands before a transition; do not omit those records from the denominator.

## Construct the live request

The accountable transition operator obtains an explicit pause window and scope,
notifies every affected C-SDLC session, and retains acknowledgments for every
inventory issue. Drain in-flight commands and reconcile remote intent uncertainty
before proceeding. Preserve dirty work, original branches and registration.
Runtime/provider/cloud services are outside this writer pause.

Write `INVOCATION/spec.json` with these **required operator inputs** (the types
below describe fields; they are not literal JSON values to copy):

| Field | Required value |
| --- | --- |
| `primary` | Absolute `PRIMARY` path |
| `candidate` | Absolute `CANDIDATE` path |
| `operation_id` | New unique operation identifier, letters/digits/hyphen/underscore only |
| `operator` | Accountable operator identity |
| `approval_reference` | Durable reference to the actual pause authorization and acknowledgments |
| `pause_expires_unix` | Authorized window end, integer Unix seconds in the future |
| `acknowledged_issues` | Every inventory issue number, once each |
| `records` | Every inventory record, once each: `{"issue": NUMBER, "checkout": "ABSOLUTE_PATH", "plan": null}` or the absolute retained-plan path in `plan` |

Construct the immutable request; this command only creates that new file:

```sh
REQUEST="$INVOCATION/request.json"
"$TRANSITION" request --spec "$INVOCATION/spec.json" --output "$REQUEST" > "$INVOCATION/request-result.json"
```

The owner derives authority, old installed executable identity, candidate identity,
its own identity, complete issue census, native/semantic source hashes, remote
state hash, registered branches/HEADs and retained-plan hashes from the actual
repository. It rejects missing or inconsistent identity. These are not operator
supplied digest placeholders. Do not edit the resulting request or reuse an
operation ID with different inputs.

## Authorized fence, conversion and paused verification

Only after the preceding live authorization exists:

```sh
"$TRANSITION" fence --request "$REQUEST" > "$INVOCATION/fence.json"
"$TRANSITION" convert --request "$REQUEST" > "$INVOCATION/convert.json"
"$TRANSITION" verify --request "$REQUEST" > "$INVOCATION/verify.json"
"$TRANSITION" status --request "$REQUEST" > "$INVOCATION/status.json"
```

Each command must exit zero. Expected states are `fenced`, `converted_paused`
and `verified_paused`. Stop the sequence on refusal (exit 2); retain stdout and
stderr. `status` lists retained markers and identities; it does not substitute
for `verify`.

`fence` retains native issue locks and semantic writer locks in independent
persistent guardians. It seals old-writer lock, issue, transaction and remote-intent namespace creation
with saved directory permissions, and rejects unsettled remote intents. This is
cooperative local-user fencing: privileged processes or manual chmod must not
bypass it. Unknown/missing guardian identity cannot be replaced by a PID guess.
The owner validates the whole census before admitting conversion.

`convert` snapshots projections, installed binary and provenance; stages admitted
legacy semantic state in a private Git directory; preserves existing current
semantic records and native history; retains new-state writer locks; activates
staged state and the exact qualified candidate; and records expected readback
hashes. It does not send remote mutations. Existing native source, registered
worktrees and dirty files remain in place.

`verify` checks candidate/provenance, healthy native semantic observations,
projections, continuous fences, unchanged original source/branches/HEADs and
remote identity for the entire census. Inspect each issue from the appropriate
checkout while paused, including primary prepared records and genuine registered
linked worktrees:

```sh
"$PRIMARY/.adl/bin/native-v3/csdlc" status "$ISSUE" --repo-root "$CHECKOUT" --json
```

`ISSUE` and `CHECKOUT` come from each frozen inventory row. Retain every result;
a failed readback forbids resume. Native `validate` uses lifecycle locking; use the transition owner’s
`verify` while its fence is held. Do not test a writer by performing a real new
lifecycle operation during this verification window.

## Recovery and explicit resume

The result reports `operation_directory`, under
`.git/csdlc-v3/local/live-transitions/OPERATION_ID`. Keep its immutable request,
inputs, snapshots, journals, guardian records and install provenance. Do not
clean these while the operation is paused or recovery is unresolved.

| Observation | Executable next action |
| --- | --- |
| Interrupted command; retained guardians authenticate and identities match | Inspect `status`, then repeat the same `fence` or `convert` with the identical request; journaled steps resume |
| Failed fence admission before conversion started | `restore --request "$REQUEST"` aborts the pause and restores namespace permissions without rolling back issue state |
| Failed paused verification, no later state/remote changes, guardians intact | `restore --request "$REQUEST"` restores saved binary/provenance/projections and quarantines converted semantic state; require `restored` and exit zero |
| Interrupted restore | Repeat `restore` with the identical request; retained restore intents reconcile completed steps |
| Lost guardian, changed native/source/current state, uncertain remote effect, or foreign installed bytes after conversion began | Automatic restore refuses. Preserve pause/evidence; obtain explicit forward-reconciliation repair. Do not replay a remote operation or remove lock/journal files |
| Resume already recorded | Restore prohibited; any later repair requires a separate authorized operation |

A restored or aborted operation is terminal; a later attempt needs a new
operation ID and fresh request. A terminal release retry is safe and returns
`terminal_release_reconciled` once release acknowledgments are observed.

After **all** paused readbacks pass, obtain a fresh resume decision. Use the
`request_digest`, `candidate_blake3` and `conversion_blake3` from native `status`
in a new `INVOCATION/resume.json` object with:

- `schema`: `csdlc.v3.live_transition_resume.v1`;
- those three exact identity fields;
- `operator` and `approval_reference` identifying this actual resume decision;
- `approved_at_unix`: actual decision time in integer Unix seconds, after conversion;
- `expires_unix`: decision expiry in integer Unix seconds, still in the future.

Then, and only then:

```sh
"$TRANSITION" resume --request "$REQUEST" --decision "$INVOCATION/resume.json" > "$INVOCATION/resume-result.json"
```

Require exit zero and `resumed`. Resume re-verifies the converted state, records
the irreversible terminal boundary and releases writer fences. Snapshot rollback
is forbidden after resume/new writes; recovery becomes forward reconciliation.
`PILOT.md` freezes the first 30 consecutive real journeys. No pilot has run here.

The historical `csdlc-conversion-rehearsal` owner and its seven-role fixtures
remain isolated evidence in `conversion-command-contract.json` and
`evidence-index.json`. They are not the live executable above. No additional
operational rehearsal, independent review or live transition ran for this
correction; the retained exception does not claim the original full rehearsal
acceptance was completed.
