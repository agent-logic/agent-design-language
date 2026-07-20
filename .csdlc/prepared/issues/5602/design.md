# Issue 5602 design

Status: approved for bounded execution.

The authoritative coverage lane partitions test execution so profiles can be
collected with bounded concurrency. Each partition must run `cargo llvm-cov
nextest --no-report`; only the existing explicit post-partition `cargo
llvm-cov report` commands may render the combined ADL and Runtime summaries.
This removes redundant partition-local whole-workspace report rendering while
preserving every test, profile, summary, threshold, and fail-closed gate.
