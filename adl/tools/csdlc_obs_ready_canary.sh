#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [[ "${ADL_OBS_READY_CANARY_INNER:-0}" != "1" ]]; then
  if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "csdlc_obs_ready_canary: tracked source changes must be committed before the isolated canary can test them" >&2
    exit 2
  fi
  source_head="$(git rev-parse HEAD)"
  short_head="${source_head:0:12}"
  canary_worktree="${ADL_OBS_READY_CANARY_WORKTREE:-/Volumes/FastWork/adl-worktrees/adl-obs-ready-canary-${short_head}}"
  if [[ ! -d "$canary_worktree/.git" && ! -f "$canary_worktree/.git" ]]; then
    git worktree add --detach "$canary_worktree" "$source_head" >/dev/null
  fi
  observed_head="$(git -C "$canary_worktree" rev-parse HEAD)"
  if [[ "$observed_head" != "$source_head" ]]; then
    echo "csdlc_obs_ready_canary: existing canary worktree is at $observed_head, expected $source_head: $canary_worktree" >&2
    exit 2
  fi
  ADL_OBS_READY_CANARY_INNER=1 bash "$canary_worktree/adl/tools/csdlc_obs_ready_canary.sh"
  exit
fi

ensure_v2_owner_binaries() {
  if [[ -x ./.adl/bin/csdlc-v2/csdlc-issue && -x ./.adl/bin/csdlc-v2/csdlc-validate && -x ./.adl/bin/csdlc-v2/csdlc-edit && -x ./.adl/bin/csdlc-v2/csdlc-doctor ]]; then
    return 0
  fi
  local build_root="${ADL_OBS_READY_CANARY_BUILD_ROOT:-/Volumes/FastWork/adl-builds/obs-ready-canary-csdlc-v2}"
  mkdir -p "$build_root" .adl/bin/csdlc-v2
  CARGO_TARGET_DIR="$build_root" cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-install -- install --repo . --destination "$ROOT/.adl/bin/csdlc-v2" >/dev/null
}

mkdir -p .csdlc/prepared/issues/511 .csdlc/prepared/issues/512 .csdlc/evidence/511 .csdlc/evidence/512

