# Issue #1036 validation evidence

Current record-correction base: `e0bc7fddc1b13807a1d1da7400ab10611fb73946`.

- Historical native proof at `493d4b051335e43106c111d9535d22424df8aceb`: 6 tests passed, 0 failed, inputs unchanged.
- That proof became stale when lifecycle records were committed at `e0bc7fddc1b13807a1d1da7400ab10611fb73946` and remains historical evidence only.
- Native `csdlc validate 1036` is run after this SOR correction to verify lifecycle digest, all six card structures, and semantic projection health.
- `git diff --check` and a required-placeholder scan of the rendered SOR are run before commit.
- Per review direction, native proof is not rerun or committed during this correction. Current-head proof remains pending until the records settle after final exact-head review.

Review and integration truth:

- The exact-head review at `740977763b` required changes.
- The remediation head `5a3d37d2b5` received a clean exact-head review with no actionable findings.
- Later commits changed the head, so that clean review is historical and SRP remains pending.
- No pull request exists. The issue branch is unmerged and publication has not begun.
- No CI, merge, integration, terminal closeout, or current-proof claim is made.
