# Full internal review — issue919

## Executive Summary

**Changes required.** The current register contains27 findings:4 P1,20 P2 and3 P3. Four aggregate remediation issues are being coordinated; individual findings retain distinct proof and disposition. Nine authored review lanes are present. Source coverage is reconciled and independent packet corrections have been verified. This is a completed internal source review with changes required, not product qualification or release approval.

## Review Scope

Frozen ADL candidate `5c4a6149771c637f3c805985b86231077965eab4`; website `a45e339c13b24716edbd3fadf29dddff36ffe02e`. Baseline `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`. The6,195 changed paths,69 core tasks,366 canonical criteria,116 closing PR references and4,778 retained evidence files are inventoried with explicit limits. A source or format check is not final installed acceptance. See acceptance, history and coverage ledgers.

## Architecture Summary

Native C-SDLC has strong typed ownership and retained journals, but three interrupted-operation windows lack a safe complete public recovery path. CodeFriend has a concrete producer/consumer generation mismatch: native v4 versus website v2. Runtime provider identity must consistently join declared aliases with accounting. The split-provider and UTS architecture is visible in source; cloud/product qualification remains a separate task.

## Security And Privacy Notes

Four security findings cover implicit transport configuration, redaction order, PDF approval binding and session revocation during request-body wait. They are scoped to reviewed source and the exact offline/loopback reproductions cited below. No compromise is alleged. Current authenticated cloud authorization revalidation was not performed. This public-candidate packet excludes credential values, model payloads, dependency caches and raw host logs. Independent privacy inspection found no actual secret or private operator path; synthetic negative fixtures are explicitly classified in the audit.

## Test Recommendations

Fix each defect with a focused counterexample that fails before the repair. Prioritize actual native v4 emissions through the website consumer; installed crash-window recovery; bounded provider responses and alias health; exact live-region attributes; elapsed virtual-time cancellation; and unknown model identity rejection. Tests must distinguish current production defects from weak regression oracles. Website141/141 passing is component support, not proof of native interoperability.

## Remediation Sequence

1. Group A, Planning #7.3: native recovery, transport safety and executable operator contracts (#1161).
2. Group B, Planning #5 GPT-6: CodeFriend/website interoperability, result integrity, session safety and immutable deploy actions (#1162).
3. Group C, Worker #9: Runtime/provider health and regression proof (#1159).
4. Group D, Worker #10: reproducible build/CI and accurate release/operator documentation (#1160).

Each owner must reconcile current source, retain finding-to-fix-to-test mapping and obtain independent review. No merge/deployment is implied. Later repaired heads are separate evidence from this frozen review.

## Residual Risks

#916 remains a bounded quality checkpoint, #918 a draft artifact candidate. #915 qualification and875 live pilot are explicitly deferred; no shadow activity is called live exposure. Full website/platform/provider journeys, unsupported-platform evidence and cloud controls retain documented assurance gaps. Historical unavailable entries and indexed-only supporting corpora remain disclosed. The lead authored917; its artifacts received independent docs/tests inspection. CODE-003 was withdrawn because unlimited request duration is intentional; it is excluded from the count.

## Specialist Coverage

| Lane | Authored result | Scope boundary |
|---|---|---|
| Code |2 P2 plus integration P1 |77 CF production paths,12 CLI entrypoints, website consumer checks |
| Security |4 P2 |Source trust boundaries, exact helper/loopback probes; no cloud clearance |
| Tests |1 P1,3 P2 and1 P3 |Direct/hunk/cross-lane source review;606 fixture corpus; focused negatives |
| Architecture |3 P2 |67 source paths, native recovery call paths, ADR/decomposition supplement |
| Dependencies |2 P1 and2 P2 |Seven Cargo roots/locks, build inputs, CI, website deploy workflow |
| Provider/cloud |2 P2 |Provider and resident source; live/cloud assurance excluded |
| Documentation |4 P2 and1 P3 |317 assignment dispositions plus66-path supplement |
| Demos/installed journeys |1 P3 |16 demo boundaries;40 criterion mappings; retained PDF inspection |
| Evidence/lifecycle |No additional defect |Known assurance gaps retained;29 historical finding crosswalk rows |

## Top Findings

### Finding DEP-002: [P1] Builder reconstruction still resolves mutable and unverified executable inputs

