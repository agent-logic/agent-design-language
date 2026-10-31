# v0.93.3 Quality Gate

## Metadata

Planning template set: 1.1.0. Milestone: v0.93.3. Date: 2026-10-30. Setup amendment: [#1405](https://github.com/agent-logic/agent-design-language/issues/1405). Planning owner: Planning #12; implementation owners assigned at sprint activation.

## Status

Setup prepared; implementation not started. Planning only. No child implementation, provider/cloud spend, deployment, Runtime restart, shared-binary replacement, tag or release is authorized. Create child issues/cards per activated sprint; current authenticated C-SDLC remains authority until separately accepted cutover.

## Scope and acceptance

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

## Validation

PVF planning_contract for this setup: registry headings, JSON/YAML mirrors, source/link validity, complete allocation and acyclic qualified dependencies. Product work must later prove each acceptance and negative case through the owning installed consumer; schema, source or CI alone is not live qualification. Reuse unchanged exact-input evidence; no broad local Rust or paid qualification campaign for planning.
