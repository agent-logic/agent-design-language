# Contributing

Keep work tied to an issue with a concrete outcome and acceptance criteria.
Use a dedicated `codex/<issue>-<description>` branch and an issue worktree on
FastWork. Preserve `main` as the integration checkout and preserve others' work.

Use the installed native Cognitive-SDLC interface when available. Discover its
current command contract and repository binding before preparing or binding an
issue; do not vendor lifecycle binaries, copy legacy wrappers, or invent cards.
Repository creation does not establish C-SDLC admission. If tooling cannot bind
this repository, record the exact limitation and resolve it before lifecycle
mutations. The bootstrap is tracked by ADL #1314.

Make small changes and run `python3 tools/check_repository.py` and
`git diff --check` from the repository root. The hygiene check examines tracked
text files; stage new intended files before running it. Add focused behavioral
checks when implementing code. This bootstrap does not prescribe an application
language or provide application test coverage.

Obtain an independent review before opening a PR and fix actionable findings.
Include the relevant closing issue reference, validation results, and remaining
limitations in the PR. Require passing applicable checks before merging and
keep native finish and worktree cleanup separate from merge. Keep repository
privacy unchanged unless the operator explicitly authorizes a change.

Do not commit credentials, personal data, generated build output, local evidence,
or model assets. Place build outputs under `.build/` and scratch under `.scratch/`
in the FastWork checkout, or use another explicit FastWork path. Do not deploy,
spend on cloud resources, or change organization permissions as incidental setup.
