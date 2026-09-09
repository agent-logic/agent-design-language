#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

PACKET = File.expand_path(__dir__)

def read_json(name)
  JSON.parse(File.read(File.join(PACKET, name)))
end

def fail!(message)
  abort(message)
end

path = ARGV.fetch(0) { fail!("usage: validate-specialist-input.rb <input.json>") }
input = JSON.parse(File.read(path))
lane = input.fetch("lane")
candidate = read_json("run_manifest.json").fetch("candidate_sha")
assignment = read_json("assignments.json").fetch("assignments").find { |row| row.fetch("lane") == lane }
fail!("unknown lane: #{lane}") unless assignment

fail!("wrong specialist input schema") unless input.fetch("schema") == "adl.v0921.internal_review_specialist_input.v1"
fail!("input is not completed at the frozen candidate") unless input.fetch("candidate_sha") == candidate && input.fetch("status") == "completed"
fail!("reviewer is not independently attributed") unless input.fetch("reviewer").start_with?("subagent:")
fail!("review method is absent") unless input.fetch("review_method").strip.length >= 20

expected_refs = assignment.fetch("denominator_refs").sort
fail!("input assignment differs from canonical lane") unless input.fetch("denominator_refs").sort == expected_refs
observations = input.fetch("observations")
actual_refs = observations.map { |row| row.fetch("ref") }
fail!("observations do not cover every assigned ref exactly once") unless actual_refs.sort == expected_refs && actual_refs.uniq.length == actual_refs.length

allowed_basis = %w[candidate_path acceptance_mapping live_state command retained_proof]
allowed_implementation = %w[implemented partial missing not_applicable]
allowed_proof = %w[proved partial missing not_applicable]
observations.each do |row|
  fail!("invalid conclusion: #{row.fetch('ref')}") unless %w[verified_no_gap finding].include?(row.fetch("conclusion"))
  fail!("content-free detail: #{row.fetch('ref')}") unless row.fetch("detail").strip.length >= 20
  basis = row.fetch("review_basis")
  fail!("invalid review basis: #{row.fetch('ref')}") unless basis.is_a?(Hash) && allowed_basis.include?(basis.fetch("kind")) && !basis.fetch("subject").strip.empty?
  next unless row.fetch("ref").start_with?("ACCEPT-")
  fail!("non-terminal implementation disposition: #{row.fetch('ref')}") unless allowed_implementation.include?(row.fetch("implementation_disposition"))
  fail!("non-terminal proof disposition: #{row.fetch('ref')}") unless allowed_proof.include?(row.fetch("proof_disposition"))
end

expected_findings = read_json("finding-input.json").fetch("findings").select { |row| row.fetch("source_lane") == lane }.map { |row| row.fetch("id") }.sort
fail!("finding IDs differ from canonical lane findings") unless input.fetch("finding_ids").sort == expected_findings

if lane == "tests"
  invocations = input.fetch("test_invocations")
  fail!("tests lane needs at least three distinct invocations") unless invocations.length >= 3 && invocations.map { |row| row.fetch("id") }.uniq.length == invocations.length
  fail!("tests lane execution scope is absent") if input.fetch("execution_scope").strip.empty?
  invocations.each do |invocation|
    output = invocation.fetch("captured_output")
    fail!("test invocation is not replayable: #{invocation.fetch('id')}") unless
      invocation.fetch("argv").is_a?(Array) && !invocation.fetch("argv").empty? &&
      invocation.fetch("exit_status") == 0 && !output.empty? &&
      Digest::SHA256.hexdigest(output) == invocation.fetch("captured_output_sha256") &&
      invocation.fetch("success_markers").is_a?(Array) && !invocation.fetch("success_markers").empty? &&
      invocation.fetch("command_artifacts").is_a?(Array) && !invocation.fetch("command_artifacts").empty?
  end
end

puts JSON.generate(status: "passed", lane: lane, candidate_sha: candidate, observations: observations.length, findings: expected_findings.length)
