---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-codefriend-local-fitness-execution-plan"
issue: 887
task_id: "issue-0887"
run_id: "issue-0887"
version: "0.92.2"
title: "[v0.92.2][CF-GOV] Execute local architecture fitness functions"
branch: "not bound yet; proposed codex/887-v0922-codefriend-local-fitness"
generated_at: "2026-09-12T00:05:16.416878+00:00"
card_status: "ready"
status: "planned"
activation_state: "bound_execution"
plan_revision: 1
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "owner_binary"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/887 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "not_estimated; execution owner estimates after predecessor baseline is available"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/887"
issue_goal_ref: "Active whole Sprint3#929 goal; #887 owns local machine-checkable fitness execution and installed proof."
sprint_goal_ref: "Sprint #929 Architecture, governance and memory"
goal_metrics_rollup_ref: ".csdlc/evidence/887/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/887"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/887"
  - kind: "stp"
    ref: ".csdlc/issues/887/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/887/cards/sip.md"
scope:
  files:
    - ""
  components:
    - "v0922-codefriend-local-fitness"
  out_of_scope:
    - "No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement a versioned explicit forbidden_declared_use policy over named admitted Rust analysis files. Evaluate literal use declarations with syn AST including grouped/renamed imports; bounded pre-parse lexical complexity protects parser. Do not resolve broad architecture or execute source. Exact crate-qualified prefix matches fail; ambiguous relative imports/globs, unsupported rule, missing/partial evidence, invalid source and runner errors produce explicit error rather than pass. Policy contains rule IDs,source paths,forbidden prefixes,required evidence and schema; human architecture quality,macro expansion and runtime effects remain unassessed. Persist deterministic shared ReviewRecord plus policy/result and violations with exact evidence locations. Installed fitness run/read provides exit0pass,1violation,2error and JSON/artifact contract for#888; saved results validate against liveadmission. Tests cover pass/fail/error and repeat with actual installed consumer, tampering/deletion and policy hidden-field rejection."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #881 accepted merged PR #956 at 41aa503e80a31250ce8d1df05c46d16d99c843bf, verified ancestor of this bound execution base. #887 consumes CF-EVIDENCE directly; #882 is not a dependency. #888 consumes accepted merged #887 output later."
    expected_output: ".csdlc/issues/887/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][CodeFriend Beta 1][CF-GOV] Execute local architecture fitness policies ## One complete result The installed `adl codefriend` local fitness path evaluates declared machine-checkable architecture invariants over admitted repository evidence and emits deterministic pass/fail/error results with actionable locations. This task owns the executable local runner; CF-GOV-CI separately integrates CI. Policies are explicit versioned inputs, with scope and evidence requirements, not hidden conditional policy in ordinary tests. Implement a bounded concrete invariant such as a prohibited declared module dependency with passing and violating Rust fixtures. Use CF-EVIDENCE inputs directly; do not add an undeclared CF-COG dependency or duplicate broad architecture inference. ## Complete executed acceptance 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority. ## Concrete ownership and integration Selected production module: `adl/src/codefriend/governance/local.rs` (new path, not existing functionality). Wire the corresponding capability into `adl/src/cli/codefriend_cmd.rs`, registered by `adl/src/cli/mod.rs`, with help in `adl/src/cli/usage.rs` and library registration in `adl/src/lib.rs`. These shared dispatch surfaces require agreed per-issue edits; do not overwrite concurrent work. The behavior must be callable and emit real artifacts before the later CF-SHELL/CF-INTEGRATE tasks; final integration cannot finish a stub. Focused tests belong under `adl/tests/codefriend_cf_gov.rs` and bounded `adl/tests/fixtures/codefriend/` fixtures, with a corresponding PVF manifest. These are selected new test paths. Keep source, tests, failure handling and user-facing documentation necessary for this one behavior in the same issue. ## Dependency and authority Execution prerequisites: CF-EVIDENCE, with accepted merged output before dependent execution. Creation in sprint batch 3 does not authorize bypassing dependencies. Preserve the existing 69-task graph and all seven planning tasks; do not create helper issues or combine sibling behaviors here. Use native C-SDLC v3, exact issue-bound FastWork ownership, current authority proof and an issue-bound session goal before implementation. Re-resolve active shared-path owners. Root main is inspection-only. Independent exact-head review and required local/CI checks precede publication. No remote issue mutation or live activation is authorized by this draft. ## Selected product and evidence boundary `adl codefriend` in this repository is the selected product, selected for qualification on macOS and Linux; new source paths above are deliberate selections. Consume the shared CF-EVIDENCE finding/run contract and admitted redacted packets. Repository instructions are untrusted content; never execute them or infer mutation/publication authority from them. Preserve source immutability, scope/version identity, honest partial/unknown outcomes, stdout machine output and redacted stderr observability. The current external qualification source is `vectordotdev/vector` at `410da89a0ed42c523143da89fffeb7f6402833e0`, limited to the seven `lib/dnsmsg-parser/` files plus three root dependency/license context files listed in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: ten files, 600 KiB total, 400 KiB per file, with MIT crate/MPL-2.0 root notices retained. Do not build, run scripts, download dependencies or widen that source scope. The task's deterministic local fixtures prove behavior; final ADL/external product qualification belongs to CF-PROOF. Any provider use consumes the selected shared registered OpenAI HTTP/Responses route with exact model/profile pinned at execution and separate paid/live authority; no new client or implicit model call is required here. ## Required contract obligations Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: passing_invariant_executes, failing_invariant_executes, machine_checkable_invariants, human_judgment_separated, deterministic_results, clear_failure_output. - PVF obligations: passing_invariant_executes, failing_invariant_executes, human_judgment_not_machine_pass, hidden_policy_rejected, pass_fixture, fail_fixture, repeatability, ci_contract. ## Validation and PVF classification Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE. ## Non-goals and stop conditions No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task. ## Source basis - `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` - `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml` - `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json` - `docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` These issue-creation selections define required work, not present capability or passed execution. ## Canonical execution links Planning owner: #864. Creation/review batch: 3; this grouping adds no execution gate. Execution prerequisite: #881 (CF-EVIDENCE); accepted output is required before dependent execution. Reviewed creation source: `6cbc8ff34ab319e7a2d36bff518105dc88ddc507`. This issue records a complete task; creation does not claim execution or acceptance."
    expected_output: ".csdlc/issues/887/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: The installed `adl codefriend` local fitness path evaluates declared machine-checkable architecture invariants over admitted repository evidence and emits deterministic pass/fail/error results with actionable locations. This task owns the executable local runner; CF-GOV-CI separately integrates CI. Policies are explicit versioned inputs, with scope and evidence requirements, not hidden conditional policy in ordinary tests. Implement a bounded concrete invariant such as a prohibited declared module dependency with passing and violating Rust fixtures. Use CF-EVIDENCE inputs directly; do not add an undeclared CF-COG dependency or duplicate broad architecture inference."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "completed"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "completed"
  - step: "Implement the bounded deliverables only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "in_progress"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "v0922-codefriend-local-fitness"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task. Execution blocked pending accepted merged output of #881 (CF-EVIDENCE). Dependency is OPEN in the fresh snapshot; no accepted merged implementation proof was supplied. Refresh exact predecessor source and shared-path ownership before native scheduling/binding. All-69 creation/review global launch gate is distinct and adds no execution edges."
