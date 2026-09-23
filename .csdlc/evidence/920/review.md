# v0.92.2 external-review handoff

Status: **preparation only; external review has not started**.

This is the reviewer entry document for TAIL-05/#920. The full internal source
review is available, but it returned **changes required**. Its 27 findings are
being repaired through four aggregate issues and still require independent
re-review. The #919 report is published and native-reconciled at that revision; PR #1153
and issue #919 remain open.
No external reviewer has been contacted, no disclosure has been authorized,
and no post-remediation external-review candidate has been selected.

## Frozen internal-review baseline

The internal review is immutable evidence about this exact baseline:

- ADL candidate: `5c4a6149771c637f3c805985b86231077965eab4`
- #918 publication-manifest SHA-256:
  `b4f031ae10b3c7a5216523680dfca5b1cd94cdc5d35d3206c852932d37f246c6`
- #919 report revision / PR #1153 head:
  `6fc19987dcb9aaa347e61e6185f4ae1718e5ec6c`
- report result: `changes_required`
- finding denominator: 27 total (4 P1, 20 P2, 3 P3)

The baseline is not automatically the external-review target. After repairs
and independent re-review, update `review-manifest.json` with the exact
post-remediation candidate and accepted predecessor identities. Do not silently
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
| A | #1161 | C-SDLC recovery and operator contracts | 9 | repair and independent re-review pending |
| B | #1162 | CodeFriend and website interoperability/integrity | 7 | repair and independent re-review pending |
| C | #1159 | Runtime/provider health and regression proof | 5 | repair and independent re-review pending |
| D | #1160 | Reproducible validation and release evidence | 6 | repair and independent re-review pending |

All 27 findings map exactly once. Issue creation and work in progress do not
establish a fix. Before external dispatch, record each finding's repair commit,
focused proof, independent exact-head re-review and any accepted residual risk.

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

Review the exact post-remediation candidate and artifact manifest eventually
recorded in `review-manifest.json`. Assess whether the v0.92.2 Beta 1 claims are
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

The activated packet must add exact accepted #917 and #918 identities, the
post-remediation candidate and the completed repair/re-review disposition
records to the #919 sources above. It must also include:

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
