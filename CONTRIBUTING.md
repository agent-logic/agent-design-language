# Contributing to Agent Design Language

Start with the [repository map](docs/REPOSITORIES.md). This repository owns public
language/contracts and CLI dispatch. Runtime implementation, providers, CodeFriend
and lifecycle tooling belong to their respective owners.

## Choose the surface

- Language meaning, schemas and versioning: `adl-spec/`, `adl-schema/` and the
  relevant public contract package.
- Public command behavior and installed-owner routing: `adl/`.
- Public test/coverage policy: `tools/public_adl/` and the current CI workflow.
- Current guidance: root entrypoints and `docs/`; preserve historical evidence.

Read [AGENTS.md](AGENTS.md), the [review guide](REVIEW.md) and any nested guidance
before editing. Propose a bounded issue with acceptance criteria and validation.
Use the [native lifecycle](docs/default_workflow.md), keep primary `main` clean,
and perform tracked work in the registered issue worktree. Create an issue-bound
session goal before implementation. Never hand-edit generated issue cards.

## Validation

Choose the smallest meaningful check for the changed surface. For public CLI work:

```sh
bash tools/public_adl/run_cli_ci.sh
```

There is no root Cargo workspace. Other public package changes use their explicit
manifest and current CI contract. Preserve deterministic, hermetic default tests;
provider, cloud and installed-product qualification remain separately declared
proof lanes. Do not build private owner products to validate documentation edits.

For documentation, check local paths/anchors, cross-repository targets and command
arguments against the current manifests and scripts, and run `git diff --check`.
Record which commands were inspected versus executed. Do not report existing
qualification as a new test run. Coverage rules and exceptions are defined by the
current [public coverage tooling](tools/public_adl/coverage.py), not a historical
percentage quoted in a guide.

## Review and delivery

Obtain independent exact-head review, fix actionable findings, record native
review and publish through the native owner. PR bodies must close the correct
tracked issue. Required CI and merge admission precede merge; native `finish`
then records terminal truth and `clean` separately handles the registered worktree.
A draft PR or green local check is not completion.

Keep changes surgical. Preserve licenses, attribution, source custody and historical
proof. A documentation correction cannot approve a release, alter a security policy
or establish installed authority. Follow [SECURITY.md](SECURITY.md) for sensitive
reports and use the [issue tracker](https://github.com/agent-logic/agent-design-language/issues)
for ordinary defects and proposals.
