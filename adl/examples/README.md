# adl/examples

ADL documents and retained historical execution fixtures. The public CLI can
print supported portable plans; workflow execution belongs to a separately
qualified Runtime installation. This is the crate-local inventory. Start with
the [public demo index](../../demos/README.md) for current runnable examples.

## Print a plan from a public clone

From the repository root:

```sh
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

This prints an inert plan without provider credentials or calls. See the
[expected output](../../demos/README.md#runnable-from-a-public-clone).

## Runtime-owned execution

`--run`, `--trace`, resume and provider-profile operations require an
owner-qualified Runtime installation selected through `ADL_RUNTIME_INSTALL_PREFIX`
and its exact `ADL_RUNTIME_ARCHIVE_SHA256`. Follow the
[public CLI owner contract](../README.md#selected-product-owners) and the
[Runtime owner](https://github.com/agent-logic/agent-logic-runtime) instructions.
A source checkout or mock-provider script alone does not qualify that installation.

The old `ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh` example was an execution
fixture for the former monolithic Runtime. It is not a standalone public-clone
execution recipe or current provider qualification.

## Example Families (Short Index)

- v0.3: concurrency, retry, bounded AEE recovery, remote provider (`v0-3-*`)
- v0.5: primitives, patterns, remote execution MVP (`v0-5-*`)
- v0.6: HITL pause/resume, profiles, delegation, instrumentation demos
- v0.7: provider portability / HTTP profile compatibility proof surface
- v0.87.1: minimal no-network runtime quickstart (`v0-87-1-minimal-runtime-demo.adl.yaml`)
- v0.87.1: no-network mock provider family proof (`v0-87-1-provider-mock-demo.adl.yaml`)
- v0.87.1: bounded HTTP family proof (`v0-87-1-provider-http-demo.adl.yaml`)
- v0.87.1: local Ollama family proof (`v0-87-1-provider-local-ollama-demo.adl.yaml`)
- v0.87.1: ChatGPT family proof (`v0-87-1-provider-chatgpt-demo.adl.yaml`)

## ADL Language Note

ADL has six core language primitives:

- `providers`
- `tools`
- `agents`
- `tasks`
- `workflows`
- `run`

The document surface uses singular `run` for the execution primitive.

`patterns` and `signature` may also appear as top-level language features, but
they are not counted as additional primitives.

This examples directory demonstrates the language surface itself. It is not the
home for later control-plane contracts or milestone-local packet schemas.

## See Also / Canonical Docs

- Canonical demo index: `../../demos/README.md`
- Provider demo proof governance: `../../docs/tooling/PROVIDER_DEMO_SURFACES.md`
- v0.7 demo matrix (canonical): [archived v0.7 demo matrix](../../docs/archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.7/DEMOS_v0.7.md")
- v0.6 release notes: [archived v0.6 release notes](../../docs/archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.6/RELEASE_NOTES_v0.6.md")
- Runtime/CLI usage: `../README.md`
- ADRs: `../../docs/adr/`
- Root project entrypoint: `../../README.md`
