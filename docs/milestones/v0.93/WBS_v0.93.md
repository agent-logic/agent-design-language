# v0.93 Work Breakdown Structure

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Status

First-pass planning under #1047. v0.93 is not open; no extraction or feature implementation is authorized by this package.

## How To Use

Use logical IDs below for planning; create actual issues only at authorized opening. EXECUTION_PLAN_v0.93.json owns dependency and completion detail.

## WBS Summary

64 candidate results: 1 opening, 11 migration, 8 Runtime v4, 16 governance, 6 security, 7 CodeFriend launch, 1 Python reduction tranche, 4 integration/qualification and 10 release-tail results. Runtime v4 RV-01 through RV-08 are mandatory; capacity planning must preserve their complete release outcome.

## Candidate WP Sequence

| ID | One completed result | Repository | Dependencies |
|---|---|---|---|
| WP-01 | Open the reviewed v0.93 execution wave | agent-design-language | Opening authorization |
| RD-01 | Refresh final-head ownership audit | agent-design-language | WP-01 |
| RD-02 | Accept repository boundaries and cutover contracts | agent-design-language | RD-01 |
| RD-03 | Distribute portable public ADL contracts | agent-design-language | RD-02 |
| RD-04 | Extract and qualify C-SDLC | cognitive-sdlc | RD-02 |
| RD-05 | Extract and qualify Runtime with Observatory | agent-logic-runtime | RD-03, RD-04 |
| RD-06 | Extract artifact-consuming infrastructure | agent-logic-infrastructure | RD-05 |
| RD-09 | Establish enterprise-security producer boundary | agent-logic-enterprise-security | RD-05 |
| RD-10 | Extract CodeFriend software and preserve its website | codefriend | RD-05 |
| RD-08 | Qualify demo ownership after product moves | agent-logic-runtime | RD-06, RD-09, RD-10 |
| RD-07 | Retire superseded source with verified public ADL | agent-design-language | RD-08 |
| RD-11 | Accept the complete split and release lockset | agent-design-language | RD-07 |
| RV-01 | Resolve plugin lifecycle design findings | agent-logic-runtime | RD-11 |
| RV-02 | Adapt built-in components to the native plugin contract | agent-logic-runtime | RV-01 |
| RV-03 | Implement recoverable plugin generation transitions | agent-logic-runtime | RV-02 |
| RV-04 | Run a governed process plugin | agent-logic-runtime | RV-03 |
| RV-05 | Run a bounded WASM component plugin | agent-logic-runtime | RV-04 |
| RV-06 | Prove cross-adapter module equivalence | agent-logic-runtime | RV-05 |
| RV-07 | Move provider instances to plugin configuration | agent-logic-runtime | RV-06 |
| RV-08 | Qualify installed plugin recovery and operations | agent-logic-runtime | RV-07 |
| CF-01 | Accept the Beta 1 to launch requirements mapping | codefriend | RD-11 |
| CF-02 | Deliver a deployable CodeFriend product | codefriend | CF-01, RV-08 |
| CF-03 | Complete external tester onboarding | codefriend | CF-02 |
| CF-04 | Qualify launch operations and recovery | codefriend | CF-03 |
| CF-05 | Qualify the external beta launch candidate | codefriend | CF-04 |
| CF-06 | Prepare the website to hand off to the qualified product | codefriend.ai | CF-05 |
| CF-07 | Launch the qualified CodeFriend service | codefriend | CF-06 |
| GOV-01 | Enforce citizen and guest actor boundaries | agent-logic-runtime | RV-08 |
| GOV-02 | Evaluate trace-grounded rights and duties | agent-logic-runtime | GOV-01 |
| GOV-03 | Apply evidence-backed standing transitions | agent-logic-runtime | GOV-02 |
| GOV-04 | Produce constitutional review from evidence | agent-logic-runtime | GOV-03 |
| GOV-05 | Resolve challenges and appeals | agent-logic-runtime | GOV-04 |
| GOV-06 | Persist governed private ToM updates | agent-logic-runtime | GOV-01 |
| GOV-07 | Preserve ToM conflict and temporal decay | agent-logic-runtime | GOV-06 |
| GOV-08 | Persist governed relationship context | agent-logic-runtime | GOV-06 |
| GOV-09 | Publish challengeable reputation projections | agent-logic-runtime | GOV-05, GOV-07, GOV-08 |
| GOV-10 | Consume redacted shared social memory | agent-logic-runtime | GOV-09 |
| GOV-11 | Enforce delegated action authority | agent-logic-runtime | GOV-03 |
| GOV-12 | Execute governed upstream cognition | agent-logic-runtime | GOV-11, GOV-07 |
| GOV-13 | Communicate without private inspection | agent-logic-runtime | GOV-11 |
| GOV-14 | Evaluate bounded social-contract obligations | agent-logic-runtime | GOV-05, GOV-10, GOV-13 |
| GOV-15 | Operate governed guild membership and action | agent-logic-runtime | GOV-14, GOV-12 |
| GOV-16 | Produce redacted polis governance health | agent-logic-runtime | GOV-15 |
| WP-S1 | Enforce the enterprise zero-trust boundary | agent-logic-enterprise-security | GOV-11, RD-09 |
| WP-S2 | Enforce per-action and per-message authorization | agent-logic-enterprise-security | WP-S1, GOV-12 |
| WP-S3 | Enforce key and secret lifecycle | agent-logic-enterprise-security | WP-S2 |
| WP-S4 | Produce tamper-evident incident evidence | agent-logic-enterprise-security | WP-S3, GOV-05 |
| WP-S5 | Enforce protected data lifecycle and isolation | agent-logic-enterprise-security | WP-S3, GOV-10 |
| WP-S6 | Operate a replayable security regression drill | agent-logic-enterprise-security | WP-S4, WP-S5 |
| PY-01 | Complete the bounded Python reduction tranche | agent-design-language | RD-11 |
| DEMO-GOV | Run integrated governance demonstrations | agent-logic-runtime | GOV-16 |
| DEMO-SEC | Run integrated security demonstrations | agent-logic-enterprise-security | WP-S6 |
| INTEGRATE | Integrate governance with security and plugin operations | agent-logic-runtime | DEMO-GOV, DEMO-SEC |
| QUALIFY | Independently qualify the installed milestone | agent-logic-runtime | INTEGRATE, CF-07, PY-01 |
| TAIL-01 | Quality gate | agent-design-language | QUALIFY |
| TAIL-02 | Documentation review and external-review handoff | agent-design-language | TAIL-01 |
| TAIL-03 | Publication finalization | agent-design-language | TAIL-02 |
| TAIL-04 | Internal milestone review | agent-design-language | TAIL-03 |
| TAIL-05 | External or third-party review | agent-design-language | TAIL-04 |
| TAIL-06 | Accepted-findings remediation or explicit deferral capture | agent-design-language | TAIL-05 |
| TAIL-07 | Next-milestone planning | agent-design-language | TAIL-06 |
| TAIL-08 | Next-milestone closeout planning | agent-design-language | TAIL-07 |
| TAIL-09 | Next-milestone planning review | agent-design-language | TAIL-08 |
| TAIL-10 | Release ceremony and milestone close | agent-design-language | TAIL-09 |

## Work Packages

[Execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.93.yaml) declare result, consumer proof, negative cases, resources, non-goals and repository for every row. The earlier A–T thematic allocation remains represented by the substantive feature documents; combined security pairs and broad social-cognition rows are replaced by single-result candidates.

## Sequencing

Opening → repository split → accepted Runtime v4 boundary → governance/security → installed integration and independent qualification → TAIL-01 through TAIL-10.

## Sequencing Notes

Only explicit dependency edges block rows; the global RD-11 feature gate also applies. Parallel source edits must have disjoint ownership. No feature starts during extraction. No predecessor milestone number substitutes for evidence of the actual consumed interface.

## Acceptance Mapping

Each feature source maps to executable candidate results and negative cases in the canonical graph. Migration acceptance is independent checkout/product proof; feature acceptance is installed consumer behavior. Decision-only rows explicitly deliver accepted decisions.

## Exit Criteria

Every admitted result has an owner, dependency, completion denominator, proof and non-goal; unresolved scope is decided before issue-wave opening. No implementation closure from docs-only scaffolds.
