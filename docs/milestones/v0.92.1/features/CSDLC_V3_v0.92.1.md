# C-SDLC v3

## Outcome

C-SDLC v3 provides one Rust executable, one command tree, one application context, one deterministic state authority, direct flags with optional typed input, exact review, typed GitHub effects, foreground watch, terminal finish, and safe cleanup.

## Source Contract

The implementation preserves PR `#77` merge `413fa9b8588dd25be3785cfe111c4f1df3af36eb`. Architecture changes require a new exact-revision review.

## Hard Gates

- V3-02 measured construction spike;
- eleven operator decisions, including Decision 11 before V3-08;
- state commit-point and fault-recovery proof;
- exact review and publication linkage proof;
- remote idempotency and cancellation proof;
- normalized parity, known-defect corpus, live canary, migration rehearsal, writer fence, authority scan, and rollback.

## Non-Claim

Initial cutover does not retire v2. V3-R01 remains a separate reviewed issue after rollback expiry.
