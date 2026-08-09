#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
SPRINT = 5855
ORDER = [5800, 5820, 5821, 5832, 5795].freeze
TLS_MERGE = "7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f"
MERGES = {
  5800 => TLS_MERGE,
  5820 => "b5bcfdfc13a6f454a715cbb9aa64e24bce3b7ba6",
  5821 => "0ea81fd61b0bf598ece4ce368ae5cf1a1923127c",
  5832 => "a5021ab7e9bff220021e3600fa51b4f0848f5524",
  5795 => "094797b6fe4be52549f447b0b7e513892c060436"
}.freeze
SERIAL_GATES = [
  "satisfied: issue 5800 supplied the trusted TLS baseline before issue 5820 Runtime resilience",
  "satisfied: issue 5821 followed stable issue 5820 Runtime ingress and released the separate issue 5862 implementation sprint",
  "satisfied: issue 5832 completed the protocol contract after the issue 5821 architecture gate",
  "satisfied: issue 5795 integrated after stable issues 5820 and 5832"
].freeze
SAFE_PARALLEL_LANES = [
  {
    "issues" => [5820],
    "gate" => "satisfied by ancestral issue 5800 TLS merge",
    "boundary" => "Runtime launch and resilience completed as the first product lane."
  },
  {
    "issues" => [5795],
    "gate" => "satisfied after terminal issues 5820 and 5832",
    "boundary" => "Local-provider work completed without redefining Runtime or protocol contracts."
  }
].freeze
PACKET = File.join(ROOT, ".csdlc/prepared/issues/5855/sprint-execution-packet.yaml")
PROMPT = File.join(ROOT, ".adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md")
REVIEW = File.join(ROOT, ".csdlc/evidence/5855/sprint-review.md")
ACTIVITY = File.join(ROOT, ".csdlc/evidence/5855/activity.jsonl")
INSTALL = ENV.fetch("CSDLC_INSTALL", File.join(ROOT, ".adl/bin/csdlc-v2/csdlc-install"))

packet = YAML.safe_load(File.read(PACKET))
abort("wrong sprint issue") unless packet.fetch("sprint_issue") == SPRINT
abort("sprint is not complete") unless packet.fetch("status") == "complete"
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

MERGES.each do |issue, merge_sha|
  system("git", "-C", ROOT, "merge-base", "--is-ancestor", merge_sha, "origin/main") ||
    abort("issue ##{issue} merge is not ancestral to origin/main")
end

activity = File.readlines(ACTIVITY, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
terminal_issues = activity.each_with_object([]) do |event, issues|
  issues << event["issue"] if event["event"] == "child_terminal"
end
abort("terminal activity does not match Sprint 2 membership") unless terminal_issues == ORDER

puts JSON.generate(
  schema: "adl.v092.sprint_closeout.v1",
  sprint_issue: SPRINT,
  ordered_issue_numbers: ORDER,
  merged_children: MERGES,
  excluded_issues: [5837, 83, 84],
  status: "complete"
)
