#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 164
WORK_PACKAGE = "V3-03"
REQUIRED_ARTIFACTS = ["csdlc-v3/Cargo.toml", "csdlc-v3/Cargo.lock", "csdlc-v3/src/main.rs", "csdlc-v3/src/lib.rs", "csdlc-v3/src/cli/mod.rs", "csdlc-v3/src/output/mod.rs", "csdlc-v3/tests/cli/help.rs", "csdlc-v3/tests/cli/input_conflicts.rs", "csdlc-v3/tests/cli/output_channels.rs"].freeze
REQUIRED_LANES = {"v3-03-focused-rust"=>["cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets"], "v3-03-diff-hygiene"=>["git", "diff", "--check", "origin/main...HEAD"]}.freeze
EXPECTED_OBSERVATIONS = {"binary_target_count"=>["eq", 1], "library_target_count"=>["eq", 1], "parser_io_invocation_count"=>["eq", 0], "stdout_diagnostic_mix_count"=>["eq", 0], "unsupported_jq_typed_error_cases"=>["gte", 1], "structured_input_conflict_cases"=>["gte", 1], "cargo_deny_violation_count"=>["eq", 0], "provenance_bound_executable_count"=>["eq", 1]}.freeze
PROOF_PATH = File.join(ROOT, ".csdlc/evidence/164/proof.json")

def fail!(message)
  abort("FAIL [V3-03]: " + message)
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
puts "PASS: V3-03 exact-head artifact, producer, and invariant proof"
