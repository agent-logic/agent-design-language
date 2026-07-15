# #5405 WP-13 Guild, Godel, And Economics Review Fix Design

## Scope

Resolve the code-side economics duplicate-validation finding from #5403 and
prepare the WP-13 truth repairs that are blocked by the stale #5383 milestone
document claim.

## Approach

- Inspect scheduler economics input validation and semantic-policy structures.
- Add duplicate semantic-policy rejection with regression coverage.
- Keep guild and Godel document claim repairs deferred until the v0.91.7 docs
  claim collision is released.

## Validation

- Focused scheduler economics tests for duplicate rejection.
- Diff hygiene before review.

## Deferred Boundary

Guild/Godel/closeout document truth repairs under `docs/milestones/v0.91.7` are
blocked by the closed #5383 stale broad claim until #5415 is resolved or the
operator clears the claim.
