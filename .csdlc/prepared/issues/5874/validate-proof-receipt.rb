#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require_relative "../5862/proof-receipt-contract"

EXPECTED_CASES = %w[
  wrong_certificate_purpose oversized_snapshot relative_replay_path wrong_target
  catalog_substitution noncanonical_catalog wrong_schema tampered_signature stale_source_epoch
  stale_applied_authority incomplete_transfer replay_mismatch expired_catalog
  content_length_mismatch corrupt_chunk replay_after_restart chunk_exceeds_signed_total
].freeze

ARGV[0] ||= ".csdlc/evidence/5874/remediation-v1/execution-proof.json"
evidence_path = ARGV.fetch(0)

Wp04ProofReceiptContract.validate(
  issue: 5874,
  wp: "WP-04.12",
  paths: ["adl-runtime/src/distributed/snapshot_catalog.rs","adl-runtime/tests/distributed_snapshot_catalog.rs"],
  test: "distributed_snapshot_catalog",
  platforms: []
)

proof = JSON.parse(File.read(evidence_path))
source = proof.fetch("source_revision")
negative_entries = proof.fetch("negative_cases")
abort "negative proof denominator mismatch" unless negative_entries.length == EXPECTED_CASES.length
abort "negative proof case mapping mismatch" unless negative_entries.map { |entry| entry.fetch("case") } == EXPECTED_CASES
machine_paths = negative_entries.map { |entry| entry.fetch("evidence_path") }.uniq
abort "negative proof must bind exactly one machine artifact" unless machine_paths.length == 1
machine_path = machine_paths.fetch(0)
machine = JSON.parse(File.read(machine_path))
abort "machine case denominator mismatch" unless machine.fetch("cases").map { |entry| entry.fetch("case") } == EXPECTED_CASES
abort "machine/proof result mismatch" unless machine.fetch("cases").map { |entry| [entry.fetch("case"), entry.fetch("result")] } == negative_entries.map { |entry| [entry.fetch("case"), entry.fetch("result")] }
producer = machine.fetch("producer_path")
stdout, stderr, status = Open3.capture3("ruby", producer, "verify", machine_path, source)
abort "machine producer verification failed: #{stderr.strip}" unless status.success?
abort "machine producer verification did not pass" unless stdout.include?("PASS: machine evidence verified")
puts "PASS: WP-04.12 machine producer and exact case mapping verified"
