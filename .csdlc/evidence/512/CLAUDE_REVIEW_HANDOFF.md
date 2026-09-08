# Claude review handoff: issues #511 and #512

## Verdict

**Changes required.** Issue #511 is substantively absorbed into the #512
implementation and does not need a separate implementation path, but its design
contract is not yet fully satisfied because required identity, ordering, and
accessibility checks are red.

## Review target

- Repository: `agent-logic/agent-design-language`
- Bound worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-512-observatory-redesign-implementation`
- Branch: `codex/512-observatory-redesign-implementation`
- Base commit: `db434645de26630697e789f57fdb51b800b2d922`
- Reviewed surface: the nine unstaged files present in the worktree on
  2026-09-05. There was no #512 PR or committed review head at review time.
- Mutation boundary: the review did not alter Claude's implementation files.

## Findings

### P1 — Canonical Polis identity validation was removed

`runtimeV3SnapshotFromFeed()` now copies the raw snake-case `polis_identity`
object into the snapshot. The established `projectPolisIdentity()` validation
and projection function was removed, as was its exported test surface. This
breaks the canonical Runtime-v3 identity contract and causes the Polis identity
tests to fail.

Evidence:

- `demos/html-observatory/app.js:1595-1601`
- `demos/html-observatory/tests/polis_identity.test.mjs:16-60`

Required result:

- Restore a canonical, validated projection for Runtime-published Polis
  identity.
- Reject malformed identity fields and incompatible feed schemas according to
  the final v3 contract.
- Keep display values source-grounded; endpoint-derived labels may be temporary
  connection labels but must not replace canonical Runtime identity.

### P1 — Roster ordering accepts a replayed authenticated cursor

`acceptRuntimeRosterSnapshot()` now accepts a higher revision carrying the
previous event cursor. The focused Observatory validator fails at
`replayed authenticated cursor rejected`. A repeated authenticated cursor must
not advance the accepted roster state.

Evidence:

- `demos/html-observatory/app.js:206-246`
- `adl/tools/test_html_observatory.sh:232-248`

Required result:

- Preserve the useful behavior that a missing cursor does not freeze the entire
  dashboard.
- Continue rejecting duplicate or replayed non-empty authenticated cursors.
- Keep the roster-ordering decision scoped to roster updates rather than using
  it to suppress unrelated live telemetry.

### P1 — Required accessibility and proof navigation contract is red

The redesign removed the addressable `#evidence` surface and the secondary
accessible navigation required by the current accessibility contract. The
Node suite stops first at `evidence must remain addressable`.

Evidence:

- `demos/html-observatory/index.html:130-148`
- `demos/html-observatory/index.html:565-578`
- `demos/html-observatory/tests/accessibility_responsive.test.mjs:11-63`

Required result:

- Restore equivalent keyboard- and screen-reader-accessible access to evidence,
  or deliberately replace the old surface and update the tests to prove the new
  interaction.
- Verify tab roles, selected state, focus treatment, landmarks, live regions,
  and narrow-viewport reachability against the final markup.

### P2 — README describes the previous interface

The README still promises the former first-viewport graph, CSM API inspector,
evidence links, and CloudWatch composition that the redesign removed or moved.

Evidence:

- `demos/html-observatory/README.md:3-35`

Required result:

- Describe the final Overview, Chat, Agents, Modules, Events, Infrastructure,
  and Logs surfaces accurately.
- Preserve the distinction between live Runtime proof, retained fallback data,
  and planned public hosting.

## Issue #511 disposition and inherited requirements

Issue #511 should be classified as **absorbed into #512** once the corrected
#512 result proves all four OBS-A acceptance criteria. No separate #511
implementation PR is necessary.

Before #511 closes, the #512 result must demonstrate:

1. Every final view has a stable information contract.
2. Empty, degraded, recovery, and revoked states are implemented and proven.
3. Keyboard and screen-reader flows are specified by the executable markup and
   passing accessibility tests.
4. No invented Runtime field or endpoint-derived substitute is presented as
   canonical Runtime identity.

Close #511 with a no-PR absorption note pointing to the corrected #512 exact
review head and its passing focused evidence. Then remove #511 as an execution
dependency for #512. Until those checks pass, #511 is functionally absorbed but
not yet safe to close as satisfied.

## Validation observed

- `node --check demos/html-observatory/app.js`: passed.
- `node --test demos/html-observatory/tests/*.test.mjs`: passed, 20/20.
- `bash adl/tools/test_html_observatory.sh`: passed.
- `node adl/tools/validate_v092_observatory_transcript_history.mjs`: passed.
- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`: passed all 8
  cases.
- `git diff --check`: passed.
- The localhost server returned JavaScript byte-identical to the corrected
  worktree product head.
- The configured live Runtime reported ready/running with no degraded reasons
  and authentic `Axioma.wuji` identity during the review.
- The operator supplied a live-browser screenshot of the corrected product at
  the `#runtime-proof` route. It visibly proves the responsive Observatory shell,
  selected polis, six-agent topology, live event stream, selected inspector tab,
  Runtime readiness, and WSS connection state. The retained image is
  `exact-browser-operator-screenshot.png` with SHA-256
  `32ec4493bfbd19bda8df967965b362a3236f2f25967b045b900cbcb3b308cafa`.

## Review limitation

The Codex browser controller rejected its own trusted browser dependency, so
Codex could not independently drive the page. The operator-provided live-browser
screenshot supplies the visual exact-product proof; executable accessibility
and navigation behavior is covered by the passing focused tests. The screenshot
does not independently prove every keyboard gesture, and no broader claim is
made.

## Completion checklist

- [x] Canonical Polis identity projection restored and tests pass.
- [x] Replayed roster cursor rejected without freezing other telemetry.
- [x] Final accessible evidence/navigation behavior is covered by focused tests.
- [x] Empty, degraded, recovery, and revoked states pass focused checks.
- [x] README matches the final interface.
- [x] Full focused Observatory suite passes.
- [x] Exact-product browser rendering is retained from the operator session.
- [ ] Fresh independent exact-head review has no unresolved findings.
- [x] #511 closed as absorbed into #512 with evidence links.
