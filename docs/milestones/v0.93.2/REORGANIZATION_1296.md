> **Current successor routing (2026-10-30):** [#1405 allocation](../v0.93.3/REORGANIZATION_1405.md): Runtime/#1352 v0.93.3; tooling and governance/continuity foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Older placement below is historical. v0.93.2 product scope is unchanged.

# Approved v0.93.2 / v0.93.3 reorganization

## Decision and authority

On 2026-10-19 the operator approved smaller milestones: v0.93.2 focuses on CodeFriend Beta 1 launch and enterprise security. Runtime v4, GHB, broader governance and citizen migration move to planned v0.93.3. This changes the earlier 67-identity, five-sprint allocation; that allocation is a preserved baseline, not the target to merge unchanged.

WP-01 #1296 owns reconciliation in draft PR #1297. Only planning is active. Neither product execution nor deployment, provider spend or v0.93.3 activation is authorized by this decision.

## Lossless allocation

[The allocation register](REALLOCATION_1296.json) preserves all 67 original definitions: 21 retained/refocused identities and 46 moved identities. No work is silently discarded. Version-specific integration, qualification and release gates must be defined separately for both milestones; they are not shared completion claims.

- Retain WP-01, WP-S1–S6, DEMO-SEC, INTEGRATE, QUALIFY, CF-92 and TAIL-01–10 in v0.93.2, with product-specific acceptance.
- Move RV-01–11, GOV-01–16, CM-01–04, DEMO-GOV, GHB-SPRINT and GHB-WP00–12 to v0.93.3.
- Python, affect and humour remain deferred outside this allocation. Moving the platform work does not activate these additional tracks.
- Keep CodeFriend #92 as the launch issue. Runtime #112 remains existing evidence for GHB-WP07; its prior closure does not qualify the successor milestone.
- ADL #1275 and C-SDLC #304 remain separately tracked sidecars. ADL #1226 remains a deferred assessment.

## CodeFriend Beta 1 work to plan explicitly

The only open CodeFriend issue observed on 2026-10-19 was #92, which covers launch. It is not an implementation backlog for Beta 1 improvements. Define bounded packages for:

1. Acceptance-gap assessment using representative repositories and user review of actual outputs; produce a prioritized defect/requirement register before claiming product readiness.
2. Useful, readable reports and exports, using the operator's comprehensive-review reference and Geist typography; preserve evidence links and usability across supported output formats.
3. Review correctness, actionable findings and predictable failure/recovery behavior, using the assessment's accepted denominator.
4. Login, repository consent, pairing, first review and results journeys; include revocation and recovery.
5. Operational readiness: bounded resource use, understandable errors, retention controls and rollback/support documentation.

These are planning outcomes, not claims that particular defects were reproduced. Avoid recreating closed work; inspect the accepted implementation and add only proven remaining gaps.

## Security without the full governance milestone

Preserve least privilege, per-message checks, revocation, secrets protection, audit integrity, tenant isolation and deletion. Replace the four broad GOV dependencies with the explicit CodeFriend consumer contracts in the allocation register. WP-S1 must validate supported Runtime interfaces before dependent implementation; any missing primitive becomes a bounded prerequisite, not an implicit full Runtime-v4 upgrade.

Full constitutional appeals, citizen standing and social-memory integration remain required in the v0.93.3 platform contracts. Narrowing Beta 1's consumer scope is not evidence that those integrations already work.

## Smaller sprint sequence

1. WP-01 planning and C-SDLC #296–299 lifecycle simplification; #299 follows the other three.
2. CodeFriend acceptance assessment and security contracts.
3. Beta 1 improvements, enterprise security and independent CI #205.
4. Candidate integration and independent qualification.
5. Separately authorized launch and release review.

This is scheduling only. The operator subsequently approved proceeding with the four C-SDLC simplification issues first.

## Reconciliation acceptance

Update current graphs, feature contracts, WBS, sprint allocation, proof/demo matrices, release gates and successor handoff together. Preserve historical v0.93.1 evidence and the original allocation. Validate unique routing, dependency closure, no Runtime-v4/GHB path into Beta 1 launch, security invariants, reused issues and accurate authorization. Renew independent review for the amended PR before publication/merge. Until that is complete, the existing draft is not ready to merge.

## Existing C-SDLC improvements

These four existing issues are included in v0.93.2 without duplication. The operator approved the plan and proceeding with C-SDLC #296–299 first. Other work remains unactivated.

- [C-SDLC #296](https://github.com/agent-logic/cognitive-sdlc/issues/296): Use one editable issue source and remove projection-drift gates — planned Sprint 1.
- [C-SDLC #297](https://github.com/agent-logic/cognitive-sdlc/issues/297): Preserve proof and review across metadata-only edits — planned Sprint 1.
- [C-SDLC #298](https://github.com/agent-logic/cognitive-sdlc/issues/298): Remove mandatory portable-review attestation and publication receipt gates — planned Sprint 1.
- [C-SDLC #299](https://github.com/agent-logic/cognitive-sdlc/issues/299): Simplify lifecycle commands with optional start and consolidated delivery — planned Sprint 1.

#299 depends on #296, #297 and #298. These run first so later work benefits from the simplification; CodeFriend assessment depends on #299. The milestone quality gate includes their completion. Existing #304 remains an additional separately tracked remount-reliability fix; it is not one of these four improvements. CI #205 is included independently, outside this four-issue group. The four issues have no GitHub milestone assigned at this planning snapshot; metadata reconciliation is outstanding.

## Independent CI work

[CI-205 / C-SDLC #205](https://github.com/agent-logic/cognitive-sdlc/issues/205) must also complete in v0.93.2. It is an independent Sprint 3 track, not part of or dependent on lifecycle simplification #296–299. It covers CI across code and non-paper documentation repositories; papers remain excluded. The milestone quality gate requires its completion. Its existing v0.93.1 title and unassigned GitHub milestone await metadata reconciliation. No execution is started.
