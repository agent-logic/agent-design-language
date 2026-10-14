# Coverage Authority And Release Proof

Current ADL coverage is public-source-only. The producer is
`tools/public_adl/coverage.py`, bounded by `tools/public_adl/coverage_inventory.json`.
The pull-request workflow runs its `pull_request` mode and enforces an 80%
line floor on changed public production files. The nightly workflow runs its
`full` mode and enforces a 90% aggregate and 80% per-file floor. Both retain
machine-readable result and execution artifacts; a local unit test alone is
not instrumented coverage proof.

| Surface | Purpose | Proof boundary |
| --- | --- | --- |
| `tools/public_adl/test_coverage.py` | Deterministic inventory, denominator and floor checks | Unit contract only; no instrumented run |
| `.github/workflows/ci.yaml` | Public PR/full coverage based on event | CI result at its exact commit |
| `.github/workflows/nightly-coverage-ratchet.yaml` | Scheduled full public coverage | Nightly result at its exact commit |

The former ADL-local `check_coverage_impact.sh`, mechanical-fallout classifier,
`run_pr_fast_coverage_lane.sh`, and `test_coverage_authority_contract.sh` served
the retired mixed Runtime/Infrastructure coverage family. Their source and
original assertions remain in Git and #1223 custody, but they are not current
ADL coverage routes. The old mechanical classifier's acceptance rules and
receipts cannot be inferred from a passing public coverage test. Runtime and
Infrastructure owner qualification is separate.

## Claim boundary

The retained `PUBLIC_COVERAGE_FULL_RESULT.json` in the #1223 decomposition
packet is evidence for its recorded source revision only. A release or merge
coverage claim requires the successful public producer result for the relevant
commit, with its denominator, floors, and retained artifacts. Source presence,
unit tests, and older results do not establish current-head coverage.
