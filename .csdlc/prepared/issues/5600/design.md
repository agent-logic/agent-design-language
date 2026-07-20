# Issue #5600 Design: Complete Typed Preparation-to-Implementation Replanning

## Problem

A prepared C-SDLC v2 issue can replace acceptance criteria and a few scalar
fields, but it cannot replace every structured planning collection needed to
move from preparation truth to a complete implementation plan. Direct edits to
rendered cards or values files would bypass lifecycle ownership, compare-and-
swap guards, atomic regeneration, and the audit log.

## Design

Extend `SemanticOperation` with card-owned replacement operations for the
remaining planning collections:

- SIP: declared scope, authority boundary, operator constraints, dependencies,
  repository inputs, and non-goals.
- STP: deliverables and acceptance criteria.
- SPP: steps, invariants, risks, and stop conditions.
- SRP: review prompts, while retaining the existing review-scope replanning
  operation.

Each operation accepts a complete replacement collection. The store validates
non-empty values, card ownership, lifecycle authorization, generation, digest,
and claim identity before changing an in-memory card set. Cross-card acceptance
coverage is checked across STP, SPP, and VPP before the transactional commit.
Any failure leaves every card, projection, generation, digest, and audit record
unchanged.

The authorized replanning phase is `Bound`, before implementation completion.
Existing post-implementation mutation protections remain unchanged. Operations
are explicit replacements rather than generic JSON-pointer mutation so the
typed contract remains reviewable and card ownership remains mechanically
enforced.

## Compatibility

Existing serialized operations and native card records remain readable. New
operations are additive enum variants. Rendering continues through the current
template registry and markdown AST validation; no rendered Markdown is edited
directly.

## Proof

Focused tests cover every new operation, wrong-card use, empty values, stale
generation, stale digest, phase rejection, atomic rollback, and acceptance-ID
drift. A #5337 fixture starts from preparation-only Bound cards, performs only
typed operations, and finishes with complete implementation truth that passes
typed validation and doctor.

