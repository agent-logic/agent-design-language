# Agent Logic repositories

This map describes ownership after the repository split. Repository access may
require authorization. An ownership entry is not a release, deployment or installed
qualification claim. Each repository owns its current setup and validation guide.

| Repository | Responsibility |
| --- | --- |
| [agent-design-language](https://github.com/agent-logic/agent-design-language) | Public ADL language, contracts and CLI; coordinates the split. |
| [cognitive-sdlc](https://github.com/agent-logic/cognitive-sdlc) | Native issue lifecycle, package admission, review/publication and terminal reconciliation. |
| [agent-logic-runtime](https://github.com/agent-logic/agent-logic-runtime) | Runtime kernel, execution, Guardian, CSM and Observatory; versioned shared product library. |
| [universal-tool-schema](https://github.com/agent-logic/universal-tool-schema) | Provider-neutral descriptive tool schema; validity does not grant execution authority. |
| [codefriend](https://github.com/agent-logic/codefriend) | CodeFriend product CLI, agent and server; consumes qualified shared-library artifacts. |
| [agent-logic-infrastructure](https://github.com/agent-logic/agent-logic-infrastructure) | Cloud, build and deployment infrastructure; consumes reviewed product artifacts. |
| [agent-logic-enterprise-security](https://github.com/agent-logic/agent-logic-enterprise-security) | Optional enterprise policy adapter; Runtime retains enforcement authority. |
| [the-cognitive-stack](https://github.com/agent-logic/the-cognitive-stack) | Private podcast editorial, production, media and feed sources; existing public feed URL and publication approvals remain unchanged. |
| [agent-logic.ai](https://github.com/agent-logic/agent-logic.ai) | Agent Logic website and its infrastructure. |
| [codefriend.ai](https://github.com/agent-logic/codefriend.ai) | CodeFriend website and site-specific services; separate from the product repository. |
| [strategic-cognitive-reserve](https://github.com/agent-logic/strategic-cognitive-reserve) | Open-weight asset preservation, inventory and continuity operations. |
| [general-intelligence-paper-private](https://github.com/agent-logic/general-intelligence-paper-private) | General intelligence manuscript, citations, figures and review records. |
| [godel-hadamard-bayes-paper](https://github.com/agent-logic/godel-hadamard-bayes-paper) | Gödel-Hadamard-Bayes manuscript and supporting research artifacts. |
| [cognitive-sdlc-paper](https://github.com/agent-logic/cognitive-sdlc-paper) | Cognitive software development lifecycle manuscript and supporting research artifacts. |

## Dependency and workflow boundaries

Public ADL builds its retained public packages. Private owners build and qualify
their own products. A consumer uses the reviewed versioned package/interface and
its authenticated identity; sibling checkouts and mutable latest artifacts are
not substitutes. Runtime produces the shared product library and policy interface;
CodeFriend and Enterprise Security document their respective acquisition contracts.

Cognitive-SDLC supplies the lifecycle executable and matching manuals. Use the
exact executable admitted for the target repository. A source checkout, downloaded
ZIP or successful manual lookup alone establishes no operational authority.
See [package manual lookup](csdlc-v3/man/README.md).

Websites and papers have their own deployment/publication boundaries. Reading or
editing their documentation grants no deployment, data collection, model download,
cloud spend or manuscript submission authority.

The [#1269 refresh](https://github.com/agent-logic/agent-design-language/issues/1269)
tracks per-repository documentation changes and no-change decisions. Historical
ADL milestones, ADRs and custody records remain evidence of their original state.

The [document inventory](repository-split/DOCUMENTATION_INVENTORY_1269.md) lists
the primary documents and related candidates by reviewed repository revision.
It is an audit scope, not a claim that every candidate needs a change.

See the [delivery checklist](repository-split/DOCUMENTATION_DELIVERY_1269.md) for
all 13 PRs, retained-document decisions, validation and adapter follow-up.

## Independent evaluation and new-owner planning boundary

[ATEC](https://github.com/agent-logic/atec) and [AVP](https://github.com/agent-logic/avp) have independent evaluation/proof work; their names here establish neither delivered qualification nor inclusion in ADL MVP. The v0.95 D03 decision must verify the selected repository/interface/revision and evidence or explicitly record non-consumption. UTS owns its descriptive specification and SDKs; Runtime retains ACC execution/authorization and benchmark ownership. Read current owner release/adoption evidence before asserting UTS1.1 or Python/TypeScript SDK readiness. No new grants, binary activation or product publication follows from this owner map.
