# Issue #821 execution plan

Status: active preparation for draft PR publication. Final-candidate proof has not run.

This issue consumes exactly `TAIL-01:retained-188-ac-1` through `ac-4` from the immutable #764 denominator. `preparation.json` preserves their criterion digests, source hashes, current prerequisite observations and individual proof requirements. These checks establish accounting integrity, not completion.

## Execution sequence

1. Preserve the historical gate and reconciliation evidence. Inspect the current runner and all required lane inputs, including the second #520 finding register. Resolve a concrete reproduction command for each proof before running it.
2. Verify already-integrated #819 proof and integrate #818 and #820 proof when available. Verify their actual terminal artifacts and Git ancestry, not just issue closure. Recheck every other P1/P2 finding from the second #520 review.
3. Pin the final remediation candidate and independently recompute the required lanes. Capture command, exact revision, exit status, artifact identity and lane-specific result. Reject missing, skipped, stale or non-proving evidence.
4. Regenerate the quality gate and final census using existing tooling. Reconcile these four rows exactly once. Keep preparation and historical evidence separate from final proof. If shared tooling needs repair, route that scope separately rather than weakening the gate.
5. Obtain bounded exact-head subagent review, resolve findings, update native SRP/SOR, and publish the reviewed PR with `Closes #821`. Final evidence requires zero unresolved release blockers. Merge and release authorization remain separate.

## Validation boundaries

The preparation check verifies four unique row IDs and matching criterion digests against both source inventories. No Rust suite or cloud lane is justified by this preparation artifact alone. Final release proof must execute the existing required lanes; docs-only CI is not a substitute.

Candidate movement or changed prerequisite evidence requires refreshing the proof plan before accepting final results. No row is marked proven in this preparation packet.

## Located reproduction routes

- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb` validates the retained gate; it does not execute its underlying proof lanes.
- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb --negative` exercises the retained validator rejection cases.
- `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py` checks pinned historical corrections; it is not final-candidate proof.
- The Ruby `--generate` route writes the canonical TAIL-01 gate, denominator and blocker artifacts from `release-tail-admission.json`. Do not invoke it until candidate-bound input refresh is established and historical evidence preservation is resolved. Merely regenerating from the old admission input cannot complete #821.

The prerequisite integration and exact-candidate input refresh remain pending. Preparation is suitable for reviewing the proof plan, not for recommending release.

Preparation validation: both retained validator commands above passed. The historical gate still reports its blocked decision; neither result establishes final proof. Independent preparation review found incorrect #819 ancestry wording; Git ancestry verified PR #827 is already included and the packet was corrected. Final exact-candidate review remains pending.

Operator requested draft publication before prerequisites finish. The draft exposes the reviewed preparation and outstanding proof requirements; it must not become merge-ready until all four obligations and final-candidate review pass.
