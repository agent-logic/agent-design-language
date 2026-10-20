# Contributor onboarding

Start with [ADL's overview](../README.md), then try the public example below.
External contributors use [fork, branch and PR](../CONTRIBUTING.md#external-contributors).
Private owner tooling and the maintainer's worktree layout are not prerequisites.

## First public example

Install Git and Rust through rustup. From a public clone:

```sh
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

This loads the document and prints its plan without executing a workflow, starting
an installed product or calling a model. The [demo index](../demos/README.md)
shows expected output and separates public examples from installed-owner and
historical material. Rust/Cargo may download build dependencies on the first run.

## Reading and changing the project

- [Public CLI](../adl/README.md): command and installed-owner boundaries.
- [Language specification](../adl-spec/README.md): ADL 1.0 draft design.
- [Compatibility](COMPATIBILITY.md): CLI, document and contract versions.
- [Contribution guide](../CONTRIBUTING.md): public fork workflow and checks.
- [Review guide](../REVIEW.md): evidence and review expectations.
- [Documentation index](README.md): current guides and retained history.

There is no root Cargo workspace. Select a manifest explicitly; the [build
entrypoint guide](PUBLIC_BUILD.md) explains the nested workspace decision.
For docs changes, check links/anchors and command-source correspondence, then
run `git diff --check`. Do not build private owners for a public docs edit.

## Maintainer sessions

Managed agents and internal maintainers use [AGENTS.md](../AGENTS.md) and the
[native workflow](default_workflow.md). Native preparation writes Git metadata;
binding creates the registered issue worktree. Generated cards and receipts are
not hand-edited. This section applies to the maintainer environment, not an
external contributor's fork.

Historical milestones, reviews and ADRs retain their dates and original claims.
Do not promote their commands into current recipes without checking availability.
For sensitive reports follow [SECURITY.md](../SECURITY.md).
