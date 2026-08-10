# frozen_string_literal: true

require "json"
require "digest"
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

stp_values = JSON.parse((ROOT / ".csdlc/issues/5854/cards/stp.values.json").read).dig("content", "values")
operative_closeout = "the five operative children (#5835, #5836, #5838, #5839, and #5840)"
raise "STP does not name the exact operative closeout boundary" unless stp_values.fetch("acceptance_criteria").any? { |criterion| criterion.include?(operative_closeout) && criterion.include?("WP-24A #5845 cannot gate") }
srp_values = JSON.parse((ROOT / ".csdlc/issues/5854/cards/srp.values.json").read).dig("content", "values")
raise "SRP does not review the exact operative closeout boundary" unless srp_values.fetch("review_prompts").any? { |prompt| prompt.include?(operative_closeout) && prompt.include?("WP-24A #5845 excluded") }

session_prompt = (ROOT / ".adl/docs/TBD/V092_SPRINT_5854_DEMO_PUBLICATION_SESSION_PROMPT.md").read
raise "session prompt falsely calls WP-24 typed-terminal" if session_prompt.include?("#5844, WP-24: terminal")
raise "session prompt omits asynchronous WP-24 typed closeout" unless session_prompt.include?("typed closeout continues asynchronously")

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
provenance = gates.fetch("provenance")
source_ref = provenance.fetch("source_evidence")
raise "unexpected live-gate source evidence path" unless source_ref == ".csdlc/evidence/5854/live-gates-source.json"
source_path = ROOT / source_ref
raise "live-gate source evidence missing" unless source_path.file?
source_digest = Digest::SHA256.file(source_path).hexdigest
raise "live-gate source evidence digest mismatch" unless source_digest == provenance.fetch("source_evidence_sha256")
source = JSON.parse(source_path.read)
raise "live-gate source schema mismatch" unless source.fetch("schema") == "adl.v092.sprint_5854_live_gate_source.v1"
raise "live-gate observation timestamp is not source-bound" unless source.fetch("collected_at") == gates.fetch("observed_at")
request_manifest = {
  "issue_requests" => source.fetch("issue_results").map { |entry| entry.fetch("request") },
  "pull_request_requests" => source.fetch("pull_request_results").map { |entry| entry.fetch("request") }
}
recomputed_request_digest = Digest::SHA256.hexdigest(JSON.generate(request_manifest))
raise "retained request manifest digest is invalid" unless recomputed_request_digest == source.fetch("request_manifest_sha256")
raise "request manifest digest mismatch" unless source.fetch("request_manifest_sha256") == provenance.fetch("request_manifest_sha256")
collector = source.fetch("collector")
raise "typed issue collector identity missing" unless collector.fetch("issue_binary") == "csdlc-github-issue"
raise "typed PR collector identity missing" unless collector.fetch("pull_request_binary") == "csdlc-github-pr"
%w[issue_binary_sha256 pull_request_binary_sha256].each do |field|
  raise "invalid collector binary digest #{field}" unless collector.fetch(field).match?(/\A[0-9a-f]{64}\z/)
end
raise "collector contract mismatch" unless collector.fetch("contract") == provenance.fetch("collector_contract")
raise "collector identity missing" if provenance.fetch("collector_identity").strip.empty?
raise "collector source operations missing" unless source.fetch("source_operations").length == 2

observed_at = Time.iso8601(gates.fetch("observed_at"))
age_seconds = Time.now.utc - observed_at
raise "live-gate snapshot is from the future" if age_seconds < -300
raise "live-gate snapshot is older than 24 hours" if age_seconds > 86_400

states = gates.fetch("issues").to_h { |row| [[row.fetch("repository"), row.fetch("issue")], row.fetch("state")] }
source_states = source.fetch("issue_results").to_h do |entry|
  issue = entry.fetch("response").fetch("issue")
  [[issue.fetch("repository"), issue.fetch("number")], issue.fetch("state")]
end
raise "live-gate issue projection differs from retained source" unless states == source_states
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
source_pr14 = source.fetch("pull_request_results").first.fetch("response")
expected_wp24_sha = "b4f23892fa5c7b23816c8c38903ed4c73395afde"
raise "canonical WP-24 PR repository mismatch" unless pr14&.fetch("repository") == "agent-logic/agent-design-language"
raise "canonical WP-24 PR is not merged" unless pr14&.fetch("state") == "merged"
raise "canonical WP-24 merge SHA mismatch" unless pr14&.fetch("merge_sha") == expected_wp24_sha
raise "canonical WP-24 closing relation mismatch" unless pr14&.fetch("closes_issue") == 10
raise "canonical WP-24 PR projection differs from retained source" unless source_pr14.fetch("repository") == pr14.fetch("repository") && source_pr14.fetch("pull_request") == pr14.fetch("pull_request") && source_pr14.fetch("merged") && source_pr14.fetch("merge_commit_sha") == pr14.fetch("merge_sha") && source_pr14.fetch("linked_issue") == pr14.fetch("closes_issue")
raise "canonical WP-24 merge is not ancestral to readiness HEAD" unless system("git", "merge-base", "--is-ancestor", expected_wp24_sha, "HEAD", out: File::NULL, err: File::NULL)

wp24a_observation = gates.fetch("out_of_band_observations").find { |row| row.fetch("issue") == 5845 }
raise "WP-24A out-of-band observation missing" unless wp24a_observation
raise "WP-24A observation can gate Sprint 5" unless wp24a_observation["gates_sprint"] == false
raise "WP-24A observation depends on Sprint 5" unless wp24a_observation["dependency_on_sprint"] == false
raise "publication was implicitly authorized" unless gates.dig("publication_authorization", "status") == "not_authorized"

umbrella = JSON.parse((ROOT / ".csdlc/issues/5854/index.json").read)
raise "umbrella is outside its readiness lifecycle" unless %w[bound implemented].include?(umbrella.fetch("phase"))
raise "umbrella code repository mismatch" unless umbrella.fetch("code_repository") == EXPECTED_CODE_REPOSITORY

umbrella_vpp = JSON.parse((ROOT / ".csdlc/issues/5854/cards/vpp.values.json").read).dig("content", "values")
readiness_lane = umbrella_vpp.fetch("lanes").find { |lane| lane.fetch("lane") == "v092-sprint5-readiness" }
raise "readiness lane missing" unless readiness_lane
raise "wall-clock-dependent readiness lane is falsely deterministic" unless readiness_lane.fetch("deterministic") == false
raise "readiness lane omits wall-clock deferral truth" unless readiness_lane.fetch("defer_reason").include?("wall-clock")

puts "sprint 5854 readiness: PASS"
