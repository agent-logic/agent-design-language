# v0.92 WP-04.16 Runtime Remediation Session Handoff

## Documentation Scope

- Target: this handoff only.
- Audience: a brand-new Codex task with no cloned conversational context.
- Canonical sprint: corporate v0.92 WP-04.16 Runtime remediation.
- Umbrella: open issue #142, `[v0.92][runtime] Operationalize distributed Runtime and polis Observatory across Wuji and AWS`.
- Active child: open issue #199, `[v0.92][WP-04.16b2] Govern learner, joint, and final voter membership transitions`.
- Publication attempted: false.
- Release approval claimed: false.
- Review approval claimed for the current implementation: false.
- Broad rewrite performed for this handoff: false.

The live #199 issue says `part_of: #142`, `depends_on: #191, #201, #202`, and `split_from: #192`. Legacy sprint #5862 is historical accounting only and is not this corporate WP-04.16 execution umbrella.

## Exact Working State

- Bound worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-199-governed-membership-preparation`
- Branch: `codex/199-governed-membership-preparation`
- Local HEAD: `139d9a7e0ed676fe44b3372800803ee483b8d62d`
- Current `origin/main`: `e172257b50ec9d6e07bbb0ab62a69a001ad1774f`
- Relationship: branch is 23 commits ahead of `origin/main`; HEAD is a merge of that exact main head.
- Typed lifecycle: issue #199 is `bound`, generation 20, digest `f12aa1e3c948eda65cff511388f53bd053af02e466d9f6cf86a4409790bfe50f`.
- Typed review, publication, and terminal records: all null.
- Live issue #199: open.
- PR state: no PR is attached; live typed issue read returned `pr_state: null`, and local publication state is null.
- Merge state: not merged.
- Required predecessor #202: merged by PR #236 at `f802430e2af3d6778013d35ab09a013fc927ef45`; that merge is ancestral to this HEAD.

Do not work from the primary checkout or make a new clone. Resume only in the bound worktree above.

## Completed Work

The branch contains the approved #199 saga design and typed bind, followed by these substantive implementation slices:

- `ba374455c`: opaque governed membership receipts and exact-current observation.
- `13cae3206`: bounded durable OpenRaft applied-membership history.
- `119536ac5`: durable membership coordinator and sealed PromoteVoter artifact.
- `ffd402edc`: coordinator binding to durable Raft membership history.
- `5ce25a3c0`: standard `add_learner` / `change_membership` orchestration.
- `394a836de`: external membership receipt verification.
- `069bf53cf`: exact 36-name public integration target.
- `efcdfb45c`: production receipt and history assertion markers.
- `57dbcaa77`: crash-phase recovery and conflicting retry proof.
- `31d33bf67`: issue-owned proof producer and validator.
- `6de252f06`: typed VPP lanes for the implemented proof surface.

The implementation includes stable Raft-ID validation, separate coarse Membership discriminators, factory-produced admission/exclusion receipts, crash-reconciled coordinator state, same-batch joint/final history retention, and public transition cases. These are implemented claims, but the final immutable proof and independent implementation review are not complete.

## Validation Actually Run

Current partial producer logs show:

- `distributed_membership_transition`: 36 passed, 0 failed.
- membership coordinator library tests: 5 passed, 0 failed.
- real four-node admission receipt test: 1 passed, 0 failed.
- pending-exclusion receipt test: 1 passed, 0 failed.
- strict library Clippy: command completed successfully.
- strict `distributed_membership_transition` Clippy: command completed successfully.
- the configured membership-history command completed with **0 tests**, so it is non-proving.

Earlier implementation validation also reported `cargo check --lib`, the correctly targeted membership-history test 1/1, and the authority-consensus suite 39/39. Those earlier results are useful development evidence, not a substitute for the unfinished immutable receipt.

The v1 producer ran all seven configured commands, then failed in its Ruby parser because the installed Ruby does not implement `Array#filter_map`. The immutable proof JSON was therefore not written, and the retained validator has not passed.

## Dirty and Uncommitted State

Preserve this state before changing anything:

- Modified: `.csdlc/prepared/issues/199/produce-proof-receipt.rb`
- Modified: `.csdlc/prepared/issues/199/validate-proof-receipt.rb`
- Untracked: `.csdlc/evidence/199/v1/*.stdout.log`
- Untracked: `.csdlc/evidence/199/v1/*.stderr.log`

