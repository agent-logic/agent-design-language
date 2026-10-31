# v0.93.3 Release Notes

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.3. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## How To Use

Read the WBS and execution graph together; qualified milestone/ID pairs are identities. Preserve dependencies and adverse cases; reuse existing owner issues. Plan review does not establish product proof.

## Summary

Runtime v4 foundations. 15 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. All RV-01–RV-11 are retained. #1352 stays here as a reused carryover. Governance, GHB, affect and humour are outside this milestone. Hand off accepted Runtime foundations and #1352 to [v0.93.4](../v0.93.4/README.md); Runtime interfaces remain the product baseline.

## Highlights

Runtime v4 foundations. 15 planned work-package identities across 3 dependency waves. Counts include coordination/review/release gates; no one-PR-per-row requirement. All RV-01–RV-11 are retained. #1352 stays here as a reused carryover. Governance, GHB, affect and humour are outside this milestone. Hand off accepted Runtime foundations and #1352 to [v0.93.4](../v0.93.4/README.md); Runtime interfaces remain the product baseline.

## What's New In Detail

Planned scope only:

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

## Upgrade Notes

No released implementation is produced here. Later delivery must document supported versions, migration/refusal paths and known-good rollback.

## Known Limitations

Setup prepared; implementation not started. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## Validation Notes

This setup uses focused planning validation; it does not claim Runtime/tooling/cognitive behavior.

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.

## What's Next

[Successor handoff](NEXT_MILESTONE_HANDOFF_v0.93.3.md).

## Exit Criteria

Every assigned outcome and required adverse case is accepted at exact input/candidate identities; review findings are resolved or explicitly dispositioned. No source/CI-only substitution for installed/live proof. Administrative cleanup is optional and asynchronous.
