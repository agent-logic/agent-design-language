# ADR-PLAT-01: Shared provider configuration and lifecycle

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: PLAT-PROVIDER (#876).
Participating owners: PLAT-PROVIDER (#876), RT-PROVIDER (#855), RT-COST (#854), PLAT-MLX (#903), PLAT-PAIR (#904), SPEC-RETEST (#905).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

Which shared owner defines a provider, and which owner admits and operates agents through it?

## Context

PLAT-PROVIDER owns validated editable definitions while RT-PROVIDER owns registry-based agent admission, conversation and lifecycle. RT-COST supplies usage/cost observations. Proposed ADR0075 covers profiles and non-authoritative shadow comparisons at the v0.92.1 scope; its historical MLX deferral differs from the explicit bounded v0.92.2 admission.

## Decision

Keep endpoint/profile loading in the shared provider-definition boundary, including validation and last-known-good behavior on invalid replacement. Configuration contains approved credential references rather than serialized secret values. Runtime consumes registered provider capabilities for admission and lifecycle instead of hard-coded provider-name allowlists. Product consumers, including CodeFriend, reuse that contract and do not fork provider clients.

RT-PROVIDER must support the declared add, replace, generated conversation, canonical-name A2A, roster projection, checkpoint, removal and rehydration lifecycle without Runtime restart. Preserve credential references and checkpoint compatibility. Its five-route matrix covers OpenAI, Anthropic, the approved Google route, Ollama and an OpenAI-compatible local endpoint; capability validation includes tools, streaming, model discovery/validation, usage accounting and health. RT-COST observations do not authorize a paid probe or turn unknown cost into zero.

The admitted MLX/Metal adapter is bounded by PLAT-MLX. PAIR remains an experiment and speculative-decoding requalification retains its own keep/repair/retire outcome. Profile materialization, local fixtures and shadow scores do not prove hosted-provider suitability or grant live execution authority.

## Alternatives Considered

A product-specific provider client is initially easy but forks credentials and capability behavior. Provider-name switches make extension require Runtime code edits. Combining configuration reload with agent lifecycle hides which owner failed or changed state. Shared definitions plus registry-driven Runtime lifecycle preserves separate accountable outcomes.

## Consequences

Providers can be introduced through registered capabilities while products share identity and privacy semantics. Each route still needs actual lifecycle qualification; configuration parity is not generated-response proof. Last-known-good configuration and truthful cost/health states add explicit operational obligations.

## Reversibility

Reject invalid definition replacements while retaining the last-known-good snapshot. Version incompatible checkpoint/capability changes and preserve credential references without copying credentials. Reverting configuration does not erase a live provider effect or restore removed agent state without the lifecycle owner's supported path.

## Validation Notes

Exercise editable definitions, invalid reload, secret rejection and last-known-good state. Run a newly registered fixture without match-statement edits, then separately authorized five-route lifecycle proof with generated responses, checkpoint/removal/rehydration and truthful projections. Verify no recurring paid probe or serialized credential is introduced.

## Supersession Relationships

Refines accepted ADR0004/0041 and proposed ADR0075 without promoting0075. Record the scope conflict explicitly: v0.92.2 admits bounded MLX, while0075 preserves its historical v0.92.1 deferral. OCI and general local-model suitability remain unclaimed.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/adr/0004-provider-profiles.md](../../../adr/0004-provider-profiles.md)
- [docs/adr/0041-provider-model-suitability-boundary-v2.md](../../../adr/0041-provider-model-suitability-boundary-v2.md)
- [docs/architecture/adr/0075-provider-profile-and-shadow-authority.md](../0075-provider-profile-and-shadow-authority.md)
- [docs/milestones/v0.92.2/DECISIONS_v0.92.2.md](../../../milestones/v0.92.2/DECISIONS_v0.92.2.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.
