# Agent Design Language (ADL)

ADL is the public language, contract and command surface for Agent Logic systems.
It provides portable workflow presentation, schemas, tool contracts and a CLI that
routes product execution to separately installed owners. Runtime, CodeFriend and
Cognitive-SDLC implementations live in their own repositories after the #1223 split.

[Project website](https://agent-logic.ai) · [Repository map](docs/REPOSITORIES.md) ·
[Contributing](CONTRIBUTING.md) · [Review guide](REVIEW.md)

## What is in this repository

| Path | Purpose |
| --- | --- |
| `adl/` | Public `adl` and `adl-process` commands; installed-product dispatch |
| `adl-schema/` | Public schema package |
| `adl-uts/` | Public tool-contract package |
| `adl-v2/` | Retained public ADL v2 package |
| `adl-legacy-contracts/` | Explicit legacy compatibility contracts |
| `adl-spec/` | Language specification and schemas |
| `tools/public_adl/` | Public validation, coverage and owner-selection contracts |
| `docs/`, `demos/` | Current guidance and clearly bounded historical evidence |

The public CLI does not contain the private Runtime engine, CodeFriend application
or C-SDLC lifecycle implementation. A command being listed in help does not mean
its product owner is installed or qualified. See the [CLI guide](adl/README.md)
for the dispatch and installation-selection boundary.

## Get started

Install Git and Rust through rustup. The repository pins Rust 1.92.0 and its
components in [rust-toolchain.toml](rust-toolchain.toml). From a fresh checkout:

```sh
git clone https://github.com/agent-logic/agent-design-language.git
cd agent-design-language
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- --help
```

This builds and displays the public CLI. It does not install a product owner,
start a service, or call a model provider. There is no root Cargo workspace:
select the package manifest explicitly.

For execution, obtain an independently qualified installation from the relevant
[owner repository](docs/REPOSITORIES.md). Do not clone sibling private source
repositories merely to build the public CLI. Runtime selection uses
`ADL_RUNTIME_INSTALL_PREFIX` and `ADL_RUNTIME_ARCHIVE_SHA256`; follow the
[public selector contract](adl/README.md#selected-product-owners) and the owner's
artifact instructions rather than supplying an arbitrary executable.

## Try a provider-free example

```sh
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

The output lists three steps: two branches followed by a join. This prints a plan;
it does not call the provider named in the document. See the [expected output](demos/README.md#runnable-from-a-public-clone)
and [build environment and timings](docs/PUBLIC_BUILD.md).

## Development and validation

For a public CLI change, the focused validation entrypoint is:

```sh
bash tools/public_adl/run_cli_ci.sh
```

It checks the public dependency boundary, formatting, lint, named tests and
installed-owner dispatch/refusal contracts. It is a build/test command, not a
required step for reading documentation. Documentation edits use focused path,
link and command-source checks; see [CONTRIBUTING.md](CONTRIBUTING.md).
Product validation belongs to the corresponding owner. Public CI does not prove
Runtime installation, provider readiness, CodeFriend release or C-SDLC admission.

## Documentation

- [Contributor onboarding](docs/onboarding.md)
- [Documentation index](docs/README.md)
- [Language specification](adl-spec/README.md)
- [Version and compatibility guide](docs/COMPATIBILITY.md)
- [Roadmap and milestones](docs/ROADMAP.md)
- [Maintainer issue workflow](docs/default_workflow.md)
- [Agent instructions](AGENTS.md)
- [C-SDLC package manuals](docs/csdlc-v3/man/README.md)
- [Demo and historical proof index](demos/README.md)
- [Source-retirement evidence](tools/source_retirement/README.md)

Milestone, architecture and demo records retain their original scope and dates.
Historical monorepo commands are not current public CLI recipes. Source movement,
merged changes and passing CI are distinct from installed-product or release
acceptance; use the relevant owner's current evidence for those claims.

## Issues, security and licensing

Report public language/CLI issues in
[the ADL issue tracker](https://github.com/agent-logic/agent-design-language/issues).
Route product defects through the [repository map](docs/REPOSITORIES.md).
Follow [SECURITY.md](SECURITY.md) for sensitive reports. Licensing remains governed
by the repository's existing license files and package declarations; this refresh
does not change their terms.

## License

The public packages declaring `MIT OR Apache-2.0` are available under either
[MIT](LICENSE-MIT) or [Apache-2.0](LICENSE), at your option. Both license texts
are included in those package directories. Package-specific declarations and
third-party notices continue to apply; historical Apache-only packages retain
their existing terms.
