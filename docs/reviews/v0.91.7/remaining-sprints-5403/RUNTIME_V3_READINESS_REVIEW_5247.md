# Runtime v3 Cutover Readiness Sprint Review

Issue: #5247
Review issue: #5403
Status: changes required
Remediation: #5412; shared records issue #5406

## Findings

### P1: Restored memory checkpoints are unauthenticated

`MemoryCheckpoint` contains facts, private references, sequence, and head hash
but no signature at `adl-runtime-kernel/src/identity_memory.rs:49-57`. The
restore path around lines 285-300 verifies only the separate identity binding
and then trusts the checkpoint contents.

Impact: a fabricated memory summary or continuity head can be injected while
the retained packet claims recovery integrity at
`docs/architecture/runtime_v3_identity_memory_5250.v1.json:14`.

Disposition: open. Route a #5250 security repair that authenticates the full
checkpoint and binds it to identity, sequence, and continuity lineage, with
forgery and substitution tests.

### P1: Private-state projection does not prove authenticity or accepted lineage

`project_private_state` at `adl-runtime-kernel/src/private_state.rs:158` checks
caller policy, a caller-supplied projection hash, and an unsigned record hash.
The path around lines 174 and 201 neither calls `verify_record` nor proves the
record was accepted into `PrivateStateLineage`.

Impact: a forged record and matching forged projection can pass the public API,
contradicting the boundary claimed at
`docs/architecture/runtime_v3_private_state_security_5249.v1.json:10`.

Disposition: open. Route a #5249 security repair requiring authenticated record
verification and accepted-lineage membership before projection.

### P2: Normal validation does not exercise the retained production-like soak

`adl-runtime-kernel/tests/guardian_soak.rs:98` asserts checked-in packet values.
The actual 100-cycle soak is ignored around line 596.

Impact: historical evidence remains readable, but normal validation cannot
detect current guardian/soak regressions.

Disposition: open. Route a bounded release or scheduled lane for the real soak,
while keeping ordinary PR validation fast.

### P2: Runtime v3 exceeds the 10K target without a located reviewed exception

Current `adl-runtime-kernel/src/**/*.rs` totals 10,461 lines. The parity matrix
sets 10,000 as the target and 20,000 as the exception ceiling at
`docs/architecture/runtime_v3_parity_matrix.v1.json:6`. The release packet at
`docs/architecture/runtime_v3_release_proof_gate_5220.v1.json:25` retains a
stale 10,324 count and explicitly makes no new LoC claim.

Impact: the target has been crossed without a current reviewed disposition or
growth account.

Disposition: open. Record a reviewed exception with ownership and reduction
plan, or reduce source below 10K before cutover acceptance.

### P2: Issue-level lifecycle closeout is not retained

No tracked six-card lifecycle bundles exist for #5247-#5254. The later
reconciliation samples only #5174, #5227, and #5285 at
`docs/architecture/runtime_v3_closeout_truth_5385.v1.json:11`, and around line
44 says prior review truth was not reconstructable for a sampled sprint.

Impact: merged GitHub state is retained, but exact review, validation, and
closeout truth cannot be audited for the full child wave.

Disposition: route through the shared typed-v2 records-retention remediation.

## Child Coverage

Reviewed #5248 through #5254 and merged PRs #5259, #5261, #5262, #5264, #5266,
#5269, and #5271. All child issues are closed and PRs merged.

## Testing-Discovered Defects

No new failing bug was found by the specialist's current test run. The retained
Horust 0.1.13 restart-budget defect remains correctly classified and prevents
Horust qualification; its reproducer is intentionally ignored without
`ADL_HORUST_BIN`. All five findings above are review-discovered; no
test-discovered defect is counted above.

## Review Result

Changes required. The two P1 authenticity defects invalidate the affected
private-state and memory-recovery integrity claims. Runtime v3 remains opt-in;
this packet does not authorize default cutover or Runtime v2 decommission.
