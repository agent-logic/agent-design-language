# CORP-05 Design

Issue: #157

## Objective

Transfer administrative control of repositories, domains, brands, and external vendors to Agent Logic while preserving availability and rollback.

## Scope

Seven approved repository copies and source authority, GitHub organization settings, domains and registrars, brand accounts, package and webhook identities, vendor ownership, redirects, and legacy-public-repository disposition.

## Dependencies

- CORP-03: issue #155
- CORP-04: issue #156

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Administrative-control manifest with source/destination identity, visibility, exact ref, owner, and live readback.
- Redirect and legacy-repository disposition plan that preserves history without destructive founder-account mutation.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-05/**`
- `docs/operations/corporate/corp-05/**`
- `.csdlc/issues/157/**`
- `.csdlc/prepared/issues/157/**`
- `.csdlc/prepared/issues/157/validate-outcome.rb`
- `.csdlc/evidence/157/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Only the seven approved migration repositories move; asksifu and Horust remain unchanged.
2. Agent Design Language remains public and all other company repositories remain private unless separately authorized.
3. Founder-account repositories are copied or dispositioned without deletion or destructive history changes.
4. Domains, brands, Apps, webhooks, packages, Pages, OIDC, and repository references receive verified dispositions.

## PVF Lanes

- `corp-05-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/157/validate-outcome.rb`.
- `corp-05-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Immutable-ref parity, visibility, owner, redirect, package, App, webhook, Pages, OIDC, and no-touch exclusion readback.

## Authority Boundary

- Issue CORP-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Deleting founder repositories
- Moving asksifu or Horust
- Changing repository visibility beyond the approved matrix

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A source ref cannot be reproduced
- A same-name or fork conflict is unresolved
- A private repository would become public
- asksifu or Horust would change

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-05`
