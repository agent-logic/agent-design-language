# Issue 517: reconciliation of the recorded gate findings

This packet re-examines the assessment merged in PR #748. It does not overwrite
that assessment or authorize release. The admitted candidate remains
`bf159eb416950dfa3399933829726a7b7e71f897`; proof from later revisions cannot be
silently attributed to it.

## Why the counts differed

The three findings reviewed on PR #748 concerned the branch adapter, missing
review-path evidence, and stale lifecycle cards. Their fixes did not themselves
resolve the release assessment's acceptance rows. Reporting completion of those
three fixes without reconciling the release findings left the broader work
unfinished.

The historical gate has 393 inventoried rows: 139 execution criteria, 227
retained predecessor criteria, and 27 future release-tail criteria. Of its 366
required rows, 121 passed and 245 were non-proving. The latter comprise 18
execution criteria and 227 retained criteria. They are not 245 observed
implementation failures. Five grouped exceptions summarize several kinds of
evidence and classification debt; they are not another five acceptance rows.

`corrections.json` retains the exact source and historical denominator digests.
Every proposed correction names the original row and criterion digest. Rows
without a reviewed correction remain unresolved, and no row is deleted.

## First evidence corrections

| Original rows | Correction | Evidence boundary |
| --- | --- | --- |
| CORP-B-ac-1 through ac-3 | Accepted operator scope amendment | The operator accepted the read-only custody register and explicitly made the operational follow-ons non-gating backlog. Their operational work is not relabeled proven. |
| OBS-B-ac-2 | Accepted removal of the obsolete #84 dependency | The current authorized issue body excludes #84 and still requires an authentic Runtime route without mock substitution. The ledger maps that replacement obligation to retained live-route evidence. |
| OBS-A-ac-1 through ac-4 | Map inherited criteria to reviewed #512 implementation | The #511 absorption comment preserves these requirements. Reviewed README, implementation, markup, tests, and retained browser observations supply their proof; closure alone does not. |

The OBS-A accessibility criterion requires specified keyboard and screen-reader
flows. The evidence supports markup relationships and keyboard behavior; this
packet does not claim comprehensive screen-reader operation was tested. Its
state criterion requires designed states; the screenshot alone is not proof
of all empty, degraded, recovery, or revoked cases.

## Complete accounting scope

The follow-up census maps all 245 historical non-proving rows exactly once,
while preserving all 393 original inventory rows and the original candidate.
The source packets distinguish 16 bounded evidence mappings, 12 accepted scope
amendments, seven review-freshness resolutions, 11 later-stage obligations,
51 source-supported rows without full execution receipts, 144 non-proving rows,
and four current quality-gate obligations. These classifications are accounting
dispositions, not an automatic recalculation of release passes.

`current-exceptions.json` records the remaining execution corrections, the
post-review delta checks and owned-path overlap review. The canonical execution
specification is synchronized with the captured live WP-01, GCP-E, HOT-01 and
OBS-B issue requirements. `release-stage-mapping.json` preserves the distinct
quality-gate, successor-planning and ceremony stages. The corporate/Runtime and
V3 packets preserve each retained criterion, evidence boundary and missing proof.

The operator clarified that this issue is documentation and accounting only.
Any actual implementation or new operational proof belongs to a separate issue.
The three native fixes developed during the audit were removed from this docs
change and preserved with their review evidence for separate issue delivery.
Their tests do not supply retrospective proof for the historical candidate.

Accounting completion requires every row and exception to have a documented
disposition and a real follow-up owner where action remains. Release eligibility
remains a separate decision: ownership alone does not prove a required lane or
waive an acceptance criterion. The original assessment stays intact.

## Merged terminal closeout

PR #750 merged as `f61deb36d5eccb4a3e391510bbea4bb6f2d51216` and
reports zero missing terminal records in the current closed-and-merged v0.92.1
issue denominator. `terminal-closeout-750.json` preserves the 22 merged receipt
and state artifacts for its 11 issues. Those lifecycle closeouts are complete;
they must not be counted as outstanding accounting work or recreated as
implementation issues. This later closeout evidence supplements the historical
candidate assessment without silently rewriting its candidate identity.

## Validation and review

Run `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py` for the initial eight corrections, then
`python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/summarize.py`
for the complete 245-row census, successor ownership and five exception groups.
The latter regenerates `census.json`; compare it with the reviewed retained
version using `git diff --exit-code -- docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/census.json`
after committing the reviewed package. The initial validator's remaining-row
count describes only its eight-correction scope, not unfinished accounting.
The offline validator checks original row identity, source digests, unique
corrections, amendment references, successor ancestry, and equality between
candidate and reviewed evidence blobs. Its success is structural evidence,
not semantic approval. The independent reviewer must examine the meaning and
coverage of each correction before it affects an updated gate decision.

PVF: deterministic local contract validation; evidence reconciliation proof;
small local CPU; required before accepting this packet. No cloud operation or
Runtime experiment is required to validate the document mapping.
