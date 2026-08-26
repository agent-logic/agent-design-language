# Independent Exact-Head Review

ADL uses the standard SRP for review. Independence comes from sending that SRP
to a fresh external session instead of reviewing inside the implementation
session.

## Review Route

1. Finish the bounded implementation and focused validation.
2. Commit the complete substantive change.
3. Generate or refresh the standard SRP for that exact commit.
4. Select a fresh review session that does not inherit the implementation
   conversation, and record its identity, exact scope, and exact revision with
   `csdlc-review assign` before review activity begins.
5. Only after assignment, give that same session the SRP,
   repository/worktree path, and exact commit SHA.
6. Require findings first, with severity and repository-relative file/line
   evidence. The reviewer must not edit the worktree or GitHub state.
7. Record the review result and findings through `csdlc-review record`, which
   updates the standard SRP.
8. Resolve every actionable finding in the implementation session.
9. If the fix changes the substantive commit, generate a current SRP and send
   it to a new review session at the new exact SHA.
10. Publish only after the standard SRP records a passing exact-head review.

An assignment created after review activity begins is backfilled metadata, not
proof of an independent handoff, and must not be accepted.

## Scope

Classify authority first. Authentication, authorization, security boundaries,
lifecycle authority, or proof-producing changes require code, security, and
evidence review even when every changed file is documentation. This precedence
applies to review-policy and validation-script changes. Other documentation-only
work uses one fresh documentation reviewer. Other code asks the fresh reviewer
to cover code and tests. Authority-critical review may use one qualified fresh
session or multiple fresh sessions when the surface genuinely requires
specialization.

Do not add a review daemon, scheduler, registry, claim, persistent reviewer,
parallel review record, or new lifecycle phase. Do not rerun broad validation
solely to prepare the review.
