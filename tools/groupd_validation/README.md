# Group D remediation proof (#1160)

Base: `6d78c1da473f95c4db4e0fa29f4fc2bd1d290106`. This is remediation of
internal review #919, coordinated by #921, with an explicitly reviewed draft PR
handoff. It does not rewrite the frozen #918/#919 candidate.

| Finding | Repair | Proving surface |
| --- | --- | --- |
| DEP-002 / SYN-005 | Digest-pinned Ubuntu index, exact Rust/rustup/AWS/Cargo tool versions, verification before direct extraction/execution, complete toolchain record | `adl/docker/adl-builder/test_builder_inputs.py`: 4 tests, including changed bytes/missing hashes/mutable inputs |
| DEP-003 / SYN-004 | Reviewed raw-host identities; no latest/unchecked URL/unversioned S3 fallback; safe archive admission | `tools/aws_remote_validation/scripts/test_verified_bootstrap.py`: 24 tests |
| DEP-004 / SYN-007 | Ecosystem inventory without lifecycle-lock pollution or 80-item truncation; explicit packet denominator | `adl/tools/skills/repo-dependency-review/scripts/test_prepare_dependency_review.py`: 6 CLI tests |
| SYN-006 | Three explicit Cargo CI jobs, mixed-path ownership, fail-closed aggregate result checks | Selector + CI path-policy suites; 24 aggregate combinations and real Git path fixtures; affected crates below |
| DOC-001 | Current incomplete/deferred qualification routing; #1148/#1149 then #1150 in v0.93.1; Runtime v4 in v0.93.2 | `test_documentation.py`: 3 tests, including rejected stale routing |
| DOC-SUP-001 | Current parser documentation matches shared source bounds and links canonical contract | Same documentation suite |
| CI-1160-001 (subsequent finding) | Exact production projection around existing top-level `cfg(test)` module bodies avoids production percentage gate for test-only edits | `adl/tools/test_coverage_test_only.py`: 5 tests, including actual #1151 fixture when historical Git objects exist; actual shell gate keeps mixed/production/malformed/unknown failures |

The subsequent CI finding was assigned after the frozen review: #1151's
`36ec0814683d0c8cc136bc9af27a0163ed506460` changes only its bad-signature test
fixture, but full-file coverage remained 3,170/4,804 (65.99%). The correction does
not lower the 80% threshold, allowlist a source file, bypass selected tests, or
skip coverage generation. It compares exact Git-derived production segments
including test module headers, names, delimiters and all following production
code. Unknown syntax/objects and mixed edits keep the original coverage gate.
The existing `test_check_coverage_impact.sh` suite also passed locally.

All three independent Cargo roots passed on Rust 1.92.0 with `--locked` tests,
format checks and strict all-target Clippy:

- adl-characterization: 35 tests passed.
- adl-resilience: 4 tests passed.
- tools/remote_validation: 12 tests passed.

The dependency scaffold replay selected all 13 material Cargo lockfiles supplied
in the frozen packet and excluded 102 ordinary locks. It does not invent missing
manifests or claim a full repository dependency audit. Installed skill files were
not replaced; the fix is in the reviewed repository skill source.

The native Cargo adapter executes the actual shell/Python suites as seven Rust
tests. The three Cargo crate suites are separately retained local proof; Linux CI
runs the three new jobs. No zero-test success is treated as proof. The historical
#1151 comparison can be skipped in shallow CI when its objects are absent; the
portable synthetic-Git positive/negative gate cases remain mandatory.

Independent review found a misplaced duplicate aggregate block and two omitted
adapter suites; both were fixed and re-reviewed. Final exact-head independent
review and native receipts are retained under `.csdlc/evidence/1160/` at execution.

## Limits

No container image was built, published, or executed; no AWS/provider workload
was run. The pinned direct downloads do not make apt resolution hermetic.
Official source identity provenance is in the builder's `INPUT_PROVENANCE.json`.
Raw-host package/preinstalled tools retain the documented trusted-host boundary;
cold hosts need approved identities or an approved preinstalled toolchain.
#916 and #922 owner changes are credited at their exact revisions in the current
release overlays; their worktrees and retained historical evidence are unchanged.
Local validation and reviewed draft publication are not merge, release acceptance,
Beta 1 qualification, or cloud execution evidence.