- Severity: P1
- Source role: dependencies
- Evidence: `adl/docker/adl-builder/Dockerfile:1` — See source artifact
- Impact: Ubuntu tag, Rust stable/rustup and unchecked AWSCLI/sccache/llvm-cov downloads may change outside reviewed source identity. Resolving a built image digest protects consumption, not reconstruction provenance. This is historical SYN-005 verified still present in current source.
- Recommended action: Pin base/toolchain and verify direct downloads; retain resolved build input identities.
- Validation gap: current source callpath inspection; no executable download or cloud operation
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DEP-003: [P1] Raw-host remote validation still installs unverified tool archives

- Severity: P1
- Source role: dependencies
- Evidence: `tools/aws_remote_validation/scripts/remote_validation_runner.sh:230` — See source artifact
- Impact: Latest release URL or mutable S3/tarball bytes are extracted and installed without independent checksum/signature/version identity. Version output does not establish approved bytes. Containerized routes may avoid this branch; no live exploitation or cloud execution was attempted.
- Recommended action: Require exact approved version, digest and S3 VersionId where used; verify before extraction/execution.
- Validation gap: current source callpath inspection; no executable download or cloud operation
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding INTEGRATION-001: [P1] Frozen website rejects the native current assessment contract

- Severity: P1
- Source role: integration
- Evidence: `app/review-assessments.mjs:64` — Actual website validator accepts its legacy fixture and rejects the same resealed record declaring native current v4; current producer and consumer callpaths independently source-confirmed.
- Impact: The successfully computed review cannot be accepted or continued through the website; hosted assessment cycles fail with result_identity_mismatch/assessment_review_invalid. Local cycle verification shares the same incompatible consumer. No live deployment failure is asserted.
- Recommended action: Update website versioned validator/prompt-digest/gap handling to native supported generations, preserve historical v2 identities, and bind native-emitted v4 complete/partial/invalid records in a cross-repository interop test.
- Validation gap: Version-gate fixture is not a complete native v4 emission. No paid/live run. Source mismatch is deterministic regardless of later v4 prompt/digest checks.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding SYN-006: [P1] Three independent Cargo graphs still lack change-triggered CI ownership

- Severity: P1
- Source role: tests
- Evidence: `adl/tools/ci_path_policy.sh:1772` — Fresh frozen selector runs for each root in tests-history/syn006-routing.json: exit 0, aggregate_status skipped, lanes {}, pr_publication_sufficient false. ci.yaml manifest commands omit all three; fallback policy 1690–1788 omits them. Original historical P1 docs/reviews/v0.92/internal-review-5846/SYNTHESIS.md:69–77.
- Impact: No selected package-local build/test/format/Clippy owner; required CI may remain green without testing changed graph. adl-resilience as another crate dependency does not execute its own package tests.
- Recommended action: Add explicit selector and CI owners with locked package-local validation and positive path-routing regression cases for each root.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding ARCH-001: [P2] Connect retained semantic journal activation to the installed recovery owner

- Severity: P2
- Source role: architecture
- Evidence: `csdlc-v3/src/application/intent/mod.rs:301` — activate() creates intent, commit, current.next then renames. observe_issue translates an unactivated intent/current.next to RecoveryRequired. Router chains effect/projection recovery only; these accept Current/ProjectionRepairRequired. Repository-wide callsite search finds describe_journal_recovery and execute_journal_recovery only in their unit tests. JournalRecoveryApproval constructor is explicitly production dead_code. Checked legacy fallback commands/local/intent.rs193-250: without a legacy transaction it reports expected_noop, leaving semantic state unchanged. Checked native doctor routing and installed-manual source: no alternate semantic journal activation route. Storage creation journal has equivalent unused native approval constructor; kept under same bridge remediation.
- Impact: All semantic operations stop on RecoveryRequired. The installed recover path has no native-approved way to activate the exact fully retained commit, despite a storage recovery implementation. Operators must modify storage or build custom recovery tooling.
- Recommended action: Expose authenticated exact preview/execute journal activation before business-effect recovery, enforce authority/topology/freshness and retain immutable bytes. Add installed-entrypoint crash regression. Inspect equivalent creation journal bridge.
- Validation gap: Source trace plus inspected exact-activation unit test retained_commit_activation_is_exact_observational_and_replayable; no executed CLI crash fixture in this lane.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding ARCH-002: [P2] Verify or resume the complete bind staging copy before activating it

