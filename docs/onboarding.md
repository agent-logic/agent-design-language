# Contributor onboarding

Start with the [project README](../README.md), [repository map](REPOSITORIES.md)
and [contribution guide](../CONTRIBUTING.md). Use this checkout for public ADL
language/contracts and CLI behavior; use the product owner for Runtime,
CodeFriend or lifecycle implementation work.

## Reading order

1. [Public CLI guide](../adl/README.md) and [language specification](../adl-spec/README.md).
2. [Agent rules](../AGENTS.md) and [review expectations](../REVIEW.md).
3. [Default workflow](default_workflow.md) and [package manual lookup](csdlc-v3/man/README.md).
4. The changed package's manifest, source and focused validation script.

There is no root Cargo workspace. Rust is pinned in `rust-toolchain.toml`.
The root README's explicit-manifest help command builds only the public CLI;
product execution needs its separately qualified owner installation.

## Work and evidence locations

Keep primary `main` clean. Native preparation uses resolved Git metadata; native
binding creates the policy-approved issue worktree. Work there with an issue-bound
goal and six truthful lifecycle cards. Never create issue artifacts on primary or
edit generated card Markdown and receipts directly.

Current guidance belongs in README, contribution, review and relevant documentation
entrypoints. Milestone records, past reviews, ADRs and retirement custody retain
their original meaning. Link to them as historical evidence, not current command
recipes. Check ignored report directories before citing local-only artifacts as
portable evidence.

For docs changes, check paths/links and command-source correspondence, then run
`git diff --check`. For public CLI changes use `bash tools/public_adl/run_cli_ci.sh`.
Record actual validation separately from owner qualification, CI, merge and native
finish. Follow [SECURITY.md](../SECURITY.md) for sensitive reports.
