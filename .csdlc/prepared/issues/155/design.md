# CORP-03 Design

Issue: #155

## Objective

Resolve provenance, licensing, trademark, model, media, contributor, and third-party dispositions for every critical asset.

## Scope

Git provenance, contributor rights, dependencies and licenses, model and dataset terms, generated media, trademarks, domains, podcast and publication assets, and unresolved third-party claims.

## Dependencies

- CORP-01: issue #153

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Provenance and licensing matrix tied to the frozen asset denominator.
- Trademark, domain, model, media, and third-party disposition register with owner and remediation route.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-03/**`
- `docs/operations/corporate/corp-03/**`
- `.csdlc/issues/155/**`
- `.csdlc/prepared/issues/155/**`
- `.csdlc/prepared/issues/155/validate-outcome.rb`
- `.csdlc/evidence/155/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every critical source, model, dataset, media, and brand asset has a provenance and use-rights disposition.
2. Dependency and license conclusions cite machine-readable manifests or authoritative source evidence.
3. Unresolved or restricted assets are excluded from transfer and release gates rather than silently accepted.
4. Trademark conclusions are explicitly bounded and routed to counsel where legal judgment is required.

## PVF Lanes

- `corp-03-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/155/validate-outcome.rb`.
- `corp-03-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Asset-to-provenance coverage, dependency manifest scan, license and model-term source verification, exclusion checks, and counsel-bound review.

## Authority Boundary

- Issue CORP-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Legal conclusions without counsel
- Changing dependencies or product behavior
- Publishing private contributor information

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A critical asset has no provenance path
- Terms cannot be verified from an authoritative source
- A legal conclusion exceeds the documented authority boundary

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-03`
