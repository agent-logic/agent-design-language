# v0.92.2 decision inventory

The complete candidate text is indexed in [README](README.md). All twelve records remain Proposed; no architecture implementation or acceptance is inferred.

| Candidate | Decision question | Accountable scope owner |
|---|---|---|
| [ADR-CF-01](../../../../architecture/adr/issue-911/adr-cf-01.md) | One immutable packet for local, GitHub and CI inputs; exact revision, scope and unsupported-language limits. | CF-ADAPTER (#878) |
| [ADR-CF-02](../../../../architecture/adr/issue-911/adr-cf-02.md) | Separate evidence identity from finding matching; canonical contracts, provenance and incomplete-run semantics. | CF-EVIDENCE (#881) |
| [ADR-CF-03](../../../../architecture/adr/issue-911/adr-cf-03.md) | Repository instructions are evidence, not authority; redact before model use and retention, enforce deletion and prevent private schema forks. | CF-EVIDENCE (#881) |
| [ADR-CF-04](../../../../architecture/adr/issue-911/adr-cf-04.md) | Separate observed structure, impact, rationale and drift from inference; expose scope and unknowns. | CF-COG (#882) |
| [ADR-CF-05](../../../../architecture/adr/issue-911/adr-cf-05.md) | Machine-checkable fitness rules run locally and in CI; human judgment remains explicit and rule limits remain visible. | CF-GOV (#887) |
| [ADR-CF-06](../../../../architecture/adr/issue-911/adr-cf-06.md) | Four isolated perspectives commit before peer visibility; synthesis preserves attribution/disagreement; remediation and test plans remain advisory. | CF-REVIEW (#890) |
| [ADR-CF-07](../../../../architecture/adr/issue-911/adr-cf-07.md) | Compatible baseline selection, stable matching and not-comparable outcomes; retrieve prior reviews through the shared Memory Palace boundary. | CF-MEMORY (#885) |
| [ADR-CF-08](../../../../architecture/adr/issue-911/adr-cf-08.md) | Approval binds the exact finding/run/artifact/target set; changes invalidate approval; Markdown, HTML and PDF preserve semantic parity. | CF-UX (#895) |
| [ADR-CF-09](../../../../architecture/adr/issue-911/adr-cf-09.md) | Local adl codefriend CLI and artifact browsing in this repository; shared services, no hosted customer service or autonomous source edits. | CF-SHELL (#891) |
| [ADR-PLAT-01](../../../../architecture/adr/issue-911/adr-plat-01.md) | Configuration/profile ownership versus Runtime lifecycle, credentials and cost observation; consumers reuse the same provider contracts; hardware experiments do not imply production suitability. | PLAT-PROVIDER (#876) |
| [ADR-CSDLC-01](../../../../architecture/adr/issue-911/adr-csdlc-01.md) | Intent-oriented commands share one lifecycle transaction owner; cards are derived projections; diagnostics are observational; local/remote effects reconcile idempotently. | SIM-04 (#870) |
| [ADR-CSDLC-02](../../../../architecture/adr/issue-911/adr-csdlc-02.md) | Staged replacement under an explicitly authorized writer fence; semantic conversion proof, exact installed provenance, bounded restore before new writes and forward repair afterwards. | SIM-08 (#874) |

## All 69 task identities

Every task has a candidate reference or a reasoned reuse/no-new-decision disposition. This accounts for scope, not implementation completion. The sources for the denominator and result types are the pinned launch map, atomic task contracts and issue wave in source-manifest.json.

| Task | Issue | Disposition |
|---|---|---|
| WP-01 | #864 | Reuse launch inventory and approval evidence; task creation adds no new architecture. |
| RT-PROVIDER | #855 | ADR-PLAT-01 |
| ARCH-SPLIT | #848 | Reconcile the owner-produced #848 repository/public-private boundary decision; no extraction or acceptance invented by #911. |
| CSDLC-MERGE | #849 | ADR-CSDLC-01 |
| QUAL-RUNTIME | #852 | Reuse accepted ADR0054 Runtime and ADR0048 observability; repair proof is not new authority. |
| RT-COST | #854 | ADR-PLAT-01 |
| CSDLC-MAN | #861 | Document ADR-CSDLC-01/02; manual is a projection of settled authority. |
| CSDLC-DECOMPOSE | #862 | ADR-CSDLC-01 |
| OBS-LIVE | #720 | Reuse accepted ADR0054/0048; deferred ADR0069 dual-client proof remains deferred. |
| SIM-UMBRELLA | #866 | ADR-CSDLC-02 |
| SIM-01 | #867 | ADR-CSDLC-01 |
| SIM-02 | #868 | ADR-CSDLC-01 |
| SIM-03 | #869 | ADR-CSDLC-01 |
| SIM-04 | #870 | ADR-CSDLC-01 |
| SIM-05 | #871 | ADR-CSDLC-01 |
| SIM-06 | #872 | ADR-CSDLC-02 |
| SIM-07 | #873 | ADR-CSDLC-02 |
| SIM-08 | #874 | ADR-CSDLC-02 |
| SIM-09 | #875 | ADR-CSDLC-02 |
| PLAT-PROVIDER | #876 | ADR-PLAT-01 |
| PLAT-UTS | #877 | Reuse accepted ADR0020 portable schema / ADR0021 ACC authority; crate packaging alone is not a new tool authority. |
| CF-ADAPTER | #878 | ADR-CF-01 |
| CF-ADAPTER-GITHUB | #879 | ADR-CF-01 |
| CF-ADAPTER-CI | #880 | ADR-CF-01 |
| CF-EVIDENCE | #881 | ADR-CF-02; ADR-CF-03 |
| CF-COG | #882 | ADR-CF-04 |
| CF-COG-IMPACT | #883 | ADR-CF-04 |
| CF-COG-RATIONALE | #884 | ADR-CF-04 |
| CF-MEMORY | #885 | ADR-CF-07 |
| CF-COG-DRIFT | #886 | ADR-CF-04; ADR-CF-07 |
| CF-GOV | #887 | ADR-CF-05 |
| CF-GOV-CI | #888 | ADR-CF-05 |
| PLAT-MEMORY | #889 | ADR-CF-07 |
| CF-REVIEW | #890 | ADR-CF-03; ADR-CF-06 |
| CF-SHELL | #891 | ADR-CF-09 |
| CF-SYNTHESIS | #892 | ADR-CF-06 |
| CF-REMEDIATE | #893 | ADR-CF-06 |
| CF-TESTPLAN | #894 | ADR-CF-06 |
| CF-UX | #895 | ADR-CF-08 |
| CF-RENDER-MD | #896 | ADR-CF-08 |
| CF-RENDER-HTML | #897 | ADR-CF-08 |
| CF-RENDER-PDF | #898 | ADR-CF-08 |
| QUAL-INVENTORY | #899 | Reuse ADR0054 Runtime ownership; inventory is evidence, not replacement architecture. |
| QUAL-RESIDENT | #900 | Reuse ADR0054; retain exact resident success/failure proof separately. |
| QUAL-PROVIDER | #901 | Consume ADR-PLAT-01 and ADR0054; qualification does not accept the proposed record. |
| QUAL-EVIDENCE | #902 | Consume ADR0054 and ADR-PLAT-01; preserve producer-specific evidence and nonzero proof denominators. |
| PLAT-MLX | #903 | ADR-PLAT-01 |
| PLAT-PAIR | #904 | ADR-PLAT-01 |
| SPEC-RETEST | #905 | ADR-PLAT-01 |
| PLAT-RUST | #906 | Behavior-preserving refactor; no new architectural decision unless the selected boundary changes. |
| CSDLC-REMOTE | #907 | ADR-CSDLC-01 |
| OPS-AWS | #908 | Operational inventory delta and runbook; no new architecture unless owner identifies a changed boundary. |
| OPS-GCP | #909 | Read-only reconciliation of prior move-in authority; no new architecture or cloud mutation. |
| OBS-S3 | #910 | Reconcile existing #679/#685 static-client design with ADR0054; record explicit deployment decision/approval provenance, without promoting deferred dual-client ADR0069. |
| ARCH-ADR | #911 | Own this inventory and candidate curation; acceptance remains explicit decision-owner authority. |
| PUB-MEDIUM | #912 | Consume ADR0025 and proposed CodeFriend decisions with accurate status; article grants no architecture authority. |
| PUB-CSDLC | #913 | Consume settled C-SDLC evidence with private manuscript separation; writing grants no architecture authority. |
| CF-INTEGRATE | #914 | ADR-CF-09 |
| CF-PROOF | #915 | Consume proposed CodeFriend records and qualification selections; independently qualify installed integrated product, do not infer proof from ADR text. |
| TAIL-01 | #916 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-02 | #917 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-03 | #918 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-04 | #919 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-05 | #920 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-06 | #921 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-07 | #922 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-08 | #923 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-09 | #924 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
| TAIL-10 | #925 | Reuse canonical ten-step release policy; record accepted product evidence and issue-specific outcome, no standalone architectural decision. |
