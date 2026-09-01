# Issue #604 full-cycle canary defects

This file records defects found while testing a real issue from live issue
readback through local lifecycle setup, implementation, review, and PR
publication.

## DEFECT-001: v3 local prep cannot initialize issue lifecycle state

- Status: open cutover defect.
- Evidence: `csdlc-v3 csdlc local` can produce a read-only preparation report
  and six-card render plan for #604, but it does not write
  `.csdlc/issues/604/**`, does not materialize `.csdlc/prepared/issues/604/**`,
  and does not emit a v2-compatible bootstrap/create request.
- Impact: a live GitHub issue cannot be started with v3 alone before cutover.
- Required fix: v3 needs a governed init/materialize path or deterministic
  v2-compatible handoff artifact.

## DEFECT-002: Fresh issue worktree lacks installed lifecycle binaries

- Status: open tooling defect.
- Evidence: the fresh FastWork worktree did not contain
  `.adl/bin/csdlc-v2/csdlc-issue`; the documented relative binary path failed.
- Impact: fresh issue worktrees cannot run documented lifecycle commands until
  install/resolve is performed.
- Required fix: one-command start must install/resolve lifecycle binaries for
  the new worktree.

## DEFECT-003: Bootstrap and bind do not compose cleanly for live issue start

- Status: open cutover defect.
- Evidence: bootstrap had to create local lifecycle state before bind; bind then
  required a distinct base/issue branch topology.
- Impact: starting from a live issue currently requires hand sequencing across
  bootstrap and execution worktrees.
- Required fix: one-command start must govern bootstrap plus bind atomically.

## DEFECT-004: Stale blocked session goal prevents required issue goal

- Status: open workflow defect.
- Evidence: `create_goal` refused the required #604 issue-bound goal because an
  old blocked Sprint 6 goal remained attached to the thread.
- Impact: required issue goal creation can become impossible in a reused
  session.
- Required fix: provide an operator-safe way to replace stale blocked goals or
  make lifecycle goal evidence independent of app goal state.

## DEFECT-005: Bind does not materialize prepared artifacts or issue-owned test scaffolding

- Status: open tooling defect.
- Evidence: bind copied `.csdlc/issues/604/**` into the exec worktree, but not
  `.csdlc/prepared/issues/604/**` request/evidence files or the issue-owned
  `csdlc-v2/tests/publication_ready.rs` target that doctor had accepted in the
  bootstrap worktree.
- Impact: the bound execution worktree can immediately become doctor-blocked
  even though bootstrap doctor had passed.
- Required fix: bind/materialize should either copy declared prepared artifacts
  and issue-owned scaffolding or reject pre-bind readiness that depends on files
  it will not carry into the execution worktree.

## DEFECT-006: Implementation finalization is hidden behind validate naming

- Status: open usability defect.
- Evidence: the typed mutation that records implementation truth is
  `csdlc-validate finalize --request <json>`, not an obvious lifecycle command
  such as `csdlc-run finalize` or `csdlc-issue finalize`.
- Impact: operators testing a real issue can miss the intended route and spend
  time enumerating binaries and help output.
- Required fix: the one-command lifecycle should expose implementation
  finalization through a discoverable verb and route the current command as a
  compatibility alias or internal owner.

## DEFECT-007: Owner-binary changes create a provenance/finalization circle

- Status: open lifecycle defect.
- Evidence: `csdlc-validate --root <worktree> finalize --request ...` rejected
  #604 after local validation because `Store::lock()` requires installed owner
  provenance, while the owner source set was necessarily dirty from the owner
  binary changes being finalized.
- Impact: issues that modify `csdlc-v2/src/**` or `csdlc-v2/operator/**`
  cannot use the normal finalization route before a source commit and
  reinstall, which makes the full cycle non-obvious and breaks the desired
  one-command flow.
- Required fix: provide a governed owner-source-change path that stages source,
  records implementation truth, and refreshes installed provenance without
  relying on ad hoc operator sequencing.
