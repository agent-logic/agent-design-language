# Issue 18 Design: Clean C-SDLC Broken-Pipe Termination

## Outcome

Route split C-SDLC machine-readable output through one library writer that
treats an early-closing stdout reader as normal termination, while preserving
JSON-only stdout and existing command exit semantics for real failures.

## Implementation Boundary

- Add a shared C-SDLC v2 stdout JSON writer with explicit `BrokenPipe`
  handling and ordinary I/O failure propagation.
- Route `csdlc-github-issue` and `csdlc-github-pr` success, schema, and typed
  error payloads through that writer.
- Add focused process-level regression tests that close each split binary's
  schema pipe early and reject panic, backtrace, or broken-pipe diagnostics.
- Document the machine-output termination contract at the existing GitHub
  client boundary.

## Invariants

- Machine-readable JSON remains on stdout.
- Human diagnostics remain on stderr.
- `BrokenPipe` is successful downstream termination only while writing stdout;
  unrelated serialization and I/O failures remain failures.
- Issue and PR action routing, schemas, credentials, and GitHub behavior do not
  change.
- No AWS or remote builder is used.

## Proof

Focused library and `gate_github_actions` tests exercise the shared writer and
both split binaries. Warning-denied Clippy, formatter checks, and typed doctor
provide the remaining local proof before independent exact-head review.

## Rollback

Revert issue #18 changes. The prior output behavior returns without changing
any GitHub action semantics or stored lifecycle state.
