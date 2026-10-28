## Outcome

Replace vacuous or scheduler-dependent validation with deterministic proof for Observatory redaction and Runtime hot-reload cancellation.

## Parent and findings

- Parent remediation issue: #522
- Source review: #520 at `fb6cbc7f619daa54f901fd2d12f480add682ace3`
- Findings: `D520-SEC-003`, `D520-TEST-001`

## Acceptance criteria

- OBS-B redaction scans the actual projected Runtime/UI/evidence publication surface, rejects representative secrets and provider payloads, and cannot pass on planning prose alone.
- Hot-reload cancellation tests synchronize on watcher receipt and pending-debounce state before revert or deletion.
- The tests prove the cancellation transition rather than merely observing that generation stayed zero.
- Repeated focused runs are deterministic without fixed scheduler sleeps as the proof mechanism.

## Owned paths

- `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh`
- `adl-runtime/tests/config_reload.rs`
- tightly coupled fixtures and validation manifests

## Validation

- Positive and negative redaction fixtures over publication-shaped data.
- Repeated focused hot-reload cancellation tests with an explicit handshake.

## Non-goals

- Observatory redesign.
- General watcher refactoring beyond the testability seam required for deterministic proof.
