# ADR-CSDLC-02: Single-writer transition and recovery

## Status

**Proposed.** Complete candidate text; not accepted and not implementation proof. Candidate identifier is issue-local; no accepted ADR number is allocated.

Accountable scope owner: SIM-08 (#874).
Participating owners: SIM-06 (#872), SIM-07 (#873), SIM-08 (#874), SIM-09 (#875), SIM-UMBRELLA (#866).
Curation owner: ARCH-ADR #911 under Sprint #935. Acceptance authority: operator or explicitly designated decision owner; acceptance is pending, not inferred from this ownership map.

## Decision Question

How can an incompatible v3 issue-record and CLI change become active without competing writers or loss of post-transition evidence?

## Context

The selected simplification program develops bounded slices and activates the replacement during an explicitly scheduled writer pause. Existing selector provenance and all retained state must survive. A restore after new remote or semantic effects is different from restoring an untouched conversion snapshot.

## Decision

Inventory installed provenance, issue generations, bindings, pending transactions, remote intents and review/terminal evidence. Rehearse conversion on isolated copies, retaining source-to-destination mappings and semantic equivalence checks. Obtain an explicit operator pause window and fence old writers before live conversion; the replacement fencing only itself is insufficient. Preserve dirty work and classify in-flight operations rather than deleting them.

Snapshot and convert through staged, restartable transition steps. Validate the full census, exact bindings, receipts, installed binary and absence of competing writers before resuming. Reject obsolete request shapes with actionable conversion diagnostics after activation. Retain old bytes and executable as restore material, not as an alternate operational route.

Before any post-conversion operational write or remote effect, restore under the same pause/fence if verification fails. Conversion and staging writes alone do not cross this restore boundary. After an operational write or remote effect, automatic snapshot restoration is forbidden: freeze writers and use separately reviewed reconciliation or forward repair so new evidence is not lost and remote work is not repeated. The pause covers C-SDLC writers only; it grants no Runtime/provider shutdown or cloud authority.

## Alternatives Considered

Permanent dual writers simplify compatibility at the cost of conflicting authority. Replacing the binary before fencing old writers leaves a race the new binary cannot prevent. Unconditional snapshot rollback after remote effects can lose evidence and duplicate actions. A coordinated transition with a before/after-effects restore boundary makes recovery explicit.

## Consequences

The transition has a controlled interruption and requires a complete census and operator coordination. It avoids silent live conversion and preserves a finite recovery story. Successful conversion does not establish post-resume reliability; the selected consecutive-journey pilot retains failed and abandoned attempts and explicit measurement limits.

## Reversibility

The snapshot is directly restorable only before new operational writes/effects. Afterwards use the frozen, reviewed forward-repair path. Keep snapshot identities, conversion mappings and installed provenance so either path can prove what was retained or changed.

## Validation Notes

Inject crashes before/after durable intent, activation, receipts and projection completion; confirm semantic equivalence and no duplicate effects. Prove old writers cannot mutate during/after conversion and run checks from primary and bound worktrees. Keep paused qualification separate from the separately authorized post-resume pilot and its full outcome denominator.

## Supersession Relationships

Refines proposed ADR0072 within v3 and complements ADR-CSDLC-01. No selector acceptance, writer pause, installation or live conversion is authorized by candidate text.

## Source Evidence

Planning/source revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. Requirements below are planning contracts; the alternatives and tradeoff assessment above are the curator's proposed rationale, not a claim that stakeholders previously debated them.

- [docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md](../../../milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md)
- [docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json](../../../milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json)
- [docs/architecture/adr/0072-csdlc-v3-native-authority.md](../0072-csdlc-v3-native-authority.md)
- [docs/csdlc-v3/CURRENT_AUTHORITY.md](../../../csdlc-v3/CURRENT_AUTHORITY.md)

## Approval Boundary

Accepting this ADR would record this bounded design decision. It would not prove implementation, authorize live provider/cloud effects or external publication, or satisfy issue acceptance tests. Required unresolved decisions remain explicit in the milestone disposition map.
