# Issue844 validation packet

The source defect is issue #844: the native PR owner had create/update/ready but
no merge mutation, leaving finish dependent on a prior external merge.
The additive native merge path and operator contract are documented in
`docs/csdlc-v3/PULL_REQUEST_MERGE.md`.

PVF: required native C-SDLC owner contract. Deterministic fake transport plus
local Git/filesystem, small CPU, no paid resource or live merge. The actual
read-only GraphQL query was also accepted by GitHub for an existing merged PR;
that schema compatibility observation is not live merge proof.

`native-v3-tests.log` records the initial full native all-target suite (231
passing tests). `merge-tests.log` covers the additional focused merge cases,
including durable response-disagreement rejection. `native-v3-clippy.log`
records strict all-target clippy. Source paths in logs are normalized to
`<worktree>`; no credential values are captured. Final validation and review
identity will be recorded before publication.

The shared owner lane wrapper still selects retained v2 Gate10A, so the matching
native `csdlc-v3/Cargo.toml` all-target suite is used directly. Dependency warmup
hardlinked 274 external dependency files from a trusted same-host FastWork
C-SDLC target; this is acceleration only, not validation proof.

No live PR merge, issue closure, stable-binary replacement, or cleanup was
performed. REST head CAS cannot provide atomic base/policy CAS: a late base
change is rejected at reconciliation and is never described as a successful
verified merge. Uncertain intent replay is read-only even when the process may
have died before dispatch; this deliberately refuses automatic duplicate writes.