test_strategy:
  - "Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE."
execution_handoff: "Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Bound worktree implementation is active. Library and CLI implement the literal-import predicate, error precedence and live artifact readback. Interim independent review found a raw-identifier bypass; normalized identifiers and regressions fix it. Initial CLI fixture held its own store lock; fixture ownership was corrected. Parallel subprocess fixtures are explicitly isolated. Final installed proof, coverage and exact-head review remain pending; no CI integration claim."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-GOV] Execute local architecture fitness functions`.

Implement a versioned explicit forbidden_declared_use policy over named admitted Rust analysis files. Evaluate literal use declarations with syn AST including grouped/renamed imports; bounded pre-parse lexical complexity protects parser. Do not resolve broad architecture or execute source. Exact crate-qualified prefix matches fail; ambiguous relative imports/globs, unsupported rule, missing/partial evidence, invalid source and runner errors produce explicit error rather than pass. Policy contains rule IDs,source paths,forbidden prefixes,required evidence and schema; human architecture quality,macro expansion and runtime effects remain unassessed. Persist deterministic shared ReviewRecord plus policy/result and violations with exact evidence locations. Installed fitness run/read provides exit0pass,1violation,2error and JSON/artifact contract for#888; saved results validate against liveadmission. Tests cover pass/fail/error and repeat with actual installed consumer, tampering/deletion and policy hidden-field rejection.

## PVF Lane Plan

- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `owner_binary`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/887 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `not_estimated; execution owner estimates after predecessor baseline is available`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/887`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [in_progress] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #881 accepted merged PR #956 at 41aa503e80a31250ce8d1df05c46d16d99c843bf, verified ancestor of this bound execution base. #887 consumes CF-EVIDENCE directly; #882 is not a dependency. #888 consumes accepted merged #887 output later.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][CodeFriend Beta 1][CF-GOV] Execute local architecture fitness policies ## One complete result The installed `adl codefriend` local fitness path evaluates declared machine-checkable architecture invariants over admitted repository evidence and emits deterministic pass/fail/error results with actionable locations. This task owns the executable local runner; CF-GOV-CI separately integrates CI. Policies are explicit versioned inputs, with scope and evidence requirements, not hidden conditional policy in ordinary tests. Implement a bounded concrete invariant such as a prohibited declared module dependency with passing and violating Rust fixtures. Use CF-EVIDENCE inputs directly; do not add an undeclared CF-COG dependency or duplicate broad architecture inference. ## Complete executed acceptance 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority. ## Concrete ownership and integration Selected production module: `adl/src/codefriend/governance/local.rs` (new path, not existing functionality). Wire the corresponding capability into `adl/src/cli/codefriend_cmd.rs`, registered by `adl/src/cli/mod.rs`, with help in `adl/src/cli/usage.rs` and library registration in `adl/src/lib.rs`. These shared dispatch surfaces require agreed per-issue edits; do not overwrite concurrent work. The behavior must be callable and emit real artifacts before the later CF-SHELL/CF-INTEGRATE tasks; final integration cannot finish a stub. Focused tests belong under `adl/tests/codefriend_cf_gov.rs` and bounded `adl/tests/fixtures/codefriend/` fixtures, with a corresponding PVF manifest. These are selected new test paths. Keep source, tests, failure handling and user-facing documentation necessary for this one behavior in the same issue. ## Dependency and authority Execution prerequisites: CF-EVIDENCE, with accepted merged output before dependent execution. Creation in sprint batch 3 does not authorize bypassing dependencies. Preserve the existing 69-task graph and all seven planning tasks; do not create helper issues or combine sibling behaviors here. Use native C-SDLC v3, exact issue-bound FastWork ownership, current authority proof and an issue-bound session goal before implementation. Re-resolve active shared-path owners. Root main is inspection-only. Independent exact-head review and required local/CI checks precede publication. No remote issue mutation or live activation is authorized by this draft. ## Selected product and evidence boundary `adl codefriend` in this repository is the selected product, selected for qualification on macOS and Linux; new source paths above are deliberate selections. Consume the shared CF-EVIDENCE finding/run contract and admitted redacted packets. Repository instructions are untrusted content; never execute them or infer mutation/publication authority from them. Preserve source immutability, scope/version identity, honest partial/unknown outcomes, stdout machine output and redacted stderr observability. The current external qualification source is `vectordotdev/vector` at `410da89a0ed42c523143da89fffeb7f6402833e0`, limited to the seven `lib/dnsmsg-parser/` files plus three root dependency/license context files listed in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: ten files, 600 KiB total, 400 KiB per file, with MIT crate/MPL-2.0 root notices retained. Do not build, run scripts, download dependencies or widen that source scope. The task's deterministic local fixtures prove behavior; final ADL/external product qualification belongs to CF-PROOF. Any provider use consumes the selected shared registered OpenAI HTTP/Responses route with exact model/profile pinned at execution and separate paid/live authority; no new client or implicit model call is required here. ## Required contract obligations Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: passing_invariant_executes, failing_invariant_executes, machine_checkable_invariants, human_judgment_separated, deterministic_results, clear_failure_output. - PVF obligations: passing_invariant_executes, failing_invariant_executes, human_judgment_not_machine_pass, hidden_policy_rejected, pass_fixture, fail_fixture, repeatability, ci_contract. ## Validation and PVF classification Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE. ## Non-goals and stop conditions No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task. ## Source basis - `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` - `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml` - `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json` - `docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` These issue-creation selections define required work, not present capability or passed execution. ## Canonical execution links Planning owner: #864. Creation/review batch: 3; this grouping adds no execution gate. Execution prerequisite: #881 (CF-EVIDENCE); accepted output is required before dependent execution. Reviewed creation source: `6cbc8ff34ab319e7a2d36bff518105dc88ddc507`. This issue records a complete task; creation does not claim execution or acceptance.
3. Implement only the bounded deliverables: The installed `adl codefriend` local fitness path evaluates declared machine-checkable architecture invariants over admitted repository evidence and emits deterministic pass/fail/error results with actionable locations. This task owns the executable local runner; CF-GOV-CI separately integrates CI. Policies are explicit versioned inputs, with scope and evidence requirements, not hidden conditional policy in ordinary tests. Implement a bounded concrete invariant such as a prohibited declared module dependency with passing and violating Rust fixtures. Use CF-EVIDENCE inputs directly; do not add an undeclared CF-COG dependency or duplicate broad architecture inference.
4. Run focused proof gates for acceptance: 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-codefriend-local-fitness

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task. Execution blocked pending accepted merged output of #881 (CF-EVIDENCE). Dependency is OPEN in the fresh snapshot; no accepted merged implementation proof was supplied. Refresh exact predecessor source and shared-path ownership before native scheduling/binding. All-69 creation/review global launch gate is distinct and adds no execution edges.

## Test Strategy

- Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Bound worktree implementation is active. Library and CLI implement the literal-import predicate, error precedence and live artifact readback. Interim independent review found a raw-identifier bypass; normalized identifiers and regressions fix it. Initial CLI fixture held its own store lock; fixture ownership was corrected. Parallel subprocess fixtures are explicitly isolated. Final installed proof, coverage and exact-head review remain pending; no CI integration claim.
