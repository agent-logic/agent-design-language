#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
git_common_stdout, git_common_stderr, git_common_status =
  Open3.capture3("git", "-C", ROOT, "rev-parse", "--git-common-dir")
raise "cannot resolve git common dir: #{git_common_stderr}" unless git_common_status.success?

GIT_COMMON = File.expand_path(git_common_stdout.strip, ROOT)

def read(path)
  File.read(File.join(ROOT, path))
end

def json(path)
  JSON.parse(read(path))
end

def assert(condition, message)
  raise message unless condition
end

issue = json(".csdlc/issues/505/index.json")
stp = read(".csdlc/issues/505/cards/stp.md")
sip = read(".csdlc/issues/505/cards/sip.md")
spp = json(".csdlc/issues/505/cards/spp.values.json")
vpp = json(".csdlc/issues/505/cards/vpp.values.json")
srp = json(".csdlc/issues/505/cards/srp.values.json")
v3_trial = json(".csdlc/evidence/505/v3-local-trial.json")
failed_issue_readback = json(".csdlc/evidence/sprints-5-6-cutover-fixes/reopened-failed-issues-20260901.json")
sprint_membership_readback = json("docs/milestones/v0.92.1/evidence/wp-01/sprint-umbrella-membership-v5-retained-readback.json")
gemini_receipt = json(".csdlc/evidence/sprints-5-6-cutover-fixes/gemini-remediation-review/receipt.json")
gemini_review = read(".csdlc/evidence/sprints-5-6-cutover-fixes/gemini-remediation-review/review.md")
pr591_state_request = json(".csdlc/prepared/issues/505/pr591-state-after-prep-refresh-request.json")
design = read(".csdlc/prepared/issues/505/design.md")
diagram = read(".csdlc/prepared/issues/505/diagram.mmd")
packet_text = [stp, sip, design, diagram].join("\n")
notice = read("docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md")
notice_inline = notice.gsub(/\s+/, " ")

assert(issue["issue"] == 505, "wrong issue")
assert(issue["repository"] == "agent-logic/agent-design-language", "wrong repository")
assert(["bound", "implemented", "reviewed", "published"].include?(issue["phase"]), "issue #505 must be bound or later for execution/publication")
assert(issue["branch"] == "codex/505-v3-f-authority-transition-decision-exec", "unexpected #505 execution branch")
assert(issue["worktree"] == "/Volumes/FastWork/adl-worktrees/adl-issue-505-v3-f-authority-transition-decision-exec", "unexpected #505 execution worktree")

[
  "Requirements #179 and #180 are mapped",
  "v2-v3 parity is measured",
  "Canary rollback is exercised",
  "Cutover and retirement require operator approval"
].each { |text| assert(stp.include?(text), "missing acceptance text: #{text}") }

deps = spp.dig("content", "values", "steps").to_s +
       spp.dig("content", "values", "stop_conditions").to_s +
       stp +
       design
assert(deps.include?("#504"), "missing #504 dependency")
assert(deps.include?("terminal") && deps.include?("ancestral"), "missing terminal/ancestral dependency language")
assert(deps.include?("#570") && deps.include?("#571"), "missing #570/#571 cutover-readiness gates")
assert(deps.include?("merged") && deps.include?("closed"), "missing merged/closed gate language")
assert(packet_text.include?("Closes #505"), "missing visible future closing-linkage requirement")
assert(packet_text.include?("C-SDLC v2 remains") || packet_text.include?("v2 remains"), "missing v2 live-authority boundary")
assert(packet_text.include?("operator approval"), "missing explicit operator approval gate")
assert(packet_text.include?("No silent v2 retirement") || packet_text.include?("Silent v2 retirement"), "missing no-silent-retirement boundary")
assert(diagram.include?("Rollback exercise"), "missing rollback diagram node")
assert(diagram.include?("Observation evidence"), "missing observation diagram node")

assert(v3_trial["schema"] == "csdlc.v3.local_preparation.v1", "v3 local trial emitted wrong schema")
assert(v3_trial["read_only"] == true, "v3 local trial must remain read-only")
assert(v3_trial["operational_authority"] == false, "v3 local trial must not grant operational authority")
assert(v3_trial.dig("result", "issue") == 505, "v3 local trial targeted wrong issue")
assert(v3_trial.dig("result", "findings", 0, "code") == "doctor_ready", "v3 local trial did not reach doctor-ready plan")
assert(v3_trial.dig("result", "cards", "card_kinds") == ["sip", "stp", "spp", "vpp", "srp", "sor"], "v3 local trial did not use six-card registry")
rendered_cards = v3_trial.dig("result", "cards", "rendered_cards")
assert(rendered_cards.is_a?(Array) && rendered_cards.length == 6, "v3 local trial must expose six rendered card targets")
rendered_cards.each do |card|
  assert(%w[sip stp spp vpp srp sor].include?(card["kind"]), "unexpected rendered card kind")
  assert(card["template_ref"].to_s.start_with?("docs/templates/prompts/"), "rendered card missing template ref")
  assert(card["rendered_ref"].to_s.start_with?(".csdlc/issues/505/cards/"), "rendered card missing issue-local rendered ref")
  assert(card["render_manifest_digest"].to_s.match?(/\A[0-9a-f]{64}\z/), "rendered card missing deterministic digest")
