# Milestone organization — #1403

Operator decisions, 2026-10-30. Scope approved; these are first-pass execution plans, not implemented or release-approved products.

| Milestone | Scope | Planned identities | Posture |
| --- | --- | --- | --- |
| v0.93.3 | Runtime v4 foundations plus existing #1352 | 15 | Setup prepared; child activation after accepted v0.93.2 inputs |
| v0.93.4 | C-SDLC v4 workflow simplification, overhead and compatibility | 9 | First-pass plan |
| v0.93.5 | GHB and required standing/continuity prerequisites | 25 | First-pass plan |
| v0.93.6 | Remaining governance, affect and humour | 38 | First-pass plan |

The [allocation register](REALLOCATION_1403.json) preserves every former v0.93.3 identity: 12 stay; 22 move to v0.93.5 and 25 move to v0.93.6. New version-qualified setup/review/release gates are additional identities, not renumbered or reopened historical issues. #1352 stays in v0.93.3; all AF-01–08 and HU-01–04 belong to v0.93.6. #1296 remains immutable provenance, resolved through this successor mapping.

## GHB prerequisite closure

GHB-WP07 requires CM-02; GHB-WP08 requires CM-03/CM-04. Their existing dependency closure is GOV-01/GOV-02/GOV-03/GOV-11 and CM-01–CM-04. These eight packages travel with GHB in v0.93.5; moving them later would create forward dependencies or weaken accepted installed-consumer obligations. Other governance remains in v0.93.6. This is the minimum existing closure, not a new governance expansion.

The sequence is v0.93.2 acceptance → Runtime foundations → tooling compatibility → GHB/prerequisites → remaining governance/affect/humour. GHB consumes accepted Runtime interfaces; affect consumes the accepted GHB baseline without forming a dependency cycle. C-SDLC overhead goals are performance acceptance, not wall-clock kill timers. Native v3 remains current authority until a separately accepted cutover. Container packaging remains deferred.

No child implementation, paid work, deployment, Runtime restart, shared-binary change, tag or release is started. Create issues/cards per sprint. Counts include setup/review/release gates, not a one-PR-per-row mandate.
