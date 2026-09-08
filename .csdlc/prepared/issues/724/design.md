# Issue #724 design: simple v3 issue creation

## Outcome

Add one operator-facing `github-issue create` form that accepts familiar issue fields and translates them into the existing typed operational dispatch. The request-file form remains available as the advanced/audit interface.

## Dependency and authority

The implementation is stacked on #721/PR #726, which supplies typed `issue_create` mutation, authenticated readback, durable intent, marker, receipt, and reconciliation behavior. This issue does not duplicate or weaken those mechanisms. The command must fail closed whenever the underlying v3 operational authority gate is inactive.

## Command boundary

The simple form accepts exactly one title and exactly one of inline body or body-file, plus repeatable labels and assignees and an optional milestone. It constructs the same `OperationalRemoteDispatchRequest` consumed by the request-file path and invokes the same preparation/execution function. It neither shells out to `gh` nor creates an alternate transport.

Authority-binding fields that are not ordinary issue content must be resolved from the same canonical lifecycle authority used by the existing route; if they cannot be resolved exactly, the command refuses before remote mutation.

## Proof

Focused command tests cover missing/ambiguous title and body inputs, optional-field projection, marker injection, pre-cutover refusal, assigned issue-number receipt, and idempotent reconciliation. Existing remote publication tests continue to prove the shared transport path.

## Documentation

Operator documentation presents the simple form first and labels request-file usage as the advanced/audit form. It states the v3 authority gate explicitly.

## Non-goals

- No raw `gh` lifecycle route.
- No v2 fallback.
- No alternate receipt or credential path.
- No generic CLI framework redesign.
- No merge or cutover authority change.
