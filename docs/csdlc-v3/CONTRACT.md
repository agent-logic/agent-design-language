# C-SDLC v3 Contract — V3-A

Status: construction input for v0.92.1 issue #500.

This contract defines the C-SDLC v3 construction boundary accepted by V3-A. It
does not make v3 operational. C-SDLC v2 remains the sole operational authority
for issue lifecycle state, GitHub writes, review, publication, finish, cleanup,
and recovery until a later operator-reviewed V3-F authority-transition decision
explicitly says otherwise.

## Authority and compatibility

- v2 remains the sole operational authority throughout V3-A, V3-B, V3-C, V3-D,
  and V3-E.
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

## Rollback and fail-closed behavior

- v2 remains the rollback target until V3-F grants authority.
- If v3 parity, migration, or state durability proof is incomplete, v3 remains
  a non-authoritative construction artifact.
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
