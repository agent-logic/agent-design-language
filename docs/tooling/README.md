# Tooling Documentation

This directory is the main entrypoint for ADL tooling guides, prompt-spec references, reviewer surfaces, editor-related proof surfaces, and maintainability utilities.

The goal of this directory is to make ADL’s tooling surfaces understandable and navigable without forcing the reader to learn the entire internal workflow system first.

## Start Here

- The excluded `adl-review` wrapper, fixed issue414 llama baseline runner, and original-v1 card-editor skill aggregate are retired. Original review compatibility and live-model/restore assertions remain historical evidence in Git and #1223 custody. Public ADL review dispatch remains available through the qualified CodeFriend owner; no new review or live-model qualification is claimed.

- The v0.91.4 evidence-bundle, merge-readiness and observation-memory aggregation scripts are retired historical tooling. Their original assertions and source remain in Git and the #1223 custody record. The immutable v0.91.4 packets retain their original commands as historical evidence; those commands are no longer current ADL entrypoints. This retirement does not claim current native lifecycle or successor test equivalence.

- The v0.91.7 closeout-register generator, legacy CodeFriend CI selector/aggregate, Opus adapter contract test and original v1 Shepherd link test are retired historical tooling. Their source/assertions remain in Git and retirement custody. Existing closeout registers and the [Shepherd contract](ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md) remain historical evidence; they do not replace current native terminal authority or establish current provider/CI acceptance.

- The v0.91.6 #4441 metrics backfill generator and its coupled typed-v2 test are retired historical regeneration tools. The [original backfill report](../milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.md), JSON and CSV remain immutable evidence; their refresh command is historical, not a current ADL entrypoint. This retirement does not rerun GitHub issue reads, port the old ABI to native v3, or claim current metrics acceptance.

