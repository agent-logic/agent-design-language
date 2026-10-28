# Native CI and coverage delivery (#1348)

`tools/ci_contracts` now owns public CI selection, selected-result aggregation,
LLVM report ownership and thresholds, coverage execution/receipts, nightly
reporting, canonical action pins, historical split validation and their tests.
The eight Python files allocated to #1348 are removed. Workflow, shell, cleanup
and Group-D callers use the native successors. Existing check names, hosted
platforms, coverage inventory and quality gates remain unchanged.

The Rust implementation preserves the 80% changed-file and full-file floors and
90% full aggregate floor. It rejects missing packages, duplicate ownership,
duplicate measurements, unlisted repository sources, hidden zero denominators,
invalid counts and production/test exclusion overlap. It uses isolated producer
targets and verifies clean inputs, exact HEAD and source hashes before accepting
coverage. Nightly reporting retains missing/malformed/failed-result refusal and
always writes its report before returning failure when receipt parsing fails.

Focused proof comprises 52 Rust tests, including the prior 18 policy cases,
6 selector tests, 10 coverage cases plus the seven-case nightly report test,
12 validation-interface tests, action pins and workflow wiring. The planning
case executes all 27 original negative mutations; the production planning command
runs those probes as well as the existing current-allocation Node suite. The
historical Python split validator stays untouched until #1347 removes it with
its preservation overlay. Original selector output matches native key/value
output; original six selector and ten coverage tests passed before removal.
`cargo clippy --all-targets -- -D warnings` and `git diff --check` passed locally.
Affected CI integration remains a separate publication gate.

## Explicit remaining dependencies

This is not full interpreter-free repository acceptance. The accepted #1345
ordering keeps the existing CLI coverage producers until #1349 atomically ports
those public CLI tests. The native coverage runner still launches those four
Python producer interfaces, preserving their instrumented coverage contribution.
Dropping them would reduce the denominator's exercised behavior.

The validation-interface tests are now native integration tests against actual
shell entrypoints. Their selector/manager/inventory subjects remain Python,
owned by #1351. The tests exercise public CLI failure, metadata, Git, report,
syntax and AWS-runner behavior without embedding a Python program. Two former
internal-only historical-function probes now go through the real public refusal
boundary instead of importing private implementation functions. #1351 must port
the subjects and update fixture provisioning, retaining these tests. Neither a
Rust test harness nor this delivery counts as completion of that production
migration. Python setup in the public install job stays until #1349 resolves its
remaining producers. Final absence and supported-host proof belong to #1352.

## Preservation and workflow

`ci-transition.json` records exact prior/current identities and retrievable Git
preimages. The existing guard composes this exact layer before #1346's Ruby
layer, then the immutable older transitions. Duplicate rows, caller drift,
restored removals, altered preimages and manifest tampering remain refusals;
unlisted drift remains visible. This transitional guard is still assigned to
#1350's native migration.

Operator-authorized Git/gh fallback continues after the recorded native prepare
regression in Cognitive-SDLC #331; no native reconciliation or cleanup success is
claimed. The pre-existing full historical-audit mismatch is tracked in #1356 and
must be resolved for final historical retirement acceptance.

PVF: deterministic local contract fixtures with bounded CPU/disk and no provider
or cloud calls; native compilation is a local prerequisite. Live coverage and
installed public qualification run in their existing CI lanes.
