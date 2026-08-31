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
design = read(".csdlc/prepared/issues/505/design.md")
diagram = read(".csdlc/prepared/issues/505/diagram.mmd")
packet_text = [stp, sip, design, diagram].join("\n")
notice = read("docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md")

assert(issue["issue"] == 505, "wrong issue")
assert(issue["repository"] == "agent-logic/agent-design-language", "wrong repository")
assert(issue["phase"] == "bound", "issue #505 must be bound for execution")
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
].each { |text| assert(notice.include?(text), "changeover notice missing: #{text}") }

docs = {
  "AGENTS.md" => read("AGENTS.md"),
  "csdlc-v2/AGENTS.md" => read("csdlc-v2/AGENTS.md"),
  "csdlc-v3/AGENTS.md" => read("csdlc-v3/AGENTS.md"),
  "docs/default_workflow.md" => read("docs/default_workflow.md"),
  "docs/onboarding.md" => read("docs/onboarding.md"),
  "docs/architecture/ADL_ARCHITECTURE.md" => read("docs/architecture/ADL_ARCHITECTURE.md"),
  "docs/tooling/adl_pr_cycle_skill.md" => read("docs/tooling/adl_pr_cycle_skill.md")
}

docs.each do |path, text|
  assert(text.include?("#505") || text.include?("V3-F"), "#{path} missing #505/V3-F changeover marker")
  assert(text.include?("v2 remains") || text.include?("C-SDLC v2 remains"), "#{path} missing v2-live boundary")
end

architecture = docs.fetch("docs/architecture/ADL_ARCHITECTURE.md")
assert(architecture.include?("SIP, STP, SPP, VPP, SRP, and SOR"), "architecture omits VPP from six-card lifecycle")
assert(architecture.include?("historical orientation only"), "architecture must classify legacy pr route as historical")
assert(!architecture.match?(/^\s*\d+\.\s*`?pr (run|finish|closeout)\b/i), "architecture contains instructional legacy pr lifecycle step")

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
      "single_executable_prebind_lane",
      "bound_execution_topology"
    ]
  }
)
