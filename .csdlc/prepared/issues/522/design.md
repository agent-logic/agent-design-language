# Issue #522: complete review-finding disposition

## Goal

Produce one ledger that accounts for all accepted review findings exactly once:
14 internal-review findings from #520 and five third-party-review findings from
#521, for a total denominator of 19.

## Boundary

#522 coordinates and proves dispositions. Each substantive fix remains a
separately reviewable change; the ledger cannot erase or silently defer work.

The source census is parsed from the digest-verified #520 and #521 review
reports read from their live-verified exact merge commits. A packet-authored
list cannot add, omit, rename, downgrade, or rewrite finding content. Fixed
dispositions require digest-verified machine-readable validation and review
results that both report pass at the exact remediated head. Deferral eligibility
is derived from source severity, status, and release-blocking flags; a
disposition cannot self-attest a blocker as non-blocking.

The external-review subset is exactly `TPR-001` through `TPR-005`. The
internal-review subset is exactly the 14 `D520-*` findings retained by #520.
Any additional or missing identifier in either source is a census failure.

#520 and #521 are each bound to their own immutable reviewed revision. They
are not required to share a candidate SHA: #520 necessarily precedes its
remediation, while #521 reviews the later frozen candidate. Both source-report
merges and every remediation merge must be ancestral to the final ledger
candidate.

A finding may require more than one reviewed remediation PR. The disposition
still owns the source finding exactly once, while its `remediations` list binds
every contributing merged fix. This is required for `D520-RET-001`, whose
accepted remediation spans #818 through #821.

Every fixed disposition resolves its remediation issue and merged PR live,
binds the exact PR head and merge commit, proves the merge is ancestral to the
ledger, and reads validation/remediation artifacts with `git show` from the
immutable PR head. A canonical passing review has zero findings and zero
blockers; outcome strings cannot override contradictory content.