- Severity: P2
- Source role: architecture
- Evidence: `csdlc-v3/src/commands/local/transactions.rs:441` — Copy helper creates destination before looping files; recovery skips copy whenever destination exists and verifies neither complete inventory nor expected lifecycle digest before target rename/cleanup. The semantic recovery wrapper calls that native operation before rereading integrity, so a later rejection does not preserve deleted source images.
- Impact: commit_bind_local_transaction treats existence of target_stage as completed copying, renames its partial tree to canonical issue root, writes completion and primary binding, then deletes complete source stage and backup. Subsequent integrity checks cannot reconstruct missing compatibility files from native transaction evidence and the issue can become stuck; native completion claims a fully bound image it never verified.
- Recommended action: Authenticate a complete staged image against retained expected digest/inventory before activation and before source cleanup. Repair or resume only the exact incomplete staging copy under lock; preserve source images until complete target readback succeeds. Add interruption test inside copy, not only after copy returns.
- Validation gap: Static exact source trace through partial directory-copy crash window and supported public recovery wrapper. No executed crash harness in this lane.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding ARCH-003: [P2] Recover a reserved finish when no terminal receipt was written

- Severity: P2
- Source role: architecture
- Evidence: `csdlc-v3/src/application/intent/terminal.rs:1378` — Finish stages read-only authenticated observation, reserves effect, then invokes native persistence. AlreadyPending only reads and attaches an existing receipt. recover dispatch has no terminal finish branch; its only terminal route reconciles absent cleanup. Native persist_terminal_finish already compares retained state and immutable receipt identity and writes state then receipt, but replay never reaches it in this window.
- Impact: Exact finish replay requires the absent receipt in AlreadyPending and fails intent_terminal_pending_readback_required. Public recover has no Finish owner, so the healthy semantic pointer retains an operation that cannot complete through supported commands, blocking terminal completion and clean.
- Recommended action: Add an exact preview/reconciliation path for pending Finish/FinishWithoutPr that authenticates the original terminal decision and safely completes missing native state/receipt through its idempotent owner, then attaches the result. Test interruption both after reservation and between state/receipt writes.
- Validation gap: Static public-entrypoint callgraph and exact durable crash-window analysis; no executed CLI crash reproduction. Distinct from ARCH-001: current semantic pointer is healthy and has a pending business effect, not an unactivated journal.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding CODE-001: [P2] Preserve source whitespace in exact citation exports

- Severity: P2
- Source role: code
- Evidence: `adl/src/codefriend/publication/markdown.rs:978` — Exact extracted production text helpers reproduced with benign Python quote: if authorized:\n\tdelete_records()\nreturn ok -> one-line excerpt. Source call sites pass VerifiedCitation::quote through field/markdown_text and field/html_text.
- Impact: The fields explicitly labeled Exact source excerpt replace newline/CR/tab with spaces in Markdown and HTML. PDF derives from that Markdown and further collapses spaces. Consumers cannot copy or inspect the verified excerpt faithfully; Python/block and string-literal semantics can change while source byte offsets still name the original bytes.
- Recommended action: Use a dedicated escaped verbatim source block for citations in Markdown/HTML and preserve line structure in PDF; verify exported multiline/tabs/CRLF quotes against the original admitted bytes or an explicitly documented visible encoding.
- Validation gap: {"status": "reproduced_helper_logic", "source": "lanes/code-excerpt-repro.rs", "limitation": "Standalone Rust harness extracts production helper bodies; privacy guard stubbed false for benign fixture. Does not run full renderer, approval chain or live service."}
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding CODE-002: [P2] Reject retry while the original shell attempt is still running

- Severity: P2
- Source role: code
- Evidence: `adl/src/codefriend/operator/mod.rs:235` — Exact extracted settlement predicate returns true for Incomplete with neither run.json nor settlement.json. Followed retry, run_from_store Store lifetime, cancel_review and late run_attempt completion paths. Existing tests cover immediate retry after Cancelled but not directly during Incomplete.
- Impact: active_attempt_settled treats Incomplete as settled, allowing active_attempt to advance. The original evidence Store lock makes the new attempt fail store_busy; shell now points at failed attempt 2 while attempt 1 continues. A subsequent cancel writes attempts/2/cancel-request.json, not attempt 1's watched path. Original completion also cannot repair overall status because run_attempt only updates it when active_attempt equals its own attempt.
- Recommended action: Require a terminal, retained settlement before changing active_attempt; serialize/recheck shell state across retry/cancel/completion. Regression should hold first provider call, request retry directly, reject it, then verify cancellation still targets attempt 1.
- Validation gap: {"status": "reproduced_guard_with_source_traced_consequence", "source": "lanes/code-retry-repro.rs", "limitation": "No provider calls, no full concurrent installed CLI execution; harness invokes extracted predicate over lightweight matching state types."}
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DEP-001: [P2] Privileged website deployment executes mutable action tags

