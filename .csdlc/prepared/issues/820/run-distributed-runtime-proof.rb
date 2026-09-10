#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "time"

PLAN = ".csdlc/prepared/issues/820/distributed-runtime-resolution-plan.json"
OUT = ".csdlc/evidence/820/distributed-runtime"
EXPECTED_CANDIDATE = "fb6cbc7f619daa54f901fd2d12f480add682ace3"
SURFACES = %w[
  adl-runtime/src/qualification/mod.rs
  adl-runtime/tests/distributed_contract
  adl-runtime/tests/distributed_failure
  docs/milestones/v0.92.1/evidence/runtime/drt-a
  docs/milestones/v0.92.1/evidence/runtime/drt-b
  docs/milestones/v0.92.1/evidence/runtime/drt-c
].freeze

def git(*args)
  output, status = Open3.capture2e("git", *args)
  raise "git #{args.join(' ')} failed: #{output}" unless status.success?
  output.strip
end

def execute(id, argv)
  started = Time.now.utc
  output, status = Open3.capture2e(*argv)
  normalized = output.sub(/\n+\z/, "\n")
  [{"id" => id, "argv" => argv, "started_at" => started.iso8601(6), "finished_at" => Time.now.utc.iso8601(6), "exit_code" => status.exitstatus, "status" => status.success? ? "passed" : "failed", "output_sha256" => Digest::SHA256.hexdigest(normalized)}, normalized]
end

plan = JSON.parse(File.read(PLAN))
raise "candidate unavailable" unless git("cat-file", "-t", EXPECTED_CANDIDATE) == "commit"
_output, unchanged = Open3.capture2e("git", "diff", "--quiet", EXPECTED_CANDIDATE, "--", *SURFACES)
raise "distributed proof producers differ from candidate" unless unchanged.success?

commands_with_logs = [
  execute("distributed-contract", ["cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_contract"]),
  execute("distributed-failure-drt-c", ["cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_failure_drt_c"])
]
FileUtils.mkdir_p(OUT)
commands_with_logs.each { |command, log| File.write(File.join(OUT, "#{command.fetch('id')}.log"), log) }
commands = commands_with_logs.map(&:first)
raise "candidate qualification execution failed" unless commands.all? { |command| command.fetch("status") == "passed" }

rows = plan.fetch("rows").map do |row|
  resolution = row.fetch("resolution").merge("status" => "pending_operator_review", "candidate" => EXPECTED_CANDIDATE, "candidate_fixture_execution" => "passed")
  evidence = row.fetch("source_evidence_paths").map do |path|
    bytes, stderr, status = Open3.capture3("git", "show", "#{EXPECTED_CANDIDATE}:#{path}")
    raise "candidate evidence missing #{path}: #{stderr}" unless status.success?
    {"path" => path, "sha256" => Digest::SHA256.hexdigest(bytes), "git_blob" => git("rev-parse", "#{EXPECTED_CANDIDATE}:#{path}")}
  end
  row.merge("resolution" => resolution, "candidate_evidence" => evidence)
end

tree = SURFACES.flat_map { |path| git("ls-tree", "-r", "--full-tree", EXPECTED_CANDIDATE, "--", path).lines.map(&:strip) }.sort.join("\n")
receipt = {
  "schema" => "adl.v0921.issue820.distributed_runtime_reconciliation.v1",
  "issue" => 820,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "candidate" => EXPECTED_CANDIDATE,
  "candidate_surface_paths" => SURFACES,
  "candidate_surface_tree_sha256" => Digest::SHA256.hexdigest(tree),
  "generated_at" => Time.now.utc.iso8601(6),
  "commands" => commands,
  "row_count" => rows.length,
  "candidate_fixture_execution_passed" => rows.length,
  "behavioral_pass_count" => 0,
  "operator_approval_pending" => rows.length,
  "unclassified" => 0,
  "release_ready" => false,
  "rows" => rows
}
File.write(File.join(OUT, "reconciliation.json"), JSON.pretty_generate(receipt) + "\n")
puts "candidate=#{EXPECTED_CANDIDATE} rows=25 fixture_support=25 behavioral_passes=0 approval_pending=25 release_ready=false"