write_obs_a_inputs() {
  cat > .csdlc/prepared/issues/511/design.md <<'EOF'
# OBS-A design bootstrap

This design input is the GitHub issue-authored Observatory experience-design
contract request for agent-logic/agent-design-language#511. It is intentionally
bounded to design artifacts and validation planning, not production UI
implementation.
EOF

  cat > .csdlc/prepared/issues/511/diagram.mmd <<'EOF'
flowchart TD
  ISSUE["#511 OBS-A issue body"]
  CONTRACT["Experience design contract"]
  LANES["PVF lanes: information, states, accessibility, Runtime field census"]
  ISSUE --> CONTRACT --> LANES
EOF

  cat > .csdlc/prepared/issues/511/bootstrap-request.json <<'EOF'
{
  "issue": 511,
  "repository": "agent-logic/agent-design-language",
  "actor": "codex:worker-6",
  "design_path": ".csdlc/prepared/issues/511/design.md",
  "diagram_path": ".csdlc/prepared/issues/511/diagram.mmd",
  "design_reviewer": "pending",
  "design_approved": false,
  "initial": {
    "title": "[v0.92.1][OBS-A] Observatory experience design",
    "slug": "obs-a-observatory-experience-design",
    "version": "v0.92.1",
    "goal": "Produce one reviewed Observatory experience-design contract.",
    "required_outcome": "One reviewed Observatory design contract covering information, interaction, states, hierarchy, and accessibility.",
    "declared_scope": ["demos/html-observatory/design/**", "docs/observatory/**", "docs/milestones/v0.92.1/evidence/observatory/obs-a/**", ".csdlc/prepared/issues/511", ".csdlc/evidence/511", ".csdlc/issues/511"],
    "authority_boundary": ["Issue authority is agent-logic/agent-design-language#511", "This issue owns design only; production implementation belongs to #512", "Runtime fields must be sourced from existing Runtime contracts and evidence"],
    "operator_constraints": ["No tracked implementation work on main", "No mock Runtime fields", "No Unity or TLS implementation"],
    "task_boundary": "Exactly one Observatory experience-design contract and its validation plan.",
    "deliverables": ["Stable per-view information contract", "Empty degraded recovery and revoked-state matrix", "Keyboard and screen-reader flow specification", "Runtime-field census with source references", "Reviewed OBS-A evidence packet"],
    "acceptance_criteria": ["AC-1: Every view has a stable information contract", "AC-2: Empty degraded recovery and revoked states are designed", "AC-3: Keyboard and screen-reader flows are specified", "AC-4: No invented Runtime field is introduced", "AC-5: One-command pre-cutover canary passes with v2 authority and v3 local non-authority evidence"],
    "dependencies": [],
    "repo_inputs": ["agent-logic/agent-design-language#511", "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#OBS-A", "demos/html-observatory/**", "docs/api/runtime-v3/**", "adl-runtime-kernel/**"],
    "non_goals": ["Production implementation", "Unity TLS work", "Runtime API mutation", "Public exposure"],
    "plan_summary": "Define the Observatory experience contract, enumerate state and accessibility behavior, verify Runtime field provenance, and retain review-ready evidence.",
    "steps": [
      {"id":"S1","action":"Inventory the existing Observatory UI and Runtime projection fields that may be consumed.","acceptance_ids":["AC-1","AC-4"],"status":"pending"},
      {"id":"S2","action":"Write the per-view information contract and state matrix.","acceptance_ids":["AC-1","AC-2"],"status":"pending"},
      {"id":"S3","action":"Write keyboard and screen-reader flows for every designed state.","acceptance_ids":["AC-3"],"status":"pending"},
      {"id":"S4","action":"Run the focused contract, accessibility-plan, Runtime-field-census, v3 local canary, and review proof.","acceptance_ids":["AC-1","AC-2","AC-3","AC-4","AC-5"],"status":"pending"}
    ],
    "affected_areas": ["Observatory design docs", "Observatory evidence", "HTML Observatory design surface"],
    "invariants": ["Design does not invent Runtime fields", "Production implementation remains in #512", "V3 remains non-authoritative before #505", "Accessibility states are first-class design truth"],
    "risks": ["Existing Runtime projection docs may be incomplete", "Design may accidentally depend on future #512 implementation", "Accessibility denominator may be underspecified"],
    "planning_profile": "medium",
    "stop_conditions": ["A design requires unavailable Runtime authority", "Accessibility denominator is incomplete", "A mock or invented Runtime field is needed", "V3 local output is treated as lifecycle authority"],
    "validation_lanes": [
      {"lane":"information-contract","proof_role":"Verify every designed view has named fields, source, state behavior, and consumer responsibility.","acceptance_ids":["AC-1"],"deterministic":true,"resource_profile":"small","budget_seconds":120,"budget_tokens":1500,"argv":["bash",".csdlc/prepared/issues/511/validate-obs-a-contract.sh"],"parallel_group":"docs","defer_reason":"Issue-owned validator is an OBS-A implementation deliverable."},
      {"lane":"state-matrix","proof_role":"Verify empty degraded recovery and revoked states are explicitly covered.","acceptance_ids":["AC-2"],"deterministic":true,"resource_profile":"small","budget_seconds":120,"budget_tokens":1500,"argv":["bash",".csdlc/prepared/issues/511/validate-obs-a-states.sh"],"parallel_group":"docs","defer_reason":"Issue-owned validator is an OBS-A implementation deliverable."},
      {"lane":"accessibility-plan","proof_role":"Verify keyboard and screen-reader flows are specified for each view and state.","acceptance_ids":["AC-3"],"deterministic":true,"resource_profile":"small","budget_seconds":120,"budget_tokens":1500,"argv":["bash",".csdlc/prepared/issues/511/validate-obs-a-accessibility.sh"],"parallel_group":"docs","defer_reason":"Issue-owned validator is an OBS-A implementation deliverable."},
      {"lane":"runtime-field-census","proof_role":"Verify each field is sourced from current Runtime artifacts or rejected.","acceptance_ids":["AC-4"],"deterministic":true,"resource_profile":"small","budget_seconds":180,"budget_tokens":1800,"argv":["bash",".csdlc/prepared/issues/511/validate-obs-a-runtime-fields.sh"],"parallel_group":"runtime-census","defer_reason":"Issue-owned validator is an OBS-A implementation deliverable."},
      {"lane":"v3-local-canary","proof_role":"Run the single csdlc binary local preparation path as non-authoritative cutover evidence.","acceptance_ids":["AC-5"],"deterministic":true,"resource_profile":"small","budget_seconds":120,"budget_tokens":1000,"argv":["cargo","run","--locked","--manifest-path","csdlc-v3/Cargo.toml","--bin","csdlc","--","local","--request",".csdlc/prepared/issues/511/v3-local-request.json","--registry","docs/templates/prompts/current.json","--registrations",".csdlc/prepared/issues/511/v3-local-registrations.json"],"parallel_group":"cutover-canary","defer_reason":null}
    ],
    "failure_policy": "Fail closed on invented Runtime fields, incomplete accessibility denominator, production implementation drift, or v3 authority overclaim.",
    "review_prompts": ["Does every designed field cite real Runtime authority?", "Are degraded and revoked states user-visible and testable?", "Can #512 implement this without design invention?"],
    "review_scope": "OBS-A design contract, accessibility/state coverage, Runtime-field provenance, and pre-cutover canary evidence only."
  }
}
EOF

  cat > .csdlc/prepared/issues/511/v3-local-request.json <<'EOF'
{
  "issue": 511,
  "title": "[v0.92.1][OBS-A] Observatory experience design",
  "repository": "agent-logic/agent-design-language",
  "branch": "codex/511-obs-a-observatory-experience-design",
  "worktree": "adl-worktrees/adl-issue-511-obs-a-observatory-experience-design",
  "registry_version": "1.0.3",
  "commands": ["prepare_issue", "bind_worktree", "plan_pvf", "doctor"]
}
EOF

  cat > .csdlc/prepared/issues/511/v3-local-registrations.json <<'EOF'
[
  {
    "branch": "codex/511-obs-a-observatory-experience-design",
    "worktree": "adl-worktrees/adl-issue-511-obs-a-observatory-experience-design",
    "primary": false
  }
]
EOF
}

