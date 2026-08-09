#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
SPRINT = 5855
ORDER = [5800, 5820, 5821, 5795, 5832].freeze
TLS_MERGE = "7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f"
TERMINAL = {
  5800 => { "pull_request" => 9, "head_sha" => "c172b2b109d516f80aa27e8088295747b398e6c4", "merge_sha" => TLS_MERGE },
  5820 => { "pull_request" => 28, "head_sha" => "93641db996f2409baf94be2e9e6f27bb1ec9039b", "merge_sha" => "b5bcfdfc13a6f454a715cbb9aa64e24bce3b7ba6" },
  5821 => { "pull_request" => 39, "head_sha" => "a8309a776fd78c0741bf108602be6c5dd28d4cd8", "merge_sha" => "0ea81fd61b0bf598ece4ce368ae5cf1a1923127c" },
  5795 => { "pull_request" => 72, "head_sha" => "7a26886c47962e71c128489f5176a045ae8e9a64", "merge_sha" => "094797b6fe4be52549f447b0b7e513892c060436" },
  5832 => { "pull_request" => 76, "head_sha" => "23df2bab4373434c9020f0c40f772f71aef2917c", "merge_sha" => "a5021ab7e9bff220021e3600fa51b4f0848f5524" }
}.freeze
MERGES = TERMINAL.each_with_object({}) { |(issue, evidence), result| result[issue] = evidence.fetch("merge_sha") }.freeze
SERIAL_GATES = [
  "satisfied: issue 5800 supplied the trusted TLS baseline before issue 5820 Runtime resilience",
  "satisfied: issue 5821 followed stable issue 5820 Runtime ingress and released the separate issue 5862 implementation sprint",
  "satisfied: issue 5821 released independent issue 5795 provider and issue 5832 protocol completion paths",
  "observed: issue 5795 merged before issue 5832; no dependency in the reverse direction is claimed"
].freeze
SAFE_PARALLEL_LANES = [
  {
    "issues" => [5820],
    "gate" => "satisfied by ancestral issue 5800 TLS merge",
    "boundary" => "Runtime launch and resilience completed as the first product lane."
  },
  {
    "issues" => [5795, 5832],
    "gate" => "satisfied after issue 5821 completed the architecture gate",
    "boundary" => "Local-provider and protocol work used separate surfaces; actual terminal order was issue 5795 then issue 5832."
  }
].freeze
PACKET = File.join(ROOT, ".csdlc/prepared/issues/5855/sprint-execution-packet.yaml")
PROMPT = File.join(ROOT, ".adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md")
REVIEW = File.join(ROOT, ".csdlc/evidence/5855/sprint-review.md")
ACTIVITY = File.join(ROOT, ".csdlc/evidence/5855/activity.jsonl")
INSTALL = ENV.fetch("CSDLC_INSTALL", File.join(ROOT, ".adl/bin/csdlc-v2/csdlc-install"))

packet = YAML.safe_load(File.read(PACKET))
abort("wrong sprint issue") unless packet.fetch("sprint_issue") == SPRINT
abort("sprint is not a closeout candidate") unless packet.fetch("status") == "closeout_candidate"
abort("unsafe issue order") unless packet.fetch("ordered_issue_numbers") == ORDER
abort("unsafe parallel routing drift") unless packet.fetch("safe_parallel_lanes") == SAFE_PARALLEL_LANES
abort("serial gate drift") unless packet.fetch("serial_gates") == SERIAL_GATES
baseline = packet.fetch("launch_baseline")
abort("wrong TLS baseline issue") unless baseline.fetch("issue") == 5800
abort("TLS baseline is not merged") unless baseline.fetch("state") == "merged"
abort("wrong TLS merge SHA") unless baseline.fetch("merge_sha") == TLS_MERGE
abort("TLS closeout must be asynchronous") unless baseline.fetch("closeout_policy") == "asynchronous and non-blocking for product execution"
abort("sprint review is missing") unless File.file?(REVIEW)
abort("sprint activity is missing") unless File.file?(ACTIVITY)
abort("issue 5837 must remain outside Sprint 2") if packet.fetch("ordered_issue_numbers").include?(5837)
abort("HTML and Unity split is not recorded") unless File.read(REVIEW).include?("`#83` and `#84`")

prompt = File.read(PROMPT)
abort("obsolete claim route retained") if prompt.include?("--reacquire-request")
abort("FastWork binding contract missing") unless prompt.include?("FastWork worktree")

resolved, resolve_error, resolve_status = Open3.capture3(INSTALL, "resolve", "--repo", ROOT, "--issue", SPRINT.to_s)
abort("cannot resolve C-SDLC generation: #{resolve_error}") unless resolve_status.success?
abort("C-SDLC v2 is not authoritative") unless JSON.parse(resolved) == "v2"

TERMINAL.each do |issue, evidence|
  merge_sha = evidence.fetch("merge_sha")
  system("git", "-C", ROOT, "merge-base", "--is-ancestor", merge_sha, "origin/main") ||
    abort("issue ##{issue} merge is not ancestral to origin/main")
  parents, parent_error, parent_status = Open3.capture3("git", "-C", ROOT, "show", "-s", "--format=%P", merge_sha)
  abort("cannot inspect issue ##{issue} merge parents: #{parent_error}") unless parent_status.success?
  abort("issue ##{issue} reviewed head is not the merge second parent") unless parents.split[1] == evidence.fetch("head_sha")
end

ORDER.each_cons(2) do |earlier, later|
  system("git", "-C", ROOT, "merge-base", "--is-ancestor", MERGES.fetch(earlier), MERGES.fetch(later)) ||
    abort("recorded terminal order is false between issues ##{earlier} and ##{later}")
end

activity = File.readlines(ACTIVITY, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
terminal_events = activity.select { |event| event["event"] == "child_terminal" }
terminal_issues = terminal_events.map { |event| event.fetch("issue") }
terminal_events.each do |event|
  issue = event.fetch("issue")
  expected = TERMINAL.fetch(issue).merge(
    "event" => "child_terminal",
    "issue" => issue,
    "issue_state" => "closed",
    "merge_ancestral_to_origin_main" => true
  )
  abort("terminal evidence drift for issue ##{issue}") unless event == expected
end
abort("terminal activity does not match Sprint 2 membership") unless terminal_issues == ORDER
membership = activity.select { |event| event["event"] == "membership_reconciled" }
abort("membership reconciliation evidence is missing or ambiguous") unless membership.length == 1
abort("excluded issue reconciliation drift") unless membership.first.fetch("removed") == [5837] && membership.first.fetch("independent_follow_ons") == [5837, 83, 84]

puts JSON.generate(
  schema: "adl.v092.sprint_closeout.v1",
  sprint_issue: SPRINT,
  ordered_issue_numbers: ORDER,
  merged_children: MERGES,
  excluded_issues: [5837, 83, 84],
  status: "closeout_candidate"
)
