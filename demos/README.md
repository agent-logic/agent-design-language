# ADL examples and demos

## Runnable from a public clone

The first example prints a plan using only public source and Cargo dependencies:

```sh
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

Expected stdout:

```text
Resolved run: v0-3-fork-join-seq-run
Workflow:     workflow
Steps:        3
  0. fork.branch.alpha  agent=coordinator provider=local_ollama task=branch_work
  1. fork.branch.beta  agent=coordinator provider=local_ollama task=branch_work
  2. fork.join  agent=coordinator provider=local_ollama task=join_results
```

The provider name comes from the example document. Printing the plan does not
connect to Ollama or execute those tasks. Cargo build messages and `adl_event`
diagnostics may appear on stderr. This example does not require credentials or
private sibling sources. See [build guidance](../docs/PUBLIC_BUILD.md).

[Public package checks](../tools/public_adl/run_package_checks.sh) exercise the
schema, UTS, language/compiler and legacy-contract libraries. They are developer
validation, not an installed Runtime demo.

## Separately installed owners

Runtime execution, CodeFriend reviews and C-SDLC delivery require independently
qualified owner installations. Start with the [repository map](../docs/REPOSITORIES.md)
and [public CLI selection contract](../adl/README.md#selected-product-owners).
Some owner repositories require access; a GitHub link or help entry is not an
installation or public availability promise. No owner demo is qualified by the
provider-free plan example above.

## Historical and currently non-runnable material

The [historical catalog](HISTORICAL_INDEX.md) preserves dated scenarios, migrated
owner links and prior proof references. It is not a list of supported commands.
In particular:

- The old v0.86 review-surface script is absent after the split; it is not a
  runnable public-clone example.
- Architecture document generation currently fails its section validator, and
  the arXiv fixture's source inventory references a removed Gödel path. Their
  existing scripts and assertions are retained; neither is advertised as a
  validated public walkthrough. Safety fixes alone do not repair those proofs.
- CodeBuddy and other packet generators produce fixture artifacts; they do not
  prove the represented reviews, skills, provider calls or product acceptance.
- Podcast production belongs to [The Cognitive Stack](https://github.com/agent-logic/the-cognitive-stack);
  see the [migration record](../docs/repository-split/PODCAST_MIGRATION_1285.md).

For generators that support create-only output, choose a new child path beneath
a temporary parent. Do not supply an already-created directory, and do not delete
existing output merely to make a rerun succeed. Consult the specific owner guide
and its current proof before running a historical generator.