end

assert(failed_issue_readback["transport"] == "typed-v2 csdlc-github-issue", "failed-issue reopen evidence must use typed v2")
assert(failed_issue_readback["raw_gh_used"] == false, "failed-issue reopen evidence must not use raw gh")
reopened = failed_issue_readback.fetch("reopened")
expected_reopened = [501, 502, 503, 504, 533, 596]
assert(reopened.map { |row| row.fetch("issue") }.sort == expected_reopened, "unexpected failed-issue reopen denominator")
reopened.each do |row|
  assert(row["readback_state"] == "open", "failed issue ##{row["issue"]} was not reopened")
  assert(row["closed_at"].nil?, "failed issue ##{row["issue"]} still has closed_at")
end
assert(
  failed_issue_readback["closure_policy"].to_s.include?("Do not close these issues again"),
  "failed-issue reopen evidence missing closure policy"
)

assert(sprint_membership_readback["transport"] == "typed-v2 csdlc-github-issue", "sprint membership readback must use typed v2")
assert(sprint_membership_readback["raw_gh_used"] == false, "sprint membership readback must not use raw gh")
sprint5 = sprint_membership_readback.fetch("umbrellas").find { |row| row["sprint"] == 5 }
sprint6 = sprint_membership_readback.fetch("umbrellas").find { |row| row["sprint"] == 6 }
assert(sprint5 && sprint5["issue"] == 533 && sprint5["state"] == "open", "Sprint 5 umbrella must be retained as open after failed review")
assert(sprint5["membership_version"] == 4 && sprint5["members"] == [500, 501, 502], "Sprint 5 membership readback drift")
assert(sprint6 && sprint6["issue"] == 534 && sprint6["state"] == "open", "Sprint 6 umbrella must remain open")
assert(sprint6["membership_version"] == 5 && sprint6["members"] == [503, 504, 505, 570], "Sprint 6 v5 membership readback drift")
assert(sprint_membership_readback["cutover_gate"].to_s.include?("#505 must not approve cutover"), "sprint membership readback missing #505 cutover gate")

assert(gemini_receipt["provider_family"] == "gemini", "Gemini review receipt missing provider family")
assert(gemini_receipt["http_status"] == 200, "Gemini review did not complete successfully")
assert(gemini_receipt["credential_material_retained"] == false, "Gemini receipt must not retain credential material")
assert(gemini_receipt["review_ref"] == ".csdlc/evidence/sprints-5-6-cutover-fixes/gemini-remediation-review/review.md", "Gemini review ref drift")
assert(gemini_review.include?("GEMINI_ACTIONABLE_FINDINGS="), "Gemini review missing actionable-finding marker")
assert(gemini_review.include?("remote command") && gemini_review.include?("cleanup"), "Gemini review missing expected remediation focus")

assert(pr591_state_request.keys.sort == ["linked_issue", "linked_issue_repository", "pull_request", "repository", "require_review", "required_checks", "token_file"].sort, "PR #591 state request must use the current typed state schema")
assert(pr591_state_request["repository"] == "agent-logic/agent-design-language", "PR #591 state request repository drift")
assert(pr591_state_request["pull_request"] == 591, "PR #591 state request must target PR #591")
assert(pr591_state_request["linked_issue"].nil?, "PR #591 state readback must not require closing linkage before operator approval")
assert(!pr591_state_request.key?("action"), "PR #591 state request must not use the retired action-envelope schema")

[504, 570, 571].each do |issue_number|
  receipt_path = File.join(GIT_COMMON, "csdlc-v2", "closeout", "#{issue_number}.json")
  assert(File.file?(receipt_path), "missing terminal closeout receipt for ##{issue_number}")
  receipt = JSON.parse(File.read(receipt_path))
  assert(receipt["issue"] == issue_number, "wrong issue in closeout receipt #{receipt_path}")
end

