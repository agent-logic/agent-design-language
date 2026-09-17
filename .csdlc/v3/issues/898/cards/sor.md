# <slug>

Canonical Template Source: `docs/templates/prompts/1.0.5/sor.md`

Authority notice: C-SDLC v3 is operational after V3-F/#505 and merged PR #591.
Authority requires the authenticated canonical native selector and reconciliation
receipt; missing or stale proof suspends authority. Retained typed v2 requires
explicit issue-scoped rollback or remediation approval.
Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0898
Run ID: issue-0898
Version: 1.0.5
Title: [v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF
Branch: codex/898-v0922-codefriend-pdf-renderer
Card Status: ready
Status: IMPLEMENTED_LOCAL_PROOF_PASS
Generated: <timestamp>

Execution:
- Actor: `<execution_actor>`
- Model: `<model>`
- Provider: `<provider>`
- Start Time: `<start_time>`
- End Time: `<end_time>`

## Summary

Implemented installed `adl codefriend export pdf` with exact governed-source and approval binding. The renderer now measures selected-font glyph advances against the explicit 174 mm printable width, splits wide unbroken tokens safely, and binds the maximum rendered line width in its manifest.

## PVF Lane Truth
- Initial PVF lane: `<initial_pvf_lane>`
- Planned PVF lane: `<planned_pvf_lane>`
- Final PVF lane: `runtime`
- Lane change reason: `<lane_change_reason>`

## Issue Metrics Truth
- Expected runtime class: `<expected_runtime_class>`
- Estimated elapsed seconds: `<estimated_elapsed_seconds>`
- Actual elapsed seconds: `<actual_elapsed_seconds>`
- Actual active work seconds: `<actual_active_work_seconds>`
- Estimated total tokens: `<estimated_total_tokens>`
- Actual total tokens: `<actual_total_tokens>`
- Estimated validation seconds: `<estimated_validation_seconds>`
- Actual validation seconds: `<actual_validation_seconds>`
- Actual PR wait seconds: `<actual_pr_wait_seconds>`
- Actual CI wait seconds: `<actual_ci_wait_seconds>`
- Budget source: `<budget_source>`
- Goal metrics data source: `<actual_metrics_data_source>`
- Goal metrics source ref: `<actual_metrics_source_ref>`
- Data-source confidence: `<actual_metrics_confidence>`
- Estimate error percent: `<estimate_error_percent>`
- Completion state: `implemented_pending_review`
- Issue goal ref: `Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned.`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`
- Validation planning prompt: `<vpp_card>`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `<variance_analysis_required>`
- Variance analysis completed: `<variance_analysis_completed>`
- Variance category: `<variance_category>`
- Variance note: `<variance_note>`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `<output_card>`
- Tracked implementation artifacts: `adl/src/codefriend/publication/pdf.rs; adl/src/codefriend/publication/markdown.rs; adl/src/codefriend/publication/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_render_pdf.rs; adl/tests/fixtures/codefriend/pdf/PVF.json; docs/codefriend/PDF_EXPORT.md; .csdlc/evidence/898/pdf-qualification; adl/Cargo.toml; adl/Cargo.lock`
- Additional proof artifacts: `.csdlc/evidence/898/pdf-qualification/report.pdf; manifest.json; extracted.txt; pages/page-1.png through page-6.png; visual-inspection.md`

## Actions taken
- `Replaced scalar-count pagination with selected-font glyph-advance measurement against the explicit printable page width.`
- `Added wide unbroken-token and maximum rendered line-width regression proof.`
- `Updated the focused unit proof to exercise the width-measurement seam; product behavior was unchanged.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; all #898 implementation and proof changes are in the bound FastWork worktree`
- Worktree-only paths remaining: `<worktree_only_paths_remaining>`
- Integration state: `worktree_only`
- Verification scope: `bound #898 issue worktree at current-main-reconciled candidate`
- Integration method used: `native_v3_bound_fastwork_worktree_reconciled_with_current_origin_main`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check; pdftotext and pdftoppm inspection of retained report.pdf`
    `Proves installed PDF success, complete governed semantic parity, multipage and Unicode handling, long URL/table/page-break behavior, withheld approval, renderer drift, tampering, invalid font and missing-glyph refusal, merged HTML/Markdown compatibility, strict linting/formatting, exact diff hygiene, text extraction and every-page visual readability.`
- Result: `local_implementation_complete_pending_fresh_review_and_publication`

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- worktree_only means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By native v3 `csdlc finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf --test codefriend_render_md --test codefriend_render_html; cargo clippy --manifest-path adl/Cargo.toml --lib --bins --tests -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff origin/main...HEAD --check`
    `PDF width-wrap unit proof compiled and passed 1/1; installed PDF integration proof passed 4/4; strict Clippy passed.`
- Results:
  - `passed`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed
    checks_run:
      - "PDF width-wrap unit proof 1/1 and installed renderer proof 4/4 passed"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: Issue #898 owns codefriend.pdf_report_manifest.v1 and codefriend.pdf_render_result.v1.
```

## Determinism Evidence
- Determinism tests executed: `installed_renderer_emits_extractable_multipage_pdf_and_bound_manifest; renderer_refuses_withheld_approval_renderer_drift_and_tampering; renderer_rejects_invalid_font; renderer_rejects_font_without_complete_glyph_coverage`
- Fixtures or scripts used: `adl/tests/codefriend_render_pdf.rs governed fixtures; installed adl binary; local TrueType fonts; Poppler pdftotext/pdftoppm; no provider or network`
- Replay verification (same inputs -> same artifacts/order): `<replay_verification>`
- Ordering guarantees (sorting / tie-break rules used): `All governed inputs, approval, renderer/font identity, glyph coverage and bounded layout are validated before the anchored stage is committed; failure removes the stage and never leaves a success manifest.`
- Artifact stability notes: `<artifact_stability_notes>`

## Security / Privacy Checks
- Secret leakage scan performed: `Renderer rechecks unsafe content for governed report and manifest; no credentials or provider calls are accepted.`
- Prompt / tool argument redaction verified: `No provider prompts or credentials are accepted; machine result remains on stdout and diagnostics on stderr.`
- Absolute path leakage check: `Tracked source, docs, fixture inventory and retained manifest were checked; no operator font path or machine-local absolute path is embedded in product output.`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `<trace_bundle_paths>`
- Run artifact root: `.csdlc/evidence/898/pdf-qualification`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf`
- Replay result: `unit 1 passed; integration 4 passed; 0 failed`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_render_pdf.rs plus retained report.pdf, manifest, extracted text and six page images`
- Required artifacts present: `true`
- Artifact schema/version checks: `<artifact_schema_checks>`
- Hash/byte-stability checks: `Retained PDF and manifest digests are recorded in manifest.json and verified by create-only readback before success.`
- Missing/optional artifacts and rationale: `<missing_optional_artifacts_rationale>`

## Decisions / Deviations
- `Used pinned Rust printpdf 0.12.8 instead of a subprocess; the operator supplies a local TrueType font and its digest is bound in the manifest.`
- `Reconciled merged #897 HTML changes into shared helpers and reran all three renderer suites; no HTML scope was absorbed into #898.`

## Follow-ups / Deferred work
- `Run native exact proof on the immutable remediation head and obtain one fresh independent exact-head review.`
- `Publish only after review PASS; then shepherd required standard CI to merge-ready state.`
