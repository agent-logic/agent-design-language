# Group D remediation proof (#1160)

Base: `6d78c1da473f95c4db4e0fa29f4fc2bd1d290106`. This is remediation of
internal review #919, coordinated by #921, with an explicitly reviewed draft PR
handoff. It does not rewrite the frozen #918/#919 candidate.

| Finding | Repair | Proving surface |
| --- | --- | --- |
| DEP-002 / SYN-005 | Digest-pinned Ubuntu index, exact Rust/rustup/AWS/Cargo tool versions, verification before direct extraction/execution, complete toolchain record | Infrastructure `adl/docker/adl-builder/test_builder_inputs.py` through `run_builder_contracts.py`: 4 tests, including changed bytes/missing hashes/mutable inputs |
| DEP-003 / SYN-004 | Reviewed raw-host identities; no latest/unchecked URL/unversioned S3 fallback; safe archive admission | Infrastructure `tools/aws_remote_validation/scripts/test_verified_bootstrap.py`: 24 retained tests, removed from this ADL adapter by #1223 |
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
- adl-resilience: 4 tests passed in the historical Group D record. The source
  and four original unit tests now belong to Runtime (`adl-resilience/`); #1223
  separately qualified the unchanged unit tests offline at Runtime revision
  `7d0db620ad79c50ff5d8fabf0fb543183583d05c`. This does not qualify a Runtime service.
- tools/remote_validation: 12 tests passed.

The dependency scaffold replay selected all 13 material Cargo lockfiles supplied
in the frozen packet and excluded 102 ordinary locks. It does not invent missing
manifests or claim a full repository dependency audit. Installed skill files were
not replaced; the fix is in the reviewed repository skill source.

The native Cargo adapter executes the actual shell/Python suites as six Rust
tests after the #1223 Infrastructure bootstrap transfer. The original seven-test
adapter remains historical evidence; its bootstrap test is not a current ADL pass. The three Cargo crate suites are separately retained local proof; Linux CI
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

## CodeFriend dependency contract resources

The dependency-inventory adapter requires an explicit absolute
`CODEFRIEND_REVIEW_SKILLS_ROOT` pointing to `review-skills/` extracted from the
authenticated CodeFriend artifact11803531674 (candidate
`e93a457ad6325017627a66a2abe31a3376fa28fd`). The local adapter verifies all seven
files of the dependency-review family against that artifact before running its
original six Python cases. Missing or changed resources fail the test; they are
never skipped. This is owner-fixture qualification, not a current dependency audit.

## Infrastructure builder contract resources

The builder adapter requires an explicit absolute `ADL_INFRASTRUCTURE_SOURCE_ROOT`
containing the reviewed Infrastructure source revision
`61c2806b6274b1003aa42f02adec99faf5f2a74b`. It verifies all ten builder-family
resource hashes before executing the unchanged original four Python tests.
Missing or changed resources fail closed. The legacy builder validation lane
uses `--suite setup-import` to execute its original setup and import shell tests.

The #1223 external qualification ran four original commands once: four Python
methods plus three shell contracts, all passed. The setup contract substitutes
its own fake Docker executable; publication/import checks are syntax and static
contracts. This proves no actual image build, registry push, AWS execution,
installed image, or cloud acceptance. These results do not rerun Group D's other
five adapters or supersede their separate proof obligations.
