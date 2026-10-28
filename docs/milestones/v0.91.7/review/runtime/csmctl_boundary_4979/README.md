# CSMCTL Boundary Proof (#4979)

## Binary ownership map

| Binary | Owner role | In scope | Out of scope |
| --- | --- | --- | --- |
| `adl` | ADL language tooling | Authoring, validation, workflow YAML execution compatibility, language/tooling management | Permanent CSM runtime ownership, runtime service administration, C-SDLC issue execution |
| `csm` | CSM runtime owner | Permanent daemon execution, service-owned runtime parsers, runtime API, continuity, backpressure, storage, AWS signal/cloud-control proof surfaces | ADL language compilation, C-SDLC issue workflow |
| `csmctl` | CSM runtime administration control plane | Runtime service administration, permission-safe status checks, diagnostics, governed cloud-control and signal operations | Direct daemon-loop execution, ADL compiler features, C-SDLC workflow commands |
| `adl-csdlc` | C-SDLC compatibility surface | Issue/PR/tooling lifecycle compatibility while repo-native `adl/tools/pr.sh` remains canonical | CSM runtime execution, ADL workflow YAML runtime execution |
| `tools/*` | Specialized utilities | Bounded scripts and proof helpers | Owning core platform roles |

## Implemented boundary

`csmctl` is added as a standalone binary with a modular command tree:

- `csmctl runtime ...` for administered runtime-local surfaces.
- `csmctl status ...` for permission-safe exact PID, PID-file, or loopback port liveness probes.
- `csmctl diagnostics process ...` for explicit diagnostic process probes.
- `csmctl cloud ...` for governed CSM cloud-control and signal proof surfaces.

Direct daemon-loop execution remains owned by `csm daemon`. `csmctl runtime daemon ...` fails closed and points operators to either `csm daemon ...` for direct runtime execution or `csmctl runtime service ...` for service administration.

## Non-claims

This issue does not rename `adl-csdlc` to `csdlc`, migrate every historical wrapper to `csmctl`, or implement the later API Gateway, port-pooling, Chronosense NTP, low-disk survivability, or final soak work. Those remain scheduled in adjacent WP-07 children.
