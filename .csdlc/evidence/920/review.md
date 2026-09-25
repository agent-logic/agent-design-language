# v0.92.2 external-review handoff

Status: **handoff ready; external review has not started**.

This is the reviewer entry document for TAIL-05/#920. The full internal source
review is available, but it returned **changes required**. Its 27 findings were
routed through four aggregate issues. Groups A, B, C and D are merged, closed,
and integrated into the exact #918 candidate. The #919 report remains immutable
changes-required evidence, while #918 supplies the post-remediation integration
record. No external reviewer has been contacted, no disclosure has been
authorized, and the external review has not started.

The handoff candidate is `d4575d1b34df78b80e71233e5d72cca9bc4339c2`.
Its publication manifest is
`docs/milestones/v0.92.2/evidence/issue-918/PUBLICATION_MANIFEST.json`, SHA-256
`6872e631b0160b676953b55d3f55796768ee2493ce727774bc76a05d61fa7d97`.
PR #1154 merged that reviewed head at
`ea0df924447e77e1c34ebc54c8da92ac5749c40e`. This is acceptance for
external-review preparation only. Qualification, release acceptance,
publication authorization, disclosure authorization and external-review results
remain pending.

Later integration facts are retained without changing that boundary. Group A
merged as ADL PR #1168 at `c8f646dc8c90d139332315c507adbc57b0c222e2`.
The CodeFriend website and ADL components of Group B merged as CodeFriend PR
#20 at `8b0c5fd12423494c7fed27059321b217d7dff430` and ADL PR #1170 at
`81a2f13503ffa67e6e14d238bff6567b5320dfc1`. Group C merged as ADL PR #1163
at `02c0aa707f17ed013cbf872222615411a78b4166`. Group D merged as ADL PR #1164
at `78f57f97c2b9e5e90301e132434d98ba2fcbc2e6`. All four repair issues are
closed. These facts are bound by the integrated #918 candidate and its
reconciliation record. They establish a reviewable handoff packet; they do not
authorize external dispatch or establish release readiness.

## Frozen internal-review baseline

The internal review is immutable evidence about this exact baseline:

- ADL candidate: `5c4a6149771c637f3c805985b86231077965eab4`
- #918 publication-manifest SHA-256:
  `b4f031ae10b3c7a5216523680dfca5b1cd94cdc5d35d3206c852932d37f246c6`
- #919 report revision / PR #1153 head:
  `6fc19987dcb9aaa347e61e6185f4ae1718e5ec6c`
- report result: `changes_required`
- finding denominator: 27 total (4 P1, 20 P2, 3 P3)

The baseline is not the external-review target. The post-remediation candidate
is bound separately above and in `review-manifest.json`. Do not silently
reinterpret the frozen findings against changed bytes.

## Packet access and identity checks

Start with these exact blobs at the #919 report revision:

- `docs/milestones/v0.92.2/evidence/issue-919/full-review/final_report.md`
- `docs/milestones/v0.92.2/evidence/issue-919/full-review/run_manifest.json`
- `docs/milestones/v0.92.2/evidence/issue-919/full-review/findings.json`
- `docs/milestones/v0.92.2/evidence/issue-919/full-review/remediation-groups.json`
- `docs/milestones/v0.92.2/evidence/issue-919/full-review/artifact-manifest.json`
- `docs/milestones/v0.92.2/evidence/issue-919/full-review/acceptance-inventory.json`

For repository access, fetch PR #1153 or the exact report revision, then verify
that it resolves as a commit. Read packet files with
`git show 6fc19987dcb9aaa347e61e6185f4ae1718e5ec6c:<path>` so a local checkout
cannot substitute newer bytes. Verify the reviewed source separately with
`git cat-file -e 5c4a6149771c637f3c805985b86231077965eab4^{commit}`. The focused #920
validator checks the declared report blobs, hashes, finding denominator and
repair mapping directly from Git.

The packet is repository evidence. Any private evidence, credentials, local
machine records, provider payloads or cloud access remain outside scope unless
the operator separately approves a precise disclosure.

## Internal findings and repair disposition

The complete findings remain in the immutable #919 register. They are grouped
for repair, without merging or weakening individual finding identities:

