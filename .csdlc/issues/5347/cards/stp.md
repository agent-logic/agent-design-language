# Structured Task Prompt

Template: 1.0.0

Issue: 5347

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare a durable reviewed execution packet for #5347 only; no deletion, product implementation, product-path claim, PR, publication, merge, AWS/raw gh, Runtime v2 edit, or root-main mutation.

## Deliverables

- All six current-registry typed cards with exact scope, authority, dependencies, and no-deferral truth
- External-band deletion design and Mermaid diagram
- Exact terminal #5346, #5344, #5343, #5358, and #5361 receipt/ancestry gate
- Canonical per-file deletion and retention manifest contract
- Machine-checkable zero-overlap proof against the exact #5346 manifest
- Authority-rooted reachability, replacement-owner, proof, and retained-rationale requirements
- Preparation-only protected paths and later typed exact-path claim-amendment rule
- COTS decision, LoC/test/time budgets, PVF lanes, rollback contract, and bounded review

## Acceptance

1. AC-1: Execution starts only when #5346, #5344, #5343, #5358, and #5361 are GitHub merged, typed closed_out with claims released, backed by valid retained merged receipts, ancestral to the candidate revision, and the #5346/#5347 dependency cycle is authoritatively reconciled
2. AC-2: A canonically ordered immutable manifest binds each candidate to normalized repo-relative path, baseline object identity, measured lines, capability, authority-rooted reachability, accepted replacement owner, exact revision, terminal receipt, proof references, disposition, and manifest digest
3. AC-3: The #5347 manifest has zero canonical path overlap with the exact reviewed #5346 language/compiler/engine/CLI manifest; ambiguous or mixed-owner files fail closed as blocked and no directory prefix grants authority
4. AC-4: Every delete_external row has accepted replacement behavior and negative proof; every retained row names owner, consumer, rationale, proof role, and sunset condition; missing or stale evidence blocks deletion
5. AC-5: Product paths are added only by typed claim amendment after manifest freeze, disjointness proof, dependency proof, and review; deletion touches exactly tracked manifest files and rejects traversal, symlink escape, duplicates, submodules, generated output, and untracked files
6. AC-6: Focused owner tests, characterization/parity, security/determinism negatives, selector rollback, current CI, consumer reachability, and post-deletion validation pass at exact revisions with deterministic redacted repo-relative evidence and no deferred acceptance
7. AC-7: Runtime v2, ADL v2, Runtime v3, C-SDLC v2, #5346 core paths, selector/cutover state, and historical non-executable evidence remain outside #5347 mutation authority
8. AC-8: The later implementation reuses Git, typed C-SDLC v2, existing characterization/parity tools, and standard JSON parsing; manifest/gate code stays within 500 nonblank lines, tests/fixtures within 800 lines and fewer than 50 tests, retained evidence within 1,200 lines, focused gates within 120/300 seconds, complete proof within 3,600 seconds, and net source change is negative with separate accounting

## Dependencies

- #5346 final-core deletion eligibility merged and typed closed_out with released claim, valid retained merged receipt, ancestral merge SHA, and reviewed exact deletion manifest
- #5344 soak/rollback and #5343 reversible selector switch merged and typed closed_out with released claims, valid retained merged receipts, and ancestral merge SHAs
- #5358 C-SDLC v2 acceptance and #5361 Runtime v3 acceptance merged and typed closed_out with released claims, valid retained merged receipts, and ancestral merge SHAs
- Authoritative issue graph reconciles the current cycle where live #5346 depends on #5347 while operator sequencing requires terminal #5346 before #5347
- WP-02 pinned baseline and ownership packet remains valid at the candidate revision or is explicitly refreshed and reviewed

## Inputs

- AGENTS.md
- GitHub issues #5347 and #5346 source prompts
- docs/templates/prompts/current.json
- csdlc-v2/operator/generation-selector.json
- docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md
- docs/milestones/v0.91.8/baseline_and_ownership_v0.91.8.json
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/features/DELETION_AND_CUTOVER_v0.91.8.md
- .adl/docs/TBD/ADL_REPOSITORY_CODE_REDUCTION_PLAN_v0.91.8.md
- future retained terminal receipts and exact manifests for #5346, #5344, #5343, #5358, and #5361

## Non Goals

- Deleting any code or claiming any product path during preparation
- Final language, compiler, engine, CLI, compatibility, or ADL v2 deletion owned by #5346
- Runtime v2, Runtime v3, C-SDLC v2, selector, soak, cutover, acceptance, release, or provider implementation
- AWS, cloud provisioning, raw gh, credential acquisition, production mutation, or live provider execution
- PR creation, publication, merge, closeout, or issue-graph mutation during preparation
- Treating code movement, generated copies, archived executables, historical references, metadata, or line targets as replacement proof
