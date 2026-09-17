# ADR-CF-09: Local CodeFriend product boundary

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: CF-SHELL (#891).
Participating owners: CF-SHELL (#891), CF-INTEGRATE (#914).
Original curation: ARCH-ADR #911 under Sprint #935. Current reconciliation owner: #945. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

What product is Beta 1, and where does it run and integrate?

## Context

The selected baseline is a local operator-controlled CLI with artifact browsing in the current repository. The historical product ADR defines evidence-bound reports but does not specify this installed product journey. A separate repository split is still a decision owned by #848.

## Decision

Deliver CodeFriend through adl codefriend in this repository, following the selected CLI dispatch and cohesive product-module paths in CREATION_SELECTIONS. The shell exposes a real review journey over the completed ingestion/evidence/review consumers and local artifacts. It consumes shared Runtime, provider and Memory Palace contracts rather than embedding parallel service owners.

CF-INTEGRATE connects the completed consumers into one installed operator journey before CF-PROOF independently qualifies it on ADL and the pinned licensed external Rust scope. Neither integration nor qualification may absorb unfinished features and call them delivered. The selected qualification environments are macOS and Linux; this proposal makes no Windows support claim.

Beta 1 is not a hosted customer service, new authentication platform or multi-tenant deployment. The separately owned static Observatory sidecar does not change that boundary. Autonomous source mutation and broader integrations remain excluded. A later repository decomposition must be an explicit decision, not an incidental bootstrap choice.

## Alternatives Considered

A hosted service introduces tenant, account and operational responsibilities outside the admitted scope. A standalone repository now would preempt #848. Keeping only disconnected review skills would not deliver the installed product and complete journey required by CF-INTEGRATE. The local product provides a bounded delivery surface while retaining future boundary choices.

## Consequences

The initial product can be qualified on the repository's selected platforms and shared services without a hosted deployment program. Installation and integration become concrete acceptance obligations. Keeping the initial implementation together does not assert permanent functional coupling or settle commercialization boundaries.

## Reversibility

Move or replace the shell only through a reviewed product/repository decision that preserves versioned contracts and replayable artifact identity. Existing installations and stored runs need an explicit migration story; moving files alone is not proof of a successful split.

## Validation Notes

Run the installed journey with actual ingestion, review, synthesis, action plans, approval and renderers, including declared failures. CF-PROOF independently uses the pinned external scope after integration. Fixtures, CLI help and module presence alone cannot satisfy those tasks.

## Supersession Relationships

Refines accepted ADR0025. Defers any contrary repository allocation to the actual #848 decision; no permanent monorepo decision or extraction authorization is claimed.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md](../../../milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md](../../../milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md)
- [docs/adr/0025-codefriend-review-packet-product-boundary.md](../../../adr/0025-codefriend-review-packet-product-boundary.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.

## Implementation Reconciliation — #945

Pinned implementation revision: `e76dd7e785d778916b524864118ef079e0a1836f`. This is source inspection, not fresh runtime execution or acceptance.

The local operator module exposes start, inspect, cancel, retry and withholding; retry requires a settled active attempt. Separate CodeFriend modules implement consumers. This pinned tree has no codefriend/integration.rs.

Proposed clarification: Distinguish the delivered shell/consumer surfaces from the still-required complete installed integration #914 and independent qualification #915. Present local placement does not settle the separately owned #848 repository decision.

- [adl/src/codefriend/operator/mod.rs](../../../../adl/src/codefriend/operator/mod.rs)
- [adl/src/cli/codefriend_cmd.rs](../../../../adl/src/cli/codefriend_cmd.rs)
- [adl/src/codefriend/mod.rs](../../../../adl/src/codefriend/mod.rs)

Decision recommendation: accept the revised text as a design decision, subject to explicit operator approval. Current disposition: pending_operator_decision. No numeric allocation or supersession enacted. See the [current decision packet](../../../milestones/v0.92.2/adr/issue-945/README.md) for exact-content hashes, all69 accounting and #925 gate consequences.
