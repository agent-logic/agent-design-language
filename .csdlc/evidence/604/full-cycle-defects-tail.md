# Issue #604 full-cycle canary defects tail

## DEFECT-010: Review scope can make publication immediately stale

- Status: open workflow defect.
- Evidence: a pre-publication review whose scope included `.csdlc/issues/604/**`
  passed, but `csdlc-publish status` then rejected publication with
  `publication review guard failed: review_stale` because recording the review
  itself mutated the scoped lifecycle files.
- Impact: a correct-looking review can become unusable for publication unless
  the operator knows to exclude lifecycle metadata from the substantive review
  scope or provide explicit metadata-only proof.
- Required fix: the one-command lifecycle should derive a publication-valid
  review scope automatically and reject self-staling review scopes before they
  are recorded.
