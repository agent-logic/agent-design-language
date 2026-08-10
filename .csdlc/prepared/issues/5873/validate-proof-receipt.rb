#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

require "digest"
require "json"

CANONICAL_PROOF = ".csdlc/evidence/5873/remediation-v4/execution-proof.json"
EXPECTED_CASES = {
  "fenced_node_excluded" => "fenced",
  "stale_advertisement_denied" => "denied",
  "wrong_trust_domain_denied" => "denied",
  "stale_membership_denied" => "denied",
  "future_fence_denied" => "denied",
  "candidate_constraints_denied" => "denied",
  "unavailable_weather_denied" => "denied",
  "unauthorized_candidate_denied" => "denied",
  "duplicate_evidence_denied" => "denied",
  "policy_bounds_enforced" => "fail_closed",
  "membership_domain_mismatch_denied" => "denied",
  "incomplete_fencing_view_denied" => "denied",
  "caller_selected_fencing_slice_unavailable" => "denied"
}.freeze

ARGV.unshift(CANONICAL_PROOF) if ARGV.empty?

Wp04ProofReceiptContract.validate(
  issue: 5873,
  wp: "WP-04.11",
  paths: ["adl-runtime/src/distributed/placement.rs","adl-runtime/tests/distributed_placement.rs"],
  test: "distributed_placement",
  platforms: []
)

proof = JSON.parse(File.read(ARGV.fetch(0)))
source_revision = proof.fetch("source_revision")
negative_cases = proof.fetch("negative_cases")
observed = negative_cases.to_h { |entry| [entry.fetch("case"), entry.fetch("result")] }
abort "negative proof names/results mismatch" unless observed == EXPECTED_CASES
abort "negative proof denominator mismatch" unless negative_cases.length == EXPECTED_CASES.length

evidence_paths = negative_cases.map { |entry| entry.fetch("evidence_path") }.uniq
abort "negative cases must bind one machine result" unless evidence_paths.length == 1
machine = JSON.parse(File.read(evidence_paths.fetch(0)))
abort "wrong machine schema" unless machine.fetch("schema") == "adl.wp04.negative_cases.machine.v1"
abort "wrong machine issue" unless machine.fetch("issue") == 5873
abort "stale machine source" unless machine.fetch("source_revision") == source_revision
producer_path = machine.fetch("producer_path")
abort "wrong machine producer" unless producer_path == ".csdlc/evidence/5873/derive-negative-cases.rb"
abort "machine producer digest mismatch" unless Digest::SHA256.file(producer_path).hexdigest == machine.fetch("producer_sha256")
expected_argv = [
  "cargo", "test", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_placement", "--", "--nocapture"
]
command = machine.fetch("command")
abort "wrong machine producer argv" unless command.fetch("argv") == expected_argv
abort "machine producer command failed" unless command.fetch("exit_code") == 0
abort "machine producer timestamps missing" if command.fetch("started_at").empty? || command.fetch("finished_at").empty?
stdout_path = machine.fetch("stdout_path")
abort "machine stdout digest mismatch" unless Digest::SHA256.file(stdout_path).hexdigest == machine.fetch("stdout_sha256")
stderr_path = machine.fetch("stderr_path")
abort "machine stderr digest mismatch" unless Digest::SHA256.file(stderr_path).hexdigest == machine.fetch("stderr_sha256")
machine_cases = machine.fetch("cases")
abort "machine negative denominator mismatch" unless machine_cases.length == EXPECTED_CASES.length
machine_observed = machine_cases.to_h { |entry| [entry.fetch("case"), entry.fetch("result")] }
abort "machine negative names/results mismatch" unless machine_observed == EXPECTED_CASES

lines = File.readlines(stdout_path, chomp: true).select do |line|
  line.include?("ADL_ISSUE_5873_NEGATIVE_CASE_V1 ")
end
abort "machine marker denominator mismatch" unless lines.length == EXPECTED_CASES.length
marker_observed = {}
line_hashes = lines.to_h do |line|
  payload = JSON.parse(line.split("ADL_ISSUE_5873_NEGATIVE_CASE_V1 ", 2).fetch(1))
  name = payload.fetch("case")
  abort "duplicate machine marker: #{name}" if marker_observed.key?(name)
  marker_observed[name] = payload.fetch("result")
  [name, Digest::SHA256.hexdigest(line)]
end
abort "machine marker names/results mismatch" unless marker_observed == EXPECTED_CASES
machine_cases.each do |entry|
  name = entry.fetch("case")
  abort "machine marker hash mismatch: #{name}" unless entry.fetch("observed_line_sha256") == line_hashes.fetch(name)
end
puts "PASS: issue 5873 canonical proof binds exactly 13 machine-derived negative cases"
