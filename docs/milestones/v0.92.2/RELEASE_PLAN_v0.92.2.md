# v0.92.2 Release Plan

Status: planned; no date commitment.

## Candidate Formation

CF-INTEGRATE first forms the installed integrated candidate from complete, merged and reviewed Beta 1 consumers. CF-PROOF then independently executes ADL and pinned external OSS qualification against that candidate. TAIL-01 requires both completed results and their current evidence before release preparation proceeds. A candidate is not a release.

## Release denominator

The release denominator is exactly the 69 work-package rows in the canonical [planned issue catalog](PLANNED_ISSUE_CATALOG_v0.92.2.md), with structure and dependencies defined by the [WBS](WBS_v0.92.2.md), [issue wave](WP_ISSUE_WAVE_v0.92.2.yaml), and [execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml). Nine rows reuse #720, #848, #849, #852, #854, #855, #861, #862, and #864; 60 remain prospective. Issues #717 and #718 are merged v0.92.1 predecessor inputs. TAIL-01 must account for all 69 rows, but merge/review completion is a no-go condition only for its declared dependencies. OBS-S3 and ARCH-ADR remain required milestone issues outside the closeout-tail gate. #848 delivers the split decision but no split implementation authority.

## Canonical Tail

1. TAIL-01 — quality gate
2. TAIL-02 — documentation review and external-review handoff
3. TAIL-03 — publication finalization
4. TAIL-04 — internal milestone review
5. TAIL-05 — external or third-party review
6. TAIL-06 — accepted-findings remediation or explicit deferral capture
7. TAIL-07 — next-milestone planning
8. TAIL-08 — next-milestone closeout planning
9. TAIL-09 — next-milestone planning review
10. TAIL-10 — release ceremony and milestone close

This order matches the canonical standard. Product dependencies are merge-based; individual closeout receipts are asynchronous.

## Go/No-Go

Go requires the quality gate, current internal and external review, disposition of accepted findings, truthful release notes, complete artifact manifests, and human release approval. No-go includes unresolved P1 findings, missing proof, privacy or provenance failure, renderer claim drift, or an exit-bar item without an owner.

## Rollback

The release candidate is a normal Git revision and published artifacts are versioned. If post-publication evidence contradicts the release claim, withdraw or supersede the affected artifacts, preserve the evidence, revert the bounded release changes, and reopen remediation without rewriting historical review truth.

## Candidate convergence within the canonical tail

TAIL-06 records the candidate revision and complete artifact-manifest digest before and after each remediation. Any substantive candidate or claim/artifact change rebuilds affected artifacts, reruns affected proof, refreshes TAIL-01 quality truth, and obtains current internal and external review against the revised candidate. Repeat within the remediation gate until findings, evidence and review agree. Earlier review remains immutable historical evidence, never approval of new bytes. TAIL-10 requires exact equality among release candidate, reviewed candidate, approved manifest and release authorization. Deferral cannot waive unresolved P1 or privacy/provenance failures. The outer ten-step order stays unchanged.

Product integration may finish before independent supporting work. TAIL-01 converges the declared release-gating support set, including #720, and verifies that the merged v0.92.1 #717/#718 predecessor contracts are consumed before milestone quality approval. OBS-S3 and ARCH-ADR are explicitly excluded from both CF-INTEGRATE and TAIL-01 dependencies; they remain ordinary required additional issues with their own acceptance rather than closeout-tail work.

## TBD disposition checkpoint

TAIL-07 and TAIL-08 must refresh the tracked [TBD scheduling reconciliation](TBD_SCHEDULING_RECONCILIATION_v0.92.2.md) against the then-current local inventory. The refresh is an accounting gate, not authority to add work. Every active source discovered after the retained #620 audit must be classified as admitted work, merged/completed provenance, an existing issue or milestone, or explicit backlog/deferred work before TAIL-09 review.

The known unadmitted product candidate that must remain visible is `.adl/docs/TBD/codefriend_ai/POLIS_MUSIC_STUDIO_CONCEPT.md`. This is not an exhaustive inventory claim; the required TAIL-07/08 refresh supplies that denominator. The operator admitted `.adl/docs/TBD/provider_model/NVIDIA_PAIR_EXPERIMENT_PLAN_v0.92.2.md` as PLAT-PAIR and `.adl/docs/TBD/GCP_ACCOUNT_MOVE_IN_PLAN.md` as OPS-GCP. The older issue #122/#268 cloud plans remain v0.92.1/cloud provenance and are not duplicate successor rows. The Runtime v4 plugin-system design remains scheduled for v0.93. Issue-local C-SDLC audit snapshots and review outputs are evidence, not product backlog rows.

## First-sprint convergence

The independently launched SIM sprint runs first. TAIL-01 consumes completed SIM-UMBRELLA after SIM-07 qualification, SIM-08 operations preparation, and the separately authorized SIM-09 pilot. The sprint does not gate CF-INTEGRATE; closed merged v0.92.1 #717/#718 are predecessor inputs, not active bugfix lanes. If activation is not authorized, record the unresolved program gate; do not silently call the sprint or milestone complete.

## Complete-task release accounting

The [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md) control the eight split families and eleven strengthened completion criteria. Candidate formation requires actual installed-product qualification, not merely authored review packets. Every implementation row must identify its production consumer and executed success/failure evidence. Article and manuscript rows deliver their complete selected writing outputs; the seven planning rows finish their declared decisions, documents or rehearsal. None may substitute for a missing Beta 1 feature, and integration must not absorb partial upstream work.

Before issue execution, reconcile the narrowed #852 failure-event task, #855 provider-lifecycle task and #862 local-decomposition task with typed issue authority and their prospective successors. Child creation remains separately authorized; this documentation correction creates none.

## Final milestone completion

OBS-S3 and ARCH-ADR keep their explicit non-dependency status for CF-INTEGRATE and TAIL-01. They are still required milestone work: TAIL-10 must account for their completed acceptance, together with every other required row, before claiming the milestone closed. This final accounting does not change the canonical ten-stage order or add early product gates.
