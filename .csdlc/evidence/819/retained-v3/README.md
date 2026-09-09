# Issue #819 retained C-SDLC v3 proof

This packet consumes the exact 152-row `retained-v3.json` bucket declared by
the #764 denominator for finding `D520-RET-001`.

- 114 rows are backed by a named test observed passing in the retained
  candidate-bound C-SDLC v3 execution log. Each row also records exact Git blob
  and SHA-256 identities for its candidate source artifacts.
- 38 rows describe architecture that the operator-reviewed V3-F cutover did
  not retain. They are explicit governed amendments with
  `behavioral_pass_claim: false`; they are not represented as implementation
  passes.
- Zero rows remain unresolved.

`reconciliation.json` is the primary proof surface. The validator checks the
exact denominator, unique row consumption, command and log digests, observed
test names, exact candidate bytes, amendment non-pass semantics, and the
operator-reviewed cutover authority. The ten-case negative matrix proves that
denominator, command, candidate, proof-surface, resolution, artifact, and
amendment-integrity violations are rejected.

The packet proves retained-proof reconciliation at candidate
`fb6cbc7f619daa54f901fd2d12f480add682ace3`. It does not claim that superseded
implementation shapes were silently restored, and it does not cover the other
retained-proof buckets owned by sibling issues under #522.
