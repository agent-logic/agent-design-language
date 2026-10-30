# avp

Private Agent Logic coding repository, initialized under
[ADL #1315](https://github.com/agent-logic/agent-design-language/issues/1315).

Product requirements and the implementation language have not been selected.
This initial scaffold provides contribution guidance and repository hygiene
checks; it does not implement an application.

## Start contributing

Clone `https://github.com/agent-logic/avp.git` into `/Volumes/FastWork/avp`.
Keep issue worktrees under `/Volumes/FastWork/avp-worktrees` and all local
build outputs and scratch on FastWork. Read [CONTRIBUTING.md](CONTRIBUTING.md),
[AGENTS.md](AGENTS.md), and [SECURITY.md](SECURITY.md) before making changes.

The bootstrap check requires Git and Python 3.9 or later, with no third-party
packages:

```sh
python3 tools/check_repository.py
git diff --check
```

GitHub Actions runs the same repository hygiene check on pull requests and
pushes to `main`. Add stack-specific builds and meaningful behavioral tests
with the first implementation issue. Select requirements, language, tooling,
and licensing explicitly before introducing dependencies or distributing code.
