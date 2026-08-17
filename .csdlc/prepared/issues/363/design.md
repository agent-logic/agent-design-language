# #363 Sequenced Implemented Plan-Summary Recovery

## Boundary

#363 changes only `csdlc-v2/src/store.rs` and focused store/gate tests. It does
not mutate #274, weaken review/publication invalidation, or add a generic
Implemented `set_field` capability.

## Recovery epoch

`correct_plan_summary_after_recovery` remains legal only in Implemented with
exact generation/digest CAS, nonempty actor/reason, and null assignment, review,
publication, readiness, and terminal truth. Authority begins at the most recent
`recover_review` audit event whose transition returned this issue from Reviewed,
Published, or MergeReady to Implemented. Every later audit event must remain in
the same nonterminal recovery epoch and be one of: design approval, SPP affected
areas/plan steps/invariants/stop conditions, VPP validation lanes, SIP authority
boundary/operator constraints, STP acceptance/deliverable recovery, SRP review
prompt repair, SOR execution-evidence repair, or advisory estimates. A later
summary correction, assign/record/recover review, publication, readiness,
terminal transition, bind/finalize, migration, unknown operation, or phase exit
ends the epoch and fails closed.

The correction appends an audit entry that names the authorizing recovery event
sequence and generation. It changes only SPP summary bytes and the derived card/
index generation and digests. Immediate generation-equal recovery remains valid.

## Proof

Focused tests reproduce #274's Published -> recover_review -> approve_design ->
SPP affected areas/steps -> VPP lanes -> summary repair sequence. Negatives cover
missing recovery, stale CAS, current assignment/review/publication/readiness/
terminal truth, unrelated later lifecycle transition, unknown intervening audit
operation, different issue/epoch, and terminal phases. Existing immediate repair
behavior remains green, and a second summary correction in the same epoch is
rejected.
