# Issue #873 independent qualification

Frozen candidate `067cb99bf5c6220f64c9faadd7da6abdca34bcc4` passes the executed qualification gates. The operator explicitly waived further reviews after 35 review rounds; no fresh final-head review is claimed. See `operator-disposition.json`.

The current exact-source corpus contains 27 passing integration targets and five exact library regressions: 283 passing tests across 33 test commands, zero failures, and one regularly ignored release test that was also run explicitly and passed. The retained #872 validator passed separately. Two earlier conversion fixture-location rejections remain under `measurements/release-gate-fixture-admission/`; they are not counted as passing runs.

The corrected same-facts retry corpus is `measurements/retry-reduction/same-facts-067cb99/`. Both variants start unprepared on issue 1505, from the same fixture generator and fake-remote schedule. The predecessor took 35 attempts with three avoidable retries; the candidate took 32 attempts with zero: a 100% reduction. All declared steps and attempt correlations passed. The candidate executed a nonzero proof, review, publication, idempotent remote effect, finish, cleanup denial guards and exact cleanup in isolated fixtures. Authentic initialized #874 adoption is retained separately and excluded from comparison.

Exact-binary prepared-start samples were 4663, 4722 and 6873 milliseconds, below 180000 milliseconds. Conditions were warm-host; no OS-cold claim is made. Candidate aggregate CI passed. Evidence-branch CI is separate and is observed after publication.

Current result records, raw command logs, exact hashes and denominators are indexed by `focused-corpus-summary.json` and `corpus-manifest.json`. Earlier `secondary/` results are historical. Rejected exploratory runs remain local under `retry-operational/` and are excluded from this publication. Raw capture paths and dirty-state flags are preserved. Candidate source paths were verified unchanged at the exact revision; unrelated lifecycle artifacts explain observed checkout dirtiness.

Native adoption of the real #873 bound state refuses retained checkpoint evidence with `LegacyMigrationRequired`. `operational-admission/` preserves the failure. No native card edit, final proof, review or publication success is claimed. The operator authorized `gh` publication if necessary. This delivery does not replace the shared binary, activate writers, convert live state, call providers, merge or release.

Replay uses the explicit predecessor/candidate/compare phases in `run-retry-qualification.sh`. Compared fixtures are fresh and unprepared; synthetic transport lives outside lifecycle state. The authentic fixture is used only for its separate acceptance guard. The drivers do not impose a wall-clock timeout. Harness regression tests use deterministic local Python/Git fixtures under the existing qualification PVF lane; 59 tests passed.