[
  "C-SDLC v2 remains the live lifecycle authority",
  "C-SDLC v3 remains construction and cutover evidence only",
  "Historical `adl_pr_cycle`, `pr.sh`, and `pr ready/run/finish/closeout`",
  "typed C-SDLC v2 GitHub issue owner",
  "informational only",
  "not approval",
  "v2 remains the rollback and live-authority target"
].each { |text| assert(notice_inline.include?(text), "changeover notice missing: #{text}") }

docs = {
  "AGENTS.md" => read("AGENTS.md"),
  "csdlc-v2/AGENTS.md" => read("csdlc-v2/AGENTS.md"),
  "csdlc-v3/AGENTS.md" => read("csdlc-v3/AGENTS.md"),
  "docs/default_workflow.md" => read("docs/default_workflow.md"),
  "docs/onboarding.md" => read("docs/onboarding.md"),
  "docs/architecture/ADL_ARCHITECTURE.md" => read("docs/architecture/ADL_ARCHITECTURE.md"),
  "docs/tooling/adl_pr_cycle_skill.md" => read("docs/tooling/adl_pr_cycle_skill.md"),
  "docs/tooling/card-lifecycle.md" => read("docs/tooling/card-lifecycle.md"),
  "docs/tooling/structured-prompt-contracts.md" => read("docs/tooling/structured-prompt-contracts.md"),
  "docs/templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md" => read("docs/templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md"),
  "docs/tooling/editor/pr_run_demo.md" => read("docs/tooling/editor/pr_run_demo.md"),
  "docs/tooling/editor/README.md" => read("docs/tooling/editor/README.md"),
  "docs/tooling/editor/index.html" => read("docs/tooling/editor/index.html"),
  "docs/tooling/editor/five_command_regression_suite.md" => read("docs/tooling/editor/five_command_regression_suite.md"),
  "docs/tooling/editor/task_bundle_editor.js" => read("docs/tooling/editor/task_bundle_editor.js"),
  "docs/templates/prompts/current.json" => read("docs/templates/prompts/current.json"),
  "docs/templates/prompts/1.0.3/sor.md" => read("docs/templates/prompts/1.0.3/sor.md"),
  "docs/templates/prompts/1.0.3/schemas/sor.structure.json" => read("docs/templates/prompts/1.0.3/schemas/sor.structure.json")
}

docs.each do |path, text|
  assert(text.include?("#505") || text.include?("V3-F"), "#{path} missing #505/V3-F changeover marker")
  assert(
    text.match?(/(?:C-SDLC\s+)?v2\b.*\b(?:remains|remain)\b.*\b(?:live|authoritative|authority)\b/i) ||
      text.match?(/\b(?:live|authoritative|authority)\b.*\b(?:C-SDLC\s+)?v2\b/i),
    "#{path} missing v2-live boundary"
  )
end

