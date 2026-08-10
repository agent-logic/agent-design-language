#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

require "digest"
require "json"

CANONICAL_PROOF = ".csdlc/evidence/5873/remediation-v2/execution-proof.json"
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
negative_cases = proof.fetch("negative_cases")
observed = negative_cases.to_h { |entry| [entry.fetch("case"), entry.fetch("result")] }
abort "negative proof names/results mismatch" unless observed == EXPECTED_CASES
abort "negative proof denominator mismatch" unless negative_cases.length == EXPECTED_CASES.length

evidence_paths = negative_cases.map { |entry| entry.fetch("evidence_path") }.uniq
abort "negative cases must bind one machine result" unless evidence_paths.length == 1
machine = JSON.parse(File.read(evidence_paths.fetch(0)))
machine_cases = machine.fetch("cases")
abort "machine negative denominator mismatch" unless machine_cases.length == EXPECTED_CASES.length
machine_observed = machine_cases.to_h { |entry| [entry.fetch("case"), entry.fetch("result")] }
abort "machine negative names/results mismatch" unless machine_observed == EXPECTED_CASES

stdout_path = machine.fetch("stdout_path")
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
