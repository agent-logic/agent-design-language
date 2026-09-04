# Issue 630 design

Status: design-ready for bounded V3-H.4 execution.

Issue #630 implements non-authoritative C-SDLC v3 route behavior for terminal
truth, cleanup classification/removal, and cutover-decision planning under the
single `csdlc` binary.

The slice must keep v2 as live operational authority until #505 lands. The v3
routes may model and validate decisions, consume typed evidence structures, and
return fail-closed JSON. They must not mutate GitHub, finish live issues, remove
registered worktrees, retire v2, or claim cutover authority.

The implementation should prioritize simple, testable Rust surfaces:

- `csdlc finish` derives terminal outcomes only from authenticated typed
  PR/issue observations and rejects stale, nonmerged, self-attested, or `part_of`
  terminal claims.
- `csdlc clean` derives cleanup eligibility from actual Git worktree
  registration plus filesystem state, with distinct outcomes for absent,
  unregistered, dirty, live, already removed, removable, and removed.
- `csdlc cutover` records a non-authoritative decision packet requiring explicit
  operator approval, rollback evidence, selected binary provenance, and
  fail-closed undo boundaries.

Success is proving route behavior and denial cases with focused tests and a
real issue canary. Cutover itself remains out of scope.
