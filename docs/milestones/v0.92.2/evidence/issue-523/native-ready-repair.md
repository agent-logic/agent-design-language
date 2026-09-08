# Native ready-for-review repair under #523

Native publication succeeded for reviewed head `6ef8cb888e47153aa5887e11886434fd8bc62310`: exact review and authenticated PR readback both produced ready results. The subsequent native draft-to-ready command used an unsupported REST `ready_for_review` endpoint. Its authenticated readback still observed a draft PR, so the existing durable intent correctly refused to claim success or repeat the write.

The bounded repair uses GitHub's documented [markPullRequestReadyForReview mutation](https://docs.github.com/en/graphql/reference/pulls#markpullrequestreadyforreview). Native code must first observe the exact repository/PR and expected head to obtain its node ID, issue a fixed GraphQL mutation with structured variables, preserve scoped credentials and private payload handling, then retain the existing authenticated final readback. It must not permit arbitrary GraphQL or caller-provided node IDs.

The failed original intent remains immutable history. A reviewed repair commit creates a new exact-head ready operation, rather than deleting or overwriting the uncertain old intent. No raw GitHub write is used.

Focused tests are deterministic local CPU transport/contract checks without network or paid resources. They gate this repair and must demonstrate fixed request construction, identity/head guards and error handling. Live acceptance requires native ready mutation and authenticated observation of PR #743 as non-draft at the final reviewed commit.

## Local proof and review

All 190 native tests pass after this repair. Formatting and Clippy with warnings denied pass. Independent reviewer `review_523` reports no actionable findings and independently executed seven ready-related tests, including all five new regressions. Existing shared private credential configuration and JSON payload handling are preserved. The final committed-head review and native non-draft readback are retained separately as publication evidence; this pre-publication document does not assert a remote ready state before observation.
