# C-SDLC v2 agent contract

- This workspace is clean-room and independent of ADL Runtime and incumbent C-SDLC implementation crates, schemas, tests, and fixtures.
- Use only the typed Rust owners and the nine thin contracts under `operator/skills/` for v2 lifecycle work.
- Cards are generated projections. Never edit Markdown/state directly; use `csdlc-edit` and markdown.rs AST validation.
- Read current authority only from `operator/generation-selector.json`. V1 must remain installed, executable, tested, recoverable, and explicitly selectable throughout the rollback window even after a reviewed v2 default switch.
- Install only into `.adl/bin/csdlc-v2/`; never target shared `.adl/bin/`. `csdlc-install verify` is fail closed on missing, symlinked, or non-executable binaries, missing v1 paths, selector drift, or provenance failures.
- Default switching is owned only by `csdlc-cutover`; v1 deletion is never authorized by that command.
