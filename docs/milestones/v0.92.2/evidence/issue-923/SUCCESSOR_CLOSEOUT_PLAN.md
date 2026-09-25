# Successor closeout and responsibility plan — #923

Status: executable planning result for #924 review; no successor execution or release is claimed. Applies separately to v0.93.1 and v0.93.2. Prepared from #922 revision dc95ab5b8b403f541c0c9a5ccf872abd087a6507; acceptance follows its merged result.

## Scope and source authority

The [split map](../../../v0.93/MILESTONE_SPLIT_v0.93.json) assigns all 83 original results once. [v0.93.1](../../../v0.93.1/EXECUTION_PLAN_v0.93.1.json) owns the repository split first, all branded template/style-guide results, and CodeFriend Beta 1 qualification and launch on a pinned compatible Runtime. [v0.93.2](../../../v0.93.2/EXECUTION_PLAN_v0.93.2.json) owns Runtime v4 and remaining platform work. Existing #1150 supplies CF-05, with #1148/#1149 as producers; do not duplicate it. Existing #875/#1145 are separate v0.93.2 sidecars. #671/#1169 completed in v0.92.2 and are not successor work.

Use the [predecessor handoff](../../NEXT_MILESTONE_HANDOFF_v0.92.2.md), [feature coverage](../../../v0.93/FEATURE_COVERAGE_v0.93.md), and [TBD source inventory](../../../v0.93/planning-review/tbd-allocation-inventory.json). Historical combined v0.93 documents remain source context; the split packages control allocation. Do not regenerate new issue identities from superseded catalogs.

## Responsible roles

1. Daniel Austin is the operator: accepts scope/deferrals and approves concrete release destinations and artifacts.
2. Planning #11 is the current planning and closeout coordinator; at successor opening it records the assigned implementation owners against existing task identities before dispatch. No unnamed future worker is presumed active.
3. Each bound task owner delivers its one result, focused proof, reviewable PR and truthful output record. Repository owners publish component artifacts and identify compatible versions.
4. An independent reviewer, different from the implementer, owns the exact-candidate findings/dispositions. Planning #11 arranges that assignment before each review begins; a role assignment is not a completed review.
5. The release operator executes only Daniel's approved publication; it retains authenticated readback. Worker #1 owns terminal bookkeeping and eligible cleanup, separately from delivery acceptance.

The coordinator can hand this plan to another session without transferring private credentials. If a role is unavailable, reassign that role explicitly; do not block unrelated accepted component work.

## Inputs and convergence

Each result supplies repository/issue/PR, immutable source SHA, artifact digest, proof command and outcome, independent review, omitted checks and remaining limits. The integration owner builds one multi-repository lockset from merged component outputs; qualification consumes those exact versions. New bytes invalidate only affected proof/review. A successful unit test, green CI, merged PR or closed issue alone is not installed qualification.

For v0.93.1, RD-11 accepts repository boundaries and the lockset; CF-05 consumes actual #1150 qualification; CF-07 records the authorized live launch; QUALIFY audits those consumed results instead of duplicating the full campaign. v0.93.2 preserves launched CodeFriend compatibility while adding v4/platform acceptance. Each separate milestone runs the following tail once.

## Canonical ten-step tail

| Order | Owner role | Entry and required evidence | Completed output / exit |
|---|---|---|---|
| TAIL-01 | Quality owner | Integrated lockset, every required result and actual proof | Gap/quality assessment; every omission assigned a truthful disposition |
| TAIL-02 | Documentation owner | Accepted quality assessment and source versions | Complete current docs and external-review packet, stable paths, source hashes |
| TAIL-03 | Publication owner | Accepted docs, artifact inventory and privacy boundaries | Final candidate/manifest; publication readiness, not presumed public release |
| TAIL-04 | Independent internal reviewer | Exact candidate and accessible evidence | Preserved original review and severity/evidence/owner ledger |
| TAIL-05 | Independent external reviewer | Candidate identity and access verified before substantive review | Original external assessment, or explicit failed/not-proven attempt; never a fabricated pass |
| TAIL-06 | Remediation owner and operator | Both reviews, immutable original findings | Fixes with affected checks/re-review, or explicit operator dispositions; final ledger reconciled |
| TAIL-07 | Planning coordinator | Settled finding dispositions and feature/TBD inventory | Complete successor scope, single-task boundaries, graph and proof feasibility |
| TAIL-08 | Closeout coordinator | Accepted successor plan | Roles, ten-step tail, evidence/approval boundaries, asynchronous finish and recovery plan |
| TAIL-09 | Independent planning reviewer | Exact TAIL-07/08 bytes and validation | Findings resolved or explicitly rejected with reasons; accepted digest-bound planning review |
| TAIL-10 | Release operator and Daniel | Accepted TAIL-09, all required product/sidecar dispositions, exact candidate/manifest approval | Authorized release or explicit not-released decision; authenticated milestone close and successor handoff |

