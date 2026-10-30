# Public build entrypoints

Install Git and rustup; the checkout selects Rust 1.92.0 plus its listed components.
From repository root, the optional Make targets delegate to existing commands:

| Command | Effect |
| --- | --- |
| `make build` | Build public `adl` using `adl/Cargo.toml` and its lockfile. |
| `make plan` | Build and print the provider-free three-step example. |
| `make check` | Run `tools/public_adl/run_cli_ci.sh`. |
| `make packages` | Run `tools/public_adl/run_package_checks.sh` for five public packages. |
| `make docs` | Check current public links/anchors and command/version contracts, including negative fixtures. |

`make docs` runs the Rust public-contract checker using Git, Make and local files.
Make is only a convenience; all underlying commands remain available in
[CONTRIBUTING.md](../CONTRIBUTING.md) and [the demo index](../demos/README.md).
No target installs a private product or calls a model provider. Build dependencies
may be downloaded by rustup/Cargo. Public checks do not imply owner qualification.

## Documentation changes in CI

Prose-only README entrypoints use repository contracts and path checks without
compiling public packages, collecting coverage or building installed demos.
The selector explicitly lists these entrypoints; arbitrary nested READMEs,
Markdown fixtures, schemas, templates and packaged resources retain conservative
validation. Source, lockfile, workflow and policy changes keep their existing
gates, including when mixed with documentation edits.

## Why keep explicit manifests?

An isolated experiment on the public base revision grouped the six selected
CLI/library/compiler packages into a root workspace. `cargo metadata --offline
--no-deps --format-version 1` accepted the group and preserved package versions.
Full `cargo metadata --offline --locked --format-version 1` refused because a new
root lockfile was required. Metadata acceptance alone does not prove compatible
resolved dependencies, package output or CI behavior.

This change therefore retains independent lockfiles and the nested `adl-v2`
workspace. The Make targets solve root-command discovery without introducing a
new dependency-resolution contract. A later workspace migration would need a
reviewed root lockfile, nested package/version handling and package/CI parity
proof; it is not necessary to contribute or run the public example.

## First-build measurement

On 2026-10-20, a fresh depth-1 HTTPS clone at
`8cf07d2208b24906b93da27f08e3ef184bf49869` built and ran the documented plan
command successfully in **8.712 seconds** wall time (Cargo reported 8.62 seconds).
Environment: macOS 27.0.1, arm64, 14 logical CPUs, Rust 1.92.0. The target directory
was new and empty. The installed toolchain and host Cargo registry cache were
reused; OS caches were uncontrolled. Provider and installed-owner overrides were
removed from the child environment. Expected stdout matched the three-step plan.

This is a first-build measurement for that empty target, not a promise for another
machine or a fully cold dependency/toolchain download. Clone duration was not
measured; no dependency-download duration is claimed. No private sibling source,
provider call or separately installed owner was required. See the
[retained measurement](public-journey-proof.json).
