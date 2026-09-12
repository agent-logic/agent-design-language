# PR #948 CI repair

This supplement records the bounded #867 repair after the original PR head
`7b59601a9bd1c5b881dd7eda92b1f76b24e0c03f` failed standalone C-SDLC CI.
The original `../index.json` and source review remain historical evidence for
source `40c068b11a2cca7303f5fcaed01d33b2614db743`; they are not relabeled.

The real-issue canary expected construction fallback when the configured
worktree parent was unavailable on Linux, and expected legacy denial JSON on
stderr locally. It now checks the precise observed denial on structured stdout,
requires human stderr, preserves its state snapshot, and rejects construction
fallback on success. The recorder's three related identity arguments are grouped
into a tuple across all six callers to satisfy strict Clippy without a waiver.
No production lifecycle semantics changed in this repair.

## Local proof

- Full crate suite: 247 passed, zero failures; binary/doc targets with zero tests
  are retained in the log and contribute no proof count.
- Strict all-target Clippy, formatting, and diff checks passed.
- The refreshed baseline retains 17 attempts: 14 completed, two blocked and one
  interrupted. Separate recorder fixtures retain two failed attempts and one
  censored collection. Fixture clocks, fake transport, synthetic review inputs,
  the simulated external merge, and separate cleanup control retain the original
  documented limits.

`index.json` binds the repaired source commit, source subtree, changed-file hashes,
candidate digest, commands, test totals and retained artifact hashes. Tests ran
against the unchanged working repair before its source-only commit; the corpus
truthfully records the original HEAD and working diff. Published log normalization
only replaces host display prefixes and trailing blank lines, with original hashes
retained. CI results for the new PR revision still require a fresh remote run.

Independent review of the final source and this supplement must precede the PR
update. This supplement does not claim that review, green CI, merge, terminal
closeout, or live writer activation has occurred.
