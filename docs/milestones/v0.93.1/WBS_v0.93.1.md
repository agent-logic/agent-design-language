# v0.93.1 Work Breakdown

## Metadata

Planning template family 1.1.0; authoring issue #922. Scope split approved; Sprint 1 preparation opened by the operator on 2026-09-25. Split execution and release are not authorized. Named execution owners and resource limits remain to be assigned.

## Status

Sprint 1 issue preparation is open; its 14 core identities and umbrella exist in ADL. No new later-sprint or closeout issues are created now; later-wave preparation waits for split acceptance and preserves existing identities. Extraction, provider spending, public launch and release are not authorized.

## How To Use

Logical IDs each identify one completed result. Execution specifications own exact dependencies, negatives and proof.

## WBS Summary

The 43 core work packages plus #1148/#1149 and operator-routed #875/#1145 map to 47 planned identities. Fourteen Sprint 1 core issues and CF-05 #1150 already exist; at most 28 core issues remain to create after split acceptance. The Sprint 1 umbrella and separately scoped supporting issues are outside this denominator.

## Candidate WP Sequence

| ID | Sprint | One completed result |
|---|---|---|
| WP-01 | 1 | Open the reviewed v0.93.1 execution wave |
| RD-01 | 1 | Refresh final-head ownership audit |
| RD-02 | 1 | Accept repository boundaries and cutover contracts |
| RD-12 | 1 | Bootstrap the approved destination repositories |
| RD-13 | 1 | Transfer and map existing Sprint 1 issue identities |
| RD-03 | 1 | Distribute portable public ADL contracts |
| RD-04 | 1 | Extract and qualify C-SDLC |
| RD-05 | 1 | Extract and qualify Runtime with Observatory |
| RD-06 | 1 | Extract artifact-consuming infrastructure |
| RD-09 | 1 | Establish enterprise-security producer boundary |
| RD-10 | 1 | Extract CodeFriend software and preserve its website |
| RD-08 | 1 | Qualify demo ownership after product moves |
| RD-07 | 1 | Retire superseded source with verified public ADL |
| RD-11 | 1 | Accept the complete split and release lockset |
| CF-01 | 2 | Accept the Beta 1 to launch requirements mapping |
| CF-02 | 2 | Deliver a deployable CodeFriend product |
| CF-03 | 2 | Complete external tester onboarding |
| CF-04 | 2 | Qualify launch operations and recovery |
| CF-05 | 3 | Qualify the external beta launch candidate |
| CF-06 | 3 | Prepare the website to hand off to the qualified product |
| CF-07 | 3 | Launch the qualified CodeFriend service |
| CT-01 | 2 | Accept the complete artifact template catalog |
| CT-02 | 2 | Release the common branded template foundation |
| CT-09 | 2 | Release the test template family |
| CT-08 | 2 | Release the diagram template family |
| CT-07 | 2 | Release the review template family |
| CT-06 | 2 | Release the document template family |
| CT-03 | 2 | Prepare artifact content through named fields |
| CT-10 | 2 | Preview and export branded artifacts |
| CT-04 | 3 | Preserve artifact compatibility across template upgrades |
| CT-05 | 3 | Qualify the complete branded artifact catalog |
| INTEGRATE | 3 | Accept the product, templates and split candidate lockset before CF-05 |
| QUALIFY | 4 | Audit accepted independent qualification and live launch evidence |
| TAIL-01 | 4 | Quality gate |
| TAIL-02 | 4 | Documentation review and external-review handoff |
| TAIL-03 | 4 | Publication finalization |
| TAIL-04 | 4 | Internal milestone review |
| TAIL-05 | 4 | External or third-party review |
| TAIL-06 | 4 | Accepted-findings remediation or explicit deferral capture |
| TAIL-07 | 4 | Next-milestone planning |
| TAIL-08 | 4 | Next-milestone closeout planning |
| TAIL-09 | 4 | Next-milestone planning review |
| TAIL-10 | 4 | Release ceremony and milestone close |

## Work Packages

Each result has a single accountable repository. Extraction carries source, fixtures, manuals and release paths together; tests supporting a result do not become administrative issues.

## Existing Launch Prerequisites

Existing #1148 (citation-grounded correctness) and #1149 (uncertain second-run recovery) are additional launch prerequisites. CF-05 reuses existing #1150 for independent qualification after scope alignment; do not seed a duplicate CF-05 issue. The 43 core work packages plus #1148/#1149 and operator-routed #875/#1145 map to 47 planned identities. Fourteen Sprint 1 core issues and CF-05 #1150 already exist; at most 28 core issues remain to create after split acceptance. The Sprint 1 umbrella and separately scoped supporting issues are outside this denominator. Preserve their identities and original #915 failures; do not create duplicate tasks or replay an uncertain request to manufacture a result.

## Sequencing

WP-01 → RD-01/RD-02/RD-12/RD-13 and extraction → RD-11 → CodeFriend/templates → INTEGRATE → CF-05 → CF-06 → CF-07 → QUALIFY → TAIL-01–TAIL-10.

## Sequencing Notes

Runtime v4 is not in this graph. RD-11 proves the baseline cutover, not future launch completeness. CT-05, the INTEGRATE lockset and accepted #1148/#1149 results gate CF-05, which reuses #1150 for independent qualification.

## Acceptance Mapping

See feature proof coverage; every result retains a consumer, exact version and negative-case proof.

## Exit Criteria

All 43 results are mapped once; existing follow-ons have links and evidence, not duplicate implementations.

## Sprint 1 preparation amendment — 2026-09-25

See [operator amendment](OPERATOR_AMENDMENT_2026-09-25.md) for the verified issue map, routing and historical-evidence boundary. RD-01 depends on acceptance of WP-01's opening/preparation checkpoint, not closure of #1178. WP-01 stays open through later-wave creation. RD-03 and RD-04 both depend on RD-13 and converge at RD-05; serial scheduling does not add an RD-03 → RD-04 dependency.

Sprint 1 closeout requires RD-11 acceptance and every Sprint 1 result accepted or explicitly deferred, recording issue/PR, source and artifact versions, actual proof, original review findings and dispositions, failures/unknowns, residual owner and cleanup status. Record WP-01's later-wave duty as an open milestone obligation. Cleanup remains separately owned. The ten-step milestone tail remains planned only; no tail or later-sprint issues are created by this amendment.