| Group | Issue | Scope | Findings | Current disposition |
| --- | ---: | --- | ---: | --- |
| A | #1161 | C-SDLC recovery and operator contracts | 9 | PR #1168 merged and issue closed; merge is an ancestor of the bound candidate |
| B | #1162 | CodeFriend and website interoperability/integrity | 7 | website PR #20 and ADL PR #1170 merged; exact cross-repository identity retained and ADL merge is an ancestor of the bound candidate |
| C | #1159 | Runtime/provider health and regression proof | 5 | PR #1163 merged and issue closed; merge is an ancestor of the bound candidate |
| D | #1160 | Reproducible validation and release evidence | 6 | PR #1164 merged and issue closed; merge is an ancestor of the bound candidate |

All 27 findings map exactly once. Issue creation and work in progress do not
establish a fix. Before external dispatch, record each finding's repair commit,
focused proof, independent exact-head re-review and any accepted residual risk.

### Observed repair evidence

- Group A source map: `docs/csdlc-v3/evidence/issue-1161/REMEDIATION.md` at
  `fdc968548458d6eb448ce01a99fa4c4d9a02101d`, SHA-256
  `34f11ce273c03d7227982f76cf8d2163f0d9e40461c4cd2be31ac30cfabe6e21`.
  ADL PR #1168 merged at `c8f646dc8c90d139332315c507adbc57b0c222e2`
  on 2026-09-23T20:21:25Z; issue #1161 closed one second later.
- Group B source map: `.csdlc/evidence/1162/FINDING_DISPOSITIONS.md` at ADL
  `cdb48f4fea83218ddf7feb545da3b7be7fb7d345`, SHA-256
  `c563d5f8131f35028258da24850d7c3650422d06335d0c29f96af3cad63c613a`.
  That map predates both component merges and remains a source mapping, not a
  current integration-status record.
- Group B website component: CodeFriend PR #20, reviewed head
  `8f7d28e7f6254a95721bfa8b15cd95770382607b`, merged at
  `8b0c5fd12423494c7fed27059321b217d7dff430` on 2026-09-23T20:00:24Z.
- Group B ADL component: PR #1170, reviewed head
  `cdb48f4fea83218ddf7feb545da3b7be7fb7d345`, merged at
  `81a2f13503ffa67e6e14d238bff6567b5320dfc1` on 2026-09-23T20:28:19Z;
  issue #1162 closed one second later.
- Group C source map: `docs/validation/issue1159/FINDING_FIX_PROOF.json` at
  `de8574324b087366423d6d60911ab63e274ad280`, SHA-256
  `a921df9a2c20f128067cbf1a816d96c8d85b2c0b1c7a1c6c88b25f1d930758e5`.
  ADL PR #1163 merged at `02c0aa707f17ed013cbf872222615411a78b4166`
  on 2026-09-23T20:04:59Z; issue #1159 closed one second later.
- Group D source map: `tools/groupd_validation/README.md` at
  `8a409c7c327f6203e45b810c9566c9c70e6c0b50`, SHA-256
  `19f9d79c58f7f0f7511ff5f81a794dcb4080518d5ab12a8e49825fa898f40b7c`.
  ADL PR #1164 merged at `78f57f97c2b9e5e90301e132434d98ba2fcbc2e6`
  on 2026-09-23T23:19:10Z; issue #1160 closed one second later. Its mapping
  keeps subsequent finding CI-1160-001 separate from the six frozen #919
  findings.

Issue #1171 / PR #1172 repaired the integration tooling needed to retire a
never-dispatched merge attempt safely. Reviewed head
`52b8e12177a74f036c044124c55867c1893e0628` merged at
`ee90f97a3d9fe31e08c3ba53af6b4d706a560788`. This follow-on is relevant to
the integration sequence but is not a #919 finding and does not change the
frozen denominator of 27.

The packet now binds exact evidence from the integrated candidate and proves
the ADL repair and follow-on merge ancestry. External contact still requires a
separately authorized reviewer and disclosure scope.

## Reproduction guide

Use `final_report.md` as the findings-first index. For each finding:

1. confirm its source path and line against the frozen ADL or website revision;
2. follow its cited lane artifact under `full-review/lanes/`;
3. rerun only the retained offline reproduction when the lane records an
   executable command or fixture;
4. record when evidence is static call-path analysis rather than execution;
5. do not infer live Runtime, provider, cloud, deployment or publication proof
   from deterministic checks or green CI.

