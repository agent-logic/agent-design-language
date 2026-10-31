# Balanced milestone organization — #1405

Approved operator amendment, 2026-10-30. Planning allocation, not implementation/installed/release acceptance. [Original #1403 decision](REORGANIZATION_1403.md) and both prior allocation registers remain unchanged historical provenance.

| Milestone | Scope | Planned WPs |
| --- | --- | --- |
| [v0.93.3](../v0.93.3/README.md) | Runtime v4 foundations | 15 |
| [v0.93.4](../v0.93.4/README.md) | C-SDLC v4 and governance/continuity foundations | 17 |
| [v0.93.5](../v0.93.5/README.md) | GHB | 17 |
| [v0.93.6](../v0.93.6/README.md) | Governance and social cognition | 17 |
| [v0.93.7](../v0.93.7/README.md) | Affect and humour | 25 |

## Allocation and dependencies

[Current register](REALLOCATION_1405.json) preserves all59 original records and acceptance/adverse cases:12 in v0.93.3,8 foundations in v0.93.4,14 GHB in v0.93.5,13 governance/demo in v0.93.6,12 original integration/qualification/tail records in v0.93.7. All12 previously added affect/humour packages also remain in v0.93.7. Existing #1352 remains v0.93.3; Runtime #112 remains the reused GHB-WP07 identity.

Move GOV-01/02/03/11 and CM-01..04 ahead of GHB into v0.93.4. Their implementation depends on accepted Runtime foundations, not the new tooling implementation; disjoint work may proceed alongside tooling after activation. The v0.93.4 exit requires installed proof for both tracks, so GHB never begins on an unaccepted migration/standing baseline.

v0.93.6 retains13 remaining governance/demo packages and adds bounded GOV-INTEGRATE/GOV-REVIEW/GOV-CLOSE. These accept governance independently, preserving every original obligation. All8 affect and4 humour packages move to v0.93.7. The original INTEGRATE/QUALIFY and ten TAIL identities also move there intact: their combined governance/security/GHB/affect qualification is not replaced by the earlier governance handoff. Counts include setup/acceptance/tail gates, not a one-PR-per-row requirement; v0.93.7 contains12 feature packages rather than25 separate feature projects.

Every package descends from its own activation gate; accepted predecessor outcomes are required, not inferred from source, CI or plan completion. No child issues/cards, product work, paid work, deployment, Runtime restart, binaries, tags, or release. Container packaging remains deferred.
