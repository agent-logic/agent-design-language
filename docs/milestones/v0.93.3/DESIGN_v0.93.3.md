# v0.93.3 Design

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.3. Date: 2026-10-30. Setup issue: [#1403](https://github.com/agent-logic/agent-design-language/issues/1403). Planning owner: Planning #12. Implementation owners assigned at sprint activation.

## Purpose

Runtime v4 foundations. 15 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. All RV-01–RV-11 are retained. #1352 stays here as a reused carryover. Governance, GHB, affect and humour are outside this milestone.

## Problem Statement

The former 59-identity platform milestone combines independent foundation, tooling and cognitive programmes. Separate them without losing contracts, historical identity or acceptance.

## Goals

Runtime v4 foundations. 15 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. All RV-01–RV-11 are retained. #1352 stays here as a reused carryover. Governance, GHB, affect and humour are outside this milestone.

## Non-Goals

No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized by this planning package. Create child issues/cards per sprint at activation; preserve native C-SDLC v3 authority until a separately accepted compatibility cutover. Component container packaging and other unadmitted TBD concepts remain deferred.

## Scope

Runtime v4 foundations. 15 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. All RV-01–RV-11 are retained. #1352 stays here as a reused carryover. Governance, GHB, affect and humour are outside this milestone.

## Requirements

| WP | Acceptance / proof | Adverse cases |
| --- | --- | --- |
| WP-01 | Accepted v0.93.2 interfaces and evidence are named; Runtime-only scope and owners are explicit.; No platform, C-SDLC v4, deployment or spend authorization inferred from setup. | Unaccepted predecessor; Missing opening approval; Duplicate issue identities |
| RV-01 | An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes.; Independent design review resolves actionable findings against the versioned plugin contract and explicit success, refusal and recovery scenarios. Implementation and installed-consumer proof are delivered by RV-02 through RV-11, not required to admit RV-02. | Manifest self-grants; Undefined crash recovery |
| RV-02 | Existing Runtime behavior and identity survive native adapter conversion with no second scheduler.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Behavior parity regression |
| RV-03 | The durable generation transaction stages, quiesces, activates and rolls back to one fenced generation after interruption; in-flight work and cancellation retain their declared disposition.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Interrupted activation loses owner; Stale generation accepts work; Rollback duplicates in-flight delivery |
| RV-09 | An installed plugin migrates declared prior-version state to the selected version and preserves the prior generation when the migrator refuses or fails.; Exact versioned input/output evidence and positive/negative scenario counts are retained; review resolves actionable findings. | Unsupported state version; Migrator failure mutates prior state; Retry repeats non-idempotent migration |
| RV-10 | Versioned configuration migration and two-phase validation/activation apply one accepted configuration, or preserve the prior version and active configuration on refusal, failure or interruption.; Exact versioned input/output evidence and positive/negative scenario counts are retained; review resolves actionable findings. | Invalid configuration activated; Interrupted apply loses known-good configuration; Credential scope changes silently; Unsupported configuration version; Configuration migrator failure loses prior version |
| RV-11 | A requested plugin removal checks dependencies and in-flight work, refuses unsafe removal, and completes authorized removal with durable recovery state.; Exact versioned input/output evidence and positive/negative scenario counts are retained; review resolves actionable findings. | Live dependent orphaned; Unsafe removal accepted; Interrupted removal resurrects stale generation |
| RV-04 | One real non-core plugin uses authenticated bounded transport, deadlines and process supervision.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Orphan process; Hung handler; Unbounded queue |
| RV-05 | The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Unapproved import; Timeout bypass |
| RV-06 | One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Adapter-specific authority; State identity drift |
| RV-07 | Provider instances consume host-owned credentials and versioned configuration through the common inference capability.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings. | Credential in manifest; Implicit provider fallback |
| RV-08 | Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback.; Installed consumer evidence identifies exact source/artifact versions and actual scenario counts; review resolves actionable findings.; Preserve the qualified ready-to-deploy CodeFriend API/artifact contract and demonstrate compatible upgrade and known-good rollback. | Bad signature; Incompatible checkpoint; Rollback cannot restore |
| ADL-1352 | Complete the existing #1352 language-limit/interpreter-free public-delivery acceptance without reopening delivered cleanup. | Hidden interpreter dependency; Language-policy exception without explicit disposition |
| FOUNDATION-REVIEW | One independent integrated review accepts the exact Runtime foundation lockset and carryover; reuse valid tests and qualification instead of repeating them. | Missing predecessor acceptance; Unsupported completion claim |
| FOUNDATION-CLOSE | Record accepted foundation interfaces, compatibility, recovery limits and successor handoff; release/tag only under a separate scoped decision. | Missing predecessor acceptance; Unsupported completion claim |

## Proposed Design

One owner for each existing execution/state authority. Preserve plugin and consumer compatibility; no duplicate schedulers, citizen identity engines or model/provider routes.

| WP | Sprint | Repository | Outcome | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Admit only the reviewed Runtime v4 foundations after accepted v0.93.2 inputs; assign the first bounded sprint without activating successor programmes. | v0.93.2/TAIL-10 |
| RV-01 | 1 | agent-logic-runtime | An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes. | v0.93.1/RD-11, WP-01 |
| RV-02 | 2 | agent-logic-runtime | Existing Runtime behavior and identity survive native adapter conversion with no second scheduler. | RV-01 |
| RV-03 | 2 | agent-logic-runtime | The durable generation transaction stages, quiesces, activates and rolls back to one fenced generation after interruption; in-flight work and cancellation retain their declared disposition. | RV-02 |
| RV-09 | 2 | agent-logic-runtime | An installed plugin migrates declared prior-version state to the selected version and preserves the prior generation when the migrator refuses or fails. | RV-03 |
| RV-10 | 2 | agent-logic-runtime | Versioned configuration migration and two-phase validation/activation apply one accepted configuration, or preserve the prior version and active configuration on refusal, failure or interruption. | RV-09 |
| RV-11 | 2 | agent-logic-runtime | A requested plugin removal checks dependencies and in-flight work, refuses unsafe removal, and completes authorized removal with durable recovery state. | RV-10 |
| RV-04 | 3 | agent-logic-runtime | One real non-core plugin uses authenticated bounded transport, deadlines and process supervision. | RV-11 |
| RV-05 | 3 | agent-logic-runtime | The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits. | RV-04 |
| RV-06 | 3 | agent-logic-runtime | One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters. | RV-05 |
| RV-07 | 3 | agent-logic-runtime | Provider instances consume host-owned credentials and versioned configuration through the common inference capability. | RV-06 |
| RV-08 | 3 | agent-logic-runtime | Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback. | RV-07 |
| ADL-1352 | 1 | agent-design-language | Complete the existing #1352 language-limit/interpreter-free public-delivery acceptance without reopening delivered cleanup. | WP-01 |
| FOUNDATION-REVIEW | 3 | agent-design-language | One independent integrated review accepts the exact Runtime foundation lockset and carryover; reuse valid tests and qualification instead of repeating them. | RV-08, ADL-1352 |
| FOUNDATION-CLOSE | 3 | agent-design-language | Record accepted foundation interfaces, compatibility, recovery limits and successor handoff; release/tag only under a separate scoped decision. | FOUNDATION-REVIEW |

## Interfaces And Contracts

Runtime foundations export accepted versioned plugin/lifecycle/state/configuration interfaces. C-SDLC exports an accepted v3-compatible workflow/cutover contract. Cognitive consumers preserve identity, consent, privacy and existing resource/governance checks.

## Validation Plan

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
