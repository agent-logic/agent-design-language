# Issue #1098 validation

Scope: staged cleanup evidence preservation and historical retry outcome truth following merged PR #1093.

- Five new installed-command tests passed: staged evidence refusal, retained successful retry reconciliation, pending success without receipt, unavailable readback, changed PR.
- One new archive unit test passed: staged modified evidence refused in ordinary and partial cleanup; unstaged evidence remains admissible.
- Five existing issue_1092 installed regressions passed, including fresh authenticated absence retirement.
- Existing installed merge/finish/interrupted-cleanup test passed.
- Strict all-target Clippy, Rust formatting and diff whitespace checks passed.

PVF: deterministic local owner and integration tests; synthetic GitHub transport, local Git and filesystem, small CPU. Required defect regression proof. No live worktrees removed and no real publication effects used as tests. CI integration is separately required before handoff.

The cleanup rule deliberately refuses all staged changes because the archive retains filesystem bytes, not index state. A publication with unavailable or mismatched readback remains pending; a successful retained receipt follows ordinary authenticated reconciliation. Idempotent reconciliation may truthfully report no new mutation while recording successful publication.

## CI module-boundary correction

The first CI run (35543879048) rejected storage calling transport. Fresh absence orchestration now lives in the existing remote intent owner; storage only authenticates retained local eligibility, and transport matches the base. All three `remote_module_decomposition` tests pass, and that test target is now a declared native validator. All six issue_1098 regressions, formatting, strict Clippy, and diff hygiene passed again after the relocation. Independent bounded review found no new defect. Renewed exact-head proof and CI are required for this candidate.
