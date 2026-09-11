# v0.92.2 Release Plan

Status: planned; no date commitment.

## Candidate Formation

CF-INTEGRATE forms a release candidate only after every Beta 1 exit-bar track has merged reviewed authority and the ADL plus external OSS proof packets exist. A candidate is not a release.

## Release denominator

The release denominator is exactly the 43 work-package rows in the canonical [planned issue catalog](PLANNED_ISSUE_CATALOG_v0.92.2.md), with structure and dependencies defined by the [WBS](WBS_v0.92.2.md), [issue wave](WP_ISSUE_WAVE_v0.92.2.yaml), and [execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml). It comprises the original 30 planning rows, newly admitted PLAT-PAIR and OPS-GCP, ten C-SDLC simplification rows (SIM-UMBRELLA plus SIM-01 through SIM-09), and one reused existing issue, #720. Issues #717 and #718 are merged v0.92.1 predecessor inputs and are not additional denominator rows. TAIL-01 must reconcile all 43 rows to current canonical issue and merge truth; an unmapped, duplicated, missing, or unreviewed row is a no-go condition.

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

Product integration may finish before independent supporting work. TAIL-01 converges every admitted v0.92.2 support issue, including #720, and verifies that the merged v0.92.1 #717/#718 predecessor contracts are consumed before milestone quality approval.

## TBD disposition checkpoint

TAIL-07 and TAIL-08 must refresh the tracked [TBD scheduling reconciliation](TBD_SCHEDULING_RECONCILIATION_v0.92.2.md) against the then-current local inventory. The refresh is an accounting gate, not authority to add work. Every active source discovered after the retained #620 audit must be classified as admitted work, merged/completed provenance, an existing issue or milestone, or explicit backlog/deferred work before TAIL-09 review.

The known unadmitted product candidate that must remain visible is `.adl/docs/TBD/codefriend_ai/POLIS_MUSIC_STUDIO_CONCEPT.md`. This is not an exhaustive inventory claim; the required TAIL-07/08 refresh supplies that denominator. The operator admitted `.adl/docs/TBD/provider_model/NVIDIA_PAIR_EXPERIMENT_PLAN_v0.92.2.md` as PLAT-PAIR and `.adl/docs/TBD/GCP_ACCOUNT_MOVE_IN_PLAN.md` as OPS-GCP. The older issue #122/#268 cloud plans remain v0.92.1/cloud provenance and are not duplicate successor rows. The Runtime v4 plugin-system design remains scheduled for v0.93. Issue-local C-SDLC audit snapshots and review outputs are evidence, not product backlog rows.

## First-sprint convergence

The independently launched SIM sprint runs first. TAIL-01 consumes completed SIM-UMBRELLA after SIM-07 qualification, SIM-08 operations preparation, and the separately authorized SIM-09 pilot. The sprint does not gate CF-INTEGRATE; closed merged v0.92.1 #717/#718 are predecessor inputs, not active bugfix lanes. If activation is not authorized, record the unresolved program gate; do not silently call the sprint or milestone complete.
