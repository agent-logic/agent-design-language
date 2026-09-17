# Beta 1 delivery requirements after ADR 0084

The operator requires both website modes in v0.92.2 / Beta 1: server-hosted reviews and website-controlled reviews executed by an installed local CodeFriend agent. Access is invitation-only with GitHub sign-in. CLI remains supported. This supersedes the earlier local-CLI-only product selection wherever that selection is used to judge Beta 1 readiness.

## Implementation gaps and gates

| Surface | Required outcome | Owning boundary and readiness consequence |
|---|---|---|
| Website access | Invited users can enter the product; uninvited users cannot access review controls or artifacts | Website implementation owner must be assigned before #914 is ready; existing coming-soon pages are insufficient |
| Server execution | Website starts and observes real review runs on Agent Logic servers with scoped evidence and approval | Bounded service implementation owner and hosting design remain required before #914 |
| Installed local agent | Installed agent connects to website, runs reviews locally and reports run/artifact state with explicit cancellation, interruption and retry outcomes | Bounded local-agent implementation owner and connection design remain required before #914 |
| Shared contracts | Both modes preserve identity, privacy, retention, authority and publication semantics | Existing component owners retain contract ownership; integration cannot redefine completed outputs silently |
| Integration #914 | One complete installed product connects accepted components for both website modes and CLI | Rebaseline typed cards and dependencies before execution; prior component dependency closure alone is insufficient |
| Qualification #915 | Independent actual journeys in both website modes plus CLI on the identified integrated candidate, selected platforms and external scope | Rebaseline typed qualification cards before execution; CLI-only proof cannot pass |
| Sprint #936 and release #925 | Readiness includes the revised #914/#915 gates | ADR acceptance satisfies only the decision-set gate; it does not satisfy product or release gates |

These are requirements and missing owner assignments, not newly created issues, accepted implementation outputs, or claims that existing typed cards have already been amended. Before starting #914/#915, the sprint conductor must assign bounded website, server and agent implementation issues and reconcile the typed cards and dependency graph. #945 does not implement those components.

## Repository boundary

The website stays in `agent-logic/codefriend.ai`; product code has a separate private repository whose name is not selected. The v0.93 extraction sequence remains separately governed. Beta 1 website functionality cannot be deferred merely because extraction happens later. Deployment topology and how pre-extraction code is packaged for the website are implementation decisions still requiring evidence.

## Proof boundary

Run the actual ingestion, review, synthesis, plans, approval and rendering journey in each mode. Record execution location, release/candidate identity, platform, source scope, failures, disconnects and recovery. Check invited-user access and isolation between users. Preserve existing PDF and publication approval obligations. Documentation checks establish these gates are recorded; they cannot prove the product works.
