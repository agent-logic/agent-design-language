# Current V3-F proof refresh — issue 771

This additive packet joins independent substantive review to a full locked
C-SDLC v3 suite at one immutable source commit. It preserves the historical
`current-exceptions.json` and `census.json`; it does not rewrite old receipt
identities, reopen the three resolved CORP-A rows, or authorize release.

`assignment.json` fixes the reviewer, source SHA and complete coupled blob
scope before review. `historical-fixture.diff` retains the exact historical
`c5d67168df2c923eb77c592b7545386ad0a39234` to candidate
`556cb4957407772abfbe135f3e320f1936011c3e` delta. The newly reviewed source
includes the detached-fixture repair in #763 and proof ownership changes in
#762 / PR #783. PR #783 and schema repair #776 are now merged and included in the frozen source. The separately approved terminal receipt-conflict repair is also included.

`suite.json` and `suite.log` record the full `cargo test --locked --manifest-path
csdlc-v3/Cargo.toml` run from a clean detached checkout, using an external build
directory. Dependency cache warming is acceleration only. Passing tests alone
cannot resolve the four review-freshness rows.

`reviews/` retains each independently authored lane receipt; `review.json` aggregates their identities, exact path coverage and findings. The previous blocked packet remains under `archive/fba2bdf5c7/`. A blocked result or any open
in-scope actionable finding prevents mapping acceptance. The mapping validator
also rejects stale source SHA, incomplete scope, stale fixture blobs, different
suite bytes, missing semantic criterion review, dirty execution, receipt drift,
and altered CORP-A dispositions. Historical census and exception bytes must match
their immutable Git blobs at the reviewed source SHA; updating both a baseline
and its mapping hash does not authorize historical changes. Its scope comparison includes additions and
deletions under the declared source prefixes. Evidence-only descendant commits
may carry the packet; any change to the coupled source invalidates it.

Run `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py --v3f-current`
for current validation, and add `--negative` for substitution tests after an
admissible baseline exists. An absent or blocked review is a blocking result,
not a successful proof. Historical reconciliation commands remain unchanged.

PVF: required deterministic local mapping contract, small CPU/Git and no
credentials or network. Full detached component suite: required deterministic
local proof, medium CPU/disk, external Cargo target. Independent substantive
review is required separately. No cloud, UI demo, merge or release action.

Validator contract tests: `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/test_validate.py`. These use explicitly synthetic receipts in isolated Git fixtures and are not V3-F execution or substantive review proof. They cover stale substitutions, blocked review, dirty/staged/untracked/committed source drift, filtered test logs, evidence-only descendants, and rewritten historical baselines with refreshed hashes.

Team review acceptance checks every preassigned lane receipt, reviewer identity, independent status, exact source/scope, parsed timestamp order, findings and criterion assessments. An aggregate cannot override a blocked or missing component. Current independent component review passes at7fecd63398bb37530801dbf3375d47f73426d637 and binds the197-test detached suite. Earlier blocked components remain immutable in archive/.

Planning provenance: `.csdlc/prepared/issues/771/design.md` retains the original review-only design and its approved digest. Its original no-product-change boundary was superseded for the three named repairs by explicit operator approvals and the later typed STP/SPP replans recorded in `.csdlc/issues/771/audit.jsonl`. It is historical planning evidence, not a restriction overriding those approvals.
