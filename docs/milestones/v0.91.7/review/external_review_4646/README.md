# v0.91.7 WP-19 External Review Packet

Status: replacement_corpus_prepared_not_sent

Issue: #4646

This directory retains the superseded prior-review record and prepares the
replacement external-review corpus. No replacement review has run.
The candidate allowlist currently contains 33 entries expanding to 70 tracked
files; that count is preparation evidence, not an immutable review digest.

## Before Sending

1. Confirm the reviewer receives only the new frozen corpus and does not
   consume v0.91.8 follow-ons #5572 / PR #5574 or #5575. #5573 is closed; its
   existing audit register is retained evidence and is not rerun here.
2. Refresh live predecessor and remediation issue truth.
3. Replace the superseded target with the new exact target revision and
   tracked-path digest in `DISPATCH_RECEIPT.md` immediately before dispatch.
4. Run the handoff's focused preflight.
5. Give the reviewer read-only authority and the explicit non-claims.

## Return Artifact

Record external findings in `FINDINGS_REGISTER.md`. Preserve the original
review output separately only when it is publication-safe and tracked.

The bounded allowlist and raw WP-18 exclusions are recorded in
`PUBLICATION_SAFE_MANIFEST.md`.

WP-20 #4647 owns synthesis and remediation after review. This packet does not
approve release or v0.92 activation.
