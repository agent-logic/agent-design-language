# #5007 Memory Palace Proof To Claim Table

Source proof is PR #5740 for #4760 at head
`94156d55d0a1f4bfda7ce32ac136437520325906`. GitHub reported the PR as
non-draft, with merge state blocked and review pending at observation time.
This table therefore records ready proof evidence without claiming merge,
closure, or `main` availability.

| #5007 ADR claim | #4760 evidence | Accepted boundary |
| --- | --- | --- |
| Memory Palace has an implementation, not only planning text. | `adl/src/memory_palace.rs`; #4760 SOR lists the implementation and ready publication. | Ready PR proof, not merged source. |
| Memory Palace emits deterministic handoff packets. | `context_packet_bytes`; `memory_palace_packet_is_deterministic_after_canonical_ordering`; `memory_palace_fixture_builds_deterministic_obs_mem_handoff`. | Deterministic for the MVP input schema and canonical ordering. |
| Memory Palace consumes ObsMem-shaped records. | `MemoryPalaceInput.records: Vec<MemoryRecord>` and citation validation against `MemoryCitation`. | ObsMem remains the underlying record/citation substrate. |
| Chronosense continuity and temporal anchors are consumed. | `MemoryTemporalAnchor`, `required_continuity_id`, stale/future temporal rejection, continuity mismatch rejection. | Memory Palace validates and carries anchors; it does not own time. |
| The long-lived-agent runtime consumes the handoff packet. | `long_lived_agent_cycle_consumes_memory_palace_context_ref`; `decision_request.memory_refs`; `cycle_manifest.artifacts.memory_palace_context`. | One bounded long-lived-agent consumer path. |
| Negative and boundary behavior fails closed. | Focused runtime log shows missing citation hash, stale context, private path/temporal mismatch, overflow, deterministic packet, fixture handoff, and long-lived-agent tests passing. | MVP fail-closed validation; broader retrieval policy remains future work. |

Validation references:

- #4760 `.csdlc/evidence/4760/diff-hygiene.log`: `git diff --check` passed.
- #4760 `.csdlc/evidence/4760/memory-palace-focused-runtime.log`: focused
  Memory Palace runtime wrapper passed.
- #4760 `.csdlc/issues/4760/cards/srp.md`: exact-head review result passed,
  with residual risk noting the stale checked-in `adl/Cargo.lock` condition.
