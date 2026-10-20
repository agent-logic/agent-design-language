# ADL v2 source layout

This directory contains the public `adl-language` and `adl-compiler` workspace
members. Use an explicit manifest; the root public CLI is a separate package:

```sh
cargo test --locked --manifest-path adl-v2/Cargo.toml
```

The sibling `adl-cli`, `adl-engine` and `adl-records` source trees
are **retained historical, unbuilt packages**, not additional supported workspace
members or installable public products. Their inherited manifests cannot be used
as standalone package builds in this checkout. Do not add them to the workspace
to work around that diagnostic: current CLI dispatch is owned by `adl/`, and
product execution belongs to the [separate repository owners](../docs/REPOSITORIES.md).

The historical [characterization shadow manifest](../adl-characterization/corpus/v2/shadow.yaml)
cites CLI, engine and records sources and tests. The former `adl-adapters` tree is archived at the published source revision in the [archive transition](../tools/public_readiness_archive/transition.json); no current public-workspace consumer was found. The 44-file identity inventory and
retention rationale are in the [retention register](../docs/archives/PUBLIC_READINESS_1325.json).
Removing them requires an explicit consumer/custody transition; preserving them
does not establish current build, runtime or release qualification.
