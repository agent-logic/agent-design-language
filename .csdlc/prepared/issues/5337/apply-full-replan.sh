#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"
editor=/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-edit
record=.csdlc/issues/5337/index.json
actor=codex:019f8188-089f-7580-8ae8-fffef0d52539
claim=claim-5337-v0918-wp03-implementation
sequence=0

apply_op() {
  local card=$1
  local slug=$2
  local reason=$3
  local operation=$4
  local generation digest request
  sequence=$((sequence + 1))
  generation=$(jq -r .generation "$record")
  digest=$(jq -r .digest "$record")
  request=$(printf '.csdlc/prepared/issues/5337/replan-%02d-%s.json' "$sequence" "$slug")
  jq -n \
    --argjson issue 5337 \
    --arg card "$card" \
    --argjson generation "$generation" \
    --arg digest "$digest" \
    --arg claim "$claim" \
    --arg actor "$actor" \
    --arg reason "$reason" \
    --argjson operation "$operation" \
    '{issue:$issue,card:$card,expected_generation:$generation,expected_digest:$digest,claim_id:$claim,actor:$actor,reason:$reason,operation:$operation,fail_after_backup:false}' > "$request"
  "$editor" --repo . apply --request "$request" >/dev/null
}

apply_op sip goal "Replace preparation-only goal with the full implementation goal" \
  '{"operation":"replan","field":"goal","value":"Implement and prove an independent, versioned characterization and determinism corpus for pinned ADL v1 behavior."}'
apply_op sip required-outcome "Replace preparation-only outcome with complete issue acceptance" \
  '{"operation":"replan","field":"required_outcome","value":"A complete adl-characterization crate, versioned corpus, narrow normalizer contract, at least three retained v1 observations per case, coverage map, deterministic reports, focused and full tests, exact-revision review, and typed publication all pass with no deferred criteria."}'
apply_op sip declared-scope "Declare the complete issue-owned implementation scope" \
  '{"operation":"replace_planning_collection","field":"declared_scope","values":["independent adl-characterization Rust crate and CLI","versioned positive and negative corpus fixtures and schema","pinned-v1 repeated raw and normalized observation evidence","normalizer contract, coverage map, deterministic reports, tests, and documentation","issue-local typed lifecycle, validation, review, and publication records"]}'
apply_op sip authority-boundary "Record clean-room and execution authority" \
  '{"operation":"replace_planning_collection","field":"authority_boundary","values":["Issue #5337 and the operator instruction authorize the complete WP-03 implementation","Pinned revision 19c2b6e2ad18bddc75db9231643a54b2a446ce72 is behavioral evidence only","The independent harness invokes a caller-supplied v1 binary and does not depend on incumbent ADL Rust code","Typed C-SDLC v2 binaries and records are lifecycle authority","No credentialed, network, remote, or AWS provider execution is authorized"]}'
apply_op sip assumptions "Make implementation assumptions explicit" \
  '{"operation":"replace_planning_collection","field":"initial_assumptions","values":["the pinned v1 revision can be built locally with Cargo output on /Volumes/FastWork","ADL_OBSERVABILITY=0 suppresses ordinary observability noise for black-box capture","fixed local fixtures and mock providers are sufficient to exercise the declared behavioral surface"]}'
apply_op sip constraints "Replace preparation-only constraints with full implementation constraints" \
  '{"operation":"replace_operator_constraints","values":["Use installed typed C-SDLC v2 binaries and card-editor semantics only","Implement every acceptance criterion in this issue; do not defer product work","Do not edit incumbent adl, Runtime v2, main, sibling worktrees, or shared milestone files","Use /Volumes/FastWork for Cargo output","Do not use AWS, raw gh, credentials, or network providers","Use COTS crates for parsing, schema validation, assertions, and temporary files"]}'

apply_op stp task-boundary "Define the complete bounded implementation task" \
  '{"operation":"replan","field":"task_boundary","value":"Build, populate, document, and prove the independent adl-characterization crate and its complete pinned-v1 corpus; modify only adl-characterization and issue-local #5337 lifecycle/evidence paths."}'
apply_op stp deliverables "Replace preparation deliverables with complete product deliverables" \
  '{"operation":"replace_planning_collection","field":"deliverables","values":["standalone adl-characterization library and adl-characterize CLI","versioned corpus manifest and JSON Schema","positive and negative ADL fixtures covering every required behavior","three or more raw pinned-v1 observations per corpus case plus deterministic normalized outcomes","declared normalizer contract and implementation that preserves semantic arrays, identifiers, errors, exits, and signature verdicts","complete behavior-to-case coverage map with fail-closed validation","focused unit, integration, CLI, and full-crate tests","README, architecture design, evidence manifest, and exact-revision review/publication proof"]}'
