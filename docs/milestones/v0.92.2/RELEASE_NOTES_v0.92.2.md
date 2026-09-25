# v0.92.2 release notes — external-review draft

**Not a release or Beta1 launch approval.** This #918 packet consumes the accepted #916 assessment ([PR #1151](https://github.com/agent-logic/agent-design-language/pull/1151), `374ecc2a1237094e031ecb5f6d82853ea72ab819`) and #917 documentation handoff ([PR #1152](https://github.com/agent-logic/agent-design-language/pull/1152), `c72c8cc1bf255ae2e26bb85b631ce0550fd886fc`). Their integration does not change the `not_proven` product qualification decision. #919's internal review is retained at PR #1153 merge `52756b5bc7b02e3ebf6886170ee59b55f8dc10d1`; external review and release approval remain separate and incomplete.

## Current qualification disposition

Issue #915 is closed as `NOT_PLANNED`: its retained independent qualification is
incomplete and explicitly deferred, not a qualification pass. Under the approved
#922 successor split, producer repairs #1148 and #1149 precede independent
qualification #1150 in v0.93.1, before Beta 1 launch. Runtime v4 belongs to
v0.93.2. The original demonstration requirements and historical evidence below
remain intact; release acceptance is not established by issue closure or this
documentation correction.

Current routing is grounded in #916 commit
`36ec0814683d0c8cc136bc9af27a0163ed506460` and #922 commit
`fc6a80c362d6c1e63766bef263cd2dccacd2dc0b`. Those owner fixes are credited;
this overlay does not rewrite their historical review packets.

Status: evidence-reconciled review draft; not approved release notes.

## Delivered scope and evidence limits

The [69-task ledger](evidence/issue-916/TASK_LEDGER.json) preserves every planned identity, its implementation or planning category, closing PR observations and acceptance status. Closed issues are not automatically accepted capabilities. The seven planning results remain design deliverables; they are not implemented behavior. The [quality decision](evidence/issue-916/QUALITY_DECISION.json) and [producer audit](evidence/issue-916/PRODUCER_SCOPE_AUDIT.md) qualify the evidence below.

- Native and website CodeFriend integration landed through ADL PRs #1137/#1138 and CodeFriend.ai PR #18. The inherited [post-merge checks](evidence/issue-916/POSTMERGE_VERIFICATION.json) record 154 affected native tests and 141 website tests at their declared historical source identities. This is component/integration evidence, not independent installed-product qualification; those tests were not rerun for #918.
- The earlier 396-test native census retains its original source identity and is not added to the later 154-test denominator. MLX smoke does not establish review-speed superiority; speculative decoding remains repair/inconclusive.
- Five historical Runtime criterion results were admitted from retained evidence. The [replay report](evidence/issue-916/runtime-admission-recovered.json) does not qualify a new integrated release candidate. PAIR endpoint discovery recovered, but discovery is not inference proof or an overturned historical REPAIR verdict.
- The [#917 document handoff](evidence/issue-917/HANDOFF.md) binds 129 documents, 69 task identities and 24 prerequisites at its own source revision. #918 preserves that checkpoint and records its changed release-note bytes separately. Refer to the [publication manifest](evidence/issue-918/PUBLICATION_MANIFEST.json) for exact inputs and artifact hashes.
- Article #912 and manuscript #913 reached operator-accepted draft handoffs. The manuscript remains private revision 4 with further editing required. These are editorial handoffs, not published works.

## Explicit deferrals and launch gate

[The approved Sprint 10 disposition](evidence/issue-916/SPRINT10_DEFERRAL.json) records original #915 qualification as **incomplete: zero of twelve journey cells accepted**, with 24 obligation rows. Follow-on #1148 owns citation-grounded correctness; #1149 owns interrupted-request recovery; #1150 owns independent qualification after both. Their accepted results are required in v0.93.1 before CodeFriend Beta1 launch. Deferral and administrative closure do not constitute qualification PASS. Preserve unknown second-run outcomes and reserved spend; no replay to bypass uncertainty.

Podcast #671 was restored to v0.92.2 and closed with launch issue #1169 through PR #1174; Apple and Spotify publication is complete. SIM-09 pilot #875 remains deferred to v0.93. Observatory #910 retains its separate deployment obligation. Accepted ARCH-ADR decisions are architecture evidence, not product acceptance. See the [review handoff](evidence/issue-917/HANDOFF.md) for the source-specific dispositions.

## Versions, formats and approvals

The milestone label is v0.92.2. The principal ADL/Runtime package declarations remain 0.92.1; independently versioned components retain their own versions. This PR changes no product version or lockfile and labels no existing binary as v0.92.2. The [version inventory](evidence/issue-918/VERSION_INVENTORY.json) preserves the inherited audit and identifies the newly added validation-only helper separately. Final installed binary identities and a coordinated release freeze remain unresolved.

The review packet includes Markdown and JSON evidence. Actual approved CodeFriend Markdown/HTML/PDF exports have not been selected or copied into it; three-format visual/content parity is **not proven**. The [destination inventory](evidence/issue-918/DESTINATIONS.json) retains private custody and pending artifact-specific approvals. No public upload, registry publication, release tag or deployment is authorized by this packet.

## Excluded scope

No Jira, Linear, Slack or broad Workspace integration; autonomous source mutation; customer-scale multi-tenant hosting; ATE; OCI model packaging; security tournaments; or Runtime v4 is claimed. MLX/Metal and Observatory sidecar evidence remain bounded to their admitted scope. See [the review guide](evidence/issue-918/REVIEW_GUIDE.md) for acceptance gaps and the external-review procedure.

## Integrated review repairs

The [repair reconciliation](evidence/issue-918/INTEGRATION_RECONCILIATION.json) records merged Groups A–D and the separate #1172 follow-on. The frozen internal review retains its accepted 27-finding denominator. Repair integration is not external-review completion or release approval.
