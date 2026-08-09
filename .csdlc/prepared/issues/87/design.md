# Issue 87 Design: ACIP minor-version compatibility predicate

## Context

ACIP currently advertises protocol version `1.0`. Its negotiation predicate compares the
constant minor version `0` against both ends of a caller-provided inclusive range. Strict
Clippy correctly observes that `0 > maximum_minor` is impossible for `u32`, making two
Sprint 4 integration-test targets fail before their owned behavior can be validated.

## Design

Keep the offer contract as an inclusive minor-version range. Reject malformed ranges and
offers whose minimum minor exceeds the locally supported minor. The existing upper-bound
comparison is redundant for the current minimum `u32` constant and is removed; this does
not weaken negotiation because any non-empty range with `minimum_minor <= 0` necessarily
contains `0`.

Add a focused, issue-owned integration test for:

- the exact supported offer `0..=0`;
- a wider compatible offer `0..=1`;
- an unsupported future-only offer `1..=1`;
- a malformed descending offer `1..=0`.

## Ownership boundary

Issue 87 owns only `adl-runtime/src/acip.rs`,
`adl-runtime/tests/acip_version_negotiation.rs`, and issue-local lifecycle evidence. It does
not modify the implementation or test modules owned by Sprint 4 children 5866, 5871, or
5872. Their named strict-Clippy targets are validation consumers of this shared fix.

## Validation

Run the ACIP-focused library test, warning-denied library Clippy, and the two strict
Clippy commands named by the issue once the child test targets are present on the
integration surface. Fail closed if version-range behavior changes outside the four
declared cases.
