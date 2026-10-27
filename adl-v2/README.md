# ADL v2 source layout

This directory contains the public `adl-language` and `adl-compiler` workspace
members. Use an explicit manifest; the root public CLI is a separate package:

```sh
cargo test --locked --manifest-path adl-v2/Cargo.toml
```

Historical `adl-cli`, `adl-engine`, `adl-records` and `adl-adapters` packages
have been retired from this checkout. Current CLI dispatch is owned by `adl/`;
product execution belongs to the [separate repository owners](../docs/REPOSITORIES.md).

Exact published Git preimages and hashes are recorded in the
[Runtime retirement manifest](../docs/archives/RUNTIME_REMNANTS_1370.json)
and the earlier [adapter archive transition](../tools/public_readiness_archive/transition.json).
Historical characterization decisions now link to immutable source revisions.
These archived packages are not supported workspace members or installed products.
