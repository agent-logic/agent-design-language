# Contributing to Agent Design Language

ADL owns public language/contracts and CLI dispatch. Start with the
[repository map](docs/REPOSITORIES.md) to choose the right project.

## External contributors

You can contribute using a public fork and an ordinary GitHub pull request.
You do not need private repository access, an installed C-SDLC owner, native
issue cards, or a maintainer's workstation paths.

1. Open a [bug report or proposal](https://github.com/agent-logic/agent-design-language/issues/new/choose)
   for a behavior change. Small documentation corrections can go straight to a PR.
2. Fork the repository on GitHub, clone your fork, and create a branch:

   ```sh
   git clone https://github.com/YOUR-ACCOUNT/agent-design-language.git
   cd agent-design-language
   git switch -c fix/public-docs
   ```

3. Read the changed package's documentation and relevant agent guidance. Keep
   the change bounded; preserve licenses, attribution and historical evidence.
4. Run the checks below and record their actual results. Push your branch to
   your fork and open a PR against `agent-logic/agent-design-language:main`.
   Describe the problem, changed behavior, tests and limitations. Link the issue.
5. Respond to review. Maintainers handle native governance, integration and
   closure; external contributors do not need to generate or edit native cards.

Sensitive reports follow [SECURITY.md](SECURITY.md), not public issue forms.

## Choose validation

Documentation changes: check relative paths/anchors and advertised commands
against current source, then run `git diff --check`. Do not run a command merely
because an old milestone record contains it. See the [public demo index](demos/README.md).

Public CLI changes:

```sh
bash tools/public_adl/run_cli_ci.sh
```

Other public library/compiler changes use the selected package's explicit
manifest; the complete public package lane is:

```sh
bash tools/public_adl/run_package_checks.sh
```

Rust and required components are pinned in [rust-toolchain.toml](rust-toolchain.toml).
A public build may download Rust tools and Cargo dependencies; it does not require
provider credentials or private sibling sources. Preserve deterministic default
tests. Installed-product, provider and cloud qualification belong to separate
owners and are not implied by public CI.

See [onboarding](docs/onboarding.md) for the first provider-free example and
[version compatibility](docs/COMPATIBILITY.md) for version meanings.

## Maintainers and managed agents

Internal issue delivery continues to follow [AGENTS.md](AGENTS.md) and the
[native maintainer workflow](docs/default_workflow.md). Keep primary `main` clean,
use the registered issue worktree and an issue-bound goal, and preserve the six
native lifecycle cards. Independently review the exact candidate before native
publication. Required CI, merge admission, finish and cleanup remain separate.
External fork contributions are reviewed and admitted by maintainers through this
process; the public route does not authorize internal agents to bypass it.

Review expectations for everyone are in [REVIEW.md](REVIEW.md). Record what ran,
what was inspected and what remains unproved. A source change or green public
check cannot establish a private-product release or deployment.
