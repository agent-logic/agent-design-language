# v0.93.2 Work Breakdown Structure

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Planned and unopened. The operator approved the scope split; this package does not approve execution, provider spend, publication or release.

## How To Use

Use version-qualified IDs. The execution graph owns exact dependencies and specifications own acceptance.

## WBS Summary

53 planned candidates: 40 inherited implementation/demo tasks plus 13 version-specific opening, integration, qualification and release-tail tasks. Existing #875 is a separately gated v0.93.1 sidecar and is not counted in 53. Podcast #671 is completed in v0.92.2 (PR #1174).

## Candidate WP Sequence

| ID | Sprint | Single result | Repository |
|---|---|---|---|
| WP-01 | 1 | Open the reviewed platform milestone from accepted v0.93.1 handoff | agent-design-language |
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
| PY-01 | 1 | Complete the bounded Python reduction tranche | agent-design-language |
| DEMO-GOV | 3 | Run integrated governance demonstrations | agent-logic-runtime |
| DEMO-SEC | 3 | Run integrated security demonstrations | agent-logic-enterprise-security |
| INTEGRATE | 4 | Integrate governance with security and plugin operations | agent-logic-runtime |
| QUALIFY | 4 | Independently qualify the installed milestone | agent-logic-runtime |
| TAIL-01 | 4 | Quality gate | agent-design-language |
| TAIL-02 | 4 | Documentation review and external-review handoff | agent-design-language |
| TAIL-03 | 4 | Publication finalization | agent-design-language |
| TAIL-04 | 4 | Internal milestone review | agent-design-language |
| TAIL-05 | 4 | External or third-party review | agent-design-language |
| TAIL-06 | 4 | Accepted-findings remediation or explicit deferral capture | agent-design-language |
| TAIL-07 | 4 | Next-milestone planning | agent-design-language |
| TAIL-08 | 4 | Next-milestone closeout planning | agent-design-language |
| TAIL-09 | 4 | Next-milestone planning review | agent-design-language |
| TAIL-10 | 4 | Release ceremony and milestone close | agent-design-language |

## Work Packages

Each row is a single outcome; supporting documentation and tests stay with that outcome. WP-01, INTEGRATE, QUALIFY and TAIL-01–TAIL-10 apply to this release separately from v0.93.1.

## Sequencing

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated. Runtime → governance/security → citizen continuity/demos → integration/qualification → release tail.

## Sequencing Notes

Same-sprint dependencies remain ordered; sprint membership is not authority to skip prerequisites. RV-01 produces design-review proof, not downstream implementation proof.

## Acceptance Mapping

RV-08 qualifies installed plugin behavior; DEMO-GOV/DEMO-SEC demonstrate integrated governance/security; CM-04 qualifies continuity. QUALIFY consumes all results and proves launched CodeFriend compatibility.

## Exit Criteria

53 scoped results are mapped once; sidecars and explicit deferrals remain visible separately.
