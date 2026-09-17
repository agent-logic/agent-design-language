# SIM-08 transition operations packet

Issue #874; consumed by #875; sprint #866. Planning #4.5 owns this packet.
The operator directed immediate PR publication with no additional rehearsals or
review rounds. This packet consumes retained SIM-06/SIM-07 evidence. No fresh
SIM-08 rehearsal or independent review is claimed. Publication does not establish
all original execution acceptance criteria or authorize live activation.

## Candidate and evidence

The qualified source is `067cb99bf5c6220f64c9faadd7da6abdca34bcc4`.
The executable SHA-256 is
`6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9`.
`candidate.json` binds the binary, source, qualification head and merged PR #1060.
`candidate-help.txt` and `candidate-contract.json` are actual read-only discovery
outputs from that executable, not reconstructed syntax. `evidence-index.json`
contains immutable upstream links and hashes. SIM-06's retained successful
conversion/restore/fence/interruption evidence used source `02c4956...`; it must
not be relabeled as a new rehearsal of the SIM-07 candidate.

The retained SIM-06 census contains seven roles: prepared, bound/dirty,
implemented, reviewed, published, terminal and pending recovery. Its inventory
records 114 files and 1,645,329 bytes. Its scenario index includes conversion,
in-flight classification, old-writer fence, interruption matrix, ambiguous remote,
remote-success/local-crash, pre-effect restore, both unsafe restore refusals,
old-schema denial and genuine linked-worktree parity. These are historical
observations, not a live inventory or a new test execution.

## Responsibilities and entry conditions

The SIM-09 operator owns the separately authorized pause window, activation and
resume decisions. Each active issue owner acknowledges their own writer status.
The transition operator records census, snapshot and journal identities; the
pilot recorder registers journeys before outcomes. One person may hold multiple
roles, but each decision must name its actor and time.

Before any live action, SIM-09 must record exact repository/checkout identity,
qualified candidate hashes, accepted qualification disposition, actual installed
binary provenance, full affected census, all owner acknowledgments, and explicit
pause/activation authority. A missing input means no activation. A changed
candidate requires a new qualification decision. Packet publication is not a
substitute for these inputs. Current runbook rehearsal/review exceptions must be
visible in the activation decision, never silently converted to passing evidence.

## Ordered procedure

1. **Inventory — transition operator; read-only.** Capture `git status --short
   --branch`, `git rev-parse HEAD`, `git worktree list --porcelain`, and resolved
   `git rev-parse --git-common-dir` from the primary and each affected registered
   checkout. Use the admitted installed executable's `status` and `validate`
   commands for each census issue. Record branch/worktree binding, semantic and
   native generations/digests, template version, legacy layout, dirty tracked and
   untracked paths, pending local transactions and remote intents, and immutable
   review/publication/terminal receipt identities. Hash retained evidence and
   record authority selector/receipt identities. Do not clean, reset or overwrite
   another owner's work. Any inaccessible, ambiguous or unsupported record stops
   that census from activation; it cannot be silently omitted.
2. **Copied-record evidence — transition operator.** Consume the indexed SIM-06
   snapshots, source-to-destination equivalence receipts, seven-role census,
   scenario results and SIM-07 qualification. No additional rehearsal is executed
   in this publication. The existing isolated runner interface is
   `python3 adl/tools/run_issue872_conversion_rehearsal.py --request PATH`, where
   PATH is an admitted isolated fixture request, not the historical request with
   stale machine paths. The retained request documents the complete input shape.
   This fixture tool is not a production conversion command and must never be
   repointed at live state. Any later rerun requires a newly bound isolated request
   and authorization consistent with the operator's current no-rehearsal direction.
3. **Schedule, notify, drain and fence — SIM-09 operator.** Obtain explicit pause
   and activation authority before sending coordination notices or stopping
   writers. Notify affected owners through the repository coordination surface;
   retain acknowledgments and pause start time. Drain known local transactions.
   Classify each pending remote intent as not dispatched, uncertain, or reconciled;
   reconcile uncertainty using the original operation identity, never replay a
   remote mutation. Fence every old writer, including old installed binaries,
   against the actual state roots. An acknowledgment alone is not technical fence
   proof. If an old writer can still commit, remain paused and do not install.
   Runtime/provider/cloud services are outside this pause.