apply_op stp dependencies "Replace preparation dependencies with executable dependencies" \
  '{"operation":"replace_planning_collection","field":"dependencies","values":["operator-authorized #5337 implementation and bound worktree","pinned incumbent revision 19c2b6e2ad18bddc75db9231643a54b2a446ce72","locally built caller-supplied v1 binary with no network or credentials","WP-02 #5336 architecture denominator as planning context; no source dependency"]}'
apply_op stp repo-inputs "Record exact implementation inputs" \
  '{"operation":"replace_planning_collection","field":"repo_inputs","values":["GitHub issue #5337 retrieved through adl-issue","docs/milestones/v0.91.8/DESIGN_v0.91.8.md","docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md","docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml","pinned ADL v1 public CLI behavior at 19c2b6e2ad18bddc75db9231643a54b2a446ce72","adl/tests/cli_smoke behavior names used only to identify public black-box surfaces"]}'
apply_op stp non-goals "Remove preparation non-goals and preserve clean-room limits" \
  '{"operation":"replace_planning_collection","field":"non_goals","values":["no porting or linking of incumbent ADL internal implementation or tests","no Runtime v2 or incumbent adl source edits","no replacement ADL compiler or runtime implementation","no credentialed, network, remote, AWS, or paid-provider execution","no normalization that erases semantic differences","no shared milestone, cutover, deletion, or v0.92 changes"]}'

apply_op spp acceptance-plan "Atomically replace acceptance criteria, implementation steps, and validation lanes" \
  '{"operation":"replace_acceptance_plan","acceptance_criteria":["AC-1: The standalone adl-characterization crate and CLI build without depending on the incumbent adl crate and pin v1 revision 19c2b6e2ad18bddc75db9231643a54b2a446ce72 in the corpus contract","AC-2: The versioned schema-valid corpus covers CLI help/version, six-primitives print-plan, graph JSON, prompts, fork/join, map and branch reorder equivalence, sequential reorder difference, argument/YAML/schema/reference/state/cycle negatives, repeated byte stability, local mock execution, and fixed Ed25519 sign/verify/tamper","AC-3: Every corpus case has at least three retained raw v1 observations recording binary digest, arguments, exit status, stdout, stderr, repetition, and derived normalized result","AC-4: Normalization is declared per case, limited to JSON object-key order, declared root prefixes, named volatile fields, and exact observability lines, while preserving arrays, IDs, errors, values, exits, prompt order, and signature verdicts","AC-5: The comparator proves repeated stability, declared equivalence and semantic difference groups, expected exits/fragments, and fails on unexplained nondeterminism or overbroad/no-op normalization","AC-6: A complete coverage map assigns every required behavior to known corpus cases and rejects missing, duplicate, or unknown mappings","AC-7: Focused unit, integration, CLI, schema, evidence, and full-crate validation pass locally with Cargo output on /Volumes/FastWork and without AWS, credentials, or network providers","AC-8: Exact-revision bounded review has no unresolved actionable findings, typed C-SDLC review/publication truth is current, and no acceptance criterion is deferred"],"steps":[{"id":"S1","action":"Implement the independent crate, typed manifest/schema model, runner, normalizer, comparator, report model, and CLI","acceptance_ids":["AC-1","AC-4","AC-5"],"status":"pending"},{"id":"S2","action":"Author the complete versioned positive and negative fixture corpus and behavior coverage map","acceptance_ids":["AC-2","AC-6"],"status":"pending"},{"id":"S3","action":"Build the pinned v1 binary locally and capture at least three immutable observations for every case","acceptance_ids":["AC-1","AC-3"],"status":"pending"},{"id":"S4","action":"Verify normalization, equivalence, semantic differences, expected failures, coverage, and deterministic reports","acceptance_ids":["AC-3","AC-4","AC-5","AC-6"],"status":"pending"},{"id":"S5","action":"Run focused and full validation with external Cargo output and retain exact evidence","acceptance_ids":["AC-7"],"status":"pending"},{"id":"S6","action":"Run bounded exact-revision review, fix every actionable finding, and publish through typed lifecycle gates","acceptance_ids":["AC-8"],"status":"pending"}],"validation_lanes":[{"lane":"characterization-unit-and-integration","proof_role":"Prove manifest/schema, runner, normalization, comparison, coverage, and negative safety contracts","acceptance_ids":["AC-1","AC-2","AC-4","AC-5","AC-6","AC-7"],"deterministic":true,"resource_profile":"medium","budget_seconds":900,"budget_tokens":4000,"argv":["cargo","test","--manifest-path","adl-characterization/Cargo.toml","--all-targets"],"parallel_group":"local-rust","defer_reason":null},{"lane":"pinned-v1-corpus-verification","proof_role":"Verify all retained repeated observations and the complete coverage map against the versioned corpus","acceptance_ids":["AC-2","AC-3","AC-4","AC-5","AC-6","AC-7"],"deterministic":true,"resource_profile":"medium","budget_seconds":900,"budget_tokens":4000,"argv":["cargo","run","--manifest-path","adl-characterization/Cargo.toml","--bin","adl-characterize","--","verify","--corpus","adl-characterization/corpus/v1/corpus.yaml","--observations","adl-characterization/observations/v1"],"parallel_group":"local-proof","defer_reason":null},{"lane":"format-and-lint","proof_role":"Prove the standalone crate is formatted and warning-free under strict Clippy","acceptance_ids":["AC-1","AC-7"],"deterministic":true,"resource_profile":"small","budget_seconds":600,"budget_tokens":2000,"argv":["cargo","clippy","--manifest-path","adl-characterization/Cargo.toml","--all-targets","--","-D","warnings"],"parallel_group":"local-rust","defer_reason":null},{"lane":"typed-review-and-publication","proof_role":"Prove exact-revision review, resolved findings, lifecycle integrity, and publication readiness","acceptance_ids":["AC-8"],"deterministic":true,"resource_profile":"small","budget_seconds":600,"budget_tokens":2000,"argv":["/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor","--repo",".","--issue","5337"],"parallel_group":"local-review","defer_reason":null}]}'

