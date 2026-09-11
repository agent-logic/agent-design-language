# [v0.92.2][CF-RENDER-HTML] Render an approved review as HTML

## One complete result

The HTML renderer consumes the governed publication contract and produces a navigable report with traceable findings and the same claims as Markdown.

Dependencies: CF-RENDER-MD. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `export html` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/html.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_html.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Render actual governed report inputs into a navigable local HTML report and manifest with the same complete claims, stable finding identities, citations, uncertainty, disagreements and action plans as the approved Markdown baseline. Execute semantic claim-set parity rather than comparing only layout snapshots.
2. Open the report in a browser and verify contents navigation, finding/evidence links, long content and readable keyboard-accessible structure. Escape hostile repository markup, attributes and script-like payloads; no untrusted script, remote asset fetch or automatic action may arise from report text.
3. Recheck exact approval, renderer/target identity, redaction and provenance before output. Reject unapproved/stale/tampered input, unsafe links/HTML, leaked credentials and claim drift. Record withheld/denied states without emitting a falsely approved report.
4. Retain actual browser/output evidence and bound manifest. A static mock, template or successful string serialization alone cannot close this renderer; public hosting is excluded.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `html_report_opened`, `markdown_claim_parity`, `manifest_binding`, `output_parity`.

pvf: `html_report_opened`, `markdown_claim_parity`, `manifest_binding`, `unsafe_markup_escaped`, `unapproved_render_denied`, `renderer_snapshot`, `redaction_recheck`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `renderer_claim_drift`, `unsafe_html_emitted`, `redaction_check_failed`, `approval_missing_or_stale`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.
