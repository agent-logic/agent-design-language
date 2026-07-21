# #5336 Design: v0.91.8 Runtime v3 Functional-Parity Planning

## Decision

Make `adl-runtime-kernel` the canonical Runtime v3 process and require live
functional parity before selector cutover or Runtime v2 deletion. Historical
fixture packets remain useful evidence, but fixture-only, library-only,
metadata-only, degraded-adapter, and fixed-bootstrap behavior do not satisfy
live parity.

## Planning Boundary

This issue updates the v0.91.8 architecture and execution package. It does not
implement runtime behavior, switch the default runtime, delete Runtime v2, or
approve v0.92 activation.

## Architecture

The plan preserves the nine historical Runtime v3 fixture groups and adds the
HTML Observatory as a tenth integrated proof surface. Four disjoint
implementation lanes feed one serialized parity, acceptance, cutover, and
deletion path:

1. canonical kernel, topology, configuration, continuity, and domain ingress;
2. reasoning graphs, adaptive learning, cognition, and feature preservation;
3. governance, delegation, agents, providers, scheduler, identity, checkpoint,
   and lifelog;
4. secure local/remote access, ACIP/A2A/cloud boundaries, Observatory,
   guardian, soak, and rollback.

## Budgets

- Target one canonical Runtime v3 implementation at no more than 12,000 Rust
  source lines under the denominator pinned by #5336.
- Keep the Runtime v3 test inventory below 1,000 tests.
- New behavior must replace duplicate or placeholder code; it must not create a
  third implementation beside `adl-runtime` and `adl-runtime-kernel`.
- Prefer maintained third-party crates for supervision, serialization,
  telemetry, HTTP/TLS, channels, signing, and system metrics.

## Live-Parity Contract

Every capability credited for cutover must be exercised through the initialized
Runtime v3 process and its canonical domain ingress, execute production
component code, emit retained evidence, prove negative behavior, and survive
graceful shutdown/recovery where stateful. Test-only fixtures and fixed
one-node bootstrap graphs are not sufficient.

Every v0.91.7 feature document and every implemented row in the ADL feature
list must receive one explicit disposition: live in Runtime v3, intentionally
owned outside Runtime v3, accepted boundary/non-claim, deferred with an owner,
or blocker. No Runtime v2 deletion is eligible while its last implementation
owns an undispositioned feature.

## Dependency Correction

The Runtime v3 implementation lanes must finish before WP-11 shadow parity.
Runtime v3 acceptance #5361 closes only after the lanes and WP-11 pass. WP-12
cutover depends on that acceptance. WP-13 deletion follows cutover and current
C-SDLC v2 acceptance.

## Non-Goals

- No AWS use.
- No hard-coded hosts, IP addresses, credentials, or alternate API ports.
- No HTTP-only runtime access; local and remote access remain secure and
  configuration-driven on the declared Runtime v3 API port.
- No Runtime v2 source reuse for Runtime v3 parity credit.
- No new feature claims beyond the existing feature list and milestone scope.
