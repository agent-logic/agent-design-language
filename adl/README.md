# Public ADL command package

The `adl/` package provides the public ADL command entrypoint after the repository
split. Its explicit binary targets are `adl` and `adl-process`. Runtime,
CodeFriend and C-SDLC execution belongs to their separately qualified owners.
The source for this boundary is [Cargo.toml](Cargo.toml) and
[the public dispatcher](src/cli/public_dispatch.rs).

## Public behavior

The public package retains portable workflow presentation, local process status
and repository-local session coordination. The workflow path handles local
presentation and delegates execution and resume through its selected Runtime
owner. `adl process` and `adl session` stay local. Session records do not grant
issue lifecycle authority.

From the repository root, inspect the public command surface with:

```sh
cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- --help
```

This command builds the public package. It does not install or qualify a product
owner. A help listing describes routing; it does not prove that a selected
Runtime supports or has passed every listed operation.

## Selected product owners

Runtime operations, including agent, identity, provider, scheduler and CSM
commands, require an owner-qualified installation selected through
`ADL_RUNTIME_INSTALL_PREFIX` and its exact `ADL_RUNTIME_ARCHIVE_SHA256`.
The [Runtime selector](src/cli/runtime_owner.rs) checks the owner's installation
metadata and preserves arguments, working directory, environment and streams.
It does not qualify a raw executable or perform a second artifact verification.
Use the [Runtime repository](https://github.com/agent-logic/agent-logic-runtime)
for its installation, qualification and operational instructions.

`adl codefriend` uses the separately qualified CodeFriend installation and the
[CodeFriend selector contract](src/cli/codefriend_owner.rs). Product source builds
and product validation belong to [CodeFriend](https://github.com/agent-logic/codefriend).

Issue and PR lifecycle work uses native C-SDLC v3 with its authenticated authority
and repository binding. See [current authority](../docs/csdlc-v3/CURRENT_AUTHORITY.md)
and [installed manual lookup](../docs/csdlc-v3/man/README.md).
The removed `adl pr` and `adl tooling` routes are not fallbacks.

## Retained repository inputs and history

The repository-local [identity profile](identity/identity_profile.v1.json) is
persistent input data. Delegation to the Runtime owner does not make that local
identity disposable. Likewise, retained coverage mappings and review fixtures
have their own public consumers and historical evidence boundaries.

The former monolithic-runtime README and its example commands remain in Git and
the #1223 retirement custody record. Their historical execution, signing,
provider and release claims are not current acceptance claims for this public
package. Source retirement does not establish installed Runtime qualification,
provider readiness or release completion.

- [Root project overview](../README.md)
- [Public contribution guidance](../CONTRIBUTING.md)
- [Documentation index](../docs/README.md)
- [Demo and historical proof index](../demos/README.md)
- [Permission-safe process status](../docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md)