- Severity: P2
- Source role: dependencies
- Evidence: `.github/workflows/deploy-site.yml:30` — ["website-source/.github/workflows/deploy-site.yml", "website-source/infra/terraform/iam_oidc.tf"]
- Impact: The same reviewed website revision can execute different third-party code with OIDC ability to assume the site deploy role. IAM grants object writes/deletes and CloudFront invalidation, so mutable action identity can bypass source review for a privileged deployment. No upstream compromise is alleged.
- Recommended action: Pin checkout and AWS credential action to reviewed immutable commit SHAs; update them by reviewed changes. Restrict id-token permission to the deploy job.
- Validation gap: static configuration and IAM trust-path inspection
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DEP-004: [P2] Dependency scaffold fills its bounded map with lifecycle locks

- Severity: P2
- Source role: dependencies
- Evidence: `external-skills/repo-dependency-review/scripts/prepare_dependency_review.py:1` — See source artifact
- Impact: The generated map contains80 lifecycle lock paths and no Cargo manifest/lock. Trusting this as coverage would omit actual dependencies. The current review replaced it with explicit manifest/lock inventory; generic tooling defect remains.
- Recommended action: Use ecosystem-aware dependency classifiers, explicit lane assignments and reject empty material dependency denominators.
- Validation gap: executed scaffold generation
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DOC-001: [P2] Route pending qualification to its approved v0.93 successors

- Severity: P2
- Source role: docs
- Evidence: `docs/milestones/v0.92.2/README.md:3` — ["README.md:3", "DEMO_MATRIX_v0.92.2.md:3", "FEATURE_PROOF_COVERAGE_v0.92.2.md:8", "RELEASE_PLAN_v0.92.2.md:3", "NEXT_MILESTONE_HANDOFF_v0.92.2.md:5", "evidence/issue-916/SPRINT10_DEFERRAL.json: operator_authorized_deferral and follow_on_issues", "evidence/issue-917/HANDOFF.md: Current qualification handoff"]
- Impact: Current status headers still describe #915 as pending, while retained SPRINT10_DEFERRAL records it CLOSED/NOT_PLANNED and routes remaining qualification to #1148/#1149/#1150 in v0.93. Current overlays and feature acceptance still send readers to the closed predecessor and imply its completion blocks this milestone rather than preserving the approved deferral and prelaunch gate.
- Recommended action: Update current status/owner overlays consistently to incomplete and deferred, preserving original #915 historical requirements and the #1150 pre-Beta1-launch qualification gate. Do not relabel as PASS or rewrite historical evidence.
- Validation gap: Independent static source comparison; no live tracker query. Frozen candidate contains both contradictory claims.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DOC-002: [P2] Remove native-only timestamps from the semantic disposition example

- Severity: P2
- Source role: docs
- Evidence: `docs/csdlc-v3/NO_PR_CLOSEOUT.md:16` — ["application/intent/terminal.rs:1162-1171 deny_unknown_fields permits four fields", "application/intent/terminal.rs:1196-1197 derives authenticated timestamps", "docs/csdlc-v3/INTENT_COMMANDS.md Explicit no-PR terminal disposition"]
- Impact: The documented finish ISSUE --disposition JSON includes two timestamp fields rejected by the strict semantic Disposition parser, so copying the sole example produces intent_finish_disposition_invalid instead of terminal reconciliation.
- Recommended action: Remove caller timestamps and explain they are authenticated internally; validate the exact documented object against the semantic parser.
- Validation gap: Static exact parser/schema comparison; no lifecycle command executed. Generated roff parity passed for all34 pages.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DOC-003: [P2] Use semantic plan and change schemas in the operator walkthrough

