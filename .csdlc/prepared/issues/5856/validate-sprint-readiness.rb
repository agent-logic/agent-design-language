# frozen_string_literal: true

require "json"
require "digest"
require "pathname"
require "time"
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

sprint5_spp = JSON.parse((ROOT / ".csdlc/issues/5854/cards/spp.values.json").read).dig("content", "values")
sprint5_summary = sprint5_spp.fetch("summary")
raise "Sprint 5 SPP still routes WP-20" if sprint5_summary.include?("#5840")
raise "Sprint 5 SPP omits its four operative children" unless [5835, 5836, 5838, 5839].all? { |issue| sprint5_summary.include?("##{issue}") }
raise "Sprint 5 SPP retains the five-child denominator" if sprint5_summary.match?(/five operative children/i)
raise "Sprint 5 SPP omits the four-child denominator" unless sprint5_summary.match?(/four operative children/i)

sprint5_sip = JSON.parse((ROOT / ".csdlc/issues/5854/cards/sip.values.json").read).dig("content", "values")
sprint5_outcome = sprint5_sip.fetch("required_outcome")
raise "Sprint 5 SIP still routes WP-20" if sprint5_outcome.include?("#5840")
raise "Sprint 5 SIP omits its four operative children" unless [5835, 5836, 5838, 5839].all? { |issue| sprint5_outcome.include?("##{issue}") }

wp20_sip = JSON.parse((ROOT / ".csdlc/issues/5840/cards/sip.values.json").read).dig("content", "values")
wp20_constraints = wp20_sip.fetch("operator_constraints")
raise "WP-20 SIP retains preparation-claim guidance" if wp20_constraints.any? { |value| value.match?(/preparation claim|release .*claim/i) }
raise "WP-20 SIP omits typed bind authority" unless wp20_constraints.any? { |value| value.include?("typed C-SDLC v2") && value.include?("issue branch and worktree") }

session_prompt = (ROOT / ".adl/docs/TBD/V092_SPRINT_5856_QUALITY_RELEASE_SESSION_PROMPT.md").read
raise "final-sprint prompt uses removed reacquire command" if session_prompt.include?("--reacquire-request")
raise "final-sprint prompt retains temporary-claim startup guidance" if session_prompt.match?(/temporary publication\s+claim|releases? that claim/i)
raise "final-sprint prompt omits real bind arguments" unless session_prompt.include?("--root . --request")
packet_markdown = (ROOT / ".csdlc/prepared/issues/5856/sprint-execution-packet.md").read
packet_yaml = (ROOT / ".csdlc/prepared/issues/5856/sprint-execution-packet.yaml").read
[packet_markdown, packet_yaml].each do |packet_text|
  raise "final-sprint packet retains claim/reacquire guidance" if packet_text.match?(/publication claim|reacquir(?:e|es)/i)
  raise "final-sprint packet omits typed bind authority" unless packet_text.match?(/typed (?:C-SDLC v2 )?bind/i)
end
bind_source = (ROOT / "csdlc-v2/src/bin/csdlc-bind.rs").read
raise "typed bind source omits --root" unless bind_source.include?("root: PathBuf")
raise "typed bind source omits --request" unless bind_source.include?("request: PathBuf")
raise "typed bind source unexpectedly supports reacquire-request" if bind_source.include?("reacquire_request")

source_path = ROOT / ".csdlc/evidence/5854/live-gates-source.json"
gates = JSON.parse((ROOT / ".csdlc/evidence/5854/live-gates.json").read)
source = JSON.parse(source_path.read)
raise "live evidence digest mismatch" unless gates.dig("provenance", "source_evidence_sha256") == Digest::SHA256.file(source_path).hexdigest
raise "live evidence timestamps disagree" unless gates.fetch("observed_at") == source.fetch("collected_at")
age_seconds = Time.now.utc - Time.iso8601(source.fetch("collected_at"))
raise "live evidence is from the future" if age_seconds < -300
raise "live evidence is older than 24 hours" if age_seconds > 86_400
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
