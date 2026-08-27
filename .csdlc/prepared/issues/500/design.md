# Issue 500 design: V3-A

## Objective

Produce one reviewed C-SDLC v3 contract and construction-decision packet that
consolidates retained requirements #161 through #163 without granting v3 any
operational authority.

## Authority boundary

- C-SDLC v2 remains the sole operational lifecycle authority throughout #500.
- The v3 artifacts are design and construction inputs only.
- #500 cannot cut over authority, retire v2, bind later V3 issues, or implement
  their workflow surfaces.
- The issue owns only `docs/csdlc-v3/**`, `csdlc-v3/Cargo.toml`,
  `csdlc-v3/src/lib.rs`, and its issue-local C-SDLC records.

## Predecessor mapping

- #161 supplies the frozen product contract, retained invariants, command tree,
  capability matrix, and recovery-path expectations.
- #162 supplies the measured Rust construction-slice evidence and dependency,
  layering, parser, template, GitHub-client, and commit-primitive findings.
- #163 supplies the operator-approved platform commit matrix, durability
  posture, Windows policy, and rollback implications.

Every retained predecessor requirement must map to a named v3 contract clause,
construction decision, or explicit deferred owner. Missing or ambiguous mapping
fails closed.

## Execution shape

1. Inventory the authoritative retained requirements and evidence from
   #161–#163.
2. Define the v3 contract, compatibility boundary, repository layout, and
   construction decisions.
3. Specify rollback and fail-closed behavior while keeping v2 authoritative.
4. Run focused proof and obtain one independent implementation review.

## Proportional lifecycle contract

V3 must simplify the lifecycle itself, not merely automate the existing v2
ceremony. Its default path removes checkpoints, projections, reviews, and state
transitions that do not materially reduce delivery risk.

- A routine three-issue sprint must be mechanically prepared and made ready in
  minutes, not hours.
- The default lifecycle has one meaningful design gate, focused validation
  proportional to the changed surface, one independent implementation review,
  and truthful closeout.
- Intermediate records are derived views, not separately maintained authority.
- Repeated generation/digest choreography, hand-authored lifecycle JSON,
  duplicate readiness reviews, and umbrella reviews that merely repeat child
  proof are excluded from the default path.
- Additional gates are risk-triggered and name the concrete hazard they
  mitigate; process completeness alone is not a reason to add a gate.
- Fail-closed authority, exact revision identity, recovery, and auditable
  outcomes remain mandatory even as ceremony is removed.

## Deliverables

- A versioned C-SDLC v3 contract under `docs/csdlc-v3/**`.
- A predecessor coverage matrix for #161–#163.
- A construction decision and rollback packet.
- A proportional-lifecycle decision classifying v2 checkpoints and review
  surfaces as retained, collapsed, derived, or removed, with a risk-based
  justification for every retained gate.
- The minimal `csdlc-v3` crate boundary needed to encode or validate the
  accepted contract, without implementing lifecycle authority.

## Validation design

- One issue-owned focused validator proves contract shape, predecessor coverage,
  authority boundaries, and the proportional-lifecycle rules.
- `git diff --check` remains lightweight hygiene, not a separate review gate.

## Stop conditions

- Any v2 coexistence or authority boundary remains ambiguous.
- Any retained predecessor requirement lacks an exact disposition.
- The construction packet requires authority cutover or v2 retirement.
- Work expands into V3-B or later implementation.
