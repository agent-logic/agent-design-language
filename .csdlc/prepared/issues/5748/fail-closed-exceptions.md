# v0.91.8 terminal closeout exceptions

This register records issues that cannot receive a terminal receipt without
inventing or bypassing C-SDLC v2 authority. Entries remain provisional until
the final exhaustive closeout audit.

## #5558 — live v1 guidance and incomplete authority guard

- GitHub issue state: closed; PR #5749 merged from exact source head
  `033b28cffa6bdf191b1d013aa5a730ce7b10d9df`; declared focused tests, the
  owner lane, Gate 10A, and exact-SHA CI all pass.
- P1: the replacement guidance guard omits live root-referenced tooling docs
  and operational skills that still teach `adl/tools/pr.sh run`, so it reports
  alignment while current operator guidance contradicts Gate 10D2.
- P1: `adl/tools/editor_action.sh` still exposes and executes a `start` action
  mapped to `./adl/tools/pr.sh start`, and its test requires that sunset path.
- P2: the advertised full C-SDLC owner lane does not run the actual
  coexistence/final-authority Gate 10A proof, allowing the narrower guidance
  tests to pass despite stale executable and instructional surfaces.
- Disposition: no recordless terminal receipt until all live v1 routes and
  guidance are removed, the owner lane proves final authority, and the result
  receives a new exact-head review.

## Recovered tooling exception

- #5499 is no longer an exception. Typed historical recovery preserved the
  newer claim-free generation-20 review authority, refreshed GitHub linkage
  inside the terminal command, and produced a claim-free generation-21
  `closed_out` projection plus retained receipt. `csdlc-doctor` passes.
