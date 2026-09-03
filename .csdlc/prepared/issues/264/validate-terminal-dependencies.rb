#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../../", __dir__)
bin = File.join(root, ".adl/bin/csdlc-v2/csdlc-finish")

unless File.executable?(bin)
  warn JSON.generate(schema: "agent_logic.podcast.submission_gate_dependencies.v1", status: "failed", reason: "missing worktree-local csdlc-finish")
  exit 1
end

[261, 262, 263].each do |issue|
  system(bin, "--root", root, "--validate-cached-issue", issue.to_s, out: File::NULL, err: File::NULL) or begin
    warn JSON.generate(schema: "agent_logic.podcast.submission_gate_dependencies.v1", status: "failed", reason: "terminal dependency #{issue} did not validate")
    exit 1
  end
end

puts JSON.generate(
  schema: "agent_logic.podcast.submission_gate_dependencies.v1",
  status: "passed",
  terminal_dependencies: [261, 262, 263]
)
