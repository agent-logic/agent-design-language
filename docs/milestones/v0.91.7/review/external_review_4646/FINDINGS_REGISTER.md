# v0.91.7 WP-19 External Finding Register

Status: superseded_pre_dispatch_review

Issue: #4646

Target revision: `bd1c12537b28122e187ce1ba9a19349731fd2825`

Packet digest: `8ae1ddd98b86ded8ef52018d0df4eb045455f586292b90954fe0056e8d18e37c`

## Outcome

The two findings below describe the historical 66-file packet and its dispatch
mechanics. Later merged evidence changed the required corpus, so this register
does not establish current external-review results or the absence of current
corpus defects. Replacement external review has not run.

## Findings

| ID | Severity | Evidence | Impact and invariant | Remediation and disposition | Residual risk |
| --- | --- | --- | --- | --- | --- |
| WP19-EXT-01 | P1 | `DISPATCH_RECEIPT.md:3-19` at the reviewed target lacked a remote identity and digest. | An external reviewer could not resolve the immutable corpus; exact-revision review identity was violated. | Fixed by pushing the target and recording PR #5579, commit `bd1c12537`, and digest `8ae1ddd9...e37c`. | None for corpus identity; later source changes remain outside the target. |
| WP19-EXT-02 | P2 | `V0917_SPRINT_REVIEW_REGISTER.md:102` at the historical target said ready while the receipt was blank. | Operational readiness was overstated before the send gate was satisfied. | Historical packet mechanics were corrected, but later corpus drift invalidated the completion claim. | Replacement review may return current findings for WP-20. |

## Non-Claims

- WP-19 does not auto-create one issue per finding.
- WP-20 #4647 owns deduplication, acceptance, routing, and remediation.
- This historical register does not claim that the replacement corpus has no findings.
- No release or v0.92 activation approval is recorded here.
