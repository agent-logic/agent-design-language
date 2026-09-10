#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "time"

PLAN = ".csdlc/prepared/issues/819/retained-v3-resolution-plan.json"
OUT = ".csdlc/evidence/819/retained-v3"
EXPECTED_CANDIDATE = "fb6cbc7f619daa54f901fd2d12f480add682ace3"

def git(*args)
  stdout, status = Open3.capture2e("git", *args)
  raise "git #{args.join(' ')} failed: #{stdout}" unless status.success?
  stdout.strip
end

def git_bytes(*args)
  stdout, stderr, status = Open3.capture3("git", *args)
  raise "git #{args.join(' ')} failed: #{stderr}" unless status.success?
  stdout
end

def execute(id, argv, env = {})
  started = Time.now.utc
  stdout, status = Open3.capture2e(env, *argv)
  finished = Time.now.utc
  output = stdout.sub(/\n+\z/, "\n")
  {
    "id" => id,
    "argv" => argv,
    "started_at" => started.iso8601(6),
    "finished_at" => finished.iso8601(6),
    "exit_code" => status.exitstatus,
    "status" => status.success? ? "passed" : "failed",
    "output_sha256" => Digest::SHA256.hexdigest(output),
    "output" => output
  }
end

plan = JSON.parse(File.read(PLAN))
candidate = EXPECTED_CANDIDATE
raise "candidate must be exact SHA" unless candidate.match?(/\A[0-9a-f]{40}\z/)
raise "candidate commit unavailable" unless git("cat-file", "-t", candidate) == "commit"

proof_surface_paths = %w[
  csdlc-v3
  docs/csdlc-v3
  .csdlc/evidence/505
  .csdlc/issues/505
  .csdlc/prepared/issues/571/validate-v3a-followup.rb
]
_stdout, proof_surface_status = Open3.capture2e("git", "diff", "--quiet", candidate, "--", *proof_surface_paths)
raise "working proof producers differ from candidate #{candidate}" unless proof_surface_status.success?

commands = [
  execute("csdlc-v3-all-tests", ["cargo", "test", "--manifest-path", "csdlc-v3/Cargo.toml"]),
  execute("csdlc-v3-clippy", ["cargo", "clippy", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets", "--", "-D", "warnings"]),
  execute("v3a-current-contract", ["ruby", ".csdlc/prepared/issues/571/validate-v3a-followup.rb"])
]

FileUtils.mkdir_p(OUT)
commands.each { |command| File.write(File.join(OUT, "#{command.fetch('id')}.log"), command.delete("output")) }
raise "one or more proof commands failed" unless commands.all? { |command| command.fetch("status") == "passed" }

test_log = File.read(File.join(OUT, "csdlc-v3-all-tests.log"))
rows = plan.fetch("rows").map do |row|
  resolution = row.fetch("resolution").dup
  if resolution.fetch("type") == "candidate_bound_execution"
    resolution["status"] = "passed"
    resolution["candidate"] = candidate
  else
    resolution["status"] = "pending_operator_review"
    resolution["candidate"] = candidate
  end
  evidence = row.fetch("source_evidence_paths").map do |path|
    candidate_bytes = git_bytes("show", "#{candidate}:#{path}")
    {
      "path" => path,
      "status" => "present",
      "sha256" => Digest::SHA256.hexdigest(candidate_bytes),
      "git_blob" => git("rev-parse", "#{candidate}:#{path}")
    }
  end
  row.merge("resolution" => resolution, "candidate_evidence" => evidence)
end

surface_paths = proof_surface_paths
surface_tree = surface_paths.flat_map do |path|
  git("ls-tree", "-r", "--full-tree", candidate, "--", path).lines.map(&:strip)
end.sort.join("\n")

receipt = {
  "schema" => "adl.v0921.issue819.retained_v3_execution_receipt.v1",
  "issue" => 819,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "candidate" => candidate,
  "candidate_surface_tree_sha256" => Digest::SHA256.hexdigest(surface_tree),
  "candidate_surface_paths" => surface_paths,
  "generated_at" => Time.now.utc.iso8601(6),
  "commands" => commands,
  "row_count" => rows.length,
  "execution_passed" => rows.count { |row| row.dig("resolution", "type") == "candidate_bound_execution" && row.dig("resolution", "status") == "passed" },
  "governed_disposition_proposals" => rows.count { |row| row.dig("resolution", "type") == "governed_disposition_proposal" },
  "operator_approval_pending" => rows.count { |row| row.dig("resolution", "status") == "pending_operator_review" },
  "unclassified" => rows.count { |row| !%w[passed pending_operator_review].include?(row.dig("resolution", "status")) },
  "release_ready" => false,
  "rows" => rows
}
File.write(File.join(OUT, "reconciliation.json"), JSON.pretty_generate(receipt) + "\n")
puts "candidate=#{candidate} rows=#{receipt['row_count']} execution=#{receipt['execution_passed']} removal_proposals=#{receipt['governed_disposition_proposals']} approval_pending=#{receipt['operator_approval_pending']} unclassified=#{receipt['unclassified']} release_ready=#{receipt['release_ready']}"
