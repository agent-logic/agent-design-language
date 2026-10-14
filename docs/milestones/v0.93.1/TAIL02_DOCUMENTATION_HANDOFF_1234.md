# v0.93.1 TAIL-02 documentation and external-review handoff

Status: **preparation, pending predecessor and final-candidate acceptance**. This packet records what a reviewer can inspect now and what must be reconciled before #1234 can claim a complete handoff. It does not authorize contacting a reviewer, sharing private evidence, deploying CodeFriend, admitting testers, or releasing v0.93.1.

## Evidence baseline and candidate

The current-state quality matrix is `docs/milestones/v0.93.1/quality-gate-1232/EVIDENCE_MATRIX.json` at ADL commit `68c38b733afd99468cd888883568e939b9982401`, SHA-256 `f8c4b7ae66b7b4d1df604f7eabc5d9cb5813a33e8de957127fab0ae15acec737`. Its independent review passes the **evidence handoff**, not final release acceptance. #1232 remains open. The matrix observes seven repository *roles* in the historical RD-11 candidate; it does not say seven repositories changed or require seven new reviews. The final #1223 lockset and clean-clone/build correspondence are pending.

| Surface | Exact retained identity | What it establishes |
| --- | --- | --- |
| Qualified deployable CodeFriend source | `aa2cca757ba4e6c37cef6c1a6e013d3896cd1de1`, tree `89ca46e84d9f1060c80e47c97704eb9ab369af2a` | CodeFriend #47 / ADL #1150 accepted this candidate, not later heads. |
| Installed CodeFriend artifacts | macOS arm64 SHA-256 `17c64d9c1c5244aa0f1811268dcab6ce2bacfa2e03339574888e49eff384f328`; Linux x86_64 SHA-256 `55520a6994b084affcc1aeb9dd88ae6aac245a88aeef7263f2402ae47cf1cb39` | Exact artifact identity for the accepted qualification; a current installation still needs authenticated readback. |
| Product CI package | run `38030979163`, artifact `11661624501`, archive SHA-256 `2bf22aa1d0f354c5a719d26a07c92f0fbd39a890733f4d7336bed80b943c2c0c` | Archived candidate provenance, not a live deployment. |
| Qualified website bridge | `62b34f30a7042c10e35cbd5924841305bf68d106` | Exact site source accepted with that CodeFriend candidate; the later observed website `main` is not automatically accepted. |

The source references and their full byte hashes are indexed as S1–S8 in the #1232 matrix. S2 is the ADL #1150 terminal qualification manifest; S3–S7 are CodeFriend requirements, installed catalog, qualification reconciliation, deployment readiness and historical launch lockset at CodeFriend commit `188dd2b701be0a4acce1ec510559fdff119fd4c7`. S1 is the historical RD-11 candidate, not a final #1223 result. Do not substitute a newer `main` head for any accepted identity.

## Documentation and claim review

