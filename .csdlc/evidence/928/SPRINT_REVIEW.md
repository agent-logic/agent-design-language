# Sprint 2 combined review

Review revision: `866a6b07937387443906a5f9e4cf1949699fba39`

## Findings

### P3 — CodeFriend CI and evidence commands are absent from public help

`adl/src/cli/codefriend_cmd.rs` advertises local, packet, and GitHub operations,
while the CI and evidence handlers reject `--help`. This is a discoverability
defect. It does not invalidate the merged acquisition or evidence behavior and
should be routed as a focused follow-up.

## Delivered scope

All nine original children appear exactly once, are closed by merged PRs to
`main`, have passing applicable checks at the accepted heads, and have merge
commits ancestral to the review revision. The immutable roster is #848, #854,
#855, #876, #877, #878, #879, #880, and #881.

Corrective #967 / PR #968 is recorded separately. It closed the deterministic
hosted A2A formatting gap after #855, passed CI run `34711079509`, and merged as
the review revision. Its retained hosted packet reports passing OpenAI,
Anthropic, and Vertex AI rows with 16 paid provider requests and five local
Ollama requests inside the approved bounds.

## Integration assessment

The sprint delivers the provider definition and lifecycle boundary, governed
UTS consumption, local/GitHub/CI repository acquisition, and durable evidence
admission. Shared packet validation, omission, redaction, Git provenance,
last-known-good, cancellation, identity, budget, restart, tombstone, bootstrap,
and tamper behavior have issue-level proof and accepted-head CI evidence.

No unresolved P1 or P2 product finding remains after #967. #848 remains the
reviewed decomposition decision and plan; it does not prove physical repository
extraction.

## Validation limits

There is no retained single end-to-end test that feeds GitHub or CI acquisition
output through durable evidence-store admission. The source contracts compose
and the individual boundaries have strong tests, but the combined path remains
an explicit limitation for future integration work.

The #967 hosted packet proves bounded Runtime outcomes and request counts. It
does not prove vendor-billed token totals or the invoice amount.

## Lifecycle and closeout truth

All nine child indices retain `phase: bound`, and their SORs retain pre-terminal
truth. Native finish and cleanup remain asynchronous accounting debt under the
umbrella contract. This review does not claim those terminal receipts exist.

## Lane coverage

| Lane | Result | Evidence |
|---|---|---|
| Gap analysis | Pass with residuals | Complete nine-child ledger, separate #967 ledger, explicit non-claims |
| Code | Pass with P3 residual | Integrated source inspection and issue-level exact-head reviews |
| Docs | Pass with follow-up | Sprint roster preserved; corrective record added separately |
| Tests | Pass with limitation | Accepted-head CI plus hosted proof; combined acquisition-to-store path absent |
| Evidence and closeout | Pass with asynchronous debt | Merge ancestry and live states verified; child terminal receipts absent |
| Synthesis | Pass with residuals | No open P1/P2 product finding; limits retained |
| Review quality | Pass | Findings-first, exact revisions, proof boundaries, and non-claims recorded |

## Disposition

The combined Sprint 2 result passes review with the P3 help defect, combined-path
test limitation, and asynchronous lifecycle closeout debt retained. This packet
does not authorize merge, release, or public publication.