4. **Snapshot, map and stage — transition operator while fenced.** Retain the old
   executable and provenance, authority records, complete source census, receipt
   bytes, dirty work and worktree registrations. Hash each retained file and the
   inventory; map every source record to its intended destination with scope,
   plan, validators, binding, generations and evidence identities. Admit only a
   complete supported census. Use the existing typed conversion owner and its
   admitted request/journal contract; do not hand-edit lifecycle records or reuse
   fake authorization from the fixture. Stage all records, validate the whole
   census, then activate through restartable journal steps. Retain sources.
   Never turn the historical `cutover` administrative command into an assumed
   SIM migration API merely because it appears in help.
5. **Verify while paused — transition operator.** Verify semantic equivalence,
   receipt identity, exact candidate executable hash, canonical authority,
   installed command discovery, bindings, pending effects and no competing writer.
   Run read-only `status`/`validate` from both primary and an actual registered
   non-primary checkout against their resolved state. Retain outputs and hashes,
   including explicit rejection of unsupported old schemas. A recreated directory
   that is not registered is not linked-worktree proof. Any missing/failed result
   keeps the fence held and enters the recovery table below.
6. **Resume — SIM-09 operator.** Resume only after all census rows verify, pending
   operations are classified, candidate readback matches and the resume decision
   names the operator, time, journal and proof. Retain old executable/snapshot as
   restore material, not as an alternate callable lifecycle route. Start the
   prospective pilot ledger before the first post-resume journey. Record the first
   new-format write or remote effect explicitly; it crosses the restore boundary.
7. **Recover — transition operator.** Follow the table below. Keep the same pause,
   fence, operation identity and journal. Never retry uncertain remote effects as
   new operations. Record recovery decisions, attempts and resulting versions.

Stable installation is administrative: the repository documents
`bash adl/tools/install_owner_binaries.sh --bin csdlc`. It builds from the selected
checkout; it does not prove that its output equals the qualified executable.
For a separately authorized installation, freeze source/toolchain provenance and
compare the resulting stable executable bytes with `candidate.json` before any
resume. Stop on mismatch. This packet does not run the installer, replace the
shared binary, or supply fabricated live activation request values.

## Recovery decision table

| Observed state | Required action | Retained evidence / exit condition |
| --- | --- | --- |
| No explicit live authority, missing qualification or changed candidate | Do not pause/convert/install | Denial and exact missing input; new authorized decision required |
| Unsupported or incomplete census, missing owner, failed old-writer fence | No activation; preserve all source records | Census exception and owner/fence resolution |
| Interruption before activation | Inspect durable journal; resume only classified staged operation | Same transaction identity; complete-census verification |
| Verification fails; no new operational write and no remote effect | Under same fence, use admitted restore to recover snapshot and prior binary | Source hashes, prior binary hash, receipt identity and primary/linked readback match before release |
| Any new-format operational write | Freeze writers; prohibit automatic snapshot restoration | Preserve new state and journal; separately approved forward-repair/reconciliation plan |
| Remote effect dispatched, uncertain or observed | Freeze writers; prohibit automatic restoration or mutation replay | Authenticated observation against original identity; reconcile before forward repair |
| History is insufficient to prove no new effects | Treat as unsafe to restore automatically | Preserve evidence and resolve uncertainty |

## Pilot handoff and limitations

`PILOT.md` freezes eligibility, all-attempt accounting, timing, waits, strata and
uncertainty for the first 30 consecutive eligible post-resume journeys. No pilot
journey has run under this packet. Budget exposure is local CPU/disk/Git and
read-only GitHub checks; no paid infrastructure or provider calls are required
for publishing it. Live timings/cost estimates remain unmeasured.

`operator-disposition.json` records the publication exception. Static integrity
validation is in `validation.json`. It proves document/evidence integrity only,
not conversion, pause, restoration, fence behavior or runtime reliability.
