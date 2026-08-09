# frozen_string_literal: true

require "json"
require "pathname"
require "time"
require "yaml"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__))
SPRINT = 5854
UNBOUND = [5835, 5836, 5838, 5839, 5840].freeze
CARDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_CODE_REPOSITORY = "agent-logic/agent-design-language"

packet = YAML.safe_load((ROOT / ".csdlc/prepared/issues/5854/sprint-execution-packet.yaml").read)
raise "sprint identity mismatch" unless packet.fetch("sprint_issue") == SPRINT
raise "sprint packet is not execution-ready" unless packet.fetch("status") == "ready_for_execution"
raise "unbound execution order mismatch" unless packet.fetch("ordered_issue_numbers") == UNBOUND
raise "WP-24 completion missing" unless packet.fetch("completed_legacy_issue_numbers") == [5844]
out_of_band = packet.fetch("out_of_band_streams")
wp24a = out_of_band.find { |stream| stream.fetch("issue") == 5845 }
raise "WP-24A is not explicitly out of band" unless wp24a
raise "WP-24A can gate Sprint 5" unless wp24a["gates_sprint"] == false
raise "WP-24A incorrectly depends on Sprint 5" unless wp24a["dependency_on_sprint"] == false
raise "WP-24A leaked into execution order" if packet.fetch("ordered_issue_numbers").include?(5845)

human = (ROOT / ".csdlc/prepared/issues/5854/sprint-execution-packet.md").read
%w[Child\ Issue\ Wave Recommended\ Execution\ Order Watcher\ Policy Budget\ And\ Goal\ Accounting Safe\ Parallel\ Lanes Serial\ Gates Sprint-Level\ Review].each do |section|
  heading = "## #{section.tr('\\', '')}"
  raise "missing packet section #{heading}" unless human.include?(heading)
end
raise "WP-24A non-gating boundary missing" unless human.include?("cannot gate Sprint 5")
raise "deferred proof is overstated" unless human.include?("never validation evidence")

UNBOUND.each do |issue|
  issue_root = ROOT / ".csdlc/issues/#{issue}"
  record = JSON.parse((issue_root / "index.json").read)
  raise "issue #{issue} is not initialized" unless record.fetch("phase") == "initialized"
  raise "issue #{issue} is unexpectedly bound" unless record["branch"].nil? && record["worktree"].nil?
  CARDS.each do |card|
    raise "issue #{issue} missing #{card}" unless (issue_root / "cards/#{card}.md").file?
    raise "issue #{issue} missing #{card} values" unless (issue_root / "cards/#{card}.values.json").file?
  end

  sip = JSON.parse((issue_root / "cards/sip.values.json").read).dig("content", "values")
  authority = sip.fetch("authority_boundary")
  raise "issue #{issue} omits Git topology authority" unless authority.any? { |line| line.include?("issue-bound Git branch and worktree") }
  raise "issue #{issue} retains active claim authority" unless authority.any? { |line| line.include?("compatibility evidence only") }

  spp = JSON.parse((issue_root / "cards/spp.values.json").read).dig("content", "values")
  affected = spp.fetch("affected_areas")
  raise "issue #{issue} has non-path ownership" if affected.any? { |path| path.start_with?("SERIALIZATION_GATE") }
  raise "issue #{issue} has a started plan step" unless spp.fetch("steps").all? { |step| step.fetch("status") == "pending" }

  stp = JSON.parse((issue_root / "cards/stp.values.json").read).dig("content", "values")
  deliverables = stp.fetch("deliverables")
  vpp = JSON.parse((issue_root / "cards/vpp.values.json").read).dig("content", "values")
  raise "issue #{issue} failure policy is not fail-closed" unless vpp.fetch("failure_policy").start_with?("Fail closed")
  missing_targets = vpp.fetch("lanes").map do |lane|
    argv = lane.fetch("argv")
    next unless %w[bash zsh node ruby python python3].include?(File.basename(argv.first))
    target = argv[1]
    next if target.nil? || target.start_with?("-") || (ROOT / target).file?
    raise "issue #{issue} missing target is not owned: #{target}" unless affected.include?(target)
    raise "issue #{issue} missing target is not a deliverable: #{target}" unless deliverables.include?(target)
    reason = lane["defer_reason"].to_s
    raise "issue #{issue} missing target lacks explicit deferral: #{target}" if reason.empty?
    target
  end.compact
  raise "issue #{issue} lacks an issue-specific proof target" if missing_targets.empty? && vpp.fetch("lanes").none? { |lane| lane.fetch("argv").any? { |arg| affected.include?(arg) && (ROOT / arg).file? } }