- Severity: P2
- Source role: docs
- Evidence: `docs/csdlc-v3/man/manual.json:1700` — ["manual.json:1699-1701", "application/intent/local.rs:175-200 Changes deny_unknown_fields", "docs/csdlc-v3/INTENT_COMMANDS.md Plan and edit content"]
- Impact: The installed workflow shows prepare --plan and edit --changes but immediately directs the operator to construct LocalPreparationRequest and design-edit.card_updates with expected_lifecycle_digest. These are retired/native-owner shapes; semantic commands require intent_plan and strict intent_changes with cards/amendment. Following the fresh-user walkthrough cannot initialize/edit the issue as described.
- Recommended action: Rewrite workflow step1 around current semantic plan/changes examples; retain native request details only under clearly separate compatibility reference. Regenerate roff and check executable schema parity.
- Validation gap: Static exact parser/schema comparison; no lifecycle command executed. Generated roff parity passed for all34 pages.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DOC-004: [P2] Include validator replacement in the published intent schema

- Severity: P2
- Source role: docs
- Evidence: `docs/csdlc-v3/intent-request.schema.json:289` — ["csdlc-v3/src/application/intent/local.rs:175-210 accepts validators as exactly one edit surface", "docs/csdlc-v3/INTENT_COMMANDS.md:123-127 supported declarations;331 replacement validators", "intent-request.schema.json:224 cargo const;289 additionalProperties=false"]
- Impact: The changes definition rejects validators as an unknown property and its oneOf only permits cards or publication. The plan definition additionally requires program=cargo. Supported requests accepted by native source and documented in INTENT_COMMANDS are therefore rejected by the published machine-readable contract.
- Recommended action: Bring the schema into parity with every supported intent content variant; add positive parity checks for validator replacement and admitted non-cargo declarations alongside negative guards.
- Validation gap: Static comparison of exact candidate schema, native deserializer and current guide; no lifecycle execution.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding PRV-001: [P2] Join resident health using the declared model reference

- Severity: P2
- Source role: provider
- Evidence: `adl-runtime-kernel/src/control.rs:6712` — ["control.rs:5539-5541", "control.rs:3855-3860", "assembly.rs:1430-1464", "control.rs:6709-6749", "control.rs:6605-6613"]
- Impact: Admission stores the native provider_model_id in sample.model (5540), while dispatch and ProviderUsage retain the declared model_ref (3857; assembly accounting). The supervisor compares them directly, cannot find successful inference, repeatedly classifies the resident inference_unverified and cannot close an incident through verified recovery. The small health endpoint already chooses provider_binding.model_ref at6608-6613.
- Recommended action: Use one stable binding identity for accounting and health joins, including provider_binding.model_ref where available. Add a regression with differing model_ref/native ID proving successful inference clears the same resident incident before and after roster refresh.
- Validation gap: Static complete data-flow check. No executable test, live endpoint, credential access or cloud operation.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding PRV-002: [P2] Bound successful provider response bytes before aggregation

- Severity: P2
- Source role: provider
- Evidence: `adl-provider-core/src/http_family.rs:153` — ["http_family.rs:90-95 client builder constrains redirects/retries but not body bytes", "http_family.rs:147-170 shared success helpers", "http_family.rs:2593-2597 generic JSON path", "registry.rs:833-834 canonical executor construction", "registry.rs:398-417 optional wrapper", "registry.rs:526-534 post-aggregation extracted output validation"]
- Impact: The shared helpers call Response.json/text without a byte bound. Runtime registry builds this executor directly; its optional demo wrapper checks only extracted output after the whole response has already been allocated and parsed. A malformed or faulty provider can consume Runtime memory far beyond the nominal 4 MiB output limit, threatening other residents and shepherd availability. The legacy Runtime adapter has a predecode bound that the new registry route does not inherit.
- Recommended action: Apply a bounded response reader before decoding every Runtime HTTP success body, including unused JSON fields; keep timeout and decoded-output bounds as separate controls. Add local bounded regressions for content-length and chunked oversized bodies.
- Validation gap: Source-confirmed complete call-path review only. No oversized response probe or executable reproduction performed.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding SEC-001: [P2] Disable implicit curl configuration for credentialed mutations