- Session archive tooling is owned by [Infrastructure at the qualified source](https://github.com/agent-logic/agent-logic-infrastructure/tree/025850d265cbd1a55b28a8e9b238b2fe0bfcaf1d/adl/tools). Run `archive_codex_sessions_to_fastwork.sh` from that owner checkout with explicit `--source` and `--destination`; ADL no longer supplies a local copy. The unchanged original safety test passed on macOS using two synthetic session files: five producer invocations and eleven explicit checks. This establishes local fixture integrity, permissions and unsafe-path refusal; it does not archive or qualify real operator sessions.

- The ten former WP04/WP08–WP13A/WP15 native workflows are historical Runtime CI contracts after the repository split. Their original definitions remain in [Runtime historical workflow custody](https://github.com/agent-logic/agent-logic-runtime/tree/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/ci/historical-workflows). Their platform matrices, exact-head source and binary provenance, semantic receipts and native scenario obligations remain unqualified by this retirement. Prepared issue scripts and receipt validators remain unchanged. Public CI keeps its existing workflow policy and six mutation refusals; the forbidden automatic-fanout fixture targets the retained manual nightly coverage workflow.

- CSM fixture validators and three original fixture tests are owned by [Infrastructure at the qualified source](https://github.com/agent-logic/agent-logic-infrastructure/tree/025850d265cbd1a55b28a8e9b238b2fe0bfcaf1d/adl/tools). Retained operator-report and v0.90.4 governed-console tests require an absolute `ADL_CSM_VALIDATOR_SOURCE_ROOT`; `tools/csm_contracts/validate_visibility.py` verifies the exact owner validator bytes/mode before dispatch. Existing assertions are preserved. Three unchanged original tests passed against six owner files plus eleven authentic external ADL fixture inputs; this does not prove standalone owner delivery, browser/live Runtime behavior, the missing operator-report renderer, or the Cargo demo.

- [Infrastructure CI report and producer-result helpers](https://github.com/agent-logic/agent-logic-infrastructure/tree/025850d265cbd1a55b28a8e9b238b2fe0bfcaf1d/adl/tools) — `summarize_ci_runtime.py`, `summarize_nextest_timings.py` and `verify_coverage_producer_results.sh` and their original tests now belong to the owner. The three unchanged fixture tests passed offline (29 assertion events); this does not qualify live CI, real nextest execution or cloud coverage producers.

- [Runtime owner guide references](RUNTIME_OWNER_GUIDES.md) — preserved documentation for Runtime-owned APIs and operations.

- [Runtime contract-market fixture owner](RUNTIME_CONTRACT_MARKET_OWNER.md) — qualified deterministic fixture, runner and summary source route.

- Prompt-spec and structured prompt surfaces: `prompt-spec.md`
- Canonical issue-card lifecycle: `card-lifecycle.md`
- Structured prompt contracts: `structured-prompt-contracts.md`
- Session coordination and root checkout policy:
  `SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`
- [Current subsystem boot paths](ACTIVE_BOOT_PATHS.md), including the sole
  ordinary native v3 lifecycle entrypoint and retained rollback boundary
- Default contributor workflow: `../default_workflow.md`
- Editor and authoring proof surfaces: `editor/README.md`
- Root project overview: `../README.md`

## Remote Build Start Here

Remote validation and CodeFriend-scale build sessions should read these docs
before launching paid or remote work:

- [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md) - scheduler
  routing truth for local, Nessus, AWS Spot, CodeBuild, and Wuji.
- [Remote Build How-To](REMOTE_BUILD_HOW_TO.md) - reusable operator playbook
  for safe dry-runs, live runs, cache proof, and result recording.
- [ADL Builder Image](ADL_BUILDER_IMAGE.md) - shared builder-image contract,
  including the requirement to use a pre-published image instead of rebuilding
  it inside each validation run.
- [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md) - CodeBuild setup,
  wrapper usage, cache posture, account checks, and live-run guardrails.
- [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md) - Spot
  wrapper usage, warm EBS cache posture, SSH/debug affordances, and cleanup
  expectations.
- [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md) - Nessus
  remote validation-manager wrapper and artifact contract.
- [Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md) - historical timing
  rows and their recorded proof posture; local execution routes are retired.

Operational defaults:

- Use the Agent Logic AWS profile, `agent-logic-admin`, for ADL AWS work.
- Keep AWS Spot and CodeBuild as explicit operator-triggered paths; do not wire
  paid lanes into ordinary PR or push CI by accident.
- Keep cache posture visible in proof: Spot should show retained warm EBS cache,
  CodeBuild should show the fixed builder image plus stable target/cache setup,
  and Nessus should distinguish cold image-backed from warm target-cache rows.
- Do not claim Wuji image-backed parity until an ARM64 or multi-arch builder
  image exists.

## Core Tooling Areas

### Prompt and Card Surfaces

These docs describe the structured prompt surfaces used to shape issues, issue-card lifecycle state, outcome records, and deterministic reviewer flows.

- [Prompt Spec](prompt-spec.md)
- [ADL Card Lifecycle](card-lifecycle.md)
- [Active Card Lifecycle Migration Readiness](active-card-lifecycle-migration-readiness-v0.91.2.md)
- [Structured Prompt Contracts](structured-prompt-contracts.md)
- [Structured Prompt Validator Binary Resolution](structured-prompt-validator-binary-resolution.md)
- [SRP, SOR, And ObsMem Handoff Model](srp-sor-obsmem-handoff-v0.91.2.md)
- [Prompt/Reviewer Surface Mapping](prompt-review-surface-mapping.md)
- [Prompt Spec Protocol Bindings](prompt-spec.md#protocol-bindings)
- [Issue Prompt Templates](issue-prompts/README.md)

### Portable ADL Project Surfaces

These docs and templates describe how external repositories can declare ADL
C-SDLC workflow policy without copying the full ADL toolchain.

- [Portable ADL Project Adapter Contract](PORTABLE_ADL_PROJECT_ADAPTER_CONTRACT_v0.91.5.md)
- [Portable ADL Project Doctor Plan](PORTABLE_ADL_PROJECT_DOCTOR_PLAN_v0.91.5.md)
- [Portable ADL Adapter Follow-Ons](PORTABLE_ADL_PROJECT_ADAPTER_FOLLOW_ONS_v0.91.5.md)
- [Portable ADL Templates](../templates/portable-adl/README.md)

### Reviewer and Validation Surfaces

These docs describe bounded reviewer behavior, deterministic output formats, and provenance/review validation surfaces.

- [Reviewer Surface](reviewer-surface.md)
- [Reviewer Output Provenance](reviewer-provenance.md)
- [Card Reviewer GPT Instructions](card-reviewer-gpt.md)
- [Deterministic Review Output Format](card-review-output-format.md)

Stable reviewer regression fixture:
- `docs/tooling/examples/reviewer-regression/issue-661/input_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/output_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/expected_review_output_661.yaml`

### Editor and Authoring Surfaces

These docs describe the bounded editor and authoring surfaces used in the v0.85 authoring/control-plane work.

- [Task Bundle Editor](editor/README.md)
- [Milestone Dashboard](milestone-dashboard/README.md)
- `editor/five_command_demo.md`
- `editor/five_command_regression_suite.md`

### Worktree and Maintainability Surfaces

These docs describe worktree governance, large-module tracking, and related maintenance guidance.

- [Worktree Governance](worktree_governance.md)
- [Session Coordination And Root Checkout Policy](SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md)
- Rust module size reports are local operational artifacts under `.adl/reports/manual/`; regenerate them with `./adl/tools/report_large_rust_modules.sh`
- [WP Issue-Wave Generation](WP_ISSUE_WAVE_GENERATION.md)
- [Historical Public Task Records](../records/README.md)

## Tooling Scripts and Utilities

Important repo-local tooling surfaces include:

- `.adl/bin/native-v3/csdlc doctor` — typed readiness and drift diagnostics
- `csdlc issue` and `csdlc bind` — typed issue preparation and exact
  Git-topology binding
- `csdlc validate`, `review`, and `publish` — typed validation,
  exact-revision review, and publication routes
- `adl-lint-prompt-spec` — direct Prompt Spec lint and validation binary
- `adl-prompt-template` — direct prompt-card editor and renderer binary
- `adl-validate-structured-prompt` — direct structured prompt contract validator
- `bash adl/tools/run_aws_codefriend_build_lane.sh` — manual GitHub Actions plus AWS CodeBuild lane wrapper for CodeFriend build orchestration; see [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md)
- CI log archival is not exposed through the removed tooling multiplexer; see
  [CI Log Archive To S3](CI_LOG_ARCHIVE_S3.md) for historical context.
- `adl/tools/validation_manager.py --run` — emit durable `adl.build_action_log.v1` packets for local validation actions; see [Build Action Logs](BUILD_ACTION_LOGS.md)
- `csdlc validate` — current native v3 lifecycle/card validation; see
  [structured prompt validation boundary](structured-prompt-validator-binary-resolution.md)
- current direct owner binaries and typed skills own planning, provenance, and
  review validation; the removed `adl tooling` multiplexer is not an
  operational route.
- `bash adl/tools/demo_v0871_operator_surface.sh` — canonical `v0.87.1` operator-surface demo for runtime bring-up and proof-surface inspection
- [Historical v0.87.1 aggregate](V0871_HISTORICAL_AGGREGATE.md) — retired runtime reviewer walkthrough and aggregate assertion custody.
- `bash adl/tools/run_validation_manager_nessus_lane.sh` — validation-manager
  wrapper that consumes one eligible local lane and routes it to the Nessus
  remote runner; see [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md)
- [ADL Builder Image](ADL_BUILDER_IMAGE.md) — reviewed Infrastructure owner
  source and offline contract routing; ADL's source setup/import commands are retired.
- [AWS Spot execution owner](AWS_SPOT_REMOTE_EXECUTION_HOW_TO.md) — separately
  governed owner instructions; offline contracts do not authorize cloud execution.
- `bash adl/tools/validation_manager.sh --platform-routing` — first-class
  local, Nessus, AWS Spot, CodeBuild, and wuji routing decisions for scheduler
  consumption without launching paid cloud resources; see
  [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md)
- `bash adl/tools/run_aws_spot_remote_validation_lane.sh` — AWS Spot EC2
  remote validation wrapper that checks the `agent-logic-admin` account against
  retained Agent Logic proof before launch and reuses the retained warm EBS
  cache volume by default; see
  [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md)
- [Historical build-platform benchmarks](BUILD_PLATFORM_BENCHMARKS.md) preserve
  the original timing workload and evidence. The ADL command is retired;
  use separately admitted Infrastructure owner routes for new execution.
- lifecycle authority is native v3 via `.adl/bin/native-v3/csdlc`, subject to its selector and authenticated receipt; retained v2 binaries require explicit rollback or transition authorization
- `adl/tools/report_large_rust_modules.sh` — non-blocking Rust source-and-test module size report; by default it scans both `adl/src` and `adl/tests`, and current snapshots should live under `.adl/reports/manual/` instead of tracked repo docs
- Legacy draft-to-task-bundle synchronization has been retired. Current cards
  are governed by native C-SDLC editing and binding; historical `.adl/cards`
  and `.adl/issues` layouts are not current write targets.

Historical evidence may mention removed v1 aliases, but active tests and docs
must not expose them as executable control-plane routes.

## Historical status snapshot

- Current closure milestone: **v0.87**
- Next active milestone: **v0.87.1**
- Role of this directory: tooling/reference entrypoint for prompt, reviewer, editor, and maintenance surfaces

## Runtime Operator Surface

For `v0.87.1`, the bounded runtime operator contract is:
- one canonical runtime invocation path via `adl-runtime run <adl-file> ...`
- one canonical runtime-root marker at `runtime_environment.json`
- one canonical per-run inspection set rooted at `run_summary.json`, `run_status.json`, and `logs/trace_v1.json`

Use `bash adl/tools/demo_v0871_operator_surface.sh` as the smallest repo-local proof of that operator surface.

## Runtime Review Surface

For `v0.87.1`, the bounded runtime reviewer contract is:
- a historical D8 walkthrough whose original command is retired; see [aggregate custody](V0871_HISTORICAL_AGGREGATE.md)
- one canonical package manifest at `artifacts/v0871/review_surface/demo_manifest.json`
- one canonical reviewer guide at `artifacts/v0871/review_surface/README.md`
- one stable package ordering rooted in D6 operator proof and D7 runtime-state proof

This historical review package is not an executable ADL proof route. Its original
assertions remain in Git and retirement custody; owner component results do not
qualify the retired aggregate.

## Notes

Tooling docs should be read as bounded engineering references. They describe the surfaces that support ADL authoring, review, and maintenance without claiming that every internal helper is equally important to every reader.

## Current Retirement Boundary

The active `adl/tools` surface intentionally excludes a small set of retired legacy
residue that no longer supports the live PR workflow, current demos, or active
regression tests. The `v0.87` bounded cleanup retired:

- `BURST_PLAYBOOK.md`
- `REPORT_SCHEMA.md`
- `default.rules.profiles.example`
- `demo_v0_4.sh`
- `pr_smoke.sh`

## Runtime proof ownership

Runtime-specific provider-neutral, gateway and lifecycle proofs run in
[agent-logic-runtime](https://github.com/agent-logic/agent-logic-runtime).
Paper Sonata expansion belongs to Infrastructure. ADL retains public dispatch
and consumer-contract coverage; owner proof execution is separate.
The [retirement record](../archives/RUNTIME_OWNER_1383.json) preserves exact
source identities, verified owner resources and historical recovery instructions.

## Runtime operator guide custody

These split Runtime documents are preserved at the exact owner revision below.
Document transfer does not establish cloud deployment, provider calls, service
health or current native acceptance. Retained ADL tests and executable fixtures
keep their separate qualification requirements.

- [docs/providers/provider-profile-hot-loading.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/providers/provider-profile-hot-loading.md)
- [docs/tooling/CALL_CLAUDE_FABLE_5.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/CALL_CLAUDE_FABLE_5.md)
- [docs/tooling/CODEX_USAGE_WATCHER.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/CODEX_USAGE_WATCHER.md)
- [docs/tooling/LOCAL_POLIS_SSM_OPERATIONS.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/LOCAL_POLIS_SSM_OPERATIONS.md)
- [docs/tooling/RUNTIME_AWS_ACIP_SNS.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/RUNTIME_AWS_ACIP_SNS.md)
- [docs/tooling/RUNTIME_AWS_HEARTBEAT.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/RUNTIME_AWS_HEARTBEAT.md)
- [docs/tooling/RUNTIME_AWS_SIGNAL_INTEGRATION.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/RUNTIME_AWS_SIGNAL_INTEGRATION.md)
- [docs/tooling/RUNTIME_CLOUD_CONTROL_CLOUDFRONT.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/RUNTIME_CLOUD_CONTROL_CLOUDFRONT.md)
- [docs/tooling/RUNTIME_POLIS_DURABLE_STORAGE.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/tooling/RUNTIME_POLIS_DURABLE_STORAGE.md)

## Remaining split guide and readback custody

The following exact owner resources preserve historical documentation or
separately bounded readback behavior. None establishes live provider/cloud or
current native acceptance. The Runtime copies of Observatory and GCP runbooks
predate ADL Infrastructure routing repairs: their Terraform paths must instead
be resolved in [Infrastructure at the reviewed revision](https://github.com/agent-logic/agent-logic-infrastructure/tree/61c2806b6274b1003aa42f02adec99faf5f2a74b/infra).
The Runtime welcome guide adds installed-default orientation documentation;
this transfer does not validate that runtime behavior.

- [demos/v0.8/godel_failure_hypothesis_experiment.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/demos/v0.8/godel_failure_hypothesis_experiment.md)
- [docs/operations/cloud/aws/observatory/S3_CLOUDFRONT_DEPLOYMENT_RUNBOOK.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/operations/cloud/aws/observatory/S3_CLOUDFRONT_DEPLOYMENT_RUNBOOK.md)
- [docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh](https://github.com/agent-logic/agent-logic-infrastructure/blob/61c2806b6274b1003aa42f02adec99faf5f2a74b/docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh)
- [docs/operations/cloud/gcp/WARM_POLIS_SNAPSHOT_RUNBOOK.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/operations/cloud/gcp/WARM_POLIS_SNAPSHOT_RUNBOOK.md)
- [docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh](https://github.com/agent-logic/agent-logic-infrastructure/blob/61c2806b6274b1003aa42f02adec99faf5f2a74b/docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh)
- [docs/runtime/WELCOME_RECOVERY.md](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/docs/runtime/WELCOME_RECOVERY.md)
- [docs/tooling/AWS_SPOT_REMOTE_VALIDATION_WRAPPER_CONTRACT.mmd](https://github.com/agent-logic/agent-logic-infrastructure/blob/61c2806b6274b1003aa42f02adec99faf5f2a74b/docs/tooling/AWS_SPOT_REMOTE_VALIDATION_WRAPPER_CONTRACT.mmd)

## Manual tools after the repository split

- The [burst summary template](https://github.com/agent-logic/cognitive-sdlc/blob/d316340c212800ff523f957b1bf290db68d8efad/docs/skill-distributions/planning-utilities/original/adl/tools/BURST_FINAL_SUMMARY_TEMPLATE.md) is archived planning evidence; it is not a current lifecycle command.
- [Browser route diagnostics](https://github.com/agent-logic/agent-logic-infrastructure/blob/ed02d55985482a0dbb1469951d36efef931b63e2/adl/tools/diagnose_browser_routes.py) are maintained in Infrastructure. Application lookup, executable version checks and optional HTTP/headless probes do not prove browser interaction. This source transfer did not run those probes.
- [Git main synchronization](https://github.com/agent-logic/cognitive-sdlc/blob/d316340c212800ff523f957b1bf290db68d8efad/adl/tools/fix_git_main_sync.sh) now requires an explicit authenticated installed resource root and delegates to the owner's [Git administration helper](https://github.com/agent-logic/cognitive-sdlc/blob/d316340c212800ff523f957b1bf290db68d8efad/tools/git_main_sync.py). It changes Git state and is not native lifecycle reconciliation. The old ADL checkout/fetch/fast-forward wrapper is retired; no Git operation or equivalence test was performed for this transfer.
