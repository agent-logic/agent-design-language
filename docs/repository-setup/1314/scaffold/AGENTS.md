# Agent guidelines

This is a private, language-neutral Agent Logic coding repository. Do not infer
product meaning, features, or a programming language from its name.

- Read the issue and CONTRIBUTING.md; agree on a bounded acceptance surface.
- Use issue branches and FastWork worktrees; keep primary `main` clean.
- Follow installed native C-SDLC contracts when applicable. Generated lifecycle
  cards belong to typed editors, never manual patches or retired wrappers.
- Preserve existing work and repository privacy. Do not change shared tooling,
  organization permissions, deployment settings, or cloud resources implicitly.
- Run `python3 tools/check_repository.py` and `git diff --check`, plus focused
  checks for changed behavior. Record exactly what ran and what remains untested.
- Obtain independent review before PR publication and fix actionable findings.
- Treat local proof, CI, review, merge, native finish, cleanup, and release as
  separate evidence. Never describe a scaffold as implemented product behavior.
- Keep credentials and generated artifacts out of Git; keep local builds and
  scratch on FastWork. Use `.build/` and `.scratch/` for disposable local output.

Initial repository setup is tracked by ADL #1314. Future work should establish
its own issue and select the implementation stack before adding build tooling.
