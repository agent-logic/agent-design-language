# DRT-06 Design

Issue: #186

## Objective

Validate that the single Observatory presents a coherent, causal, authority-aware, redacted distributed evidence surface.

## Scope

Quorum-leased singleton ownership, coherent authority cuts, node and agent correlation, causation, terms, commit indexes, state revisions, stale-read denial, redaction, partition and recovery visibility.

## Dependencies

- DRT-03: issue #183
- DRT-04: issue #184

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Observatory coherent-cut and causal-trace proof packet.
- Stale ownership, stale read, split-view, redaction, and singleton negative evidence.

## Owned Paths

- `observatory/tests/v0921_coherent_evidence.rs`
- `adl/tools/v0921/drt-06/**`
- `.csdlc/issues/186/**`
- `.csdlc/prepared/issues/186/**`
- `.csdlc/evidence/186/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Exactly one Observatory owns the quorum lease at any instant and successor binding follows old-lease expiry.
2. Every displayed operation correlates agent, node, polis, identity, authority, trace, term, commit index, and state revision.
3. Partitions and leadership changes cannot present stale authority as current or combine an incoherent cut.
4. Secrets, credentials, private legal data, and unredacted provider payloads never appear in retained or visible evidence.

## PVF Lanes

- `drt-06-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/186/validate-outcome.rb`.
- `drt-06-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-06/validate.sh`.
- `drt-06-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Lease timeline, coherent-cut recomputation, causal trace cross-check, stale-read and split-view negatives, singleton assertion, and redaction scan.

## Authority Boundary

- Issue DRT-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Implementing new Observatory features
- Using screenshots as sole proof
- Allowing multiple active owners

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Two Observatory owners overlap
- A displayed row lacks authority or revision context
- A stale read appears current
- Sensitive data is exposed

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-06`
