# v0.93 Planned Issue Catalog

Status: first-pass candidates; no execution issues have been created. IDs are logical identifiers, not GitHub numbers.

| ID | Sprint | One completed result | Repository | Dependencies |
|---|---|---|---|---|
| WP-01 | 1 | Open the reviewed v0.93 execution wave | agent-design-language | Opening authorization |
| RD-01 | 1 | Refresh final-head ownership audit | agent-design-language | WP-01 |
| RD-02 | 1 | Accept repository boundaries and cutover contracts | agent-design-language | RD-01 |
| RD-12 | 1 | Bootstrap the approved destination repositories | agent-design-language | RD-02 |
| RD-13 | 1 | Materialize destination issue identities | agent-design-language | RD-12 |
| RD-03 | 1 | Distribute portable public ADL contracts | agent-design-language | RD-13 |
| RD-04 | 1 | Extract and qualify C-SDLC | cognitive-sdlc | RD-13 |
| RD-05 | 1 | Extract and qualify Runtime with Observatory | agent-logic-runtime | RD-03, RD-04 |
| RD-06 | 1 | Extract artifact-consuming infrastructure | agent-logic-infrastructure | RD-05 |
| RD-09 | 1 | Establish enterprise-security producer boundary | agent-logic-enterprise-security | RD-05 |
| RD-10 | 1 | Extract CodeFriend software and preserve its website | codefriend | RD-05 |
| RD-08 | 1 | Qualify demo ownership after product moves | agent-logic-runtime | RD-06, RD-09, RD-10 |
| RD-07 | 1 | Retire superseded source with verified public ADL | agent-design-language | RD-08 |
| RD-11 | 1 | Accept the complete split and release lockset | agent-design-language | RD-07 |
| RV-01 | 2 | Resolve plugin lifecycle design findings | agent-logic-runtime | RD-11 |
| RV-02 | 2 | Adapt built-in components to the native plugin contract | agent-logic-runtime | RV-01 |
| RV-03 | 2 | Implement recoverable plugin generation transitions | agent-logic-runtime | RV-02 |
| RV-09 | 2 | Migrate versioned plugin state | agent-logic-runtime | RV-03 |
| RV-10 | 2 | Reconfigure a plugin atomically | agent-logic-runtime | RV-09 |
| RV-11 | 2 | Remove a plugin without breaking dependents | agent-logic-runtime | RV-10 |
| RV-04 | 2 | Run a governed process plugin | agent-logic-runtime | RV-11 |
| RV-05 | 2 | Run a bounded WASM component plugin | agent-logic-runtime | RV-04 |
| RV-06 | 2 | Prove cross-adapter module equivalence | agent-logic-runtime | RV-05 |
| RV-07 | 2 | Move provider instances to plugin configuration | agent-logic-runtime | RV-06 |
| RV-08 | 2 | Qualify installed plugin recovery and operations | agent-logic-runtime | RV-07 |
| CF-01 | 2 | Accept the Beta 1 to launch requirements mapping | codefriend | RD-11 |
| CF-02 | 3 | Deliver a deployable CodeFriend product | codefriend | CF-01, RV-08 |
| CF-03 | 3 | Complete external tester onboarding | codefriend | CF-02 |
| CF-04 | 3 | Qualify launch operations and recovery | codefriend | CF-03 |
| CF-05 | 4 | Qualify the external beta launch candidate | codefriend | CF-04, CT-05 |
| CF-06 | 4 | Prepare the website to hand off to the qualified product | codefriend.ai | CF-05 |
| CF-07 | 4 | Launch the qualified CodeFriend service | codefriend | CF-06 |
| CT-01 | 2 | Accept the complete artifact template catalog | codefriend | CF-01 |
| CT-02 | 2 | Release the common branded template foundation | codefriend | CT-01 |
| CT-09 | 2 | Release the test template family | codefriend | CT-02 |
| CT-08 | 2 | Release the diagram template family | codefriend | CT-02 |
| CT-07 | 2 | Release the review template family | codefriend | CT-02 |
| CT-06 | 2 | Release the document template family | codefriend | CT-02 |
| CT-03 | 3 | Prepare artifact content through named fields | codefriend | CT-06, CT-07, CT-08, CT-09, CF-02 |
| CT-10 | 3 | Preview and export branded artifacts | codefriend | CT-03 |
| CT-04 | 4 | Preserve artifact compatibility across template upgrades | codefriend | CT-10 |
| CT-05 | 4 | Qualify the complete branded artifact catalog | codefriend | CT-04 |
| GOV-01 | 5 | Enforce citizen and guest actor boundaries | agent-logic-runtime | RV-08 |
| GOV-02 | 5 | Evaluate trace-grounded rights and duties | agent-logic-runtime | GOV-01 |
| GOV-03 | 5 | Apply evidence-backed standing transitions | agent-logic-runtime | GOV-02 |
| GOV-04 | 5 | Produce constitutional review from evidence | agent-logic-runtime | GOV-03 |
| GOV-05 | 5 | Resolve challenges and appeals | agent-logic-runtime | GOV-04 |
| GOV-06 | 5 | Persist governed private ToM updates | agent-logic-runtime | GOV-01 |
| GOV-07 | 5 | Preserve ToM conflict and temporal decay | agent-logic-runtime | GOV-06 |
| GOV-08 | 5 | Persist governed relationship context | agent-logic-runtime | GOV-06 |
| GOV-09 | 5 | Publish challengeable reputation projections | agent-logic-runtime | GOV-05, GOV-07, GOV-08 |
| GOV-10 | 5 | Consume redacted shared social memory | agent-logic-runtime | GOV-09 |
| GOV-11 | 5 | Enforce delegated action authority | agent-logic-runtime | GOV-03 |
| GOV-12 | 5 | Execute governed upstream cognition | agent-logic-runtime | GOV-11, GOV-07 |
| GOV-13 | 5 | Communicate without private inspection | agent-logic-runtime | GOV-11 |
| GOV-14 | 5 | Evaluate bounded social-contract obligations | agent-logic-runtime | GOV-05, GOV-10, GOV-13 |
| GOV-15 | 5 | Operate governed guild membership and action | agent-logic-runtime | GOV-14, GOV-12 |
| GOV-16 | 5 | Produce redacted polis governance health | agent-logic-runtime | GOV-15 |
| WP-S1 | 5 | Enforce the enterprise zero-trust boundary | agent-logic-enterprise-security | GOV-11, RD-09 |
| WP-S2 | 5 | Enforce per-action and per-message authorization | agent-logic-enterprise-security | WP-S1, GOV-12 |
| WP-S3 | 5 | Enforce key and secret lifecycle | agent-logic-enterprise-security | WP-S2 |
| WP-S4 | 5 | Produce tamper-evident incident evidence | agent-logic-enterprise-security | WP-S3, GOV-05 |
| WP-S5 | 5 | Enforce protected data lifecycle and isolation | agent-logic-enterprise-security | WP-S3, GOV-10 |
| WP-S6 | 5 | Operate a replayable security regression drill | agent-logic-enterprise-security | WP-S4, WP-S5 |
| CM-01 | 6 | Accept citizen migration and descendant identity contracts | agent-logic-runtime | RV-08, GOV-01 |
| CM-02 | 6 | Migrate one citizen without conflicting active identity | agent-logic-runtime | CM-01, WP-S3 |
| CM-03 | 6 | Create a governed descendant with distinct identity | agent-logic-runtime | CM-01, GOV-11, WP-S5 |
| CM-04 | 6 | Qualify citizen migration and reproduction recovery | agent-logic-runtime | CM-02, CM-03 |
| PY-01 | 2 | Complete the bounded Python reduction tranche | agent-design-language | RD-11 |
| DEMO-GOV | 6 | Run integrated governance demonstrations | agent-logic-runtime | GOV-16 |
| DEMO-SEC | 6 | Run integrated security demonstrations | agent-logic-enterprise-security | WP-S6 |
| INTEGRATE | 7 | Integrate governance with security and plugin operations | agent-logic-runtime | DEMO-GOV, DEMO-SEC, CM-04 |
| QUALIFY | 7 | Independently qualify the installed milestone | agent-logic-runtime | INTEGRATE, CF-07, PY-01 |
| TAIL-01 | 8 | Quality gate | agent-design-language | QUALIFY |
| TAIL-02 | 8 | Documentation review and external-review handoff | agent-design-language | TAIL-01 |
| TAIL-03 | 8 | Publication finalization | agent-design-language | TAIL-02 |
| TAIL-04 | 8 | Internal milestone review | agent-design-language | TAIL-03 |
| TAIL-05 | 8 | External or third-party review | agent-design-language | TAIL-04 |
| TAIL-06 | 8 | Accepted-findings remediation or explicit deferral capture | agent-design-language | TAIL-05 |
| TAIL-07 | 8 | Next-milestone planning | agent-design-language | TAIL-06 |
| TAIL-08 | 8 | Next-milestone closeout planning | agent-design-language | TAIL-07 |
| TAIL-09 | 8 | Next-milestone planning review | agent-design-language | TAIL-08 |
| TAIL-10 | 8 | Release ceremony and milestone close | agent-design-language | TAIL-09 |

## Numbered sprint allocation

All 83 candidate results are assigned exactly once across 8 proposed sprints in [the sprint plan](SPRINT_v0.93.md). Sprint 1 completes the repository split before any feature work. Same-sprint dependencies remain ordered; issue scopes and acceptance criteria are unchanged. Existing #875 and #671 are separately gated sidecars, not duplicate candidates.
