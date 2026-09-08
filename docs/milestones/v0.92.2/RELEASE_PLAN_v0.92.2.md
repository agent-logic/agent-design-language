# v0.92.2 Release Plan

Status: planned; no date commitment.

## Candidate Formation

CF-INTEGRATE forms a release candidate only after every Beta 1 exit-bar track has merged reviewed authority and the ADL plus external OSS proof packets exist. A candidate is not a release.

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

Product integration may finish before independent supporting work. TAIL-01 converges every admitted support issue, including #717/#718/#720, before milestone quality approval.

## First-sprint convergence

The independently launched SIM sprint runs first alongside Runtime. TAIL-01 consumes completed SIM-UMBRELLA (including SIM-07 qualification and its authorized activation/pilot boundary). The sprint does not gate CF-INTEGRATE or urgent #718. If activation is not authorized, record the unresolved program gate; do not silently call the sprint or milestone complete.
