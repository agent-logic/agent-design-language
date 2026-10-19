# v0.93.2 Work Breakdown Structure

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Milestone opening authorized on 2026-10-19; [WP-01 #1296](https://github.com/agent-logic/agent-design-language/issues/1296) is in progress in Planning #12. See [opening record](OPENING_1296.md). Downstream execution, provider spend, deployment and release are not authorized by this opening.

## How To Use

Use version-qualified IDs. The execution graph owns exact dependencies and specifications own acceptance.

## WBS Summary

67 planned identities: the retained 52 platform candidates, existing CodeFriend #92, plus `GHB-SPRINT` and `GHB-WP00` through `GHB-WP12`. Existing #875 is a separately gated v0.93.1 sidecar and is not counted in 52. Podcast #671 is completed in v0.92.2 (PR #1174).

## Candidate WP Sequence

| ID | Sprint | Single result | Repository |
|---|---|---|---|
| WP-01 | 1 | Open the reviewed v0.93.2 execution wave | agent-design-language |
| RV-01 | 1 | Resolve plugin lifecycle design findings | agent-logic-runtime |
| RV-02 | 1 | Adapt built-in components to the native plugin contract | agent-logic-runtime |
| RV-03 | 1 | Implement recoverable plugin generation transitions | agent-logic-runtime |
| RV-09 | 1 | Migrate versioned plugin state | agent-logic-runtime |
| RV-10 | 1 | Reconfigure a plugin atomically | agent-logic-runtime |
| RV-11 | 1 | Remove a plugin without breaking dependents | agent-logic-runtime |
| RV-04 | 1 | Run a governed process plugin | agent-logic-runtime |
| RV-05 | 1 | Run a bounded WASM component plugin | agent-logic-runtime |
| RV-06 | 1 | Prove cross-adapter module equivalence | agent-logic-runtime |
| RV-07 | 1 | Move provider instances to plugin configuration | agent-logic-runtime |
| RV-08 | 1 | Qualify installed plugin recovery and operations | agent-logic-runtime |
| GOV-01 | 2 | Enforce citizen and guest actor boundaries | agent-logic-runtime |
| GOV-02 | 2 | Evaluate trace-grounded rights and duties | agent-logic-runtime |
| GOV-03 | 2 | Apply evidence-backed standing transitions | agent-logic-runtime |
| GOV-04 | 2 | Produce constitutional review from evidence | agent-logic-runtime |
| GOV-05 | 2 | Resolve challenges and appeals | agent-logic-runtime |
| GOV-06 | 2 | Persist governed private ToM updates | agent-logic-runtime |
| GOV-07 | 2 | Preserve ToM conflict and temporal decay | agent-logic-runtime |
| GOV-08 | 2 | Persist governed relationship context | agent-logic-runtime |
| GOV-09 | 2 | Publish challengeable reputation projections | agent-logic-runtime |
| GOV-10 | 2 | Consume redacted shared social memory | agent-logic-runtime |
| GOV-11 | 2 | Enforce delegated action authority | agent-logic-runtime |
| GOV-12 | 2 | Execute governed upstream cognition | agent-logic-runtime |
| GOV-13 | 2 | Communicate without private inspection | agent-logic-runtime |
| GOV-14 | 2 | Evaluate bounded social-contract obligations | agent-logic-runtime |
| GOV-15 | 2 | Operate governed guild membership and action | agent-logic-runtime |
| GOV-16 | 2 | Produce redacted polis governance health | agent-logic-runtime |
| WP-S1 | 2 | Enforce the enterprise zero-trust boundary | agent-logic-enterprise-security |
| WP-S2 | 2 | Enforce per-action and per-message authorization | agent-logic-enterprise-security |
| WP-S3 | 2 | Enforce key and secret lifecycle | agent-logic-enterprise-security |
| WP-S4 | 2 | Produce tamper-evident incident evidence | agent-logic-enterprise-security |
| WP-S5 | 2 | Enforce protected data lifecycle and isolation | agent-logic-enterprise-security |
| WP-S6 | 2 | Operate a replayable security regression drill | agent-logic-enterprise-security |
| CM-01 | 3 | Accept citizen migration and descendant identity contracts | agent-logic-runtime |
| CM-02 | 3 | Migrate one citizen without conflicting active identity | agent-logic-runtime |
| CM-03 | 3 | Create a governed descendant with distinct identity | agent-logic-runtime |
| CM-04 | 3 | Qualify citizen migration and reproduction recovery | agent-logic-runtime |
| DEMO-GOV | 3 | Run integrated governance demonstrations | agent-logic-runtime |
| DEMO-SEC | 3 | Run integrated security demonstrations | agent-logic-enterprise-security |
| GHB-SPRINT | 4 | [GHB][Sprint] Deliver governed recursive improvement in the installed Agent Logic Runtime | agent-logic-runtime |
| GHB-WP00 | 4 | [GHB][WP00] Source-to-caller audit and baseline | agent-logic-runtime |
| GHB-WP01 | 4 | [GHB][WP01] Normative specification and invariants | agent-design-language |
| GHB-WP02 | 4 | [GHB][WP02] Schemas and installed contract parity | agent-design-language |
| GHB-WP03 | 4 | [GHB][WP03] Belief update and experiment selection | agent-logic-runtime |
| GHB-WP04 | 4 | [GHB][WP04] Candidate generation and archive | agent-logic-runtime |
| GHB-WP05 | 4 | [GHB][WP05] Isolated body builder and experiment executor | agent-logic-runtime |
| GHB-WP06 | 4 | [GHB][WP06] Independent evaluator and promotion decision | agent-logic-runtime |
| GHB-WP07 | 4 | [P1] Complete the governed resident-to-birthday activation path | agent-logic-runtime |
| GHB-WP08 | 4 | [GHB][WP08] Successor activation and rollback | agent-logic-runtime |
| GHB-WP09 | 4 | [GHB][WP09] Live GHB resident loop and Gate A | agent-logic-runtime |
| GHB-WP10 | 4 | [GHB][WP10] Bounded recursive improvement | agent-logic-runtime |
| GHB-WP11 | 4 | [GHB][WP11] Longitudinal qualification, fleet adoption and Gate B | agent-logic-runtime |
| GHB-WP12 | 4 | [GHB][WP12] Research evidence and operator documentation | agent-logic-runtime |
| INTEGRATE | 5 | Integrate governance with security and plugin operations | agent-logic-runtime |
| QUALIFY | 5 | Independently qualify the installed milestone | agent-logic-runtime |
| TAIL-01 | 5 | Quality gate | agent-design-language |
| TAIL-02 | 5 | Documentation review and external-review handoff | agent-design-language |
| TAIL-03 | 5 | Publication finalization | agent-design-language |
| TAIL-04 | 5 | Internal milestone review | agent-design-language |
| TAIL-05 | 5 | External or third-party review | agent-design-language |
| TAIL-06 | 5 | Accepted-findings remediation or explicit deferral capture | agent-design-language |
| TAIL-07 | 5 | Next-milestone planning | agent-design-language |
| TAIL-08 | 5 | Next-milestone closeout planning | agent-design-language |
| TAIL-09 | 5 | Next-milestone planning review | agent-design-language |
| TAIL-10 | 5 | Release ceremony and milestone close | agent-design-language |
| CF-92 | 5 | Deploy and launch the qualified CodeFriend service | codefriend |
| PY-01 | Deferred | Retained for a later milestone; excluded from the 67 active identities | agent-design-language |

## Work Packages

Each row is a single outcome; supporting documentation and tests stay with that outcome. WP-01, INTEGRATE, QUALIFY and TAIL-01–TAIL-10 apply to this release separately from v0.93.1.

## Sequencing

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated. Runtime → governance/security → citizen continuity/demos → integration/qualification → release tail.

## Sequencing Notes

Same-sprint dependencies remain ordered; sprint membership is not authority to skip prerequisites. RV-01 produces design-review proof, not downstream implementation proof.

## Acceptance Mapping

RV-08 qualifies installed plugin behavior; DEMO-GOV/DEMO-SEC demonstrate integrated governance/security; CM-04 qualifies continuity. QUALIFY consumes all results and proves qualified ready-to-deploy CodeFriend compatibility.

## Exit Criteria

67 scoped results are mapped once; sidecars and explicit deferrals remain visible separately.

## v0.93.2 scope integration

The milestone retains the existing platform tracks and adds CodeFriend plus the approved full GHB program. CodeFriend public launch is routed through existing CodeFriend #92 and still requires exact deployment authorization. GHB requires Gate A, Gate B, and WP12 before milestone qualification; planning approval does not authorize experiments, provider spend, deployment, or issue creation. Runtime-v4 activation/generation and citizen-continuity/CM mechanisms are shared dependencies for GHB-WP07/WP08, not duplicate engines, schedulers, or transaction owners. ADL #1226 remains a deferred assessment.

## CodeFriend launch allocation

`CF-92` reuses existing CodeFriend #92 with predecessors ADL #1150, CodeFriend #45/#46, and website #21. Acceptance requires an explicitly authorized exact candidate, audience, environment and budget, followed by public-journey and rollback evidence. Planning scope is not deployment authority.

## #1242 scope reconciliation

Reuse merged #1259; 67 planned identities retain the full approved scope except PY-01. Its complete original definition is preserved in `tail07-1242/SUCCESSOR_PLAN.json`. Affect/humour remain outside v0.93.2; no later milestone is activated here. #1241 final dispositions and selected installed lockset correspondence were accepted in #1245; see [opening record](OPENING_1296.md); source retirement #1223 is merged, not a public deployment.
