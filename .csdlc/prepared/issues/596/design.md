# Design: Sprint 5/6 cutover blocker remediation

## Intent

Create a bounded remediation issue for the sprint 5/6 cutover review blockers
without granting C-SDLC v3 live lifecycle authority. The issue exists to carry
the corrective PR that repairs concrete review findings discovered while testing
the v3 cutover path and the new typed GitHub operations.

## Scope

- C-SDLC v3 local construction and delivery-kernel defects identified by sprint
  review.
- C-SDLC v2 typed GitHub PR operations required to avoid raw `gh` during
  remediation.
- CI and documentation surfaces needed to keep v3 construction visible before
  cutover.
- Real issue canary evidence for issue creation/readiness defects.

## Authority boundary

C-SDLC v2 remains the live lifecycle authority until explicit #505 cutover.
This issue may repair v3 construction code, v2 transport gaps, proof lanes, and
documentation, but it must not merge, finish, clean, or close #505.

## Approach

1. Preserve primary checkout cleanliness and work only in the bound FastWork
   remediation worktree.
2. Ensure the PR for this tracking issue visibly uses `Closes #596`, while
   representing #505/#534 only as `Part-Of`.
3. Repair reviewed implementation defects with behavior-backed tests rather
   than string assertions.
4. Record real-tool canary defects as bounded evidence, not as cutover claims.
5. Keep validation focused on the touched v2/v3/tooling surfaces and defer
   full authority cutover to #505 review.

## Non-claim

This design does not authorize C-SDLC v3 as live lifecycle authority and does
not claim the full sprint cutover is complete.
