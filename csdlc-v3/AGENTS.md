# C-SDLC v3 Agent Guidance

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

Root `AGENTS.md` governs operational work. The pre-cutover construction records
remain immutable evidence; they do not prohibit the approved native v3 routes.

- Keep lifecycle decisions deterministic, typed, explicit and independently testable.
- Preserve exact repository, branch, worktree, revision and digest checks.
- Inspect `csdlc --contract` and `csdlc <command> --describe` for the selected binary's declared input/effect/result contract. Descriptor discovery is not operational admission, and a candidate build is not evidence of active installation.
- Preserve legacy owner result fields when evolving the additive `/envelope` (`csdlc.v3.command_result.v1`). Keep process status, lifecycle status, authority, effects and evidence distinct; unknown or unreported values must not become no-effect claims.
- Keep operational local routes fail-closed when authority context is unavailable. Only explicit `local` selects historical construction inspection; never use it as automatic fallback.
- Keep `doctor`, `validate`, `eligibility`, `schedule` and `shepherd` observational: no lifecycle lock creation or interrupted-transaction replay. Report recovery-required state instead.
- Treat `proof` and `install` as guarded current native routes. `shadow` and `soak` retain inspection contracts with execution retired; historical grouping does not grant authority.
- Never replace a failed guard with a raw GitHub write or handwritten state.
- Preserve argv, status, stdout/stderr, timeout, cancellation, truncation and redaction distinctions at adapter boundaries.
- Cards and evidence views are generated projections. Use typed editors and validation, never direct Markdown/state edits.
- Operate only in the bound issue worktree; retain independent exact-head review before publication and separate terminal reconciliation from cleanup.
- The prepared-issue start target is three minutes once dependencies are satisfied, without weakening authority or proof.

See `operator/authority-selector.json`, `operator/native-authority-receipt.json`
and `docs/csdlc-v3/CURRENT_AUTHORITY.md`. Missing or stale authority
proof must be repaired through the declared typed route. An explicitly authorized
v2 transition exception must remain issue-scoped and must not switch the default.
