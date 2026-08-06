#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"

SPRINT = 5855
CHILDREN = [5800, 5820, 5821, 5832, 5795, 5837].freeze
READY_CARDS = %w[sip stp spp vpp].freeze
PRE_PHASE_CARDS = %w[srp sor].freeze
PACKET_YAML = ".csdlc/prepared/issues/#{SPRINT}/sprint-execution-packet.yaml"
PACKET_MD = ".csdlc/prepared/issues/#{SPRINT}/sprint-execution-packet.md"
SESSION_PROMPT = ".adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md"
LAUNCH_GATE = "issue 5801 is terminal and its merge is ancestral to current origin/main"
REQUIRED_SECTIONS = [
  "Child Issue Wave",
  "Recommended Execution Order",
  "Watcher Policy",
  "Budget And Goal Accounting",
  "Watcher Plan",
  "Safe Parallel Lanes",
  "Candidate Parallel Lanes",
  "Serial Gates",
  "Parallelism Outcome Plan",
  "Sprint Closeout Rollup Expectations"
].freeze

def read_json(path)
  JSON.parse(File.read(path))
rescue Errno::ENOENT, JSON::ParserError => error
  abort("invalid JSON artifact #{path}: #{error.message}")
end

def card_status(issue, card)
  read_json(".csdlc/issues/#{issue}/cards/#{card}.values.json").fetch("status")
end

packet = YAML.safe_load(File.read(PACKET_YAML))
abort("wrong sprint issue") unless packet.fetch("sprint_issue") == SPRINT
abort("wrong execution mode") unless packet.fetch("execution_mode") == "hybrid"
abort("unsafe or impossible child order") unless packet.fetch("ordered_issue_numbers") == CHILDREN
abort("missing WP-02A launch gate") unless packet.fetch("launch_gate") == LAUNCH_GATE
abort("missing sprint review path") unless packet.fetch("review_path") == ".csdlc/evidence/5855/sprint-review.md"
abort("missing sprint activity path") unless packet.fetch("activity_log_path") == ".csdlc/evidence/5855/activity.jsonl"

packet_markdown = File.read(PACKET_MD)
REQUIRED_SECTIONS.each do |section|
  abort("missing packet section: #{section}") unless packet_markdown.include?("## #{section}")
end

session_prompt = File.read(SESSION_PROMPT)
abort("session prompt omits WP-02A terminal gate") unless session_prompt.include?("WP-02A #5801 is terminal and ancestral")
abort("session prompt retains obsolete reacquire route") if session_prompt.include?("--reacquire-request")
abort("session prompt omits branch/worktree authority") unless session_prompt.include?("Branch/worktree binding is ownership authority")

CHILDREN.each do |issue|
  index_path = ".csdlc/issues/#{issue}/index.json"
  index = read_json(index_path)
  abort("issue mismatch in #{index_path}") unless index.fetch("issue") == issue
  abort("unexpected retained compatibility claim on ##{issue}") unless index["claim"].nil?

  approval = index.dig("design_review", "approved")
  revision = approval && approval["revision"]
  abort("missing design approval for ##{issue}") unless revision&.match?(/\A[0-9a-f]{64}\z/)
  design_path = index.fetch("design_path")
  abort("missing design for ##{issue}") unless File.file?(design_path) && !File.zero?(design_path)
  vpp = read_json(".csdlc/issues/#{issue}/cards/vpp.values.json")
  abort("VPP design revision drift for ##{issue}") unless vpp.dig("content", "values", "design_digest") == revision

  READY_CARDS.each do |card|
    abort("##{issue} #{card} is not ready") unless card_status(issue, card) == "ready"
  end
  PRE_PHASE_CARDS.each do |card|
    abort("##{issue} #{card} is not pre_phase") unless card_status(issue, card) == "pre_phase"
  end
end

puts JSON.generate(
  schema: "adl.v092.sprint_readiness.v1",
  sprint_issue: SPRINT,
  execution_mode: packet.fetch("execution_mode"),
  ordered_issue_numbers: CHILDREN,
  child_count: CHILDREN.length,
  card_contract: "ready_pre_phase",
  compatibility_claims: "null_non_authoritative",
  launch_gate_contract: "present_live_check_deferred",
  design_approvals: "revision_matched",
  status: "prepared"
)