- Severity: P2
- Source role: security
- Evidence: `csdlc-v3/src/adapters/mod.rs:754` — ["operational argv at lines751-781 has no -q", "run_process at840-850 appends private --config without disabling default config", "read-only run_observational_curl at869-875 explicitly adds -q", "security-probes/transport_probe.rs"]
- Impact: The operational transport invokes curl without -q. Clearing HOME does not disable curl default configuration lookup (the same source explicitly acknowledges account-database fallback for read-only transport). Undeclared configuration can write response/trace files or alter destinations while the selected GitHub credential and mutation body are present.
- Recommended action: Use -q as curl first argument for operational as well as observational invocations; add operational-path regression with isolated default config and effect sentinel.
- Validation gap: executed exact extracted run_process against real curl, isolated synthetic CURL_HOME wrapper, file:// payload; unexpected output created. -q negative control suppresses write. No live mutation or credential used.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding SEC-002: [P2] Redact credentialed mutation output before truncation

- Severity: P2
- Source role: security
- Evidence: `csdlc-v3/src/adapters/mod.rs:360` — ["lines360-366", "lines955-968", "security-probes/transport_probe.rs"]
- Impact: run_process calls process_output, which truncates stdout/stderr, before RealProcessAdapter applies full-value replacement. A truncated credential prefix no longer matches the full credential and survives in returned diagnostics. Read-only curl already implements the correct order.
- Recommended action: Redact raw child output before either stream is truncated for every credentialed adapter, and retain a cutoff-inside-secret regression.
- Validation gap: executed exact extracted functions with synthetic GITHUB_TOKEN and8-byte output limit; redacted result still equals first8bytes of synthetic secret. No actual secret accessed.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding SEC-003: [P2] Verify PDF content rather than trusting resealed relay metadata

- Severity: P2
- Source role: security
- Evidence: `adl/src/codefriend/publication/relay.rs:240` — ["agent/publication.rs:958-1098 public verifier call path", "publication/relay.rs:237-260", "publication/pdf.rs:562-606"]
- Impact: verify_stage reaches verify_rendered, whose PDF branch checks only the prefix and hash-shaped semantic/font fields. pdf::validate_manifest validates metadata identities but never ties PDF content or semantic_digest to the approved report. common only establishes self-consistent caller hashes. An unrelated or malformed PDF can therefore be reported as complete; the external_resources=false claim is also not inspected against bytes. Markdown/HTML branches reconstruct expected content and reject this substitution.
- Recommended action: Bind returned PDF to trusted renderer evidence or validate its actual allowed structure and expected approved semantics; negative-test substituted PDF with all unkeyed hashes recomputed. Do not treat digest self-consistency as content provenance.
- Validation gap: static complete call-path analysis; malicious PDF terminal-stage production invocation not executed. No claim of observed exploitation.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding SEC-004: [P2] Recheck browser authorization after reading a mutation body

- Severity: P2
- Source role: security
- Evidence: `app/http.mjs:147` — ["app/http.mjs:121-123 initial authorization", "app/http.mjs:147-155 body wait then dispatch", "app/http.mjs:190-197 publication body waits", "app/session.mjs:16-27 fresh revocation checks", "security-probes/session_probe.mjs"]
- Impact: The route retains user from the initial session check and dispatches the hosted review after revocation. Publication decision/render routes have the same pre-body authorization pattern. Gateway subject credentials do not bind the revoked browser session. Journey routes already reauthorize after asynchronous boundaries.
- Recommended action: Centralize fresh authorization immediately after awaited body reads and before effect dispatch; recheck before releasing sensitive responses. Apply browser-session checks consistently across publication and run routes, retaining HMAC service identity handling separately.
- Validation gap: Executed actual frozen website application and Sessions on loopback with in-memory store and recording hosted adapter. Headers authenticate, session revoked before body completes, response 202 and one dispatch; fresh Sessions.require rejects. Unused crypto/parser imports replaced by throwing stubs, never called. No provider or external network.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding TESTS-001: [P2] Bind live-region assertions to each transcript element

- Severity: P2
- Source role: tests
- Evidence: `demos/html-observatory/tests/accessibility_responsive.test.mjs:51` — tests-accessibility-negative-repro.json: isolated exact candidate copy with two attribute removals exits0 and prints accessibility/responsive proof PASS. Regex crosses tags via [\s\S]*.
- Impact: Accessibility proof still reports PASS, permitting loss of announced conversation/room updates to escape this regression gate. Current candidate attributes are present; no current UI failure is asserted.
- Recommended action: Parse actual element attributes or query both transcript nodes in DOM; include a negative case removing each attribute independently.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding TESTS-002: [P2] Assert cancellation completes before the probe timeout