apply_op spp plan-summary "Replace preparation plan with the complete execution plan" \
  '{"operation":"replan","field":"plan_summary","value":"Implement the standalone harness and full corpus, capture the pinned v1 behavior at least three times per case, verify narrow normalization and coverage fail closed, run all crate proof, fix exact-revision review findings, and publish only with every acceptance criterion complete."}'
apply_op spp affected-areas "Declare only issue-owned implementation areas" \
  '{"operation":"replace_planning_collection","field":"affected_areas","values":["adl-characterization crate, corpus, observations, tests, and documentation","issue-local .csdlc/issues/5337, .csdlc/prepared/issues/5337, and .csdlc/evidence/5337"]}'
apply_op spp invariants "Replace preparation invariants with implementation invariants" \
  '{"operation":"replace_planning_collection","field":"invariants","values":["no tracked work on main or sibling worktrees","no dependency on or edits to incumbent adl or Runtime v2 source","raw observations remain immutable and normalized observations remain derived","array order, IDs, errors, exits, prompt content/order, and signature verdicts are semantic","every case repeats at least three times","network and credential variables are denied to child processes","every acceptance criterion has required local proof and none is deferred","all card changes use typed C-SDLC v2 operations"]}'
apply_op spp risks "Record complete implementation risks" \
  '{"operation":"replace_planning_collection","field":"risks","values":["normalization could hide a semantic regression","graph output alone may hide sequential ordering, requiring print-plan comparison","environmental noise may cause false nondeterminism","fixed signing evidence could accidentally retain private material","large raw evidence could obscure corpus coverage","v1 build provenance could drift from the pinned revision"]}'
apply_op spp stop-conditions "Fail closed on incomplete or unsafe implementation" \
  '{"operation":"replace_planning_collection","field":"stop_conditions","values":["the v1 binary cannot be proven to come from the pinned revision","any case requires credentials, network, remote, or AWS execution","a normalizer would need to erase semantic arrays, identifiers, errors, exits, or signature verdicts","any required behavior lacks three retained observations or coverage mapping","unexplained repeated-run divergence remains","exact-revision review has unresolved actionable findings","publication would require bypassing typed lifecycle truth"]}'
apply_op spp replan-triggers "Record bounded replan triggers" \
  '{"operation":"replace_planning_collection","field":"replan_triggers","values":["the public v1 CLI shape at the pinned revision differs from a planned command","a required behavior needs a safer local fixture to avoid network or credentials","a declared normalizer rule matches no evidence or masks a semantic difference","the coverage map reveals a required behavior with no executable case"]}'

apply_op srp review-prompts "Replace preparation review prompts with full code and architecture review prompts" \
  '{"operation":"replace_planning_collection","field":"review_prompts","values":["Does the crate remain independent of incumbent ADL source and accept only a caller-supplied pinned v1 binary?","Does the corpus cover every required positive, negative, ordering, determinism, mock-execution, and signing behavior?","Can any normalizer rule erase array order, identifiers, error class/value, exit status, prompt order/content, or signature verdicts?","Are all cases repeated at least three times with immutable raw evidence and exact binary provenance?","Do equivalence, difference, stability, and coverage checks fail closed on unexplained or missing evidence?","Can any command execute a network, credentialed, remote, AWS, or paid provider?","Are tests PVF-classified, deterministic, complete, and run with external Cargo output?","Are all findings resolved at the exact substantive revision with no deferred acceptance criteria?"]}'
apply_op srp review-scope "Bind review to the complete exact implementation revision" \
  '{"operation":"replan","field":"review_scope","value":"Exact #5337 implementation revision: adl-characterization source, corpus schema and fixtures, retained repeated v1 observations, normalizer contract, coverage map, tests, documentation, issue-local lifecycle/evidence, and all no-network/no-credential boundaries."}'

apply_op spp start-s1 "Start the complete implementation plan after typed replanning" \
  '{"operation":"update_plan_step","step_id":"S1","status":"in_progress"}'
