#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
SPRINT = 5855
ORDER = [5800, 5820, 5821, 5832, 5795, 5837].freeze
READY_CHILDREN = [5820, 5821, 5832, 5795, 5837].freeze
TLS_MERGE = "7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f"
SERIAL_GATES = [
  "merged issue 5800 supplies the trusted TLS baseline; issue 5820 completes Runtime launch and resilience",
  "issue 5821 follows stable issue 5820 Runtime ingress and gates the separate issue 5862 implementation sprint",
  "issue 5832 follows terminal issues 5821 and 5862",
  "issue 5795 integrates after issues 5800 and 5820 plus WP-14 issue 5832 contract stability",
  "issue 5837 integrates after issues 5820 and 5832 and its WP-18 dependency"
].freeze
SAFE_PARALLEL_LANES = [
  {
    "issues" => [5820],
    "gate" => "merged issue 5800 TLS baseline is ancestral",
    "boundary" => "Runtime launch and resilience owns the first active product lane."
  },
  {
    "issues" => [5795],
    "gate" => "preparation only until issues 5820 and 5832 stabilize Runtime and protocol contracts",
    "boundary" => "Local-provider preparation cannot redefine Runtime, Observatory, or protocol contracts."
  }
].freeze
PACKET = File.join(ROOT, ".csdlc/prepared/issues/5855/sprint-execution-packet.yaml")
PROMPT = File.join(ROOT, ".adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md")
INSTALL = ENV.fetch("CSDLC_INSTALL", File.join(ROOT, ".adl/bin/csdlc-v2/csdlc-install"))
DOCTOR = ENV.fetch("CSDLC_DOCTOR", File.join(ROOT, ".adl/bin/csdlc-v2/csdlc-doctor"))

packet = YAML.safe_load(File.read(PACKET))
abort("wrong sprint issue") unless packet.fetch("sprint_issue") == SPRINT
abort("unsafe issue order") unless packet.fetch("ordered_issue_numbers") == ORDER
abort("unsafe parallel routing drift") unless packet.fetch("safe_parallel_lanes") == SAFE_PARALLEL_LANES
abort("serial gate drift") unless packet.fetch("serial_gates") == SERIAL_GATES
baseline = packet.fetch("launch_baseline")
abort("wrong TLS baseline issue") unless baseline.fetch("issue") == 5800
abort("TLS baseline is not merged") unless baseline.fetch("state") == "merged"
abort("wrong TLS merge SHA") unless baseline.fetch("merge_sha") == TLS_MERGE
abort("TLS closeout must be asynchronous") unless baseline.fetch("closeout_policy") == "asynchronous and non-blocking for product execution"

prompt = File.read(PROMPT)
abort("obsolete claim route retained") if prompt.include?("--reacquire-request")
abort("FastWork binding contract missing") unless prompt.include?("FastWork worktree")

resolved, resolve_error, resolve_status = Open3.capture3(INSTALL, "resolve", "--repo", ROOT, "--issue", SPRINT.to_s)
abort("cannot resolve C-SDLC generation: #{resolve_error}") unless resolve_status.success?
abort("C-SDLC v2 is not authoritative") unless JSON.parse(resolved) == "v2"
abort("resolved doctor is missing: #{DOCTOR}") unless File.executable?(DOCTOR)

system("git", "-C", ROOT, "merge-base", "--is-ancestor", TLS_MERGE, "origin/main") || abort("TLS merge is not ancestral to origin/main")

READY_CHILDREN.each do |issue|
  stdout, stderr, status = Open3.capture3(DOCTOR, "--repo", ROOT, "--issue", issue.to_s)
  abort("doctor failed for ##{issue}: #{stderr}") unless status.success?
  report = JSON.parse(stdout)
  abort("##{issue} is not execution-ready: #{report.fetch('findings')}") unless report.fetch("status") == "pass" && report.fetch("ready")
end

puts JSON.generate(
  schema: "adl.v092.sprint_readiness.v1",
  sprint_issue: SPRINT,
  ordered_issue_numbers: ORDER,
  merged_baseline: { issue: 5800, merge_sha: TLS_MERGE, closeout: "asynchronous" },
  ready_children: READY_CHILDREN,
  status: "prepared"
)
