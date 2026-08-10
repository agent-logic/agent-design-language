# CORP-02 Design

Issue: #154

## Objective

Execute counsel-reviewed intellectual-property assignment and corporate acceptance with private originals and public redacted proof.

## Scope

Founder and contributor assignments, invention and work-product schedules, corporate approval and acceptance, effective dates, signatures, exclusions, and retained redacted receipts.

## Dependencies

- CORP-01: issue #153

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Counsel-approved private execution packet and corporate acceptance record.
- Redacted chain-of-title receipt index containing document identifiers, digests, dates, parties by role, authority, scope, exclusions, custody, and verification result.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-02/**`
- `docs/operations/corporate/corp-02/**`
- `.csdlc/issues/154/**`
- `.csdlc/prepared/issues/154/**`
- `.csdlc/prepared/issues/154/validate-outcome.rb`
- `.csdlc/evidence/154/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Qualified counsel approves the instrument set before execution.
2. All required parties and corporate authorities execute or receive an explicit blocking disposition.
3. Private instruments remain outside the public repository and company-controlled custody is verified.
4. Redacted receipts bind each executed instrument to the asset schedule without exposing signatures, addresses, secrets, or privileged advice.

## PVF Lanes

- `corp-02-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/154/validate-outcome.rb`.
- `corp-02-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Counsel and corporate-authority checklist, private-custody readback, digest recomputation, schedule coverage, exclusion, and redaction review.

## Authority Boundary

- Issue CORP-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Drafting legal advice in repository prose
- Representing redacted receipts as the legal instruments
- Transferring operational accounts

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Counsel has not approved the final form
- Corporate acceptance authority is unclear
- An asset or contributor lacks a disposition
- Private material would be committed

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-02`
