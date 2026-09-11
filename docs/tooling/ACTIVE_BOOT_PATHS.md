# Active boot paths

This table is the current subsystem map. It separates lifecycle authority from
ADL product commands and Runtime processes; the presence of several Rust
binaries does not imply several C-SDLC control planes.

| Subsystem | Ordinary entrypoint | Installed/generated location | Source owner | Selector or configuration authority | Classification |
|---|---|---|---|---|---|
| C-SDLC issue preparation, binding, editing, validation, diagnosis, scheduling, and eligibility | `csdlc <route> --request <file>` | `.adl/bin/native-v3/csdlc` | `csdlc-v3/src/main.rs`; `csdlc-v3/src/commands/local/` | `csdlc-v3/operator/authority-selector.json`, its authenticated receipt, typed request, prompt registry, and worktree registration | active lifecycle authority |
| C-SDLC review, GitHub issue/PR operations, publication, finish, and cleanup | `csdlc review`, `csdlc github-issue`, `csdlc github-pr`, `csdlc publish`, `csdlc finish`, and `csdlc clean` | `.adl/bin/native-v3/csdlc` | `csdlc-v3/src/review/`, `csdlc-v3/src/publication/`, and `csdlc-v3/src/commands/terminal.rs` | the same selector and receipt plus exact-head typed remote or terminal requests | active lifecycle authority |
| ADL language/compiler CLI | `adl` | `.adl/bin/adl` | `adl/src/main.rs` and `adl/src/cli/` | ADL source/configuration supplied to the requested product command | product surface, not C-SDLC authority |
| Polis service and Runtime API control | `csm runtime-v3 <route>` | `.adl/runtime-v3/current/bin/csm` for the permanent Wuji installation; `.adl/bin/csm` for generated developer tooling | `adl/src/bin/csm.rs` and `adl/src/cli/csm_runtime_v3_cmd.rs` | the active Runtime init/configuration file and service-manager state described by `START_CSM_RUNBOOK.md` | product/runtime control surface, not C-SDLC authority |
| Runtime Guardian | started and supervised by the `csm runtime-v3` service path | `.adl/runtime-v3/current/bin/adl-runtime-guardian` | `adl-runtime/src/bin/adl-runtime-guardian.rs` | Runtime init file, service-manager environment, and Guardian supervision contract | product/runtime process |
| Runtime kernel and HTTPS/WSS API | started by Guardian; not an independent lifecycle route | `.adl/runtime-v3/current/bin/adl-runtime-kernel` | `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs` | Runtime init file plus kernel-owned authenticated API and state configuration | product/runtime process |
| Retained C-SDLC v2 | no ordinary entrypoint | `.adl/bin/csdlc-v2/` when deliberately installed for an approved exception | `csdlc-v2/` | explicit operator-authorized rollback or bounded transition-remediation contract only | retained rollback/transition-only source |
| Historical wrappers, notices, fixtures, and review packets | no executable current route | historical tracked paths only | their source-time owners | immutable source-time context | historical evidence |

## Lifecycle invariant

`ordinary_lifecycle_entrypoint: .adl/bin/native-v3/csdlc`

Native C-SDLC v3 is the only ordinary lifecycle path. A missing or stale
selector, receipt, exact-head proof, or reconciliation suspends authority; it
does not authorize fallback to v2. Retained v2 may be invoked only after the
explicit exception named above. Historical examples remain evidence and must
not be followed as current instructions.

## Installation and verification

The native v3 binary is generated, not committed. Install it through the
current native v3 installation/proof route and verify its provenance before
use. ADL product owner binaries are installed separately by
`adl/tools/install_owner_binaries.sh`; that script does not grant C-SDLC
authority. The permanent Runtime installation is governed by
[`START_CSM_RUNBOOK.md`](START_CSM_RUNBOOK.md), which keeps exactly one active
kernel binary and one Guardian-supervised service path.

The focused authority contract is:

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest
```

It verifies the canonical selector, actual native v3 help, this denominator,
the source paths above, and rejection of stale ordinary-v2 guidance while
allowing explicitly labelled rollback and historical text.
