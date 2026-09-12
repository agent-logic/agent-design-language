# ADR-CF-07: Longitudinal comparison and Memory Palace

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-MEMORY (#885).
Participating owners: CF-MEMORY (#885), PLAT-MEMORY (#889), CF-COG-DRIFT (#886).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

When may CodeFriend compare findings across runs, and how does the shared Memory Palace participate without becoming a second evidence store?

## Context

Longitudinal comparison is a required Beta 1 consumer. Accepted ADR0058 defines Memory Palace as a bounded deterministic context handoff layer, not a replacement for underlying memory storage. The new product requires compatible prior-review retrieval and explicit matching outcomes.

## Decision

CF-MEMORY owns baseline compatibility and finding matching against CF-EVIDENCE identities. Compare repository, scope and contract versions and the completion coverage needed by the comparison. Emit added, resolved, changed, unchanged or not-comparable with a match reason; ambiguity is explicit. Absence from a partial or narrower run cannot automatically resolve a finding.

PLAT-MEMORY connects the actual second-review path to shared Memory Palace retrieval. Reuse its bounded context, citation, privacy and continuity semantics; do not introduce a product-specific memory authority or redefine Memory Palace as the durable evidence store. CF-EVIDENCE retains evidence admission/retention ownership. Redaction and deletion remain effective when selecting a prior review. Architecture drift consumes the same compatibility decision rather than inventing a weaker comparison rule.

## Alternatives Considered

Selecting the newest available result is simple but may compare unrelated scopes or incomplete runs. Treating text similarity as sufficient matching confuses reworded findings with resolved defects. Building a separate CodeFriend memory store duplicates established ownership. Explicit compatibility plus the shared context handoff meets the second-review requirement without replacing the existing substrate.

## Consequences

A user can trace a comparison to two actual runs and understand refusal or ambiguity. Some histories remain not-comparable, which is a truthful limitation rather than a failed search. Integration must prove the real retrieval consumer; a callable library or fixture-only hook does not complete PLAT-MEMORY.

## Reversibility

Retain original runs and comparison manifests when matching rules change. Create a newly identified comparison under the new version; do not rewrite earlier match reasons. Removing a prior review must affect later retrieval and derived reports according to the recorded deletion policy.

## Validation Notes

Run the actual second-review retrieval and comparison path. Exercise compatible history, changed scope, incompatible version, partial coverage, ambiguous identity and deleted/redacted baselines. Verify architecture drift does not report resolution from missing coverage.

## Supersession Relationships

Refines accepted ADR0058 for the CodeFriend consumer and accepted ADR0025 for review evidence. Memory Palace remains a context handoff layer; neither accepted record is superseded.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md](../../../milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/adr/0058-memory-palace-context-handoff-architecture.md](../../../adr/0058-memory-palace-context-handoff-architecture.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.
