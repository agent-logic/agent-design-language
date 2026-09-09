# C-SDLC v3 Agent Guidance

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

Root `AGENTS.md` governs operational work. The pre-cutover construction records
remain immutable evidence; they do not prohibit the approved native v3 routes.

- Keep lifecycle decisions deterministic, typed, explicit and independently testable.
- Preserve exact repository, branch, worktree, revision and digest checks.
- Never replace a failed guard with a raw GitHub write or handwritten state.
- Preserve argv, status, stdout/stderr, timeout, cancellation, truncation and redaction distinctions at adapter boundaries.
- Cards and evidence views are generated projections. Use typed editors and validation, never direct Markdown/state edits.
- Operate only in the bound issue worktree; retain independent exact-head review before publication and separate terminal reconciliation from cleanup.
- The prepared-issue start target is three minutes once dependencies are satisfied, without weakening authority or proof.

See `operator/authority-selector.json`, `operator/native-authority-receipt.json`
and `docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md`. Missing or stale authority
proof must be repaired through the declared typed route. An explicitly authorized
v2 transition exception must remain issue-scoped and must not switch the default.
