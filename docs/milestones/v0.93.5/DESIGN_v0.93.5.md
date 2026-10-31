# v0.93.5 Design

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.5. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Purpose

GHB. 17 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off accepted GHB interfaces to [governance v0.93.6](../v0.93.6/README.md) and later [affect/humour v0.93.7](../v0.93.7/README.md); neither is a GHB baseline dependency.

## Problem Statement

The former 59-identity platform milestone combines independent foundation, tooling and cognitive programmes. Separate them without losing contracts, historical identity or acceptance.

## Goals

GHB. 17 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off accepted GHB interfaces to [governance v0.93.6](../v0.93.6/README.md) and later [affect/humour v0.93.7](../v0.93.7/README.md); neither is a GHB baseline dependency.

## Non-Goals

Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover. Component container packaging and other unadmitted TBD concepts remain deferred.

## Scope

GHB. 17 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. Hand off accepted GHB interfaces to [governance v0.93.6](../v0.93.6/README.md) and later [affect/humour v0.93.7](../v0.93.7/README.md); neither is a GHB baseline dependency.

## Requirements

| WP | Acceptance / proof | Adverse cases |
| --- | --- | --- |
| WP-01 | Accept all required v0.93.4 foundation and compatibility outcomes before GHB activation; broader governance and affect/humour remain later. | Missing predecessor acceptance; Unsupported completion claim |
| GHB-SPRINT | The GHB roster, ownership, scope, budgets-to-approve, dependency order, and separate Gate A/Gate B acceptance are reviewable before child execution. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP00 | source-to-caller matrix at pinned revisions; no duplicated owner planned; runnable unchanged-agent baseline; measurable Gate A scenario; approved evaluation protocol and task/data licenses. A missing existing owner is an explicit gap rather than assumed absence. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP01 | every transition has inputs, outputs, authority, side effects and stop conditions; fresh inference versus replay is explicit; historical docs link to the normative successor after review; conformance labels remain proposed until accepted. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP02 | canonical digest/round-trip tests, malformed and stale-reference rejection, schema examples, installed Runtime compatibility test, and a migration policy for existing artifacts. New tests are classified in the validation inventory when authored. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP03 | analytically known posterior fixtures; zero/impossible likelihood handling; correlated evidence cannot be silently double-counted; replay reproduces updates; informative experiments can differ from high-utility candidates; sparse evidence defers promotion. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP04 | candidate changes match the declared mutation class; exact parent and proposal provenance retained; archive cannot change production head; budget exhaustion terminates cleanly; at least two distinct useful candidate strategies are exercised. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP05 | denied network/filesystem/capability escapes; killed and timed-out experiments leave no production changes; exact candidate digest in results; measured resource accounting; controlled-adapter and live-provider execution classifications remain distinct. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP06 | these malicious/invalid candidates fail; raw outcomes can be independently recomputed; unseen final tasks are excluded from candidate context; thresholds declared before trials; non-improvement is a legitimate terminal outcome. Promotion recommendation cannot itself activate a body. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP07 | Runtime #112's actual-resident positive/negative criteria pass without test-support authority substitutes; no implicit birthday on startup; one birthday under retry/concurrency/interruption; authenticated restored resident performs a useful task. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP08 | failpoints at durable boundaries; stale parents and concurrent conflicting activations reject; crash recovery yields one authoritative head; accepted successor is actually loaded; rollback restores compatible execution without erasing history or replaying external effects. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP09 | full Gate A scenario from a clean installed Runtime environment with no ADL source checkout or development binaries; separate controlled-adapter regression and live-provider evidence; successor handles later tasks; replay performs no external mutations; two competing cycles cannot both activate against one parent. Export an exact-version evidence packet. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP10 | at least one admitted successor changes improvement behavior and executes the next cycle using that change; independent tests verify the change did not modify protected evaluation or authority rules. If no improvement emerges within budget, report the capability as implemented but empirical improvement unproven; do not manufacture acceptance. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP11 | qualification matrix names each tested domain/configuration; one resident reaches Gate B with useful post-restore continuation; every intended fleet profile is either qualified, pending or excluded with a reason; no blanket claim that all agents qualify from one demonstration. Rollout authorization is separate from local qualification. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP12 | claim-to-evidence ledger; reproducible benchmark instructions; installed command/manual parity where applicable; no unsupported theorem, novelty, personhood or general-performance claim. Scholarly publication is outside this plan's authorization. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-REVIEW | Independently accept the complete installed GHB baseline and its required continuity/standing proof, reusing unchanged input evidence. | Missing predecessor acceptance; Unsupported completion claim |
| GHB-CLOSE | Record the exact accepted GHB/continuity interfaces and limitations for v0.93.6. Any release remains separately authorized. | Missing predecessor acceptance; Unsupported completion claim |

