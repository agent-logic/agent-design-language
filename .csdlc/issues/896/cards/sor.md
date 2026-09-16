# v0922-codefriend-markdown-renderer

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

Task ID: issue-0896
Run ID: issue-0896
Version: 0.92.2
Title: [v0.92.2][CF-RENDER-MD] Render an approved review as Markdown
Branch: codex/896-v0922-codefriend-markdown-renderer
Card Status: ready
Status: IMPLEMENTED_LOCAL_PROOF_PASS
Generated: 2026-09-12T00:10:01.454600+00:00

Execution:
- Actor: `codex/root under active Sprint 4 #930 goal`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-16T00:00:00-07:00`
- End Time: `2026-09-16T00:00:00-07:00`

## Summary

Implemented installed `adl codefriend export markdown` with complete governed semantic rendering, variable-length safe code-span citations, exact approved-input identity binding, directory-handle-anchored create-only publication, bound manifest, deterministic race regressions, and retained installed output. One fresh review failed and all actionable findings are remediated; distinct fresh review and publication remain pending.

## PVF Lane Truth
- Initial PVF lane: `owner_binary`
- Planned PVF lane: `owner_binary`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; the declared focused runtime renderer lane and bounded security/concurrency units executed.`

## Issue Metrics Truth
- Expected runtime class: `bounded local CPU/filesystem and installed adl process`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implemented_pending_review`
- Issue goal ref: `Active Sprint 4 #930 execution goal; #896 is the current bounded child objective`
- Sprint goal ref: `issue-926; all-eleven-sprint management only, not an execution dependency`
- Goal metrics rollup ref: `.csdlc/evidence/896/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/896/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution or estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/896/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/publication/markdown.rs; adl/src/codefriend/publication/mod.rs; adl/src/codefriend/review/synthesis.rs; adl/src/codefriend/actions/remediation.rs; adl/src/codefriend/actions/test_plan.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_render_md.rs; adl/tests/fixtures/codefriend/markdown/PVF.json; docs/codefriend/MARKDOWN_EXPORT.md; .csdlc/evidence/896/installed-markdown-render; .csdlc/evidence/896/installed-markdown-render-r2`
- Additional proof artifacts: `.csdlc/evidence/896/installed-markdown-render-r2/PROOF.md; report.md; manifest.json; render-result.json; prior bc01a144 proof retained under installed-markdown-render`

## Actions taken
- `Replaced path-based staging and replace-capable rename with no-follow directory-handle traversal, create-only file writes, and platform no-replace commit.`
- `Rendered every governed synthesis-source, remediation-action, and test-case field and added semantic parity assertions.`
- `Added adversarial backtick, competing-target, and parent-swap regressions and retained new installed output with verified hashes.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; all implementation and proof changes are in the bound FastWork worktree`
- Worktree-only paths remaining: `All implementation, proof, and lifecycle paths remain in the bound #896 FastWork worktree pending review and publication.`
- Integration state: `worktree_only`
- Verification scope: `bound #896 issue worktree and installed local adl process`
- Integration method used: `native_v3_bound_fastwork_worktree`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --manifest-path adl/Cargo.toml --lib codefriend::publication::markdown::tests; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_render_md -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check`
    `Proves installed export, complete governed semantic rendering, variable-length non-clickable citation spans, exact approval/input binding, no-replace destination commit, directory-handle confinement across parent swaps, current artifact readback, denied stale/tampered inputs, and predecessor compatibility.`
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
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis --test codefriend_remediate --test codefriend_testplan --test codefriend_ux; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_render_md -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check`
    `Focused renderer proof passed 5/5; citation/create-only/confinement units passed 3/3; current installed output hashes matched; strict focused Clippy passed; formatting and diff hygiene are rerun before immutable commit.`
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
      - "Focused installed Markdown renderer proof: 5 tests passed"
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
      approved: Issue #896 owns codefriend.markdown_report_manifest.v1 and codefriend.markdown_render_result.v1; the synthesis reader and empty canonical predecessor plans are required downstream interoperability.
```

## Determinism Evidence
- Determinism tests executed: `installed_renderer_emits_complete_bound_report_and_manifest; renderer_refuses_missing_or_withheld_approval_and_identity_drift; renderer_refuses_changed_approved_artifacts_and_missing_provenance; renderer_handles_empty_and_long_partial_evidence_without_claim_drift; renderer_escapes_links_and_rechecks_secret_like_claims; code_spans_keep_backtick_paths_non_clickable; anchored_commit_refuses_a_competing_target_without_replacement; anchored_parent_handle_cannot_be_redirected_by_path_swap`
- Fixtures or scripts used: `Pinned Vector review/synthesis at 410da89a0ed42c523143da89fffeb7f6402833e0; generated canonical remediation and test-plan bundles; installed adl binary; isolated local approval store and destination; no provider or network`
- Replay verification (same inputs -> same artifacts/order): `Executed focused installed-renderer tests plus deterministic competing-target, parent-swap, and adversarial-backtick regressions; all eight passed.`
- Ordering guarantees (sorting / tie-break rules used): `All source bundles, plan partitions, approval identity, renderer identity, destination identity, and report bytes are validated before output creation. Files are created through no-follow directory handles in a private stage, fsynced, and committed by a no-replace rename; parent identity and committed bytes are rechecked through anchored handles.`
- Artifact stability notes: `The failed-review proof at installed-markdown-render is retained as immutable historical evidence. installed-markdown-render-r2 is the current remediation proof with exact result-bound hashes and create-only output.`

## Security / Privacy Checks
- Secret leakage scan performed: `Renderer rechecks unsafe content for every field, final report and manifest; secret-like publication claim is rejected by focused proof.`
- Prompt / tool argument redaction verified: `No provider prompts or credentials are accepted by the renderer; machine result remains on stdout and human diagnostics on stderr.`
- Absolute path leakage check: `Tracked report, manifest, result, docs, fixture inventory and source were scanned; no machine-local absolute path is retained.`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/896/installed-markdown-render-r2/{PROOF.md,report.md,manifest.json,render-result.json}`
- Run artifact root: `.csdlc/evidence/896/installed-markdown-render-r2`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --manifest-path adl/Cargo.toml --lib codefriend::publication::markdown::tests`
- Replay result: `Focused renderer 5/5 passed; anchored citation/create-only/confinement units 3/3 passed`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_render_md.rs plus retained installed-render output`
- Required artifacts present: `true`
- Artifact schema/version checks: `Passed canonical synthesis, remediation, test-plan, publication, Markdown manifest, render-result, and JSON readback checks.`
- Hash/byte-stability checks: `Current report digest b9af969342e2108d3d827e69c68e73a5b3f07d6fca9d8c862c3efc3b5541da88; current manifest digest 13c3ab6a7c2aa7dafa66698d0089eb4a74ad2bad9ca0038da56ed60a6e2cbc02; renderer reopened committed files through the anchored directory and verified exact bytes before success.`
- Missing/optional artifacts and rationale: `HTML, PDF, browser, provider, remote-publication, and customer-hosting artifacts are explicit non-goals for #896.`

## Decisions / Deviations
- `Accepted complete reviews with zero findings now produce empty provenance-bound remediation and test plans, enabling the required #896 empty-finding case without invented actions.`
- `Evidence citations use variable-length CommonMark code spans; complete source/action/test semantics are rendered; output publication uses opened directory handles and platform no-replace rename. HTML/PDF and external publication remain non-goals.`

## Follow-ups / Deferred work
- `Commit the remediation candidate and obtain a distinct fresh independent exact-head review.`
- `Publish only after review PASS; then shepherd required CI and merge under current native authority.`
