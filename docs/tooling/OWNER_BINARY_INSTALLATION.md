# ADL owner binary installation

ADL operational owner binaries are generated tools. They are not committed to
Git, and Cargo `target/` directories are disposable build/cache output rather
than operational authority.

## ADL product owners

ADL product owner binaries use the stable generated directory `.adl/bin/`.
Install commands such as `adl`, `csm`, `csmctl`, `adl-review`, and the
provider/remote helpers with:

```sh
bash adl/tools/install_owner_binaries.sh
```

That script installs product commands from reviewed `adl/` source. It does not
install or select C-SDLC lifecycle authority.

## C-SDLC lifecycle owner

The sole ordinary C-SDLC lifecycle executable is generated separately at:

```text
.adl/bin/native-v3/csdlc
```

Its authority is conditional on the canonical v3 selector, authenticated
receipt, reconciliation, and exact typed request evidence described in
[`../csdlc-v3/CURRENT_AUTHORITY.md`](../csdlc-v3/CURRENT_AUTHORITY.md) and
[`ACTIVE_BOOT_PATHS.md`](ACTIVE_BOOT_PATHS.md). Never select lifecycle authority
from a Cargo target directory.

## Retained v2 exception surface

The `csdlc-v2/` source and generated `.adl/bin/csdlc-v2/` directory are retained
only for an explicitly authorized rollback or bounded transition remediation.
They are not an ordinary installation or fallback path. Historical owner names
include `csdlc-github`, `csdlc-github-issue`, `csdlc-github-pr`,
`csdlc-pr-state`, `csdlc-finish`, and `csdlc-clean`.

Current issue and PR actions use `csdlc github-issue` and `csdlc github-pr`;
current review, publication, finish, and cleanup use their corresponding routes
on the one native v3 `csdlc` executable. Missing v3 proof suspends authority; it
does not authorize automatic fallback to v2.
