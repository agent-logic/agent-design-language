# [v0.92.2][SIM-05] Rebuild six card projections from the semantic issue record

## Outcome

An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.

## Dependencies and execution boundary

- Depends on SIM-04's merged application/transaction owner and semantic record.
- Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer.
- Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation.

## Current source and bounded implementation

- Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation).
- Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`.
- Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources.
- Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents.

## Acceptance and executed proof

1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged.
2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner.
3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table.
4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation.
5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions.
6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout.
7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task.

## Validation / PVF

Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer.

## Stop conditions and non-goals

Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot.
