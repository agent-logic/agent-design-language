# Roadmap and milestones

ADL's public repository owns language contracts, schemas and CLI dispatch.
Product implementations have [separate repository owners](REPOSITORIES.md).
For installation and a first local run, start with the [quickstart](../README.md).

## Current work

The current delivery plan is [v0.93.2](milestones/v0.93.2/README.md):
CodeFriend Beta 1 and enterprise security across the owning repositories.
Use its [created issue index](milestones/v0.93.2/ISSUES_v0.93.2.md) and
[execution graph](milestones/v0.93.2/EXECUTION_PLAN_v0.93.2.json) for scope and
ordering. The [Sprint 2 preparation packet](milestones/v0.93.2/sprint-2-preparation/README.md)
records a newer checkpoint than the opening prose retained in the milestone
README. Planning pages and prepared cards do not establish delivered features.

[Public repository readiness #1318](https://github.com/agent-logic/agent-design-language/issues/1318)
is the separate cleanup wave for onboarding, safety, security intake and
controlled historical-material retention. Its seven children carry the work;
it is not a new product release or a replacement for the existing sprint plan.

## Approved successor sequence — 2026-10-30

| Milestone | Scope | Planned WPs |
| --- | --- | --- |
| [v0.93.3](milestones/v0.93.3/README.md) | Runtime v4 foundations | 15 |
| [v0.93.4](milestones/v0.93.4/README.md) | C-SDLC v4 and governance/continuity foundations | 17 |
| [v0.93.5](milestones/v0.93.5/README.md) | GHB | 17 |
| [v0.93.6](milestones/v0.93.6/README.md) | Governance and social cognition | 17 |
| [v0.93.7](milestones/v0.93.7/README.md) | Affect and humour | 25 |

[Current allocation and lossless routing](milestones/v0.93.3/REORGANIZATION_1405.md). Eight foundations move ahead of GHB; remaining governance and affect/humour have independent milestone boundaries. Counts include setup/review/release gates. Child issues/cards are created per activated sprint; planning authorizes no implementation or release.

## Milestone index

This index was checked on **2026-10-20**. GitHub links show live scope and state;
tracked summaries retain dated planning and acceptance evidence. Counts are
omitted deliberately because they change independently of release acceptance.

| Milestone | GitHub state at check | Tracked scope and remaining outcome |
| --- | --- | --- |
| [v0.92.1](https://github.com/agent-logic/agent-design-language/milestone/1) | Open | [Summary](archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.92.1/README.md"). Retained Runtime/Observatory backlog includes [#84](https://github.com/agent-logic/agent-design-language/issues/84) and [#251](https://github.com/agent-logic/agent-design-language/issues/251). Resolve their owner/disposition and the [release criteria](archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.92.1/RELEASE_PLAN_v0.92.1.md") before declaring the milestone terminal. Public ADL cleanup does not revive Unity work. |
| [v0.92.2](https://github.com/agent-logic/agent-design-language/milestone/2) | Closed | [Accepted handoff and explicit limitations](archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.92.2/NEXT_MILESTONE_HANDOFF_v0.92.2.md"). A closed milestone does not erase the retained deferrals. |
| [v0.93](https://github.com/agent-logic/agent-design-language/milestone/3) | Open, deferred planning | [Historical planning package](milestones/v0.93/README.md). Its original scope was subsequently split; follow the current [v0.93.2 allocation register](milestones/v0.93.2/REALLOCATION_1296.json) rather than treating the original candidate list as an active delivery commitment. Remaining scope needs explicit allocation and owner acceptance. |
| [v0.93.1](https://github.com/agent-logic/agent-design-language/milestone/4) | Open | [Dated status](milestones/v0.93.1/README.md) and [publication candidate](milestones/v0.93.1/publication-1236/RELEASE_NOTES_CANDIDATE.md). Zero open issues at the checked milestone snapshot is not final release acceptance: the retained candidate remains an unapproved draft with final lockset, document/reviewer and launch acceptance limits. Reconcile those records with the successor scope before closure. |
| [v0.93.2](https://github.com/agent-logic/agent-design-language/milestone/5) | Open | [Current plan](milestones/v0.93.2/README.md), [five sprint umbrellas](milestones/v0.93.2/ISSUES_v0.93.2.md), and [release criteria](milestones/v0.93.2/RELEASE_PLAN_v0.93.2.md). Integration, qualification, review and authorized launch remain distinct outcomes. |

No milestone was closed or issue reassigned by this documentation change.
Historical descriptions may predate the repository split or successor scope;
use the dated evidence and current issue links together. Do not infer that a
merged implementation, an empty issue list or a release tag proves deployment.

## What the versions mean

- **CLI/package version:** the version declared by that package and its release
  artifact. Read the matching manifest and release notes; packages need not
  share a milestone number.
- **Document/schema version:** the format or contract identified by that
  document/schema. It is not a promise that a similarly numbered CLI supports
  every historical or private-owner feature.
- **Milestone version:** a planning and acceptance grouping. It can include
  work across repositories and remain open after source changes merge.
- **Release:** an explicitly published, identified source or artifact set.
  [GitHub releases](https://github.com/agent-logic/agent-design-language/releases)
  are the publication record; a source-only release is not a prebuilt executable
  distribution or evidence of a deployed private product.

For older plans, use the [milestone directory](milestones/). Preserve their
historical claims as dated evidence rather than silently rewriting them into
current product promises.
