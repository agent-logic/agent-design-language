# frozen_string_literal: true

require "json"
require "digest"
require "open3"
require "pathname"
require "time"
require "yaml"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__))
SPRINT = 5854
UNBOUND = [5835, 5836, 5838, 5839].freeze
CARDS = %w[sip stp spp vpp srp sor].freeze
EXPECTED_CODE_REPOSITORY = "agent-logic/agent-design-language"
EXPECTED_WAVE_GATES = [
  "WP-17 after #5826, #5827, and #5834",
  "WP-18 after #5825 through #5830, canonical WP-14 #209 / PR #215, #5833, and #5834; legacy #5832 is superseded",
  "WP-18B after canonical WP-14 #209 / PR #215, #5834, and WP-18",
  "WP-19 after #5834 and WP-17 plus accepted v0.93 allocation",
  "WP-24 final claims after WP-23"
].freeze

def path_overlap?(left, right)
  left_path = Pathname.new(left).cleanpath.to_s.sub(%r{/+\z}, "")
  right_path = Pathname.new(right).cleanpath.to_s.sub(%r{/+\z}, "")
  left_path == right_path || left_path.start_with?("#{right_path}/") || right_path.start_with?("#{left_path}/")
end

def validate_parallel_contract!(machine_lanes, wave_lanes, human_lanes)
  raise "canonical parallel-lane contract differs across authorities" unless machine_lanes == wave_lanes && machine_lanes == human_lanes
end

def validate_parallel_ownership!(lanes, affected_by_issue)
  lanes.each do |lane|
    lane.fetch("issues").combination(2) do |left_issue, right_issue|
      overlaps = affected_by_issue.fetch(left_issue).product(affected_by_issue.fetch(right_issue)).select do |left_path, right_path|
        path_overlap?(left_path, right_path)
      end
      raise "parallel ownership overlap between #{left_issue} and #{right_issue}: #{overlaps.inspect}" unless overlaps.empty?
    end
  end
end