Tail acceptance is serial. Drafting and independent component implementation may overlap where their actual dependencies permit. A later acceptance does not erase an earlier failed review or convert a deferred qualification into a pass.

## Asynchronous finish and cleanup

Downstream starts depend on the specific merged result and acceptance gate, not all workers finishing housekeeping. Native `finish` establishes terminal bookkeeping after observed merge; `clean` is separate and requires exact-worktree eligibility. Worker #1 can perform them later without holding another task whose required result is already accepted. Neither a parent's closeout nor a child's cleanup is a prerequisite for the other's result acceptance. Preserve dirty/unmerged/foreign state and backups; do not delete ambiguous worktrees.

## Residual and TBD reconciliation

| Source/result | Disposition and next accountable consumer |
|---|---|
| #915 installed product qualification | Explicitly incomplete/deferred; #1148/#1149 produce fixes, #1150 independently qualifies before v0.93.1 Beta 1 launch |
| #919 internal review | All findings fixed; retained source-time review is immutable |
| #920/#921 external attempt | Attempt failed; operator closed #921 without further remediation. Preserve original assessment, do not claim external pass or create speculative successor tasks |
| Versioned templates and 4+1 architecture outputs | v0.93.1 CT/CF scope; qualification must exercise actual branded outputs and upgrades |
| Runtime v4, citizen migration, governance/security | v0.93.2 RV/CM/GOV/SEC packages; no v4 dependency invented for Beta 1 |
| AWS Terraform conversion | Completed #495/PR #590; retained rollback is not a duplicate conversion task |
| rust-hot-reloader evaluation and other deferred inventory rows | Retain existing inventoried disposition; no automatic task admission; coordinator requests scope decision only when a concrete consumer needs it |
| #875 pilot and #1145 Observatory | Preserve original identities and separate effect approvals in v0.93.2 |
| Podcast #671/#1169 | Completed Apple/Spotify launch in v0.92.2; deferred outlets are outside successor core counts, social promotion belongs to Darlicia |
| Historical source inventory rows | Apply split map to retained source task IDs; unknown/new work stays an explicit backlog decision, never an assumed implementation issue |

## Recovery and escalation

1. Before review, compare packet revision and manifest to the checkout and test access to every required evidence location. On mismatch, repair the handoff before asking for substantive review.
2. If a test fails, the task owner repairs that bounded result and reruns affected proof; coordinator routes independent review of changed bytes. Preserve original failures.
3. If review fails or does not execute, record failed/not-proven and let Daniel choose remediation or an explicit residual disposition. Never present a gate assessment as a completed code review.
4. For uncertain remote writes, observe/reconcile the original native operation before any retry. Do not bypass a guard with a new request identity.
5. Changed artifact hashes return to the affected review/approval step. Release operator must not apply approval for old bytes to new bytes.
6. Unavailable reviewers/owners are assignment problems; missing product proof is a qualification problem. Report which applies and progress unrelated safe work.

## Tabletop validation and handoff

The following planning walkthrough was performed; it is not an operational failure drill or product qualification:

1. Delayed worker cleanup after accepted merge: the dependent task proceeds; Worker #1 finishes later. No finish cycle.
2. New template changes after qualification: rebuild the lockset and repeat affected output/upgrade checks and review; keep unrelated evidence.
3. External reviewer opens the wrong checkout: stop substantive-review claims, correct candidate/access, or record failed attempt and seek an explicit disposition.
4. Release request lacks exact artifact identity: prepare the concrete candidate and destination for approval; do not publish.
5. Sidecar remains deferred: keep its explicit issue/disposition, exclude it from unrelated core task counts; do not claim its live proof.

#924 reviews this complete packet and the accepted #922 package independently. #925 owns v0.92.2's final closeout. The operator authorized the next milestone after closure and a 15-minute break; measure that interval from authenticated milestone closure, then record opening. This document does not start the clock or open execution.
