# ADL Platform CLI Binary Taxonomy

ADL separates command families so each owner has a bounded operational and
validation surface.

## Platform Owners

| Owner surface | Command family | Current status |
|---|---|---|
| ADL language and compiler | `adl` | canonical |
| C-SDLC workflow lifecycle | `.adl/bin/native-v3/csdlc` | sole ordinary authority after #505 / PR #591 |
| Cognitive Spacetime runtime | `csm` | canonical |
| CSM administration | `csmctl` | planned; no C-SDLC commands belong here |
| Runtime compatibility | `adl-runtime` | compatibility product surface outside C-SDLC authority |
| Review compatibility | `adl-review` | compatibility product surface outside C-SDLC authority |

## C-SDLC lifecycle authority

PR #591 completed the #505 cutover. The sole ordinary C-SDLC operational
authority is the native Rust executable `.adl/bin/native-v3/csdlc`, subject to
the canonical selector and authenticated receipt.

Current lifecycle routes include:

- `csdlc issue`, `bind`, `edit`, `doctor`, `validate`, and `shepherd`
- `csdlc github-issue`, `github-pr`, `review`, and `publish`
- `csdlc finish` and `clean`

Missing or stale v3 proof suspends authority. It never selects v2 automatically.
The `csdlc-v2/` source and `.adl/bin/csdlc-v2/` generated binaries are retained
only for explicitly authorized rollback or bounded transition remediation.

The removed v1
`pr.sh` wrappers, prompt-template wrappers, `csdlc-import`, and `adl-csdlc`
compatibility route are not valid operator paths. See
[`ACTIVE_BOOT_PATHS.md`](ACTIVE_BOOT_PATHS.md) for the complete subsystem table.

## Validation Boundary

Validate the native v3 command manifest and the contract touched by a change:

```bash
cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest
git diff --check
```

Do not rewrite historical evidence to make it look current. Gate 10A-D2
artifacts retain their source-time meaning but do not override the completed
native v3 cutover.

## Non-Claims

- This taxonomy does not make `csmctl` ready.
- It does not move Runtime or review compatibility commands into C-SDLC.
- It does not authorize direct card Markdown mutation; typed card edits remain
  governed by native v3 `csdlc edit` and `csdlc validate`.