def parse_human_parallel_lanes(text)
  section = text.split("## Safe Parallel Lanes", 2).fetch(1).split(/\n## /, 2).first
  rows = section.lines.select { |line| line.lstrip.start_with?("|") }
  raise "human parallel-lane table is incomplete" if rows.length < 3
  rows.drop(2).map do |line|
    cells = line.split("|").map(&:strip).reject(&:empty?)
    raise "human parallel-lane row is malformed: #{line.strip}" unless cells.length == 4
    issues = cells.fetch(1).scan(/#(\d+)/).flatten.map(&:to_i)
    raise "human parallel-lane row has fewer than two issues: #{line.strip}" if issues.length < 2
    {
      "issues" => issues,
      "boundary" => cells.fetch(2),
      "gate" => cells.fetch(3)
    }
  end
end

if ARGV == ["--negative-overlap"]
  lane = { "issues" => [1, 2], "gate" => "terminal prerequisites", "boundary" => "disjoint paths" }
  begin
    validate_parallel_contract!([lane], [lane.merge("gate" => "weaker gate")], [lane])
    raise "negative authority control accepted divergent lane semantics"
  rescue RuntimeError => error
    raise unless error.message == "canonical parallel-lane contract differs across authorities"
  end
  divergent_human = <<~MARKDOWN
    ## Safe Parallel Lanes

    | Lane | Issues | Why parallel-safe | Required coordination |
    |---|---|---|---|
    | expected | `#1`, `#2` | disjoint paths | terminal prerequisites |
    | hidden extra | `#2`, `#3` | unknown paths | weaker gate |
  MARKDOWN
  begin
    validate_parallel_contract!([lane], [lane], parse_human_parallel_lanes(divergent_human))
    raise "negative authority control accepted an extra human-only lane"
  rescue RuntimeError => error
    raise unless error.message == "canonical parallel-lane contract differs across authorities"
  end
  begin
    validate_parallel_ownership!([lane], { 1 => ["docs/shared"], 2 => ["docs/shared/child.md"] })
    raise "negative overlap control accepted descendant ownership"
  rescue RuntimeError => error
    raise unless error.message.start_with?("parallel ownership overlap")
  end
  puts "sprint 5854 overlap negative control: PASS"
  exit 0
end

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

wave = YAML.safe_load((ROOT / "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml").read)
wave_sprint = wave.fetch("execution_sprints").find { |candidate| candidate.fetch("issue") == SPRINT }
raise "Sprint 5 missing from canonical issue wave" unless wave_sprint
raise "canonical issue-wave gates differ from packet" unless wave_sprint.fetch("serial_gates") == EXPECTED_WAVE_GATES
wave_wp24a = wave_sprint.fetch("out_of_band_streams").find { |stream| stream.fetch("issue") == 5845 }
raise "canonical issue wave does not exclude WP-24A" unless wave_wp24a&.fetch("member") == "WP-24A"
raise "canonical issue wave lets WP-24A gate" unless wave_wp24a["gates_sprint"] == false && wave_wp24a["dependency_on_sprint"] == false

human = (ROOT / ".csdlc/prepared/issues/5854/sprint-execution-packet.md").read
%w[Child\ Issue\ Wave Recommended\ Execution\ Order Watcher\ Policy Budget\ And\ Goal\ Accounting Safe\ Parallel\ Lanes Serial\ Gates Sprint-Level\ Review].each do |section|
  heading = "## #{section.tr('\\', '')}"
  raise "missing packet section #{heading}" unless human.include?(heading)
end
raise "WP-24A non-gating boundary missing" unless human.include?("cannot gate Sprint 5")
raise "deferred proof is overstated" unless human.include?("never validation evidence")
raise "human packet retains an ungoverned candidate parallel lane" if human.include?("## Candidate Parallel Lanes")
human_parallel_lanes = parse_human_parallel_lanes(human)

stp_values = JSON.parse((ROOT / ".csdlc/issues/5854/cards/stp.values.json").read).dig("content", "values")
operative_closeout = "the four operative children (#5835, #5836, #5838, and #5839)"
raise "STP does not name the exact operative closeout boundary" unless stp_values.fetch("acceptance_criteria").any? { |criterion| criterion.include?(operative_closeout) && criterion.include?("WP-24A #5845 cannot gate") }
srp_values = JSON.parse((ROOT / ".csdlc/issues/5854/cards/srp.values.json").read).dig("content", "values")
raise "SRP does not review the exact operative closeout boundary" unless srp_values.fetch("review_prompts").any? { |prompt| prompt.include?(operative_closeout) && prompt.include?("WP-24A #5845 excluded") }
expected_card_children = UNBOUND.sort
{
  "SIP required outcome" => JSON.parse((ROOT / ".csdlc/issues/5854/cards/sip.values.json").read).dig("content", "values", "required_outcome"),
  "SPP summary" => JSON.parse((ROOT / ".csdlc/issues/5854/cards/spp.values.json").read).dig("content", "values", "summary")
}.each do |surface, text|
  observed_children = text.scan(/#(\d+)/).flatten.map(&:to_i).uniq.sort
  raise "#{surface} child denominator mismatch: #{observed_children.inspect}" unless observed_children == expected_card_children
end

session_prompt = (ROOT / ".adl/docs/TBD/V092_SPRINT_5854_DEMO_PUBLICATION_SESSION_PROMPT.md").read
session_text = session_prompt.split.join(" ")
raise "session prompt falsely calls WP-24 typed-terminal" if session_prompt.include?("#5844, WP-24: terminal")
raise "session prompt omits asynchronous WP-24 typed closeout" unless session_prompt.include?("typed closeout continues asynchronously")
raise "session prompt still requires ordinary pre-bind child doctor" if session_prompt.include?("typed doctor for #5854 and each child")
raise "session prompt omits expected split-authority diagnosis" unless session_text.include?("ordinary doctor is expected to report `repository_identity_drift` until typed bind")
raise "session prompt omits post-bind doctor" unless session_text.include?("Run ordinary doctor in the child worktree only after bind succeeds")
raise "session prompt omits WP-19 serial gate" unless session_prompt.include?("#5839 follows #5834 and #5835 plus accepted v0.93 allocation")
raise "session prompt does not route WP-20 to final sprint" unless session_prompt.include?("WP-20 (`#5840`) is the first child of final release-tail sprint `#5856`")

bind_manifest = JSON.parse((ROOT / ".csdlc/prepared/issues/5854/split-authority-bind-requests.json").read)
raise "split-authority manifest schema mismatch" unless bind_manifest.fetch("schema") == "adl.v092.sprint_5854_split_authority_bind_requests.v1"
raise "split-authority issue repository mismatch" unless bind_manifest.fetch("issue_repository") == "danielbaustin/agent-design-language"
raise "split-authority code repository mismatch" unless bind_manifest.fetch("code_repository") == EXPECTED_CODE_REPOSITORY
raise "ordinary pre-bind doctor behavior is not explicit" unless bind_manifest.fetch("ordinary_doctor_before_bind") == "expected_repository_identity_drift"
bind_entries = bind_manifest.fetch("requests")
raise "split-authority manifest child set mismatch" unless bind_entries.map { |entry| entry.dig("request", "issue") } == UNBOUND
manifest_work_packages = bind_entries.map { |entry| entry.fetch("work_package") }
raise "machine packet work-package order differs from bind manifest" unless packet.fetch("ordered_work_packages") == manifest_work_packages
raise "canonical issue-wave membership differs from machine packet" unless wave_sprint.fetch("members") == packet.fetch("ordered_work_packages")
raise "completed WP-24 leaked into operative membership" if wave_sprint.fetch("members").include?("WP-24")
raise "machine packet completed work-package mismatch" unless packet.fetch("completed_work_packages") == ["WP-24"]
completed_wp24 = wave_sprint.fetch("completed_streams").find { |stream| stream.fetch("member") == "WP-24" }
raise "canonical issue wave does not classify completed WP-24" unless completed_wp24&.fetch("legacy_issue") == 5844 && completed_wp24.fetch("canonical_issue") == 10

machine_parallel_lanes = packet.fetch("safe_parallel_lanes")
wave_parallel_lanes = wave_sprint.fetch("parallel_lanes").map { |lane| lane.slice("issues", "gate", "boundary") }
validate_parallel_contract!(machine_parallel_lanes, wave_parallel_lanes, human_parallel_lanes)
wave_sprint.fetch("parallel_lanes").each do |lane|
  expected_members = lane.fetch("issues").map do |issue|
    bind_entries.find { |entry| entry.dig("request", "issue") == issue }.fetch("work_package")
  end
  raise "canonical parallel lane work packages differ from issue owners" unless lane.fetch("members") == expected_members
end

bind_source = (ROOT / "csdlc-v2/src/lifecycle.rs").read
diagnosis_offset = bind_source.index("crate::doctor::diagnose_with_code_repository(")
topology_offset = bind_source.index("let listed = git::worktrees(store.root())?")
raise "typed bind no longer performs split-authority source diagnosis" unless diagnosis_offset && topology_offset && diagnosis_offset < topology_offset
raise "typed bind does not pass request.code_repository to diagnosis" unless bind_source.include?("request.code_repository.as_deref(),")
gate2 = (ROOT / "csdlc-v2/tests/gate2.rs").read
raise "split-authority bind regression proof missing" unless gate2.include?("split_without_contract") && gate2.include?("code_repository\": \"agent-logic/agent-design-language")

affected_by_issue = {}

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
  affected_by_issue[issue] = affected
  raise "issue #{issue} has non-path ownership" if affected.any? { |path| path.start_with?("SERIALIZATION_GATE") }
  raise "issue #{issue} has a started plan step" unless spp.fetch("steps").all? { |step| step.fetch("status") == "pending" }

  bind_entry = bind_entries.find { |entry| entry.dig("request", "issue") == issue }
  request = bind_entry.fetch("request")
  raise "issue #{issue} split-authority generation drift" unless bind_entry.fetch("source_generation") == record.fetch("generation")
  raise "issue #{issue} split-authority digest drift" unless bind_entry.fetch("source_digest") == record.fetch("digest")
  raise "issue #{issue} unexpectedly records a code repository before bind" unless record["code_repository"].nil?
  raise "issue #{issue} bind request base mismatch" unless request.fetch("base_branch") == "main"
  raise "issue #{issue} bind request branch mismatch" unless request.fetch("branch").start_with?("codex/#{issue}-")
  raise "issue #{issue} bind request is not on FastWork" unless request.fetch("worktree").start_with?("/Volumes/FastWork/adl-worktrees/adl-issue-#{issue}-")
  raise "issue #{issue} bind request code repository mismatch" unless request.fetch("code_repository") == EXPECTED_CODE_REPOSITORY

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

validate_parallel_ownership!(machine_parallel_lanes, affected_by_issue)

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
collector_bin_dir_value = ENV.fetch("CSDLC_V2_BIN_DIR", "").strip
unless collector_bin_dir_value.empty?
  collector_bin_dir = Pathname.new(collector_bin_dir_value)
  collector_bin_dir = ROOT / collector_bin_dir unless collector_bin_dir.absolute?
  {
    "issue_binary_sha256" => "csdlc-github-issue",
    "pull_request_binary_sha256" => "csdlc-github-pr"
  }.each do |field, binary_name|
    binary_path = collector_bin_dir / binary_name
    raise "installed collector binary missing: #{binary_name}" unless binary_path.file?
    raise "collector binary digest mismatch: #{binary_name}" unless Digest::SHA256.file(binary_path).hexdigest == collector.fetch(field)
  end
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
source_umbrella = source.fetch("issue_results").find do |entry|
  issue = entry.fetch("response").fetch("issue")
  issue.fetch("repository") == "danielbaustin/agent-design-language" && issue.fetch("number") == SPRINT
end&.fetch("response")&.fetch("issue")
raise "live source issue #5854 missing" unless source_umbrella
umbrella_body = source_umbrella.fetch("body")
operative_section = umbrella_body.split("## Operative Child Issues", 2).fetch(1).split("\n## ", 2).first
live_operative_children = operative_section.scan(/^- #(\d+)/).flatten.map(&:to_i)
raise "live source issue operative-child denominator mismatch" unless live_operative_children == UNBOUND
raise "live source issue does not route WP-20 to final sprint" unless umbrella_body.include?("#5840") && umbrella_body.include?("#5856")
raise "live source issue retains the obsolete all-child exit" if umbrella_body.include?("Every child is merged")
raise "live source issue retains podcast-package exit scope" if umbrella_body.include?("podcast packages")
raise "live source issue does not exclude WP-24A" unless umbrella_body.include?("#5845 (WP-24A) has no Sprint 5 dependency") && umbrella_body.include?("cannot block exit")

tooling_74 = source.fetch("issue_results").find do |entry|
  issue = entry.fetch("response").fetch("issue")
  issue.fetch("repository") == "agent-logic/agent-design-language" && issue.fetch("number") == 74
end&.fetch("response")&.fetch("issue")
raise "typed tooling issue #74 observation missing" unless tooling_74
raise "tooling issue #74 is not closed" unless tooling_74.fetch("state") == "closed" && tooling_74.fetch("closed_at") == "2026-08-10T00:11:18Z"
expected_states = {
  5819 => "closed",
  5825 => "closed",
  5826 => "closed",
  5827 => "closed",
  5828 => "closed",
  5829 => "closed",
  5830 => "closed",
  5832 => "closed",
  5833 => "closed",
  5834 => "closed",
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
raise "WP-18 canonical dependency set is incomplete" unless human.include?("canonical WP-14 `agent-logic/agent-design-language#209` / PR `#215`") && human.include?("legacy `#5832` is superseded")
raise "WP-17 is not classified as ready to bind" unless human.include?("| `#5835` | WP-17 | prepared and unbound; `#5826`, `#5827`, and `#5834` are terminal | ready to bind")
raise "WP-18 is not classified as ready to bind" unless human.include?("| `#5836` | WP-18 | prepared and unbound;") && human.include?("reviewed ancestral merge proof")

wp16_validator = [
  "ruby", ".csdlc/prepared/issues/5834/validate-review-packet.rb",
  "--packet", "docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md",
  "--manifest", "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json",
  "--schema", "docs/milestones/v0.92/review/first-birthday-review-packet.schema.json"
]
wp16_stdout, wp16_stderr, wp16_status = Open3.capture3(*wp16_validator, chdir: ROOT.to_s)
raise "WP-16 dependency authority failed: #{wp16_stderr}#{wp16_stdout}" unless wp16_status.success?

canonical_wp14 = source.fetch("issue_results").find do |entry|
  issue = entry.fetch("response").fetch("issue")
  issue.fetch("repository") == EXPECTED_CODE_REPOSITORY && issue.fetch("number") == 209
end&.fetch("response")&.fetch("issue")
raise "canonical WP-14 issue #209 observation missing" unless canonical_wp14
raise "canonical WP-14 issue #209 is not closed" unless canonical_wp14.fetch("state") == "closed"

pr14 = gates.fetch("pull_requests").find { |row| row.fetch("pull_request") == 14 }
source_pr14 = source.fetch("pull_request_results").first.fetch("response")
expected_wp24_sha = "b4f23892fa5c7b23816c8c38903ed4c73395afde"
raise "canonical WP-24 PR repository mismatch" unless pr14&.fetch("repository") == "agent-logic/agent-design-language"
raise "canonical WP-24 PR is not merged" unless pr14&.fetch("state") == "merged"
raise "canonical WP-24 merge SHA mismatch" unless pr14&.fetch("merge_sha") == expected_wp24_sha
raise "canonical WP-24 closing relation mismatch" unless pr14&.fetch("closes_issue") == 10
raise "canonical WP-24 PR projection differs from retained source" unless source_pr14.fetch("repository") == pr14.fetch("repository") && source_pr14.fetch("pull_request") == pr14.fetch("pull_request") && source_pr14.fetch("merged") && source_pr14.fetch("merge_commit_sha") == pr14.fetch("merge_sha") && source_pr14.fetch("linked_issue") == pr14.fetch("closes_issue")
raise "canonical WP-24 merge is not ancestral to readiness HEAD" unless system("git", "merge-base", "--is-ancestor", expected_wp24_sha, "HEAD", out: File::NULL, err: File::NULL)

pr215 = gates.fetch("pull_requests").find { |row| row.fetch("pull_request") == 215 }
source_pr215 = source.fetch("pull_request_results").find { |entry| entry.fetch("response").fetch("pull_request") == 215 }&.fetch("response")
expected_wp14_sha = "a77519c3fca9f64752af41c9a2ebd396468891f7"
raise "canonical WP-14 PR #215 observation missing" unless source_pr215
raise "canonical WP-14 PR repository mismatch" unless pr215&.fetch("repository") == EXPECTED_CODE_REPOSITORY
raise "canonical WP-14 PR is not merged" unless pr215&.fetch("state") == "merged" && source_pr215.fetch("merged")
raise "canonical WP-14 merge SHA mismatch" unless pr215&.fetch("merge_sha") == expected_wp14_sha && source_pr215.fetch("merge_commit_sha") == expected_wp14_sha
raise "canonical WP-14 closing relation mismatch" unless pr215&.fetch("closes_issue") == 209 && source_pr215.fetch("linked_issue") == 209
raise "canonical WP-14 merge is not ancestral to readiness HEAD" unless system("git", "merge-base", "--is-ancestor", expected_wp14_sha, "HEAD", out: File::NULL, err: File::NULL)

wp24a_observation = gates.fetch("out_of_band_observations").find { |row| row.fetch("issue") == 5845 }
raise "WP-24A out-of-band observation missing" unless wp24a_observation
raise "WP-24A observation can gate Sprint 5" unless wp24a_observation["gates_sprint"] == false
raise "WP-24A observation depends on Sprint 5" unless wp24a_observation["dependency_on_sprint"] == false
raise "publication was implicitly authorized" unless gates.dig("publication_authorization", "status") == "not_authorized"

umbrella = JSON.parse((ROOT / ".csdlc/issues/5854/index.json").read)
raise "umbrella is outside its readiness lifecycle" unless %w[bound implemented reviewed published].include?(umbrella.fetch("phase"))
raise "umbrella code repository mismatch" unless umbrella.fetch("code_repository") == EXPECTED_CODE_REPOSITORY
raise "validator is outside the recorded lifecycle-authority worktree" unless Pathname.new(umbrella.fetch("worktree")).realpath == ROOT.realpath
sparse_value, sparse_error, sparse_status = Open3.capture3("git", "config", "--bool", "core.sparseCheckout", chdir: ROOT.to_s)
raise "unable to inspect sparse-checkout state: #{sparse_error}" unless sparse_status.success? || sparse_value.strip.empty?
raise "recorded lifecycle-authority worktree still uses sparse checkout" if sparse_value.strip == "true"

umbrella_vpp = JSON.parse((ROOT / ".csdlc/issues/5854/cards/vpp.values.json").read).dig("content", "values")
readiness_lane = umbrella_vpp.fetch("lanes").find { |lane| lane.fetch("lane") == "v092-sprint5-readiness" }
raise "readiness lane missing" unless readiness_lane
raise "wall-clock-dependent readiness lane is falsely deterministic" unless readiness_lane.fetch("deterministic") == false
raise "readiness lane omits wall-clock deferral truth" unless readiness_lane.fetch("defer_reason").include?("wall-clock")

puts "sprint 5854 readiness: PASS"
