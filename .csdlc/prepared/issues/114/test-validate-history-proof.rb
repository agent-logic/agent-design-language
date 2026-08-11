#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "tmpdir"

ROOT = File.expand_path("../../../..", __dir__)
VALIDATOR = File.join(__dir__, "validate-history-proof.rb")
MANIFEST = File.join(__dir__, "history-proof-cases.json")
SCHEMA = File.join(__dir__, "history-proof-receipt-schema.v2.json")
candidate_stdout, candidate_error, candidate_status = Open3.capture3("git", "-C", ROOT, "rev-parse", "HEAD")
raise "cannot resolve test candidate: #{candidate_error}" unless candidate_status.success?
CANDIDATE = candidate_stdout.strip

def receipt(kind, extra = {})
  {
    "kind" => kind,
    "candidate_sha" => CANDIDATE,
    "status" => "passed",
    "evidence_ref" => ".csdlc/evidence/114/#{kind}.json"
  }.merge(extra)
end

def valid_results
  cases = JSON.parse(File.read(MANIFEST)).fetch("cases").map { |name| { "name" => name, "status" => "passed" } }
  {
    "schema" => "adl.conversation_history.proof_receipts.v2",
    "issue" => 114,
    "candidate_sha" => CANDIDATE,
    "cases" => cases,
    "receipts" => [
      receipt("rust_store", "command" => %w[cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history --no-tests=fail], "test_count" => 42),
      receipt("runtime_api", "command" => %w[cargo nextest run --locked --manifest-path adl/Cargo.toml --test conversation_history_runtime_api --no-tests=fail], "test_count" => 8),
      receipt("browser", "command" => %w[node adl/tools/validate_v092_html_observatory_history.mjs], "runtime_backed" => true, "browser_check_count" => 9),
      receipt("strict_clippy", "command" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history -- -D warnings], "deny_warnings" => true),
      receipt("diff_hygiene", "command" => ["git", "diff", "--check"], "clean" => true),
      receipt("independent_review", "reviewer" => "independent-reviewer", "independent" => true, "findings_status" => "resolved")
    ]
  }
end

def run_validator(results_path, candidate = CANDIDATE)
  Open3.capture3(
    "ruby", VALIDATOR,
    "--repo", ROOT,
    "--candidate-sha", candidate,
    "--manifest", MANIFEST,
    "--schema", SCHEMA,
    "--results", results_path
  )
end

def expect_pass(name, results, directory, candidate = CANDIDATE)
  path = File.join(directory, "#{name}.json")
  File.write(path, JSON.pretty_generate(results) + "\n")
  stdout, stderr, status = run_validator(path, candidate)
  raise "#{name}: expected pass, got #{stderr}#{stdout}" unless status.success?
end

def expect_fail(name, results, directory)
  path = File.join(directory, "#{name}.json")
  File.write(path, JSON.pretty_generate(results) + "\n")
  stdout, stderr, status = run_validator(path)
  raise "#{name}: false evidence unexpectedly passed: #{stdout}" if status.success?
  raise "#{name}: missing fail-closed diagnostic" unless stderr.include?("history proof invalid:")
end

Dir.mktmpdir("history-proof-self-test") do |directory|
  expect_pass("valid", valid_results, directory)
  expect_pass("head-selector", valid_results, directory, "HEAD")

  stale_path = File.join(directory, "stale-selector.json")
  File.write(stale_path, JSON.pretty_generate(valid_results) + "\n")
  _stdout, stderr, status = run_validator(stale_path, "0" * 40)
  raise "stale-selector: false evidence unexpectedly passed" if status.success?
  raise "stale-selector: missing fail-closed diagnostic" unless stderr.include?("history proof invalid:")

  stale_results = valid_results
  stale_results["candidate_sha"] = "0" * 40
  expect_fail("stale-aggregate-sha", stale_results, directory)

  duplicate_case = valid_results
  duplicate_case["cases"][-1] = duplicate_case["cases"][0].dup
  expect_fail("duplicate-case", duplicate_case, directory)

  reordered_case = valid_results
  reordered_case["cases"][0], reordered_case["cases"][1] = reordered_case["cases"][1], reordered_case["cases"][0]
  expect_fail("reordered-case", reordered_case, directory)

  incomplete_case = valid_results
  incomplete_case["cases"][0].delete("status")
  expect_fail("incomplete-case", incomplete_case, directory)

  nonpassing_case = valid_results
  nonpassing_case["cases"][0]["status"] = "failed"
  expect_fail("nonpassing-case", nonpassing_case, directory)

  missing_receipt = valid_results
  missing_receipt["receipts"].pop
  expect_fail("missing-receipt", missing_receipt, directory)

  duplicate_receipt = valid_results
  duplicate_receipt["receipts"][-1] = duplicate_receipt["receipts"][0].dup
  expect_fail("duplicate-receipt", duplicate_receipt, directory)

  reordered_receipt = valid_results
  reordered_receipt["receipts"][0], reordered_receipt["receipts"][1] = reordered_receipt["receipts"][1], reordered_receipt["receipts"][0]
  expect_fail("reordered-receipt", reordered_receipt, directory)

  incomplete_receipt = valid_results
  incomplete_receipt["receipts"][0].delete("evidence_ref")
  expect_fail("incomplete-receipt", incomplete_receipt, directory)

  nonpassing_receipt = valid_results
  nonpassing_receipt["receipts"][0]["status"] = "failed"
  expect_fail("nonpassing-receipt", nonpassing_receipt, directory)

  stale_receipt = valid_results
  stale_receipt["receipts"][0]["candidate_sha"] = "0" * 40
  expect_fail("stale-receipt-sha", stale_receipt, directory)

  false_command = valid_results
  false_command["receipts"][0]["command"] = ["true"]
  expect_fail("false-command", false_command, directory)
end

puts JSON.generate(schema: "adl.conversation_history.proof_validator_self_test.v1", cases: 14, outcome: "passed")
