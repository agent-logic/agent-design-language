# frozen_string_literal: true

require "json"
require "pathname"
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
raise "WP-24A checkpoint classification missing" unless packet.fetch("active_checkpoint_issue_numbers") == [5845]

human = (ROOT / ".csdlc/prepared/issues/5854/sprint-execution-packet.md").read
%w[Child\ Issue\ Wave Recommended\ Execution\ Order Watcher\ Policy Budget\ And\ Goal\ Accounting Safe\ Parallel\ Lanes Serial\ Gates Sprint-Level\ Review].each do |section|
  heading = "## #{section.tr('\\', '')}"
  raise "missing packet section #{heading}" unless human.include?(heading)
end
raise "episode checkpoint overstated" unless human.include?("nine episodes remain")
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

podcast = JSON.parse((ROOT / ".csdlc/issues/5845/index.json").read)
raise "WP-24A was rolled backward" unless %w[implemented reviewed published merge_ready].include?(podcast.fetch("phase"))

gates = JSON.parse((ROOT / ".csdlc/evidence/5854/live-gates.json").read)
states = gates.fetch("issues").to_h { |row| [[row.fetch("repository"), row.fetch("issue")], row.fetch("state")] }
raise "repository migration is not terminal" unless states[["danielbaustin/agent-design-language", 5819]] == "closed"
raise "birthday packet gate was lost" unless states[["danielbaustin/agent-design-language", 5834]] == "open"
raise "WP-24 legacy issue is not terminal" unless states[["danielbaustin/agent-design-language", 5844]] == "closed"
raise "WP-24A must remain open" unless states[["danielbaustin/agent-design-language", 5845]] == "open"
pr14 = gates.fetch("pull_requests").find { |row| row.fetch("pull_request") == 14 }
pr69 = gates.fetch("pull_requests").find { |row| row.fetch("pull_request") == 69 }
raise "canonical WP-24 PR is not merged" unless pr14&.fetch("state") == "merged"
raise "episode 001 checkpoint is not merged" unless pr69&.fetch("state") == "merged" && pr69["part_of_issue"] == 5845
raise "publication was implicitly authorized" unless gates.dig("publication_authorization", "status") == "not_authorized"

umbrella = JSON.parse((ROOT / ".csdlc/issues/5854/index.json").read)
raise "umbrella is outside its readiness lifecycle" unless %w[bound implemented].include?(umbrella.fetch("phase"))
raise "umbrella code repository mismatch" unless umbrella.fetch("code_repository") == EXPECTED_CODE_REPOSITORY

puts "sprint 5854 readiness: PASS"
