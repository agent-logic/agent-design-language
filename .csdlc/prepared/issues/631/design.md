# Issue 631 design

Status: design-ready for bounded V3-H.5 execution.

Issue #631 implements non-authoritative C-SDLC v3 route behavior for proof,
shadow, soak, and install under the single `csdlc` binary.

The slice keeps v2 as live operational authority until #505 cutover. The v3
routes may parse typed requests, validate evidence boundaries, produce durable
plans, and compare declared parity. They must not run hidden v2 lifecycle
authority, switch selectors, rely on disposable Cargo target paths, or claim a
live cutover.

The implementation should stay small and behavior-first:

- `csdlc proof` validates proof manifests, evidence refs, stale evidence, and
  deterministic lane declarations before emitting a durable proof plan.
- `csdlc shadow` compares declared v2/v3 observations only when both sides are
  explicitly bounded and refuses broad equivalence claims.
- `csdlc soak` classifies bounded soak evidence without hidden state or live
  provider side effects.
- `csdlc install` produces a stable one-binary installation plan with source
  provenance, selected binary digest, selector metadata, and cutover gating.

Success is proving positive and denial behavior with focused tests, preserving
the one-binary manifest, and recording any canary/setup defects for #632.
