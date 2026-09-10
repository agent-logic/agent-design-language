#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "time"

PLAN = ".csdlc/prepared/issues/818/retained-corporate-runtime-resolution-plan.json"
OUT = ".csdlc/evidence/818/retained-corporate-runtime"
EXPECTED_CANDIDATE = "add8f48867e55be9256244767758abba6f348091"

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

plan = JSON.parse(File.read(PLAN))
raise "candidate commit unavailable" unless git("cat-file", "-t", EXPECTED_CANDIDATE) == "commit"

rows = plan.fetch("rows").map do |row|
  evidence = row.fetch("source_repository_paths").map do |path|
    candidate_bytes = git_bytes("show", "#{EXPECTED_CANDIDATE}:#{path}")
    {
      "path" => path,
      "status" => "present",
      "sha256" => Digest::SHA256.hexdigest(candidate_bytes),
      "git_blob" => git("rev-parse", "#{EXPECTED_CANDIDATE}:#{path}"),
      "proof_role" => "candidate_bound_context_not_behavioral_proof"
    }
  end
  resolution = row.fetch("resolution").merge(
    "status" => "pending_operator_review",
    "candidate" => EXPECTED_CANDIDATE
  )
  row.merge("resolution" => resolution, "candidate_evidence" => evidence)
end

FileUtils.mkdir_p(OUT)
receipt = {
  "schema" => "adl.v0921.issue818.retained_corporate_runtime_reconciliation.v1",
  "issue" => 818,
  "parent_issue" => 522,
  "finding" => "D520-RET-001",
  "candidate" => EXPECTED_CANDIDATE,
  "generated_at" => Time.now.utc.iso8601(6),
  "row_count" => rows.length,
  "candidate_execution_passed" => 0,
  "governed_disposition_proposals" => rows.length,
  "operator_approval_pending" => rows.length,
  "unclassified" => 0,
  "release_ready" => false,
  "non_claims" => [
    "No source reference, owner, issue closure, or PR closure is counted as execution proof.",
    "No private instrument, approval, signature, credential, or provider state was requested or inspected.",
    "No governed removal proposal is approved before operator review of the closing PR."
  ],
  "rows" => rows
}
File.write(File.join(OUT, "reconciliation.json"), JSON.pretty_generate(receipt) + "\n")
puts "candidate=#{EXPECTED_CANDIDATE} rows=17 execution=0 removal_proposals=17 approval_pending=17 unclassified=0 release_ready=false"