## Proposed Design

One owner for each existing execution/state authority. Preserve plugin and consumer compatibility; no duplicate schedulers, citizen identity engines or model/provider routes.

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Consume accepted Runtime v4 and v0.93.4 tooling/governance/continuity inputs, then activate only the reviewed GHB sprint. | v0.93.3/FOUNDATION-CLOSE, v0.93.4/RELEASE |
| GHB-SPRINT | 1 | agent-logic-runtime | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. | WP-01, v0.93.3/RV-08 |
| GHB-WP00 | 1 | agent-logic-runtime | Owner: program + Runtime. Dependencies: none. | GHB-SPRINT |
| GHB-WP01 | 1 | agent-design-language | Owner: ADL. Dependencies: WP00. | GHB-SPRINT, GHB-WP00 |
| GHB-WP02 | 1 | agent-design-language | Owner: ADL. Dependencies: WP01. | GHB-SPRINT, GHB-WP01 |
| GHB-WP03 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP04 | 2 | agent-logic-runtime | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. | GHB-SPRINT, GHB-WP02, GHB-WP03 |
| GHB-WP05 | 2 | agent-logic-runtime | Owner: Runtime/execution. Dependencies: WP02. | GHB-SPRINT, GHB-WP02 |
| GHB-WP06 | 2 | agent-logic-runtime | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. | GHB-SPRINT, GHB-WP00, GHB-WP02, GHB-WP03, GHB-WP05 |
| GHB-WP07 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. | GHB-SPRINT, GHB-WP00, v0.93.3/RV-09, v0.93.3/RV-10, v0.93.4/CM-02 |
| GHB-WP08 | 2 | agent-logic-runtime | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. | GHB-SPRINT, GHB-WP02, GHB-WP05, GHB-WP06, GHB-WP07, v0.93.3/RV-11, v0.93.4/CM-03, v0.93.4/CM-04 |
| GHB-WP09 | 3 | agent-logic-runtime | Owner: Runtime resident integration. Dependencies: WP03–WP08. | GHB-SPRINT, GHB-WP03, GHB-WP04, GHB-WP05, GHB-WP06, GHB-WP07, GHB-WP08 |
| GHB-WP10 | 3 | agent-logic-runtime | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. | GHB-SPRINT, GHB-WP09 |
| GHB-WP11 | 3 | agent-logic-runtime | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. | GHB-SPRINT, GHB-WP10 |
| GHB-WP12 | 3 | agent-logic-runtime | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. | GHB-SPRINT, GHB-WP01, GHB-WP11 |
| GHB-REVIEW | 3 | agent-design-language | Independently accept the complete installed GHB baseline and its required continuity/standing proof, reusing unchanged input evidence. | GHB-WP12, v0.93.4/CM-04 |
| GHB-CLOSE | 3 | agent-design-language | Record the exact accepted GHB interfaces and limitations for v0.93.6 governance and v0.93.7 affect/humour. Any release remains separately authorized. | GHB-REVIEW |

## Interfaces And Contracts

Runtime foundations export accepted versioned plugin/lifecycle/state/configuration interfaces. C-SDLC exports an accepted v3-compatible workflow/cutover contract. Cognitive consumers preserve identity, consent, privacy and existing resource/governance checks.

## Validation Plan

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