The packet's `acceptance-inventory.json`, coverage records and specialist
reports show reviewed scope and unresolved acceptance. The source-provided
reproduction logs are evidence, not permission to contact providers or mutate
cloud/repository state.

## Required activation gates

All of the following must be true before review begins:

1. Every #919 finding has a truthful repair or retained-risk disposition and
   an independent exact-head re-review.
2. #917 and #918 provide accepted handoff, candidate and artifact identities.
3. The post-remediation candidate revision and artifact-manifest SHA-256 agree
   across the handoff, internal re-review and this manifest.
4. The reviewer is independent of implementation and internal-review
   authorship, with retained evidence for that conclusion.
5. External contact and the exact disclosure scope have explicit authorization.
6. Every required source is accessible without exposing material outside the
   approved scope.

Authorization and reviewer independence must be retained as separate typed
evidence. A sent request is not a completed review. If any gate changes, stop
and refresh the packet.

## External review scope

Review the exact post-remediation candidate and artifact manifest recorded in
`review-manifest.json`. Assess whether the v0.92.2 Beta 1 claims are
supported by the provided source and retained evidence, with particular
attention to:

- candidate and manifest identity consistency;
- all 27 internal findings and their repair/re-review dispositions;
- quality-gate and qualification claims;
- CodeFriend Beta 1 product behavior and supported non-claims;
- security, privacy, provenance and credential boundaries;
- documentation, diagrams, tests, deployment claims and release truth;
- release blockers in the milestone quality gate and release plan.

Do not broaden the task into roadmap design. Do not infer successful Runtime,
provider, cloud, deployment or publication behavior from plans, deterministic
validators or green CI alone.

## Required source set

The packet includes exact scoped #917 and #918 preparation identities, the
post-remediation candidate and integrated repair records alongside the #919
sources above. The external reviewer must also receive:

- `docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md`;
- `docs/milestones/v0.92.2/QUALITY_GATE_v0.92.2.md`;
- `docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md`;
- the exact candidate diff and all evidence referenced by material claims.

## Requested response

Return one context-free report containing:

1. reviewer identity and independence basis;
2. exact reviewed candidate revision and artifact-manifest SHA-256;
3. methods, sources accessed, checks or reproductions actually performed, and
   any resource/provider use;
4. actionable P0-P3 findings first, each with stable evidence and impact;
5. disposition verification for every internal finding;
6. verified non-findings;
7. limitations, inaccessible evidence, omitted scope and unresolved uncertainty;
8. one verdict: `pass`, `fail` or `not_proven`.

An adverse or `not_proven` review is valid retained evidence. Route every
accepted external finding to #921 without editing the original assessment.

## Findings intake

Copy the received assessment into immutable issue-local evidence, then populate
`findings.json` without changing the reviewer's meaning. The reviewed revision
and manifest digest must equal the activated manifest. Preserve negative
results and limitations. The retained assessment must use
`adl.external_review_assessment.v1`, and the intake must reproduce its verdict,
findings, verified non-findings, validation performed, limitations and internal
finding dispositions exactly. A completed `fail` or `not_proven` result must
retain non-empty findings and limitations. Its disposition list must contain
exactly one evidence-backed entry for every one of the 27 accepted #919 finding
IDs. Any substantive candidate or claim change invalidates the assessment and
requires a fresh review under the release plan.

The current truthful verdict is `not_proven`.


## Current documentation reconciliation after the frozen candidate

The documentation audit starts from integration revision
`b0896ac1c22612aab9366edb9120fa9eda55eb64`. This later supplement does not
change the exact candidate or hashes in the #918 publication manifest.
The retained #917 handoff's podcast deferral is superseded: #671 and #1169
closed in v0.92.2 through PR #1174, with Apple and Spotify launch completed.
The #875 pilot deferral is independent and remains unchanged.

The #919 full-review packet and AUDIT_DISPOSITIONS govern the internal review;
provisional reports and failed/partial helper runs remain historical evidence.
The 27-finding repair mapping does not itself establish fresh independent
execution of all repaired behaviors. External assessment has not started.

See the [documentation audit](documentation-audit/README.md) and its full file inventory for current corrections, reviewed historical originals, and remaining coverage limits.
