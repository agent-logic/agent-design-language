# ADR-CF-05: Executable governance

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-GOV (#887).
Participating owners: CF-GOV (#887), CF-GOV-CI (#888).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

How should architecture fitness checks act as local and CI gates without presenting human judgment or execution errors as a machine pass?

## Context

The milestone splits a local fitness runner from its CI adapter. Their atomic contracts require actual passing and failing invariant execution, shared result semantics, actionable locations, and explicit handling of runner errors and missing artifacts.

## Decision

Declare machine-checkable architecture invariants and execute them through the local CF-GOV runner. Emit deterministic pass/fail results and actionable locations for the declared rule and scope. Keep questions needing human judgment visible as such; neither an unevaluated recommendation nor a missing policy may become a pass. Reject hidden policy that cannot be traced to the declared input.

CF-GOV-CI invokes the same local runner and propagates its pass, fail and error states to actual CI exit status and retained artifacts. CI must not contain a second interpretation of the invariant. A runner error or missing result artifact cannot produce successful gate evidence. This governs the bounded architecture fitness function, not all policy decisions in ADL or unrestricted execution of repository scripts.

## Alternatives Considered

A separate CI implementation allows local and hosted runs to disagree on the same rule. A prose review labeled as fitness proof obscures the absence of execution. Converting errors into warnings improves apparent stability but makes the gate untrustworthy. One runner and explicit outcome propagation avoid those failures.

## Consequences

An operator can reproduce a CI finding locally and inspect the declared invariant. Rule authors must define meaningful inputs, failure locations and resource limits. The gate proves only the evaluated invariant on its admitted scope; a green run is not broad architectural approval.

## Reversibility

Change rules through a versioned, reviewed configuration or implementation change, preserving the prior result and source revision. A policy rollback changes future enforcement; it does not turn earlier failures into historical passes.

## Validation Notes

Run one passing and one failing invariant locally, then through the actual CI adapter and compare result semantics. Inject runner failure, missing artifacts and human-only questions and confirm none becomes success. Record deterministic local CPU proof separately from hosted CI evidence.

## Supersession Relationships

Refines accepted ADR0025 within the bounded Beta 1 product. No accepted record is superseded or rewritten by this proposal.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ADR_PLAN_v0.92.2.md](../../../milestones/v0.92.2/ADR_PLAN_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/adr/0025-codefriend-review-packet-product-boundary.md](../../../adr/0025-codefriend-review-packet-product-boundary.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove the implementation, authorize provider/cloud execution or external publication, or satisfy the associated issue acceptance tests. Conflicts and required unresolved decisions remain visible in the milestone disposition map.