end

gates = JSON.parse((ROOT / ".csdlc/evidence/5854/live-gates.json").read)
observed_at = Time.iso8601(gates.fetch("observed_at"))
age_seconds = Time.now.utc - observed_at
raise "live-gate snapshot is from the future" if age_seconds < -300
raise "live-gate snapshot is older than 24 hours" if age_seconds > 86_400

states = gates.fetch("issues").to_h { |row| [[row.fetch("repository"), row.fetch("issue")], row.fetch("state")] }
expected_states = {
  5819 => "closed",
  5825 => "open",
  5826 => "open",
  5827 => "open",
  5828 => "open",
  5829 => "open",
  5830 => "open",
  5832 => "closed",
  5833 => "open",
  5834 => "open",
  5835 => "open",
  5836 => "open",
  5837 => "open",
  5838 => "open",
  5839 => "open",
  5840 => "open",
  5843 => "open",
  5844 => "closed",
  5854 => "open"
}
expected_states.each do |issue, expected|
  actual = states[["danielbaustin/agent-design-language", issue]]
  raise "issue #{issue} live-gate mismatch: expected #{expected}, got #{actual.inspect}" unless actual == expected
end

raise "WP-17 dependency set is incomplete" unless human.include?("`#5826`, `#5827`, and `#5834`")
raise "WP-18 dependency set is incomplete" unless human.include?("`#5825`-`#5830` and `#5832`-`#5834`")

pr14 = gates.fetch("pull_requests").find { |row| row.fetch("pull_request") == 14 }
expected_wp24_sha = "b4f23892fa5c7b23816c8c38903ed4c73395afde"
raise "canonical WP-24 PR repository mismatch" unless pr14&.fetch("repository") == "agent-logic/agent-design-language"
raise "canonical WP-24 PR is not merged" unless pr14&.fetch("state") == "merged"
raise "canonical WP-24 merge SHA mismatch" unless pr14&.fetch("merge_sha") == expected_wp24_sha
raise "canonical WP-24 closing relation mismatch" unless pr14&.fetch("closes_issue") == 10
raise "canonical WP-24 merge is not ancestral to readiness HEAD" unless system("git", "merge-base", "--is-ancestor", expected_wp24_sha, "HEAD", out: File::NULL, err: File::NULL)

wp24a_observation = gates.fetch("out_of_band_observations").find { |row| row.fetch("issue") == 5845 }
raise "WP-24A out-of-band observation missing" unless wp24a_observation
raise "WP-24A observation can gate Sprint 5" unless wp24a_observation["gates_sprint"] == false
raise "WP-24A observation depends on Sprint 5" unless wp24a_observation["dependency_on_sprint"] == false
raise "publication was implicitly authorized" unless gates.dig("publication_authorization", "status") == "not_authorized"

umbrella = JSON.parse((ROOT / ".csdlc/issues/5854/index.json").read)
raise "umbrella is outside its readiness lifecycle" unless %w[bound implemented].include?(umbrella.fetch("phase"))
raise "umbrella code repository mismatch" unless umbrella.fetch("code_repository") == EXPECTED_CODE_REPOSITORY

puts "sprint 5854 readiness: PASS"
