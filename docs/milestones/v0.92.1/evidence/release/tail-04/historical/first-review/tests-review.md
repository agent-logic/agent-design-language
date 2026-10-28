# Issue #520 test and proof review

## Review identity

- Candidate: `c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Base: `f0a011a5c59d46c763d669f69a10308b3f870ba4`
- Review checkout: detached read-only checkout at the exact candidate SHA
- Review date: 2026-09-09 UTC
- Role: tests, validation, CI, and proof adequacy

## Findings

### T520-TEST-001 — P1 — Required C-SDLC v3 suite fails at the exact candidate

- Evidence: `.github/workflows/ci.yaml:191-215` defines `csdlc-v3-standalone` as a required path-selected job and runs the full locked C-SDLC v3 test suite. `csdlc-v3/tests/terminal_cleanup_cutover_commands.rs:1365-1417` constructs a shadow `doctor` request using the real checkout as the worktree while hard-coding the historical #505 execution branch; `csdlc-v3/tests/terminal_cleanup_cutover_commands.rs:1421-1446` then requires the route to be `Ready`.
- Reproduction: `CARGO_TARGET_DIR=<external-review-target> cargo test --locked --manifest-path csdlc-v3/Cargo.toml` exited 101. In `terminal_cleanup_cutover_commands`, 20 tests passed and 10 failed at line 1446. Every failure reported `ProofRouteStatus::Blocked`, `shadow_command_not_successful`, and a child `csdlc doctor` exit status of 2.
- Failing cases: `approved_cutover_atomically_installs_selector_and_rollback_receipt`; `approved_cutover_refuses_existing_receipt_before_mutation`; `authenticated_cutover_rejects_unmerged_authority_pr`; `cutover_and_rollback_share_one_mutation_lock`; `cutover_recovers_interrupted_boundaries_and_rollback_is_idempotent`; `cutover_rejects_intermediate_output_parent_symlink_escape`; `executable_cutover_cannot_bypass_request_findings`; `executable_cutover_requires_authenticated_github_authority_before_mutation`; `rollback_fails_closed_on_stale_selector_digest`; `tracked_selector_revert_rolls_back_from_fresh_worktree`.
- Scenario: a future change under the C-SDLC v3 path selects the required standalone lane in a normal detached CI checkout.
- Impact: that required lane is deterministically red and the cutover/rollback regression surface has no current passing proof. This blocks safe changes to the current lifecycle authority and makes the nominal green baseline misleading.
- Required remediation/proof: make the fixture self-contained rather than dependent on historical branch/worktree topology; preserve fail-closed production behavior; run the full locked C-SDLC v3 suite from a detached exact-head checkout and retain the successful result.

### T520-TEST-002 — P1 — Retained predecessor requirements have no candidate-bound proof

- Evidence: `docs/milestones/v0.92.1/evidence/release/tail-01/quality-gate.json:32-42` records 227 retained-predecessor lanes as non-proving, five unresolved exceptions, a blocked decision, and `downstream_unlock: false`. `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json:33-39` classifies the 227 rows as a product blocker.
- Scenario: v0.92.1 is judged from the publication packet without mapping predecessor criteria to evidence for this candidate.
- Impact: 227 required product behaviors cannot be shown to hold for the release candidate; document inventory and accounting cannot substitute for execution evidence.
- Required remediation/proof: for every retained criterion, bind a candidate revision, proving lane and result, or an explicit governed amendment/removal; rerun the denominator and quality-gate validators until no required row remains non-proving.

### T520-TEST-003 — P1 — Two executed issues lack current exact-head review proof

- Evidence: `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json:6-12` records two executed issues and seven affected rows with substantive post-review changes but no current exact-head review.
- Scenario: execution evidence is accepted after the reviewed revision changed.
- Impact: seven claimed results are not bound to the bytes proposed for release, so semantic regressions can pass through stale review evidence.
- Required remediation/proof: identify the two exact heads, obtain independent review of each immutable head and its complete substantive scope, then regenerate the seven row mappings from those review receipts.

### T520-TEST-004 — P2 — Eleven current criteria lack candidate-bound semantic proof

- Evidence: `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json:24-30` records 11 current planned criteria without candidate-bound proof or amendment authority.
- Scenario: the publication packet is treated as sufficient although a current acceptance criterion has only structural or historical evidence.
- Impact: the release can claim planned behavior without a proving observation at the candidate revision.
- Required remediation/proof: add a deterministic behavior-level lane for each criterion or record explicit amendment authority, then bind the result to the exact candidate.

### T520-TEST-005 — P2 — Four live criteria and specification rows remain unsynchronized

- Evidence: `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json:15-21` identifies four affected rows across WP-01, GCP-E, HOT-01, and OBS-B.
- Scenario: validators or reviewers select different requirement text as authoritative.
- Impact: the proof denominator is ambiguous; apparently passing evidence may prove a different contract than the release specification.
- Required remediation/proof: reconcile each live criterion with canonical planning authority and rerun the criterion inventory and semantic mapping checks.

### T520-TEST-006 — P2 — Three shared-path resolutions lack owner sign-off

- Evidence: `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json:42-48` records three shared paths without explicit owner approval.
- Scenario: concurrent work changes shared release surfaces and the final content is accepted only from merge/accounting state.
- Impact: tests cannot establish that the merged shared-path result preserves every owner's requirement, even when no obvious requirement loss is visible.
- Required remediation/proof: obtain exact-content owner sign-off for all three rows, bind it to the candidate, and make the validator reject missing or stale sign-off.

## Missing proof map

| Gap | Rows | Existing evidence | Missing proving evidence | Release effect |
|---|---:|---|---|---|
| C-SDLC v3 cutover/rollback suite | 10 failing tests | Exact-candidate local run; required CI job definition | Passing detached-head full suite | Required lifecycle lane red |
| Exact-head review | 7 | Issue execution records | Review receipts for two post-change heads | Product blocker |
| Current semantic criteria | 11 | Planned criteria and structural inventory | Candidate-bound behavioral proof or amendment authority | Proof debt |
| Retained predecessor criteria | 227 | Historical predecessor inventory | Criterion-level successor mapping and candidate-bound result | Product blocker |
| Live/spec synchronization | 4 | Both live and planning text | One canonical reconciled criterion per row | Proof denominator ambiguous |
| Shared-path ownership | 3 | Final merged content | Exact-content owner approvals | Proof debt |

## Assignment coverage

The base-to-candidate inventory contained 5,481 changed paths (5,168 added and 313 modified). A deterministic path inventory classified 1,848 test, validation, proof, fixture, CI, or evidence paths. The executable-test pass then inspected all 97 changed Rust, shell, Node, and Terraform test sources; the proof pass covered the retained release-tail gate, blocker register, publication validators, issue validators, fixtures, reports, and zero-byte command-capture evidence. Empty command captures were not automatically treated as defects: their companion stderr/readback and asserted absence semantics were checked before classification.

No assigned path was sampled. Large generated evidence sets were reviewed through their deterministic manifests, hashes, validators, and exception registers, with representative source-to-record checks where the validator supplied the full enumeration.

## Validation performed

- Full locked C-SDLC v3 suite: **FAIL**, 10 failures described in T520-TEST-001.
- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb`: **PASS**, truthfully returns `gate_decision: blocked`, 393 inventoried lanes and 366 required lanes.
- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb --negative`: **PASS**, 11 adversarial fixtures.
- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --all`: **PASS** for the bounded publication contract; it records `release_approval: false`.
- `bash adl/tools/test_ci_path_policy.sh`: **PASS**, including its expected failure-injection cases.
- Shell syntax over every changed `.sh` path: **PASS**.
- The review checkout remained clean after all checks.

## Residual risk

- No paid cloud, GPU, runtime, browser, or live-provider execution was performed. This review therefore does not independently reproduce those historical lanes.
- The shadow child command's stderr is deliberately redacted in the test report. The common topology dependence is directly visible and the failure is reproducible, but the narrow internal reason for exit 2 still needs repair-time instrumentation or an isolated fixture assertion.
- Passing publication and manifest validators prove document integrity and accounting only. `docs/milestones/v0.92.1/evidence/release/tail-03/README.md:9-11` explicitly says they do not supply missing product proof or release approval.
