#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"
require "digest"
require "json"
require "rbconfig"

proof_path = ARGV.fetch(0, ".csdlc/evidence/5868/execution-proof.json")
proof = JSON.parse(File.read(proof_path))
negative_paths = Array(proof["negative_cases"]).map { |entry| entry["evidence_path"] }.uniq
abort "machine-derived negative-case evidence must have one exact path" unless negative_paths.length == 1
negative_path = negative_paths.fetch(0)
machine_evidence = JSON.parse(File.read(negative_path))
machine_cases = Array(machine_evidence["cases"]).map { |entry| [entry["case"], entry["result"]] }
proof_cases = Array(proof["negative_cases"]).map { |entry| [entry["case"], entry["result"]] }
abort "proof negative cases do not exactly match executed machine cases" unless proof_cases == machine_cases
machine_digest = Digest::SHA256.file(negative_path).hexdigest
abort "proof negative-case digest does not bind machine evidence" unless Array(proof["negative_cases"]).all? { |entry| entry["evidence_sha256"] == machine_digest }
producer = File.expand_path("derive-negative-cases.rb", __dir__)
verified = system(
  RbConfig.ruby,
  producer,
  "verify",
  negative_path,
  proof.fetch("source_revision")
)
abort "machine-derived negative-case verification failed" unless verified
ARGV.replace([proof_path])

Wp04ProofReceiptContract.validate(
  issue: 5868,
  wp: "WP-04.06",
  paths: ["adl-runtime/src/distributed/failure_detection.rs","adl-runtime/tests/distributed_failure_detection.rs"],
  test: "distributed_failure_detection",
  platforms: []
)