architecture = docs.fetch("docs/architecture/ADL_ARCHITECTURE.md")
architecture_inline = architecture.gsub(/\s+/, " ")
assert(architecture.include?("SIP, STP, SPP, VPP, SRP, and SOR"), "architecture omits VPP from six-card lifecycle")
assert(architecture_inline.include?("historical orientation only"), "architecture must classify legacy pr route as historical")
assert(!architecture.match?(/^\s*\d+\.\s*`?pr (run|finish|closeout)\b/i), "architecture contains instructional legacy pr lifecycle step")

[
  "docs/tooling/card-lifecycle.md",
  "docs/tooling/structured-prompt-contracts.md",
  "docs/templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md",
  "docs/GLOSSARY.md",
  "docs/cognitive-sdlc/README.md",
  "docs/cognitive-sdlc/card-lifecycle.md",
  "docs/cognitive-sdlc/five-minute-sprint-demo.md",
  "docs/templates/MILESTONE_CHECKLIST_TEMPLATE.md",
  "docs/templates/SPRINT_TEMPLATE.md",
  "docs/templates/README_TEMPLATE.md",
  "docs/templates/STRUCTURED_PLAN_PROMPT_TEMPLATE.md",
  "docs/templates/STRUCTURED_REVIEW_POLICY_TEMPLATE.md",
  "docs/templates/sprints/README.md",
  "docs/templates/portable-adl/README.md",
  "docs/templates/portable-adl/1.0.0/AGENTS.md",
  "docs/templates/planning/fixtures/minimal/sprint.md",
  "docs/templates/planning/fixtures/minimal/readme.md",
  "docs/templates/planning/fixtures/minimal/readme_generated.md",
  "docs/templates/planning/1.0.0/readme.md",
  "docs/templates/planning/1.0.0/milestone_checklist.md",
  "docs/templates/planning/1.0.0/sprint.md",
  "docs/templates/planning/1.1.0/readme.md",
  "docs/templates/planning/1.1.0/milestone_checklist.md",
  "docs/templates/planning/1.1.0/sprint.md"
].each do |path|
  text = docs.fetch(path) { read(path) }
  assert(text.include?("SIP -> STP -> SPP -> VPP -> SRP -> SOR"), "#{path} omits VPP from canonical lifecycle")
end

[
  "docs/tooling/editor/pr_run_demo.md",
  "docs/tooling/editor/README.md",
  "docs/tooling/editor/five_command_regression_suite.md",
  "docs/tooling/editor/task_bundle_editor.js",
  "docs/tooling/editor/index.html",
  "docs/templates/STRUCTURED_PLAN_PROMPT_TEMPLATE.md",
  "docs/templates/prompts/1.0.3/sor.md",
  "docs/templates/prompts/1.0.3/schemas/sor.structure.json"
].each do |path|
  text = docs.fetch(path) { read(path) }
  unless path == "docs/templates/STRUCTURED_PLAN_PROMPT_TEMPLATE.md"
    assert(text.include?("historical") || text.include?("retired"), "#{path} must classify legacy editor route as historical/retired")
  end
  assert(!text.match?(/current pr run command|supported control-plane run surface today|current routing guidance/i), "#{path} contains active legacy route guidance")
  assert(!text.match?(/`pr (doctor|finish|closeout)`.*\b(should|must|now|reports?|treats?)\b/i), "#{path} contains normative legacy pr route guidance")
  assert(!text.match?(/Copy pr run command/i), "#{path} exposes retired pr-run button text")
end

active_registry = json("docs/templates/prompts/current.json")
assert(active_registry["csdlc_prompt_template_set"] == "1.0.3", "unexpected active prompt template set")
active_registry.fetch("templates").each do |_kind, entry|
  path = entry.fetch("path")
  next unless path.start_with?("docs/templates/prompts/1.0.3/")

  text = read(path)
  assert(!text.match?(/`pr (run|finish|closeout|ready|doctor)`.*\b(should|must|normally|current|use|run)\b/i), "#{path} contains active normative legacy pr guidance")
  schema_path = entry["structure_schema_path"]
  next unless schema_path

  schema_text = read(schema_path)
  assert(!schema_text.match?(/`pr (run|finish|closeout|ready|doctor)`.*\b(should|must|normally|current|use|run)\b/i), "#{schema_path} contains active normative legacy pr guidance")
end

adl_pr_cycle = docs.fetch("docs/tooling/adl_pr_cycle_skill.md")
assert(adl_pr_cycle.include?("Historical compatibility documentation"), "tracked adl_pr_cycle guidance must be historical")
assert(adl_pr_cycle.include?("Do not install, resync, invoke, or route current ADL work through"), "tracked adl_pr_cycle guidance must block active routing")

lanes = vpp.dig("content", "values", "lanes")
assert(lanes.is_a?(Array) && lanes.length == 1, "pre-bind #505 should expose exactly one executable preparation lane")
lane = lanes.first
assert(lane["lane"] == "prebind-v3-f-preparation", "unexpected pre-bind lane")
assert(lane["argv"] == ["ruby", ".csdlc/prepared/issues/505/validate-authority-transition-prep.rb"], "pre-bind lane must target this validator")
assert(lane["defer_reason"].nil?, "pre-bind validator must be executable, not deferred")

review_prompts = srp.dig("content", "values", "review_prompts") || []
combined = review_prompts.join("\n") + "\n" + packet_text
["#504", "#179", "#180", "Closes #505"].each do |needle|
  assert(combined.include?(needle), "missing review/planning prompt marker #{needle}")
end

puts JSON.generate(
  {
    schema: "adl.csdlc_v3.issue505.prebind_validation.v1",
    status: "pass",
    issue: 505,
    phase: issue["phase"],
    checked: [
      "acceptance_denominator",
      "504_terminal_dependency",
      "predecessor_terminal_closeout_receipts",
      "570_571_cutover_readiness_gates",
      "v2_live_authority_boundary",
      "advance_changeover_notice",
      "agents_docs_and_skill_guidance",
      "six_card_lifecycle_includes_vpp",
      "legacy_route_not_instructional",
      "no_silent_v2_retirement",
      "operator_approval_gate",
      "future_closing_linkage",
      "current_pr591_state_request_schema",
      "non_authoritative_v3_local_trial",
      "local_trial_render_manifest",
      "failed_issue_reopen_readback",
      "sprint_membership_v5_readback",
      "gemini_assisted_review_receipt",
      "single_prebind_validator_lane",
      "bound_execution_topology"
    ]
  }
)
