# ADL Platform CLI Binary Taxonomy

ADL uses owned command binaries so command families do not all share one
validation and operational blast radius.

## Canonical Binaries

| Binary | Owner surface | Current status |
|---|---|---|
| `adl` | ADL language/compiler/manager entrypoint | canonical |
| `csdlc` | C-SDLC workflow control plane: issue lifecycle, cards, PR lifecycle, validation planning, review, and closeout | canonical as of v0.91.7 |
| `csm` | Cognitive Spacetime runtime execution surface | canonical |
| `csmctl` | CSM runtime administration and operator control | planned; do not place C-SDLC commands here |
| `adl-csdlc` | Compatibility alias for C-SDLC commands | retained during migration |
| `adl-runtime` | Runtime compatibility alias | retained during migration |
| `adl-review` | Review tooling compatibility surface | retained during migration |

## C-SDLC Usage

Use `csdlc` for direct C-SDLC control-plane commands:

```bash
csdlc --help
csdlc issue run <issue>
csdlc tooling prompt-template validate-structure --kind sor --input <path>
```

Agent issue execution should still prefer the repo wrapper while it owns
session-ledger, worktree, queue, and finish policy:

```bash
bash adl/tools/pr.sh run <issue>
bash adl/tools/pr.sh finish <issue> --title "<title>" --paths "<paths>"
```

`adl-csdlc` remains available for compatibility and should produce the same
C-SDLC dispatch behavior, but new docs and scripts should prefer `csdlc` when
they need the owner binary directly.

## Validation Boundary

Changes to one binary family should not automatically require broad validation
for all binaries. For C-SDLC binary-surface changes, prefer:

```bash
cargo fmt --manifest-path adl/Cargo.toml --all -- --check
cargo check --manifest-path adl/Cargo.toml --bin csdlc --bin adl-csdlc
cargo test --manifest-path adl/Cargo.toml --test cli_smoke csdlc_cli_binary_help_and_version_smoke -- --exact
git diff --check
```

Broader owner-lane validation is still available when the touched surface
changes delegation policy:

```bash
bash adl/tools/run_owner_validation_lane.sh csdlc --build
```

## Non-Claims

- This document does not remove compatibility binaries.
- This document does not make `csmctl` ready; it reserves the boundary.
- This document does not move every C-SDLC helper out of `adl` in one step.
