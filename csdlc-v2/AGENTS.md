# C-SDLC v2 agent contract

- This workspace is clean-room and independent of ADL Runtime and incumbent C-SDLC implementation crates, schemas, tests, and fixtures.
- Use only the typed Rust owners and the nine thin contracts under `operator/skills/` for v2 lifecycle work.
- Cards are generated projections. Never edit Markdown/state directly; use `csdlc-edit` and markdown.rs AST validation.
- During Gate 10A, v1 remains the repository default and must stay installed, executable, tested, and recoverable.
- Install only into `.adl/bin/csdlc-v2/`; never target shared `.adl/bin/`. `csdlc-install verify` is fail closed on missing, symlinked, or non-executable binaries, missing v1 paths, selector drift, or provenance failures.
- No default switch or v1 deletion is in scope here.
