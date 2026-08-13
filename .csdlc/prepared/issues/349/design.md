# Issue #349 design: coherent deferred-validator admission across ready

## Problem and authority

The current execution-readiness check admits an exact, issue-owned deferred validator only while an issue is `initialized`, while doctor advertises `advance_ready` as the next operation for that same passing packet. The unchanged `ready` packet then fails readiness and cannot return to `initialized`. This tooling defect is independent of #342; #342 is reproduction evidence only and is read-only here.

## Bounded change

Keep the #79 deferred-target predicates unchanged and admit them in both pre-bind phases, `initialized` and `ready`. `execution_readiness_findings_for_cards` may set `allow_deferred` only for those two phases. Bound and every later execution phase continue to deny deferred admission, so a missing validator, zero selected tests, or absent proof still fails closed.

Doctor remains truthful: a finding-free initialized packet may advertise `advance_ready`; after the typed transition the unchanged packet remains doctor-clean and bindable. A finding-free ready packet advertises inspection/bind readiness without inventing execution proof. Bind may still atomically advance initialized to ready and then bound, or bind an explicitly ready packet. No reverse transition and no manual state repair are introduced.

## Regression contract

Extend the focused Gate 2 regression to follow the actual advertised protocol:

1. Create an initialized packet with exact issue-owned source/test paths, explicit validator deliverable, proving argv, bounded non-placeholder defer reason, and fail-closed policy.
2. Prove initialized doctor PASS, `ready=true`, and `next_operation=advance_ready`.
3. Apply the typed `advance_phase` edit to `ready` using exact generation/digest guards.
4. Prove the unchanged ready packet remains doctor PASS and can bind.
5. Prove bound state rejects the still-missing source/validator and lacks an issue-specific denominator.
6. Materialize the exact owned source and test target, then prove doctor PASS.
7. Retain the existing #79 mutation matrix so arbitrary paths, placeholder defer reasons, non-proving lanes, and weak failure policy remain rejected.

## Ownership

Owned product paths are limited to:

- `csdlc-v2/src/cards.rs`
- `csdlc-v2/src/doctor.rs` only if truthful ready guidance needs an explicit assertion or wording repair
- `csdlc-v2/tests/gate2.rs`
- issue-local `.csdlc/issues/349`, `.csdlc/prepared/issues/349`, and `.csdlc/evidence/349`

`csdlc-v2/src/lifecycle.rs` is read-only contract evidence unless review proves a necessary defect in bind semantics. #342 and every other issue record, card, worktree, product path, and GitHub surface are forbidden.

## Validation and review

Run the issue-owned preparation validator before review. After bind and implementation, run the focused Gate 2 regression, existing deferred-target mutation tests, C-SDLC v2 unit/integration tests appropriate to the touched crate, formatting, and strict Clippy. Record that pre-bind deferral is planning admission only, never execution proof. Require a fresh exact-head implementation review before publication.

## Failure policy

Stop on any widening beyond initialized/ready, any weakened deferred-target predicate, any post-bind deferred admission, any #342 mutation, any unrelated lifecycle redesign, any regression failure, or any need for placeholder files.
