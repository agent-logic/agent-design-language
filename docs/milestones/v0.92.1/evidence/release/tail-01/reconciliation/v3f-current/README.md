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
#762 / PR #783. Publication therefore depends on #783.

`suite.json` and `suite.log` record the full `cargo test --locked --manifest-path
csdlc-v3/Cargo.toml` run from a clean detached checkout, using an external build
directory. Dependency cache warming is acceleration only. Passing tests alone
cannot resolve the four review-freshness rows.

`review.json` is the independent review result. A blocked result or any open
in-scope actionable finding prevents mapping acceptance. The mapping validator
also rejects stale source SHA, incomplete scope, stale fixture blobs, different
suite bytes, missing semantic criterion review, dirty execution, receipt drift,
and altered CORP-A dispositions. Its scope comparison includes additions and
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
