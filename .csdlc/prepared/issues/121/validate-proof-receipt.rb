#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

require_relative "../5862/proof-receipt-contract"

EXPECTED_NEGATIVE_CASES = {
  "fence_without_holder_key" => "fenced",
  "revoke_without_holder_key" => "fenced",
  "fence_same_epoch" => "denied",
  "fence_epoch_gap" => "denied",
  "fence_stale_epoch" => "denied",
  "fenced_mutation" => "denied",
  "restore_current_index" => "recovered",
  "recovery_floor_retained" => "fenced",
  "premature_activation" => "denied",
  "holder_operation_possession" => "denied",
  "atomic_fence_failure" => "fail_closed"
}.freeze
ISSUE_EVIDENCE_PREFIX = ".csdlc/evidence/121/"
MACHINE_MARKER = "ADL_NEGATIVE_CASE_V1 "

def fail_receipt(message)
  abort "issue 121 receipt: #{message}"
end

def issue_evidence_file(path, digest, label)
  fail_receipt("#{label} escapes issue evidence") unless path.is_a?(String) &&
    path.start_with?(ISSUE_EVIDENCE_PREFIX) &&
    !path.split("/").include?("..")
  fail_receipt("#{label} missing") unless File.file?(path)
  fail_receipt("#{label} digest malformed") unless digest.to_s.match?(/\A[0-9a-f]{64}\z/)
  fail_receipt("#{label} digest mismatch") unless Digest::SHA256.file(path).hexdigest == digest
end

evidence_path = ARGV.fetch(0, ".csdlc/evidence/121/execution-proof.json")
fail_receipt("execution proof missing") unless File.file?(evidence_path)
proof = JSON.parse(File.read(evidence_path))
fail_receipt("execution proof issue mismatch") unless proof["issue"] == 121

outer_cases = Array(proof["negative_cases"])
outer_map = outer_cases.to_h { |entry| [entry["case"], entry["result"]] }
fail_receipt("outer negative-case denominator or names mismatch") unless
  outer_cases.length == EXPECTED_NEGATIVE_CASES.length &&
  outer_map.length == EXPECTED_NEGATIVE_CASES.length &&
  outer_map == EXPECTED_NEGATIVE_CASES

machine_paths = outer_cases.map { |entry| entry["evidence_path"] }.uniq
machine_digests = outer_cases.map { |entry| entry["evidence_sha256"] }.uniq
fail_receipt("outer cases do not bind one machine artifact") unless
  machine_paths.length == 1 && machine_digests.length == 1
machine_path = machine_paths.fetch(0)
issue_evidence_file(machine_path, machine_digests.fetch(0), "machine negative cases")
machine = JSON.parse(File.read(machine_path))
fail_receipt("machine evidence schema mismatch") unless
  machine["schema"] == "adl.wp04.negative_cases.machine.v1"
fail_receipt("machine evidence issue mismatch") unless machine["issue"] == 121
fail_receipt("machine evidence source mismatch") unless
  machine["source_revision"] == proof["source_revision"]

producer_path = machine["producer_path"]
fail_receipt("producer path mismatch") unless
  producer_path == ".csdlc/evidence/121/derive-negative-cases.rb"
issue_evidence_file(producer_path, machine["producer_sha256"], "machine producer")
stdout_path = machine["stdout_path"]
stderr_path = machine["stderr_path"]
issue_evidence_file(stdout_path, machine["stdout_sha256"], "machine stdout")
issue_evidence_file(stderr_path, machine["stderr_sha256"], "machine stderr")

observed = File.readlines(stdout_path, chomp: true).each_with_object([]) do |line, entries|
  next unless line.start_with?(MACHINE_MARKER)
  payload = JSON.parse(line.delete_prefix(MACHINE_MARKER))
  entries << {
    "case" => payload.fetch("case"),
    "result" => payload.fetch("result"),
    "observed_line_sha256" => Digest::SHA256.hexdigest(line)
  }
end
observed_map = observed.to_h { |entry| [entry["case"], entry["result"]] }
fail_receipt("executed negative-case denominator or names mismatch") unless
  observed.length == EXPECTED_NEGATIVE_CASES.length &&
  observed_map.length == EXPECTED_NEGATIVE_CASES.length &&
  observed_map == EXPECTED_NEGATIVE_CASES
fail_receipt("machine case records do not match executed markers") unless
  Array(machine["cases"]) == EXPECTED_NEGATIVE_CASES.keys.map { |name|
    observed.find { |entry| entry["case"] == name }
  }

ARGV[0] = evidence_path
Wp04ProofReceiptContract.validate(
  issue: 121,
  wp: "WP-04.07",
  paths: ["adl-runtime/src/distributed/lease.rs", "adl-runtime/tests/distributed_lease.rs"],
  test: "distributed_lease",
  platforms: []
)
