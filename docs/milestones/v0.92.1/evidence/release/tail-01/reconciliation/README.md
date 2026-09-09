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

## Remaining work

The eight proposed corrections do not complete the full assessment. Retained
criteria still need criterion-level evidence or explicit amendments, and the
remaining execution gaps and grouped exceptions still need dispositions.
Downstream closeout criteria must be mapped to their actual stages before any
change to gate eligibility. The presence of a later stage is not by itself
permission to discard a predecessor obligation.

The GCP #730/#740 corrective proof is being checked for state-recovery and
local-residue coverage. No additional GCP criterion is accepted by this packet
yet. Historical construction and cutover documents may preserve earlier
blocking states; a later merge alone does not establish that every detailed
retained requirement passed.

## Validation and review

Run `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py`.
The offline validator checks original row identity, source digests, unique
corrections, amendment references, successor ancestry, and equality between
candidate and reviewed evidence blobs. Its success is structural evidence,
not semantic approval. The independent reviewer must examine the meaning and
coverage of each correction before it affects an updated gate decision.

PVF: deterministic local contract validation; evidence reconciliation proof;
small local CPU; required before accepting this packet. No cloud operation or
Runtime experiment is required to validate the document mapping.
