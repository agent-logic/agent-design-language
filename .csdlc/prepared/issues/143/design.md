# Issue 143: v0.92 ADR candidate packet design

## Status

Design-ready for independent approval. This issue authors Proposed or Deferred
architecture records only. It does not accept decisions or change runtime
behavior.

## Source hierarchy

1. `docs/adr/README.md` and accepted ADRs define current accepted authority.
2. Landed implementation, focused tests, retained receipts, and exact-head
   reviews define what a candidate may claim as implemented proof.
3. Canonical v0.92 feature contracts define milestone scope and non-claims.
4. Forward plans may justify Deferred candidates, but never implementation
   claims.

## Candidate lifecycle

Each reserved number from ADR 0059 through ADR 0071 has exactly one document in
`docs/architecture/adr/`. Its status is:

- **Proposed** when repository evidence supports the durable decision and its
  validation boundary; or
- **Deferred** when implementation or executable proof is not yet landed.

No candidate is copied into `docs/adr/` or marked Accepted by this issue.

## Required document contract

Every candidate contains Status, Context, Decision, Consequences, Alternatives
Considered, Source Evidence, Validation Evidence, Supersession Relationships,
Non-Claims, and Approval Boundary. Evidence references are repository-relative
and identify actual files, tests, receipts, or review records. Planned work is
identified as planned and receives no proof credit.

## Corrected scope boundaries

- ADR 0070 is Proposed as the v0.92 planning boundary for future cross-polis
  continuity and transfer. Its proving surface is deterministic consistency
  with the canonical feature contract, including copied-state rejection and
  explicit deferral of operational migration. It does not require or claim an
  end-to-end production transfer. Its Proposed status records the durable
  deferral and copied-state rejection rule, not operational capability.
- ADR 0069 and ADR 0071 are Deferred unless real WP-18A and WP-18B executable
  proof is present at the authored revision.
- ADR 0067 may describe only transport and TLS behavior supported by landed
  Runtime implementation and tests. It cannot infer public deployment or
  certificate operations from planning prose.

## Outputs

- corrected `docs/milestones/v0.92/ADR_PLAN_v0.92.md`;
- candidate catalog and evidence index;
- ADR 0059 through ADR 0071 Proposed or Deferred documents;
- focused deterministic packet validator;
- fresh exact-head architecture, security-boundary, and documentation review.

## Validation

The issue-owned validator checks numbering, unique filenames, allowed status,
exact index rows and status parity, structured Source Evidence and Validation
Evidence references, executable-proof references for implementation-bearing
Proposed records, accepted-file non-mutation, deferred proof boundaries, and
standardized non-claims. Diff hygiene and fresh independent review are separate
gates.

## Non-goals

- runtime, provider, TLS, identity, migration, or governance implementation;
- accepted ADR promotion;
- synthetic proof for planned work;
- production cross-polis migration;
- broad Rust validation for this documentation-only issue.
