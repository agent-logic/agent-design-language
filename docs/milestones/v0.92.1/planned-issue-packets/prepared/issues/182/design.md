# DRT-02 Design

Issue: #182

## Objective

Prove deterministic ACIP identity, authority, ordering, causation, duplicate, denial, and replay conformance before live distribution.

## Scope

Canonical envelopes and encodings, identity and authority bindings, sequence and term rules, duplicate and replay behavior, negative vectors, deterministic receipts, and cross-polis denial.

## Dependencies

- DRT-01: issue #181

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Versioned positive and negative ACIP conformance vector corpus.
- Deterministic producer and independent replay verifier with exact digest contract.

## Owned Paths

- `adl-runtime/tests/v0921_acip_conformance.rs`
- `adl/tools/v0921/drt-02/**`
- `adl/tools/v0921/drt-02/validate.sh`
- `.csdlc/issues/182/**`
- `.csdlc/prepared/issues/182/**`
- `.csdlc/prepared/issues/182/validate-outcome.rb`
- `.csdlc/evidence/182/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Canonical encode-decode-reencode is byte-stable for every supported message family.
2. Identity, authority, permit, causation, correlation, sequence, term, and polis bindings reject every declared mutation.
3. Duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis messages produce typed deterministic outcomes.
4. Independent replay from retained inputs reproduces the exact committed outcome and digest.

## PVF Lanes

- `drt-02-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/182/validate-outcome.rb`.
- `drt-02-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-02/validate.sh`.
- `drt-02-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Canonical vectors, mutation matrix, property tests, duplicate/order/term cases, cross-polis negatives, and independent replay digest comparison.

## Authority Boundary

- Issue DRT-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Provisioning a distributed cluster
- Replacing ACIP implementation
- Accepting hard-coded assertion labels as producer proof

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A semantic field is authenticated but not canonically bound
- A noncanonical representation round-trips to a different value
- Replay requires hidden mutable state

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-02`
