# ADL v0.91.8

ADL v0.91.8 completes the ADL Core Rearchitecture bridge: ADL v2, Runtime v3,
and C-SDLC v2 are accepted as an integrated platform with exact-revision
evidence, rollback paths, release-tail review, and a bounded v0.92 handoff.

## Highlights

- Added the clean-room ADL v2 language core, deterministic compiler, portable
  execution engine, signed records, Runtime v3 adapter, provider and governed
  tool adapters, and thin CLI/selector.
- Completed Runtime v3's Axum/Tokio/Rustls service path, durable local and redb
  state, provider/protocol adapters, observability, platform lifecycle proof,
  and parity work while retaining explicit authority boundaries.
- Made C-SDLC v2 the sole C-SDLC lifecycle authority and removed the retired
  v1 command surfaces.
- Proved normalized shadow parity, opt-in soak, rollback, reversible cutover,
  and reviewed deletion of replaced incumbent surfaces.
- Completed integrated platform acceptance, demo convergence, quality gates,
  documentation alignment, two internal reviews, an independent external
  review, and findings remediation.
- Prepared the v0.92 planning and activation inputs with explicit non-claims
  for future work.

## Quality And Review

- WP-16 audited 67 issue outcomes with 0 unacceptable outcomes and 0 release
  blockers at its quality-gate revision.
- ADL v2, Runtime v3, and C-SDLC v2 focused and integrated validation evidence
  is retained under `docs/milestones/v0.91.8/` and `.csdlc/evidence/`.
- Internal review ran twice; external findings were remediated before the
  release tail advanced.
- WP-22 reviewed the complete v0.92 planning package before this ceremony.

## Compatibility And Operations

- Stable operational binaries live under `.adl/bin/`; Cargo target directories
  remain disposable build output.
- Runtime and lifecycle rollback retain the previous selector and generation
  path rather than depending on mutable build caches.
- v0.92 activation is deliberately separate from this release.

## Known Boundaries

- This release does not claim completion of v0.92 birthday work, Adaptive
  Learning implementation, or future distributed-polis work.
- Preparation-only v0.92 packets remain planning inputs until their own issues
  execute and pass review.