write_obs_b_inputs() {
  cat > .csdlc/prepared/issues/512/design.md <<'EOF'
# OBS-B design bootstrap

This design input is the GitHub issue-authored Observatory redesign
implementation request for agent-logic/agent-design-language#512. Execution is
blocked until OBS-A (#511) and Sprint 8 coordination (#536) are terminal.
EOF

  cat > .csdlc/prepared/issues/512/diagram.mmd <<'EOF'
flowchart TD
  OBS_A["#511 reviewed contract"]
  SPRINT8["#536 Sprint 8 coordination"]
  OBS_B["#512 HTML Observatory redesign implementation"]
  RUNTIME["Authentic Runtime projections"]
  OBS_A --> OBS_B
  SPRINT8 --> OBS_B
  RUNTIME --> OBS_B
EOF

  cat > .csdlc/prepared/issues/512/bootstrap-request.json <<'EOF'
{
  "issue": 512,
  "repository": "agent-logic/agent-design-language",
  "actor": "codex:worker-6",
  "design_path": ".csdlc/prepared/issues/512/design.md",
  "diagram_path": ".csdlc/prepared/issues/512/diagram.mmd",
  "design_reviewer": "pending",
  "design_approved": false,
  "initial": {
    "title": "[v0.92.1][OBS-B] Observatory redesign implementation",
    "slug": "obs-b-observatory-redesign-implementation",
    "version": "v0.92.1",
    "goal": "Produce one implemented Observatory redesign backed by authentic Runtime projections.",
    "required_outcome": "One implemented Observatory redesign consuming authentic Runtime projections.",
    "declared_scope": ["demos/html-observatory/app.js", "demos/html-observatory/styles.css", "adl/tools/validate_layer8_authority_observatory_ui.sh", "docs/milestones/v0.92.1/evidence/observatory/obs-b/**", ".csdlc/prepared/issues/512", ".csdlc/evidence/512", ".csdlc/issues/512"],
    "authority_boundary": ["Issue authority is agent-logic/agent-design-language#512", "OBS-A #511 and Sprint 8 #536 are execution gates", "Issue #84 is independent backlog and non-gating", "V3 remains non-authoritative before #505"],
    "operator_constraints": ["No tracked implementation work on main", "No mock substitute for required Runtime route", "No TLS public exposure or Unity implementation"],
    "task_boundary": "Exactly one implemented HTML Observatory redesign consuming authentic Runtime projections.",
    "deliverables": ["OBS-A contract implementation", "Authentic Runtime projection consumption", "Browser and accessibility proof", "Redaction and recovery proof", "Review-ready OBS-B evidence packet"],
    "acceptance_criteria": ["AC-1: OBS-A contracts are implemented", "AC-2: Runtime projections are source-grounded", "AC-3: Accessibility and recovery cases pass", "AC-4: No mock substitutes for the required Runtime route", "AC-5: One-command pre-cutover canary passes but execution remains blocked until #511 and #536 are terminal"],
    "dependencies": ["#511 reviewed and terminal", "#536 Sprint 8 coordination terminal"],
    "repo_inputs": ["agent-logic/agent-design-language#512", "agent-logic/agent-design-language#511", "agent-logic/agent-design-language#536", "demos/html-observatory/app.js", "demos/html-observatory/styles.css", "adl/tools/validate_layer8_authority_observatory_ui.sh"],
    "non_goals": ["TLS 1.2 implementation owned by #251", "Public exposure owned by #122", "Unity integration owned by independent backlog #84", "Mock Runtime substitution"],
    "plan_summary": "After OBS-A and Sprint 8 gates are terminal, implement the HTML Observatory redesign against authentic Runtime projections and prove browser, accessibility, redaction, and recovery behavior.",
    "steps": [
      {"id":"S1","action":"Confirm #511 reviewed contract and #536 Sprint 8 coordination are terminal before binding execution.","acceptance_ids":["AC-1","AC-5"],"status":"pending"},
      {"id":"S2","action":"Implement OBS-A view/state/accessibility contracts in the HTML Observatory.","acceptance_ids":["AC-1","AC-3"],"status":"pending"},
      {"id":"S3","action":"Replace any mock data with authentic Runtime projection consumption and redaction handling.","acceptance_ids":["AC-2","AC-4"],"status":"pending"},
      {"id":"S4","action":"Run exact browser, accessibility, redaction, recovery, and review proof.","acceptance_ids":["AC-1","AC-2","AC-3","AC-4","AC-5"],"status":"pending"}
    ],
    "affected_areas": ["HTML Observatory", "Observatory UI validator", "OBS-B evidence"],
    "invariants": ["#511 is terminal before execution", "#536 is terminal before execution", "No mock substitutes for required Runtime route", "#84 remains non-gating backlog", "V3 remains non-authoritative before #505"],
    "risks": ["OBS-A may change implementation requirements", "Runtime projection route may be unavailable", "Mock data may be accidentally retained", "Accessibility recovery behavior may lag visual redesign"],
    "planning_profile": "medium",
    "stop_conditions": ["Issue #511 is not reviewed and terminal", "Issue #536 is not terminal", "A mock substitutes for the required Runtime route", "The implementation requires #84, #251, or #122"],
    "validation_lanes": [
      {"lane":"authentic-runtime-route","proof_role":"Prove the HTML Observatory consumes the required authentic Runtime route, not a mock.","acceptance_ids":["AC-2","AC-4"],"deterministic":true,"resource_profile":"medium","budget_seconds":300,"budget_tokens":2500,"argv":["bash","adl/tools/validate_layer8_authority_observatory_ui.sh"],"parallel_group":"runtime-route","defer_reason":"Execution is blocked until #511 and #536 are terminal."},
      {"lane":"exact-browser-cases","proof_role":"Run exact browser-facing redesign cases against the implemented OBS-A contract.","acceptance_ids":["AC-1","AC-3"],"deterministic":true,"resource_profile":"medium","budget_seconds":300,"budget_tokens":2500,"argv":["bash",".csdlc/prepared/issues/512/validate-obs-b-browser.sh"],"parallel_group":"browser","defer_reason":"Issue-owned validator is an OBS-B implementation deliverable."},
      {"lane":"accessibility","proof_role":"Verify keyboard and screen-reader behavior for implemented views and states.","acceptance_ids":["AC-3"],"deterministic":true,"resource_profile":"small","budget_seconds":180,"budget_tokens":1800,"argv":["bash",".csdlc/prepared/issues/512/validate-obs-b-accessibility.sh"],"parallel_group":"browser","defer_reason":"Issue-owned validator is an OBS-B implementation deliverable."},
      {"lane":"redaction","proof_role":"Verify projected Runtime data remains redacted in UI and evidence.","acceptance_ids":["AC-2"],"deterministic":true,"resource_profile":"small","budget_seconds":180,"budget_tokens":1800,"argv":["bash",".csdlc/prepared/issues/512/validate-obs-b-redaction.sh"],"parallel_group":"privacy","defer_reason":"Issue-owned validator is an OBS-B implementation deliverable."},
      {"lane":"recovery","proof_role":"Verify empty degraded recovery and revoked UI states match OBS-A.","acceptance_ids":["AC-1","AC-3"],"deterministic":true,"resource_profile":"small","budget_seconds":180,"budget_tokens":1800,"argv":["bash",".csdlc/prepared/issues/512/validate-obs-b-recovery.sh"],"parallel_group":"browser","defer_reason":"Issue-owned validator is an OBS-B implementation deliverable."},
      {"lane":"v3-local-canary","proof_role":"Run the single csdlc binary local preparation path as non-authoritative cutover evidence.","acceptance_ids":["AC-5"],"deterministic":true,"resource_profile":"small","budget_seconds":120,"budget_tokens":1000,"argv":["cargo","run","--locked","--manifest-path","csdlc-v3/Cargo.toml","--bin","csdlc","--","local","--request",".csdlc/prepared/issues/512/v3-local-request.json","--registry","docs/templates/prompts/current.json","--registrations",".csdlc/prepared/issues/512/v3-local-registrations.json"],"parallel_group":"cutover-canary","defer_reason":null}
    ],
    "failure_policy": "Fail closed on unmet #511 or #536 gates, mock Runtime substitution, redaction leakage, or v3 authority overclaim.",
    "review_prompts": ["Were #511 and #536 terminal before execution?", "Does the UI consume authentic Runtime projections?", "Are recovery and accessibility cases proven rather than visually asserted?"],
    "review_scope": "OBS-B HTML implementation, authentic Runtime route, accessibility, redaction, recovery, and pre-cutover canary evidence only."
  }
}
EOF

  cat > .csdlc/prepared/issues/512/v3-local-request.json <<'EOF'
{
  "issue": 512,
  "title": "[v0.92.1][OBS-B] Observatory redesign implementation",
  "repository": "agent-logic/agent-design-language",
  "branch": "codex/512-obs-b-observatory-redesign-implementation",
  "worktree": "adl-worktrees/adl-issue-512-obs-b-observatory-redesign-implementation",
  "registry_version": "1.0.3",
  "commands": ["prepare_issue", "bind_worktree", "plan_pvf", "doctor"]
}
EOF

  cat > .csdlc/prepared/issues/512/v3-local-registrations.json <<'EOF'
[
  {
    "branch": "codex/512-obs-b-observatory-redesign-implementation",
    "worktree": "adl-worktrees/adl-issue-512-obs-b-observatory-redesign-implementation",
    "primary": false
  }
]
EOF
}

bootstrap_if_missing() {
  local issue="$1"
  if [[ ! -f ".csdlc/issues/${issue}/index.json" ]]; then
    ./.adl/bin/csdlc-v2/csdlc-issue --root . create --request ".csdlc/prepared/issues/${issue}/bootstrap-request.json"
  fi
}

advance_ready_if_needed() {
  local issue="$1"
  local reason="$2"
  local doctor_json
  doctor_json="$(./.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue "$issue" || true)"
  if grep -q '"phase":"ready"' <<<"$doctor_json"; then
    return 0
  fi
  ruby -rjson -e '
    issue = Integer(ARGV.fetch(0))
    reason = ARGV.fetch(1)
    index = JSON.parse(File.read(".csdlc/issues/#{issue}/index.json"))
    request = {
      "issue" => issue,
      "actor" => "codex:worker-6:obs-ready-canary",
      "expected_generation" => index.fetch("generation"),
      "expected_digest" => index.fetch("digest"),
      "card" => "spp",
      "operation" => {"operation" => "advance_phase", "phase" => "ready"},
      "reason" => reason
    }
    File.write(".csdlc/prepared/issues/#{issue}/advance-ready.json", JSON.pretty_generate(request) + "\n")
  ' "$issue" "$reason"
  ./.adl/bin/csdlc-v2/csdlc-edit apply --request ".csdlc/prepared/issues/${issue}/advance-ready.json"
}

run_v3_local() {
  local issue="$1"
  cargo run --quiet --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- local \
    --request ".csdlc/prepared/issues/${issue}/v3-local-request.json" \
    --registry docs/templates/prompts/current.json \
    --registrations ".csdlc/prepared/issues/${issue}/v3-local-registrations.json" \
    > ".csdlc/evidence/${issue}/v3-local-canary.json"
}

write_obs_a_inputs
write_obs_b_inputs
ensure_v2_owner_binaries

bootstrap_if_missing 511
bootstrap_if_missing 512

./.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 511
./.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 512

run_v3_local 511
run_v3_local 512

advance_ready_if_needed 511 "OBS-A has no remote dependencies; bootstrap, v2 validation, and single-binary v3 local canary passed. Stop before implementation binding."

./.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 511 > .csdlc/evidence/511/doctor-after-canary.json
./.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 512 > .csdlc/evidence/512/doctor-after-canary.json || true

ruby -rjson -e '
  a = JSON.parse(File.read(".csdlc/evidence/511/doctor-after-canary.json"))
  b = JSON.parse(File.read(".csdlc/evidence/512/doctor-after-canary.json"))
  report = {
    "schema" => "csdlc.obs_ready_canary.v1",
    "authority" => {
      "live_lifecycle" => "csdlc-v2",
      "v3_operational_authority" => false,
      "operator_entrypoint" => "adl/tools/csdlc_obs_ready_canary.sh",
      "v3_binary" => "csdlc"
    },
    "issues" => {
      "511" => {
        "target" => "OBS-A",
        "doctor_status" => a["status"],
        "ready" => a["ready"],
        "phase" => a["phase"],
        "v3_canary" => ".csdlc/evidence/511/v3-local-canary.json"
      },
      "512" => {
        "target" => "OBS-B",
        "doctor_status" => b["status"],
        "ready" => b["ready"],
        "phase" => b["phase"],
        "blocked_on" => ["#511 reviewed and terminal", "#536 Sprint 8 coordination terminal"],
        "v3_canary" => ".csdlc/evidence/512/v3-local-canary.json"
      }
    }
  }
  File.write(".csdlc/evidence/obs-ready-canary.json", JSON.pretty_generate(report) + "\n")
  puts JSON.pretty_generate(report)
'
