# C-SDLC v2 Gate 9 Cutover Decision

Decision: **PROCEED to Gate 10 operator migration and cutover review**

Issue: #5239

Default after this decision: **v1**

Rollback clock started: **no**

Importer-expiry clock started: **no**

## Why proceed

- All 15 required qualification scenarios have passing evidence. Merge and
  closeout are one atomic terminal transaction, so they have one interruption
  scenario rather than a fabricated persisted midpoint.
- Three representative packets were constructed automatically with all six
  cards, an issue design, and a Mermaid diagram.
- Normalized parity compared three representative cases with zero unexplained
  critical differences.
- The standalone implementation is 8,005 Rust LoC against the 8,000 review
  threshold. The bounded reviewer approved the five-line evidence-integrity
  exception (0.0625% above threshold).
- The complete suite is 73 tests against the 60–100 target and 150 ceiling;
  final warm full validation completes in 65.00 seconds.
- The exact-source clean seven-owner build completes in 198.83 seconds against
  the 209.275-second ceiling on the baseline measurement filesystem; median
  warm construction is 0.27 seconds against 0.8125.
- Seven installed stripped binaries total 19,676,608 bytes against the
  73,400,320-byte ceiling; the largest is 5,570,864 bytes against 15,728,640.
- Init plus doctor p95 is 0.96 seconds and bind p95 is 1.20 seconds over 21
  process-isolated fixtures.

The first clean build failed at 441.81 seconds. Gate 9 fixed the standalone
release profile and repeated the measurement from an empty target. This
decision relies on the corrected 198.83-second result and retains the failed
result, plus a contended 226.06-second exact-source `/tmp` run, in the validation
record.

## What this authorizes

Gate 10 may begin its reviewable operator-contract migration and deletion wave.
It must migrate root/nested AGENTS guidance, operator docs, the conductor, and
thin lifecycle/editor skills before changing the default or removing commands
that those contracts name.

This decision does not itself switch the default, delete v1, publish or merge a
Gate 10 PR, or start either compatibility clock. Those remain explicit Gate 10
state transitions after its own review and green checks.

## Required Gate 10 safeguards

- Preserve an executable rollback path for 14 calendar days after the actual
  default switch, then remove it unless a reviewed extension was approved
  before expiry.
- Preserve the read-only importer for 30 calendar days after the actual default
  switch, then remove it unless a reviewed extension was approved before
  expiry.
- Target at least 90% deletion of the pinned incumbent control plane. A measured
  80–89% result requires enumerated retained paths, owners, justification, and
  explicit cutover approval; below 80% fails.
- Keep the standalone workspace independently buildable and testable without
  ADL or Runtime.
- Split the deletion wave if one PR cannot preserve exact-revision review,
  rollback safety, and truthful closeout.

## Residual risks

- Live GitHub latency and availability remain external provider behavior; the
  local proof establishes fail-closed idempotent reconciliation.
- Operator-contract migration spans repository policy and skills. Gate 10 must
  prove no active contract still mandates a deleted v1 command.

Machine decision: `CUTOVER_DECISION.json`

Measured proof: `VALIDATION.md`

Scenario mapping: `SOAK_MATRIX.md`
