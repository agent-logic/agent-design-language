# Design: deterministic process-backend terminal precedence

## Context

Required Runtime CI run `31563230539`, job `94009803658`, observed `timeout` from `process_backend_timeout_and_oversized_file_leave_no_artifacts` while the oversized file path expected `output_limit`. The process backend currently waits for child completion under one timeout and checks file size only afterward, so scheduler timing can allow the deadline branch to win after the child has already materialized oversized output.

## Decision

Define a deterministic arbitration boundary in the process backend: when the execution deadline fires, terminate and reap the owned process tree, then inspect the file-output path using the same bounded size rule before returning the terminal classification. If oversized output is observably present at that server-owned boundary, `output_limit` takes precedence; otherwise the terminal result remains `timeout`. Cleanup remains guard-owned and must remove the output artifact on every return path.

The correction is limited to the process-backend implementation, its parity fixture/test, and issue evidence. It does not touch conversation cleanup hooks, #244, or #112 authority surfaces.

## Proof

- Repeated focused pressure alternates hanging stdout and oversized file-output cases and asserts stable terminal codes plus an empty output root.
- Existing parity coverage retains ordinary timeout, output-limit, cancellation/process-tree, and cleanup behavior.
- The required Runtime lane, strict Clippy, Observatory proof, and fresh exact-head review must pass before publication.

## Non-claims

This does not alter conversation admission, birthday authority, cancellation policy, general deadline duration, or optional CI topology.