| Reader need | Current safe statement and source | Reconciliation before final handoff |
| --- | --- | --- |
| Entry points | CLI, hosted website and paired local-agent website are named qualification surfaces in [CodeFriend launch scope](features/CODEFRIEND_LAUNCH_v0.93.1.md); #1150 qualifies the exact product/site candidate above. | Identify the exact public user and operator pages shipped with the final lockset. A source route or disabled preview is not a live endpoint. |
| Access and privacy | [External tester onboarding](https://github.com/agent-logic/codefriend/blob/188dd2b701be0a4acce1ec510559fdff119fd4c7/docs/codefriend/EXTERNAL_TESTER_ONBOARDING.md) requires approved identity, repository, scope, retention, installed candidate, provider permission and support/stop owner before invitation. #1150 retains an operator-witnessed Q03 second-user denial corroborated by bound subject enforcement; it has no retained raw HTTP capture. | Check the final site copy and instructions against the actual invitation, consent, data-class, retention and deletion policy. No public signup or tester admission is implied. |
| Support and feedback | The [operations guide](https://github.com/agent-logic/codefriend/blob/188dd2b701be0a4acce1ec510559fdff119fd4c7/docs/codefriend/OPERATIONS_RECOVERY.md) records Daniel as support owner through GitHub issues for the accepted **private-run policy**. | Confirm the final reviewer-facing contact and stop path with the launch owner; do not copy an internal/private-run contact into public instructions without its audience decision. |
| Recovery and limits | ADL #1149 establishes no blind replay after uncertain provider effect. The [operations guide](https://github.com/agent-logic/codefriend/blob/188dd2b701be0a4acce1ec510559fdff119fd4c7/docs/codefriend/OPERATIONS_RECOVERY.md) covers cancellation, restart, retention and rollback boundaries. | Check the final installed instructions against the accepted candidate and explicitly state that local settlement does not prove provider effect or cost. Preserve the original #915 unknowns and Linux Vector first-group failure. |
| Documents and exports | The retained installed Linux catalog executes **44/44** kinds across nine scenarios (two denial, seven positive). #1150 accepts **24/24** obligations and **12/12** semantic/release cells, with 23 passing and one failed original execution group; the compatible repeat satisfies Q06. | CodeFriend [#101](https://github.com/agent-logic/codefriend/issues/101) remains an open readability defect. Neither catalog completion nor export existence proves readable final output. Check representative final rendering after the owner fixes it. |
| Deployment and launch | [Deployment readiness](https://github.com/agent-logic/codefriend/blob/188dd2b701be0a4acce1ec510559fdff119fd4c7/docs/codefriend/DEPLOYMENT_READINESS_v0.93.1.md) accepts disabled, undeployed source readiness. Its waiting-list proof has 25 scenarios and app-deployment proof has 10. | Live launch remains CodeFriend [#92](https://github.com/agent-logic/codefriend/issues/92), outside this milestone's undeployed-readiness scope. Do not advertise live availability or infer deployment from readiness. |

The CodeFriend onboarding and operations pages retain earlier Sprint 1 identity blocks and historical qualification language. Their exact-current status must be read from the later #47/#1150 qualification, not inferred from those older blocks. The multi-repository README refresh belongs to ADL [#1269](https://github.com/agent-logic/agent-design-language/issues/1269); this packet does not rewrite its owned pages. The final TAIL-02 review must compare the resulting user/operator pages with the exact candidate and record any conflict as an actionable finding.

## Reviewer walkthrough and disclosure boundary

1. Authenticate the #1232 matrix bytes and independent review at the stated ADL commit; read its `decision`, `acceptance_authority`, all 15 gate rows and two retained adverse observations. Its 44/44 catalog count is the full catalog, not a smaller subset. If #1232 has not been accepted for the final candidate, record the missing predecessor rather than calling this packet complete.
2. Compare S2–S7 identities to the exact CodeFriend/website candidate above. Sample one passed row, the original failed Linux Vector group, Q03's witness limitation and the open #101 defect. Do not add overlapping test denominators or rerun product CI solely to read this packet.
3. Check final user/operator documentation for CLI and website entry points, approved access, scope/retention, support/feedback, stop and recovery paths, installed artifact identity, readable outputs and known limits. Record the page revision and audience for each claim. Any page still pointing at an old Sprint 1 candidate needs an explicit historical label or a bounded owner correction.
4. Obtain #1223's final role/edge lockset and TAIL-03/#1236's publication candidate when they exist. Classify changed versus unchanged roles from that ledger. Verify changed interfaces and retained proof for unchanged roles; do not presume seven changed repositories or seven new builds.
5. Put public-safe identity, status, counts and redacted finding summaries in the tracked report. Keep raw tester/repository content, authorization receipts, credentials, provider records, private installed traces and private evidence locations in the access-controlled packet. A public link to a private artifact is not an accessible evidence index. Verify each authorized link and hash before distributing a packet.

The reviewer should return a findings-first report with exact file/revision, consequence, owner and disposition, plus an explicit `not_proven` or `changes_required` result where evidence is missing. TAIL-05 owns independent external review; TAIL-06 owns accepted-finding remediation or explicit deferral. This TAIL-02 packet neither sends material to reviewers nor decides those results.

## Open handoff questions

| Owner | Required answer |
| --- | --- |
| ADL #1232 / #1223 | What is the accepted final quality decision and seven-role lockset, including actual changed-role and interface set? The reviewed current-state matrix is insufficient. |
| ADL #1269 and CodeFriend #101 | Which exact final user/operator pages and representative readable exports can the reviewer use? Resolve stale identity blocks and open readability findings. |
| TAIL-03/#1236 | What are the frozen source/artifact/document versions, public/private destinations, access permissions and approved disclosure manifest? |
| Launch/support owner | What audience, feedback channel, stop path and retention wording are approved for the final reviewer packet? |

No product tests, builds, CI dispatch, provider calls, deployments, or shared binary replacements were performed for this documentation preparation. The final handoff requires a current independent review and native #1234 lifecycle truth; this draft does not satisfy #1234's acceptance by itself.
