Part of #522. Additional remediation for internal finding `D520-V3F-001`.

## Outcome

Make native C-SDLC v3 pull-request-ready recovery safely handle a retained legacy intent that predates `resolved_ready_target`, so an authenticated retry cannot become stuck at `github_pr_ready_target_missing`.

## Confirmed defect

The post-remediation #835 current-candidate proof passed 219 tests, strict clippy, and the V3-A contract, but the live V3-F validator rejected `current_source_drift`. Independent exact-head review then reproduced this recovery gap in `csdlc-v3/src/commands/remote/mod.rs`: an existing ready intent with `resolved_ready_target: null` is accepted and reused, then dispatch fails because the retained target identity is absent.

## Deliverables

- Recover a legacy ready intent by resolving and durably retaining the authenticated PR number, node ID, draft state, and exact head before dispatch.
- Preserve retry idempotence and fail closed on target/head mismatch or uncertain mutation state.
- Add focused regression coverage beginning from a retained legacy intent with no `resolved_ready_target`; the test must not manually repair the intent.
- Refresh the #835 release-gate projection only after this fix merges and the complete current-candidate proof passes.

## Acceptance criteria

- [ ] A matching retained legacy ready intent recovers automatically and reaches the same guarded ready/reconciliation path as a newly created intent.
- [ ] A mismatched or unverifiable target fails before mutation.
- [ ] Repeated recovery remains idempotent and does not leave private input residue.
- [ ] Focused tests, the V3-F current-source validator, and independent exact-head review pass.
- [ ] The refreshed #835 projection records the resulting immutable candidate rather than the already-merged stale projection.

## Non-goals

- Broad GitHub mutation redesign.
- Raw `gh` lifecycle writes.
- Treating the merged #840 projection as current proof.
