#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 175
WORK_PACKAGE = "V3-12"
REQUIRED_ARTIFACTS = ["csdlc-v3/src/commands/review/mod.rs", "csdlc-v3/src/commands/review/recover.rs", "csdlc-v3/src/commands/publish/mod.rs", "csdlc-v3/tests/review/exact_revision.rs", "csdlc-v3/tests/review/recovery.rs", "csdlc-v3/tests/review/independence.rs", "csdlc-v3/tests/review/publication_linkage.rs"].freeze
REQUIRED_LANES = {"v3-12-focused-rust"=>["cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets"], "v3-12-diff-hygiene"=>["git", "diff", "--check", "origin/main...HEAD"]}.freeze
EXPECTED_OBSERVATIONS = {"unknown_revision_approval_count"=>["eq", 0], "substantive_change_current_review_count"=>["eq", 0], "recovery_dependent_truth_survivor_count"=>["eq", 0], "merged_or_closed_recovery_acceptance_count"=>["eq", 0], "same_principal_publication_count"=>["eq", 0], "missing_authenticated_principal_publication_count"=>["eq", 0], "ambiguous_linkage_authorization_count"=>["eq", 0], "full_linkage_recovery_journey_count"=>["gte", 2]}.freeze
PROOF_PATH = File.join(ROOT, ".csdlc/evidence/175/proof.json")

def fail!(message)
  abort("FAIL [V3-12]: " + message)
end

def git!(*args)
  stdout, stderr, status = Open3.capture3("git", *args, chdir: ROOT)
  fail!("git #{args.join(" ")} failed: #{stderr.strip}") unless status.success?
  stdout.strip
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

fail!("missing issue-specific proof #{PROOF_PATH}") unless File.file?(PROOF_PATH)
proof = JSON.parse(File.read(PROOF_PATH))
fail!("wrong proof schema") unless proof["schema"] == "adl.csdlc_v3.issue_proof.v1"
fail!("wrong issue") unless proof["issue"] == ISSUE
fail!("wrong work package") unless proof["work_package"] == WORK_PACKAGE

head = git!("rev-parse", "HEAD")
fail!("proof is not bound to exact HEAD") unless proof["revision"] == head && head.match?(/\A[0-9a-f]{40}\z/)

artifacts = proof.fetch("artifacts")
fail!("artifact entries must be an array") unless artifacts.is_a?(Array)
by_path = artifacts.to_h { |entry| [entry.fetch("path"), entry] }
fail!("artifact paths are duplicated") unless by_path.length == artifacts.length
missing = REQUIRED_ARTIFACTS - by_path.keys
fail!("missing required artifacts: #{missing.join(", ")}") unless missing.empty?

by_path.each do |relative, entry|
  path = Pathname.new(relative)
  fail!("artifact path must be repository-relative: #{relative}") if path.absolute? || path.each_filename.include?("..")
  absolute = File.join(ROOT, relative)
  fail!("artifact is missing: #{relative}") unless File.file?(absolute)
  fail!("artifact is not tracked: #{relative}") unless git!("ls-files", "--error-unmatch", "--", relative) == relative
  actual = sha256(absolute)
  fail!("artifact digest mismatch: #{relative}") unless entry["sha256"] == actual && actual.match?(/\A[0-9a-f]{64}\z/)
end

receipts = proof.fetch("producer_receipts")
fail!("producer receipts must be an array") unless receipts.is_a?(Array)
by_lane = receipts.to_h { |entry| [entry.fetch("lane"), entry] }
fail!("producer lanes are duplicated") unless by_lane.length == receipts.length
fail!("producer lane denominator mismatch") unless by_lane.keys.sort == REQUIRED_LANES.keys.sort
REQUIRED_LANES.each do |lane, expected_command|
  receipt = by_lane.fetch(lane)
  fail!("#{lane} revision mismatch") unless receipt["revision"] == head
  fail!("#{lane} command mismatch") unless receipt["command"] == expected_command
  fail!("#{lane} did not exit successfully") unless receipt["exit_code"] == 0
  %w[stdout_sha256 stderr_sha256].each do |field|
    fail!("#{lane} missing #{field}") unless receipt[field].is_a?(String) && receipt[field].match?(/\A[0-9a-f]{64}\z/)
  end
  fail!("#{lane} missing monotonic duration") unless receipt["duration_ms"].is_a?(Integer) && receipt["duration_ms"] >= 0
end

observations = proof.fetch("observations")
sources = proof.fetch("observation_sources")
fail!("observation denominator mismatch") unless observations.keys.sort == EXPECTED_OBSERVATIONS.keys.sort
fail!("observation-source denominator mismatch") unless sources.keys.sort == EXPECTED_OBSERVATIONS.keys.sort

EXPECTED_OBSERVATIONS.each do |name, (predicate, expected)|
  value = observations.fetch(name)
  source = sources.fetch(name)
  artifact = source.fetch("artifact")
  lane = source.fetch("receipt_lane")
  fail!("#{name} cites an unverified artifact") unless by_path.key?(artifact)
  fail!("#{name} artifact digest mismatch") unless source["artifact_sha256"] == by_path.fetch(artifact).fetch("sha256")
  fail!("#{name} cites an unverified producer lane") unless by_lane.key?(lane)
  passed = case predicate
           when "eq" then value == expected
           when "gte" then value.is_a?(Numeric) && value >= expected
           when "lte" then value.is_a?(Numeric) && value <= expected
           else false
           end
  fail!("#{name} observed #{value.inspect}, expected #{predicate} #{expected.inspect}") unless passed
end

fail!("generic pass flags are forbidden") if proof.key?("passed") || proof.key?("acceptance_results")
puts "PASS: V3-12 exact-head artifact, producer, and invariant proof"
