# R771-ROUTES-1 local reproduction

Reviewed source: `7c5b7873418249726d7c7d14f6e7f787358a80bf`.

One proposed regression was appended to a scratch copy of the exact native crate. Tracked source was not edited. No live GitHub call, credentials, or remote mutation was used. The existing fixture creates a synthetic Git repository and native authority receipts; `SequencedProcessAdapter` records argv and returns synthetic responses.

## Command and observed result

`cargo test --offline --locked --manifest-path <scratch>/csdlc-v3/Cargo.toml --lib review771_comment_after_first_page_must_not_be_reposted -- --nocapture`

Exit 101. Exactly one regression ran: 0 passed, 1 failed, 50 filtered out.

The synthetic remote collection has 101 comments, with the requested operation marker already on comment 101. The production adapter's first-page request is modeled by returning comments 1 through 100. With a persisted intent and `RetryAfterAuthenticatedAbsence`, production `execute_github_mutation` sends one additional operational POST, then reports `github_mutation_reconciliation_pending`. The regression expects zero mutation dispatches because a page without the marker does not establish absence from the whole collection.

This proves the current retry control flow with synthetic transport. It does not claim to have reproduced against live GitHub. The production first-page URL is independently visible at `csdlc-v3/src/adapters/mod.rs:499-500`; no pagination traversal exists there. The caller consumes only the response body in `csdlc-v3/src/commands/remote/mod.rs:1562-1614` and permits retry on `github_mutation_not_reconciled` at lines 980-991.

## V3-F AC2 parity relevance

The actual #505 STP states `AC-2: v2-v3 parity is measured` at `.csdlc/issues/505/cards/stp.md:27`.

Retained v2 `csdlc-v2/src/github.rs:1407-1443` scans comment pages with explicit `per_page=100` and incremented `page`, stopping only when fewer than 100 results are returned. The retained issue-comment route at lines 296-334 scans before posting, rejects multiple matching markers, reuses an existing comment ID, and scans after posting to require one exact ID. V3's single-page absence classification therefore fails to retain this predecessor reconciliation behavior. This is current source parity evidence, not a request to modify immutable historical cutover receipts.

## Smallest safe repair recommendation

Make issue-comment reconciliation complete before claiming absence: traverse bounded pages in the typed adapter, preserving status/truncation/credential rules; return an incomplete/unavailable finding on any failed page, truncation or exhausted bound. Aggregate all candidate comments and reject multiple exact operation matches rather than selecting the first. Only a complete scan with zero matches may authorize explicit absence recovery; a complete scan with one match should reuse it without POST. A minimal fail-closed interim guard could reject absence from any full page, but that would preserve safety while leaving operational parity incomplete.

Required focused coverage: marker only on page 2 yields no POST; complete multi-page absence permits one explicitly requested retry; later-page failure/truncation denies retry; duplicate markers across pages deny reconciliation. Keep this proposed regression and output as diagnosis only until separately authorized product repair.

Files: `.csdlc/evidence/771/pagination-defect.rs`, `.csdlc/evidence/771/pagination-defect.log`.
