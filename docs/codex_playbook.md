# C-SDLC operator playbook

C-SDLC v3 is operational after V3-F/#505 and merged PR #591. Authority requires the native selector and authenticated reconciliation proof against canonical `origin/main`. Use `.adl/bin/native-v3/csdlc`; inspect its help and typed request contracts before invoking a lifecycle route. Missing or stale proof suspends authority. V2 is retained only for an explicitly authorized rollback or bounded transition remediation.

Follow [the default workflow](default_workflow.md) and root `AGENTS.md`.
Keep the primary checkout on clean `main`; implementation belongs in the bound
issue worktree. Preserve all six durable cards, use editor skills and typed
requests, and run focused validation. Independent review must be current for
the exact revision before publication. Terminal delivery and cleanup remain
separate typed operations. There is no `csdlc-closeout` writer.

Historical v1 commands in `docs/legacy/CODEX_PLAYBOOK_V1.md` are migration evidence.
