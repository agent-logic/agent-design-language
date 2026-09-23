# #1162 finding-to-fix-to-test map

This register covers the seven Group B findings accepted from the immutable #919 review. The original review artifacts remain unchanged. A disposition is complete only after the named implementation, proving regression, independent exact-head review, and applicable CI all agree.

| Finding | Severity | Component | Fix | Proving regression | Current disposition |
| --- | --- | --- | --- | --- | --- |
| INTEGRATION-001 | P1 | CodeFriend website | Accept native review-lane v4, including lane assessment gaps and run assessment coverage, while retaining v2/v3 compatibility. | Authentic native-emitted complete, privacy-partial, and gap fixtures plus mixed/stale/tampered/future-version negatives. | Implementation in website component; authentic fixture import pending. |
| CODE-001 | P2 | ADL publication | Render exact citations as preformatted Markdown and HTML; PDF derives its semantic text from the preserved block. | Exact CRLF, line-break, tab, and metacharacter unit regressions plus Markdown/HTML/PDF renderer lanes. | Implemented; focused proof pending final candidate. |
| CODE-002 | P2 | ADL operator shell | Treat incomplete attempts as unsettled and require a matching typed settlement for failed/cancelled attempts. | Held provider call rejects retry, preserves active attempt 1, and routes cancellation to attempt 1. | Implemented; focused proof pending final candidate. |
| SEC-003 | P2 | ADL publication relay | Parse actual PDF bytes, reject active/external content, bind page count, and compare extracted semantics with independently reconstructed approved semantics. | Replace a valid PDF's content, recompute report/manifest/export/stage digests, and require verification rejection. | Implemented; clean-candidate public-path proof pending. |
| SEC-004 | P2 | CodeFriend website | Re-authorize browser sessions after awaited request bodies and after remote result/download acquisition. | Slow-body and delayed-result/download revocation negatives plus controlled-identity positive cases. | Implemented in website component; exact-head review pending. |
| DEP-001 | P2 | CodeFriend website | Pin privileged actions to verified upstream commit SHAs and scope OIDC permission to the deploy job. | Workflow-policy regression validates exact immutable SHAs and job-scoped permission. | Implemented in website component; exact-head review pending. |
| DEMOS-001 | P3 | ADL documentation | Describe the Cargo PDF test as source-built component and CLI proof. | Documentation/path review and exact wording assertion. | Implemented; independent review pending. |

No finding was invalidated during current-source reconciliation. No merge, deployment, hosted-provider execution, or modification of #918/#919 is authorized by this issue.