- Severity: P2
- Source role: tests
- Evidence: `adl-runtime-kernel/tests/shepherd.rs:1359` — tests-virtual-time-repro/result.json: 1 test passes while explicitly measuring 600 seconds. Isolated counterexample using pinned Tokio 1.49, original yield/cancel/await oracle; not a mutation of the production crate.
- Impact: Paused Tokio time automatically advances; the test can pass after 600 virtual seconds and cannot prove its stated prompt-shutdown invariant. Current production select has cancellation; no present 600s production delay is asserted.
- Recommended action: Measure tokio::time::Instant around cancellation and assert a small virtual-time bound, or wrap JoinHandle in a timeout far shorter than 600s. Prove mutation removing cancellation fails.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding TESTS-003: [P2] Reject unknown base-blob identities before asserting same-model comparison

- Severity: P2
- Source role: tests
- Evidence: `adl/tools/issue905_runtime_speculative_retest.py:348` — tests-905-identity-negative.json: unmodified AST-extracted model_identity evaluated offline with two different FROM aliases and manifest digests; both null blob IDs pass every model-identity equality field. Source lines 133-144 permit null, 347-354 admit it, 427 emits true.
- Impact: Both base_blob_sha256 values become null and compare equal; the report then unconditionally declares same_base_blob=true. Missing weight identity is promoted to proof, allowing a speculative benchmark to claim a controlled same-weight comparison without observing it. No claim that the retained #905 run used differing weights.
- Recommended action: Require two nonempty valid base-blob digests before equality; reject unsupported metadata with an explicit non-proving outcome. Add missing/malformed FROM negative cases.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DEMOS-001: [P3] Label the PDF Cargo test as source-built CLI proof

- Severity: P3
- Source role: demos
- Evidence: `docs/codefriend/PDF_EXPORT.md:46` — adl/tests/codefriend_render_pdf.rs:183 uses Command::new(env!("CARGO_BIN_EXE_adl")); no installed-binary override. Reviewed source and actual retained PDF output separately.
- Impact: The test unconditionally executes CARGO_BIN_EXE_adl, so passing results say nothing about an installed binary or its provenance; documentation overstates this command as installed proof. Separate retained PDF output qualification remains valid for its declared artifact.
- Recommended action: Call this source-built CLI/component proof, and document a separate selected-installed-binary invocation if installed acceptance is required.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding DOC-SUP-001: [P3] Update operational Rust parser limits in architecture and fitness guides

- Severity: P3
- Source role: docs
- Evidence: `docs/codefriend/ARCHITECTURE.md:74` — Direct comparison of ARCHITECTURE.md74-79 and LOCAL_FITNESS.md37-41 with rust_parse.rs10-18 and RUST_PARSER_BOUNDS.md; deterministic source/document mismatch, no execution required.
- Impact: Both current guides say 32 KiB and 128 lexical units, including comments/literals, while the shared parser admits 400 KiB and 32768 tokens with depth/path limits and opaque comments/literals. Users can unnecessarily split or exclude ordinary files and misdiagnose which budget actually failed. The allocation-free wording also fails to describe the worker-stack resource boundary.
- Recommended action: Replace obsolete limit paragraphs with the current bounds and a link to RUST_PARSER_BOUNDS.md. Preserve historical proof records as historical evidence.
- Validation gap: Source-level finding; a fix and focused regression have not yet been verified.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.

### Finding TEST-TRANS-001: [P3] Reader concurrency test can fail solely because the writer finishes first

- Severity: P3
- Source role: tests
- Evidence: `csdlc-v3/tests/transactions.rs:2815` — The writer has no handshake with the reader; the parent reads only while !writer.is_finished() and then requires at least one successful Current observation. Joining the writer and checking final generation does not guarantee any loop iteration.
- Impact: The final observed > 0 assertion fails even when every commit and observation is coherent, making a purported deterministic local gate scheduler-dependent.
- Recommended action: Coordinate at least one reader observation with the writer using an explicit handshake, then assert coherent observations and final generation without depending on scheduler fairness.
- Validation gap: Static schedule analysis of lines 2777-2831; no executable reproduction or test execution performed.
- Disposition: Open at frozen candidate; grouped remediation pending independent verification.
