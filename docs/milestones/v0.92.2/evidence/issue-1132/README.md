# Issue 1132: cycle review Journey integration

Current observation for #917: the website patch is now merged through codefriend.ai PR #18 at `a45e339c13b24716edbd3fadf29dddff36ffe02e`; the [post-merge checkpoint](../issue-916/POSTMERGE_VERIFICATION.json) records141 passing website tests. The original patch/publication instructions and139-test result below are source-time history, not outstanding integration work. No deployment or final #915 qualification is inferred.

The native companion delivered by #1133 / PR #1138 selects the successful original review inside a completed
update cycle. Hosted ownership verifies cycle execution, candidate, request,
provider, source revision, original admission and retained review artifacts.
The installed agent retains the gateway admission without extending retention,
preserves its distinct cycle operation ID, and binds downstream jobs and native
verification to the enclosing original agent report and receipt. Original local
admission deletion still denies reuse; expiry scrubs both admission stores.
Publication uses that same selected review and existing exact approval guards.
No Journey operation repeats model review.

The earlier native implementation on this branch was superseded by #1133.
PR #1137 retains the website delivery only, avoiding two competing native
bridges. Native source and associated tests are owned by PR #1138.

The website implementation is in `agent-logic/codefriend.ai`, branch
`codex/1132-cycle-review-journey`, commit series `f695c18` through `0f7a5ba`. It is stacked on the
committed #1126 assessment integration (`20f6bf0`). Authenticated routes select
hosted or paired-agent owners, validate the enclosing cycle, and retain full
original report shape. Source-free run summaries expose validated eligibility
for the baseline selector. Unsupported, failed, reviewless and old unbound
cycles remain unavailable; they are not converted into legacy review identities.

## Validation and review

Native results below describe the prior component proof; current native fixes
and proof are retained by #1133. The website series remains independently
reviewed and tested at the exact commit below.

- Website declared `npm test`: 139 passed, zero failed, Node 24 or newer.
- Native agent Journey/delivery/verification tests: 14 passed. New cycle
  regression additionally verifies the actual returned snapshot with the
  native verifier, original-evidence deletion and expiry cleanup.
- Built-server integration test: passed. Hosted assessment cycle prepares,
  replays and exposes its graph with the provider call count unchanged; foreign
  ownership is denied. The provider transport is a deterministic loopback fixture.
- Native library Clippy with warnings denied passed; formatting passed.
- Independent subagent source review found one baseline-selector regression;
  repaired with validated summary metadata and HTTP/DOM regression tests.
  No remaining actionable findings. Reviewer did not run builds/tests.

These are component proofs, not deployment, paid-provider execution, or #915's
original twelve journeys/Q01-Q24 qualification. Native gateway and installed
agent updates must accompany website rollout; no shared binaries or services
were replaced. At the operator's request, #1132 is delivered in the ADL PR. The mistakenly
opened website draft PR #16 was closed. `codefriend-website.patch` retains the
exact independently reviewed website commits above for application to the
separate website checkout after its #1126 base is present. The website branch
also retains those commits; the ADL PR does not move or vendor the website runtime.
Applying this patch to the website's release branch and deploying both native
and website components remains a separate integration action, not a completed
rollout claim. Merge and deployment are not authorized.

## Original review evidence proxy integration

The website patch also forwards #1133's authenticated GET
`/v1/operations/:id/review-evidence` through the paired model proxy. The original
cycle operation reservation and agent identity remain required. Evidence reads
share the result cancellation/status guard, rechecked after the final asynchronous
credential validation before returning evidence; cancelled, interrupted, expired,
revoked, wrong-agent, wrong-token and unreserved requests do not reach the
gateway. The focused HTTP/agent suite passes eight tests; the full website
suite passes 139. The native #1133 backend must accompany this website change
before local Journey qualification. No native #1133 contract is reimplemented.
