# Issue #522: complete review-finding disposition

## Goal

Produce one ledger that accounts for all accepted review findings exactly once:
14 internal-review findings from #520 and five third-party-review findings from
#521, plus every finding returned by the immutable-candidate #833 review. The
original source denominator remains 19; #833 returned six additional findings,
so the terminal disposition denominator is 25.

## Boundary

#522 coordinates and proves dispositions. Each substantive fix remains a
separately reviewable change; the ledger cannot erase or silently defer work.

The source census is parsed from the digest-verified #520 report and retained
failed #521 third-party report, both read from their live-verified exact merge
commits. The #521 source remains explicitly non-proving because it lacked a
candidate SHA; that missing binding is `TPR-001`, not a reason to erase the
other four findings. A packet-authored list cannot add, omit, rename,
downgrade, or rewrite finding content. Fixed
dispositions require digest-verified machine-readable validation and review
results that both report pass at the exact remediated head. Deferral eligibility
is derived from source severity, status, and release-blocking flags; a
disposition cannot self-attest a blocker as non-blocking.

The original external-review subset is exactly `TPR-001` through `TPR-005`.
The internal-review subset is exactly the 14 `D520-*` findings retained by
#520. The six #833 findings are normalized separately and remain digest-bound
to the merged report and executable addendum; this preserves the 19-row source
denominator without omitting findings returned by the required re-review.

#520 is bound to its historical immutable reviewed revision. The retained
failed #521 source is deliberately recorded with a missing revision and
`non_proving: true`; the successful exact-candidate re-review belongs to the
`TPR-001` remediation evidence. Both source-report merges and every remediation
merge must be ancestral to the final ledger candidate.

A finding may require more than one reviewed remediation PR. The disposition
still owns the source finding exactly once, while its `remediations` list binds
every contributing merged fix. This is required for `D520-RET-001`, whose
accepted remediation spans #818 through #821.

The same rule applies when final proof exposes an incomplete earlier fix.
`D520-V3F-001` requires both #817 and #843: #817 reconciles the finalized
predecessor evidence, while #843 repairs the retained legacy ready-intent path
that the #835 current-candidate proof found after #840 had merged. This does not
add a twentieth finding; it records two reviewed remediations for one accepted
internal finding.

Every fixed disposition resolves its remediation issue and merged PR live,
binds the exact PR head and merge commit, proves the merge is ancestral to the
ledger, and reads validation/remediation artifacts with `git show` from the
immutable PR head. A canonical passing review has zero findings and zero
blockers; outcome strings cannot override contradictory content.
