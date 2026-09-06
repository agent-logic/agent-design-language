### Verdict
**FAIL — Cutover readiness remains blocked by unresolved P1 storage and CI defects.**

While the provided diff successfully repairs the remote command's evidence derivation, the cleanup Git registration validation, and the historical sprint/issue states, several critical P1 defects identified in the Sprint 5/6 synthesis remain untouched in this changeset.

### Actionable findings ordered by severity

1. **P1 — Required CI does not execute the standalone `csdlc-v3` crate:** The diff contains no changes to `adl/tools/ci_path_policy.sh` or `.github/workflows/ci.yaml`. v3 code can still merge with failing tests or strict-Clippy violations.
2. **P1 — Durable reopen ignores interrupted intents:** The diff contains no changes to `csdlc-v3/src/storage/mod.rs`. The recovery classifier is still bypassed during `DurableTransactionStore::open`.
3. **P1 — Failed durable state replacement leaves in-memory authority advanced:** No changes were made to `csdlc-v3/src/storage/mod.rs` to rollback or poison the in-memory store if the atomic state write fails.
4. **P1 — Projection repair cannot truthfully represent an unanticipated post-commit write failure:** No changes were made to `csdlc-v3/src/storage/mod.rs` to handle projection write failures after canonical state commits.
5. **P2 — V2 import accepts unverified issue and card digest labels:** The diff contains no changes to `csdlc-v3/src/application/mod.rs` to recompute and verify digest strings against the record/AST.

### What looks repaired

* **End-to-end derivation from typed evidence (Focus 1):** `csdlc-v3/src/commands/remote/mod.rs` now implements `verify_remote_bridge_request` and `validate_evidence_ref`. It explicitly rejects `caller:` and `inline:` prefixes, enforces JSON schemas, and derives the result from hashed, repo-local evidence files rather than caller-manufactured strings.
* **Cleanup identity Git registration (Focus 2):** `csdlc-v3/src/publication/mod.rs` now includes `validate_git_worktree_registration_files`, which cross-validates the `<worktree>/.git` file and the `<repo>/.git/worktrees/<name>/gitdir` file before accepting a cleanup identity digest.
* **Failed historical issues reopened (Focus 3):** `.csdlc/prepared/issues/505/validate-authority-transition-prep.rb` and `remediation-status.md` confirm that failed issues (#501, #502, #503, #504, #533, #596) were truthfully reopened using typed v2, and Sprint 6 umbrella membership v5 is correctly captured and retained.
* **v3 live authority claim (Focus 4):** `csdlc-v3/src/commands/remote/mod.rs` now hardcodes `operational_authority: false` and `trusted_authority: false`, explicitly returning a blocker stating that v2 remains the operational authority until #505 explicitly switches it.

### Evidence limits
* The review is strictly limited to the provided `git-diff-stat` and diff contents.
* No live commands, CI pipelines, or GitHub mutations were executed.
* The underlying `derive_finish` logic in `csdlc-v3/src/publication/mod.rs` was not shown in the diff, so it is unclear if the terminal linkage gap (P1) was fully resolved beyond the remote command parsing the readbacks.

### Next exact validation ideas
1. Inspect `.github/workflows/ci.yaml` to ensure a dedicated `csdlc-v3` lane exists with `--locked` cargo test and clippy commands.
2. Review `csdlc-v3/src/storage/mod.rs` to verify that `DurableTransactionStore::open` invokes the recovery classifier and that `commit` safely handles disk write failures.
3. Review `csdlc-v3/src/application/mod.rs` to ensure v2 imports recompute and validate integrity digests.

GEMINI_ACTIONABLE_FINDINGS=5
