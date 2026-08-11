# frozen_string_literal: true

require "json"
require "pathname"
require "yaml"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__))
ISSUE = 5856
ISSUES = [5840, 5786, 5841, 5842, 5843, 5846, 5847, 5848, 5849, 5850, 5851, 5852].freeze
WORK_PACKAGES = %w[WP-20 WP-21 WP-21A WP-22 WP-23 WP-25 WP-26 WP-27 WP-28 WP-28A WP-29 WP-30].freeze
CARDS = %w[sip stp spp vpp srp sor].freeze

packet = YAML.safe_load((ROOT / ".csdlc/prepared/issues/5856/sprint-execution-packet.yaml").read)
raise "final-sprint identity mismatch" unless packet.fetch("sprint_issue") == ISSUE
raise "final sprint must remain sequential" unless packet.fetch("execution_mode") == "sequential"
raise "final-sprint child order mismatch" unless packet.fetch("ordered_issue_numbers") == ISSUES
raise "final sprint must not declare parallel lanes" unless packet.fetch("safe_parallel_lanes").empty?
bind_request = JSON.parse((ROOT / packet.fetch("split_authority_bind_request_path")).read)
raise "final-sprint bind issue mismatch" unless bind_request.fetch("issue") == ISSUE
raise "final-sprint bind base mismatch" unless bind_request.fetch("base_branch") == "main"
raise "final-sprint bind branch mismatch" unless bind_request.fetch("branch") == "codex/5856-quality-release-tail"
raise "final-sprint bind worktree is not on FastWork" unless bind_request.fetch("worktree").start_with?("/Volumes/FastWork/adl-worktrees/adl-issue-5856-")
raise "final-sprint canonical code repository missing" unless bind_request.fetch("code_repository") == "agent-logic/agent-design-language"

wave = YAML.safe_load((ROOT / "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml").read)
final_sprint = wave.fetch("execution_sprints").find { |entry| entry.fetch("issue") == ISSUE }
sprint5 = wave.fetch("execution_sprints").find { |entry| entry.fetch("issue") == 5854 }
raise "final sprint missing from canonical wave" unless final_sprint
raise "final-sprint work-package order mismatch" unless final_sprint.fetch("members") == WORK_PACKAGES
raise "WP-20 remains in Sprint 5" if sprint5.fetch("members").include?("WP-20")
raise "WP-20 producer gate missing" unless final_sprint.fetch("serial_gates").first == "WP-20 after WP-18, WP-18A, WP-18B, and WP-19"
raise "WP-20 does not precede WP-21" unless final_sprint.fetch("serial_gates").include?("WP-20 before WP-21")

issue_root = ROOT / ".csdlc/issues/5856"
CARDS.each do |card|
  raise "final sprint missing #{card}" unless (issue_root / "cards/#{card}.md").file?
  raise "final sprint missing #{card} values" unless (issue_root / "cards/#{card}.values.json").file?
end
stp = JSON.parse((issue_root / "cards/stp.values.json").read).dig("content", "values")
raise "typed final-sprint dependency denominator mismatch" unless stp.fetch("dependencies") == ISSUES.map { |issue| "##{issue}" }
vpp = JSON.parse((issue_root / "cards/vpp.values.json").read).dig("content", "values")
lane = vpp.fetch("lanes").find { |entry| entry.fetch("lane") == "v092-final-sprint-readiness" }
raise "final-sprint readiness lane missing" unless lane
raise "final-sprint readiness lane targets wrong proof" unless lane.fetch("argv") == ["ruby", ".csdlc/prepared/issues/5856/validate-sprint-readiness.rb"]

source = JSON.parse((ROOT / ".csdlc/evidence/5854/live-gates-source.json").read)
live = source.fetch("issue_results").find do |entry|
  issue = entry.fetch("response").fetch("issue")
  issue.fetch("repository") == "danielbaustin/agent-design-language" && issue.fetch("number") == ISSUE
end&.fetch("response")&.fetch("issue")
raise "retained live #5856 readback missing" unless live
raise "live #5856 is not open" unless live.fetch("state") == "open"
body = live.fetch("body")
section = body.split("## Child Issues", 2).fetch(1).split("\n## ", 2).first
live_children = section.scan(/^- #(\d+) \(/).flatten.map(&:to_i)
raise "live #5856 child denominator mismatch" unless live_children == [5840, 5786, 5841, 5842, 5843]
raise "live #5856 range omits WP-25 through WP-30" unless section.include?("#5846 through #5852")
raise "live #5856 omits WP-20 producer gate" unless body.include?("WP-20 starts only after #5836, #5837, #5838, and #5839 are terminal")
raise "live #5856 omits WP-20 before WP-21" unless body.include?("WP-20 precedes WP-21")

puts "sprint 5856 readiness: PASS"
