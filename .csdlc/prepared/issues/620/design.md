# #620 v0.92.2 first-pass planning audit

## Goal

Turn the existing candidate v0.92.2 documentation into one coherent,
number-free, execution-oriented planning package while keeping the milestone
closed until the operator explicitly opens it.

## Design

The work begins from the existing package under `docs/milestones/v0.92.2/`.
It establishes a canonical document and work-package denominator, reconciles
the human-readable plans with both machine-readable work-wave files, and
records every relevant active TBD source in a findings-first scheduling table.

The TBD table separates sources already represented in v0.92.2 from later
milestone work, backlog/deferred work, operational references, completed
provenance, and unresolved scheduling gaps. An unresolved gap is reported for
operator judgment; it is not silently scheduled. Existing GitHub issues and
completed work are cross-checked to prevent duplicate scope.

The package remains number-free and planned. This issue does not create WP-01,
child issues, a milestone, or a version label.

## Outputs

- refreshed canonical documents under `docs/milestones/v0.92.2/`
- `docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md`
- aligned issue catalog, issue wave, execution specifications, sprint plan,
  readiness surfaces, feature index, and ten-step release tail
- focused validator and review handoff

## Boundary

`.adl/docs/TBD/` is read-only source evidence for this issue. Unity, ATE,
OCI packaging, cloud migrations, broad integrations, Runtime v4, and other
deferred work remain outside v0.92.2 unless the operator makes an explicit
later decision. The operator explicitly admitted a bounded MLX/Metal provider
adapter to v0.92.2; that admission is recorded in the milestone decisions.
