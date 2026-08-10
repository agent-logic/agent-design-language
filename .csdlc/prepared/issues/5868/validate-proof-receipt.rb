#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"
require "json"
require "rbconfig"

proof_path = ARGV.fetch(0, ".csdlc/evidence/5868/execution-proof.json")
proof = JSON.parse(File.read(proof_path))
negative_paths = Array(proof["negative_cases"]).map { |entry| entry["evidence_path"] }.uniq
abort "machine-derived negative-case evidence must have one exact path" unless negative_paths.length == 1
producer = File.expand_path("derive-negative-cases.rb", __dir__)
verified = system(
  RbConfig.ruby,
  producer,
  "verify",
  negative_paths.fetch(0),
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