The two Ruby files contain an uncommitted compatibility change from `filter_map` to `map { ... }.compact`. The untracked logs are the partial producer output from the failed run. They must not be described as an immutable proof receipt.

This handoff file is an additional intentional uncommitted documentation path.

## Unresolved Findings and Blockers

1. The membership-history argv is wrong in the producer, validator, and typed VPP. It names `distributed::transport::governed::polis_runtime::tests::membership_history_retains_joint_and_uniform_entries_from_one_apply_batch`; the implemented test is under `polis_runtime::authority_consensus_tests`. The current command silently ran zero tests.
2. The producer and validator must explicitly reject zero-test internal lanes, not merely accept exit code 0.
3. The Ruby compatibility edit is uncommitted and has not been rerun.
4. Partial untracked logs make the worktree non-clean, while the producer intentionally requires exact cleanliness before producing evidence.
5. No immutable proof introduction commit exists yet.
6. No fresh independent exact-head implementation review has occurred.
7. The 36-name integration target contains some broad `public_contract_asserted` mappings. The fresh reviewer must verify that the combined internal production lanes bind the actual factory, durable history, and crash-recovery behavior rather than accepting the name denominator alone.
8. No PR exists, so CI, publication, and merge have not started.

## Dependency Gates and Sprint Order

- Merged/ancestral prerequisites for #199: #191, #201, and #202.
- #199 is the only active implementation child in this worktree.
- Do not bind or implement #203 until #199 is independently reviewed, merged, ancestral, then #203 is resynced and its stale governed-path design is refreshed and reapproved.
- Remaining corporate sequence currently carried by the sprint: `#199 -> #203 -> (#205 and #210 preparation may overlap, landing serially) -> #204 -> #211 -> #193 -> #194 -> #142`.
- #193 and #194 are #142 remediation children, not #5862 children. #194 live AWS work also requires separate operator spend/account authorization.
- Final #142 demonstrations remain strictly serial: Wuji first with complete cleanup proof, then Wuji/AWS. They are far downstream and must not be started from this worktree.

## Exact Next Action

In the bound #199 worktree:

1. Inspect and preserve the dirty patch and partial logs.
2. Correct the membership-history argv to the exact `authority_consensus_tests` path in both proof scripts and through the typed VPP editor; do not hand-edit rendered cards.
3. Harden producer and validator acceptance so the internal history lane must execute exactly one named test and all other focused denominators are exact.
4. Decide how to preserve the partial logs outside the clean proving tree without claiming or deleting evidence, then commit the bounded proof-contract repair.
5. From a clean exact head still containing current `origin/main`, rerun the producer, commit the immutable evidence introduction, and run the validator plus typed `csdlc-validate` and doctor.
6. Record truthful SPP/SOR execution state through typed editor routes.
7. Send the exact clean head to a different independent reviewer. Fix every actionable finding before publication.
8. Only after a PASS: record review through typed v2, publish one ready PR with `Closes #199`, use the authorized one-large-runner/required-only CI policy, watch to full required green, then use typed finish if merge authority still applies.

## Non-Goals and Ownership Boundaries

- Do not resume any other child issue from this worktree.
- Do not implement #202 transport again; #199 consumes its governed ports and receipts.
- Do not absorb #200 concrete authority-store reconciliation.
- Do not touch kernel continuity, Guardian/API/WSS integration, models, AWS, live demonstrations, or final #142 integration here.
- Do not use AWS or Wuji, run optional CI, launch broad tests, publish, merge, close issues, or clean lifecycle/worktrees merely to repair the #199 proof.
- Do not use raw `gh` or sunset v1 lifecycle wrappers for covered lifecycle actions; use installed typed C-SDLC v2 owners.
- Do not delete or rewrite the current dirty logs without first preserving and classifying them.
- Lifecycle cleanup remains deferred unless separately authorized.

## Source Evidence

- Live typed issue reads for #142 and #199 on 2026-08-11.
- `.csdlc/issues/199/index.json` and all six #199 cards.
- `.csdlc/prepared/issues/199/design.md` and proof scripts.
- Git HEAD, `origin/main`, worktree registration, branch log, and #202 merge ancestry.
- Partial `.csdlc/evidence/199/v1/` command logs.

No implementation or lifecycle operation was performed to create this handoff.
