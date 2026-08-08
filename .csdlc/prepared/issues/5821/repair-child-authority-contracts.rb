#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "fileutils"

ROOT = File.expand_path("../../../..", __dir__)
OUT = File.join(__dir__, "child-authority-repair")
EDIT = ARGV.fetch(0)

CONTRACTS = {
  5869 => {
    outcome: "Implement OpenRaft majority-committed authority, joint membership, canonical AuthorityCertificateV1 endorsements, activation-key possession, monotonic epochs, bounded leases, and fail-closed mutation-sink verification.",
    proof: "Exact nextest target distributed_lease proves three-voter majority, joint membership, canonical AuthorityCertificateV1 encoding, distinct purpose-bound majority endorsements, activation-key possession, voter-generation and applied-index checks, monotonic epochs, renewal, expiry, revocation, quorum loss, malicious-leader/minority denial, clock uncertainty, stale-holder denial, and restart recovery.",
    safety: "Restore authority only from a majority-committed prefix; local durability, a leader assertion, or a minority history never grants authority."
  },
  5870 => {
    outcome: "Enforce one authoritative owner at every mutation sink using a current majority-endorsed AuthorityCertificateV1, activation-key possession, committed voter generation, applied index, epoch, operation class, and lease safety.",
    proof: "Exact nextest target distributed_fencing proves mutation-sink enforcement, majority-certificate validation, voter-generation and applied-index checks, activation possession, stale epoch, cloned state, malicious-leader/minority denial, split-brain, wrong owner, expiry/revocation, post-partition, and recovery fencing.",
    safety: "Keep uncertain owners fenced and restore only a quorum-committed owner or newer majority-committed epoch after the prior lease safety window."
  },
  5875 => {
    outcome: "Implement migration with source authority retained before fence, majority-committed fencing and source-permit revocation at the boundary, and certificate-bound target activation afterward.",
    proof: "Exact nextest target distributed_migration proves every transition, idempotence, source retention before fence, majority-committed fencing, source-permit revocation, activation-certificate and activation-key checks, interruption on both sides of the fence boundary, and split-brain denial.",
    safety: "Before fence the source may resume; after fence both candidates remain non-authoritative and recovery routes through WP-04.14."
  },
  5876 => {
    outcome: "Recover one owner only from a majority-committed OpenRaft prefix and valid AuthorityCertificateV1, leaving both candidates fenced when no quorum can prove authority.",
    proof: "Exact nextest target distributed_recovery proves each migration failure stage, restart, target/source loss, quorum loss, divergent local histories, malicious-leader/minority denial, certificate expiry/revocation, audit continuity, safety-window enforcement, and one-owner restoration from a majority-committed prefix.",
    safety: "Never select the numerically highest local epoch or last durable local owner without quorum proof; ambiguity requires operator trust-domain recovery."
  }
}.freeze

def index(issue)
  JSON.parse(File.read(File.join(ROOT, ".csdlc/issues", issue.to_s, "index.json")))
end

def write_request(issue, name, body)
  FileUtils.mkdir_p(OUT)
  path = File.join(OUT, "#{issue}-#{name}.json")
  File.write(path, JSON.pretty_generate(body) + "\n")
  path
end

def run_edit(path, subcommand)
  stdout, stderr, status = Open3.capture3(EDIT, "--repo", ROOT, subcommand, "--request", path)
  warn stderr unless stderr.empty?
  abort "#{subcommand} failed for #{path}: #{stdout}" unless status.success?
end

CONTRACTS.each do |issue, contract|
  current = index(issue)
  unless current.dig("design_review", "approved", "reviewer") == "codex:5821-wp04-child-authority-design-review"
    request = {
      issue: issue,
      expected_generation: current.fetch("generation"),
      expected_digest: current.fetch("digest"),
      reviewer: "codex:5821-wp04-child-authority-design-review"
    }
    run_edit(write_request(issue, "approve-design", request), "approve-design")
  end

  current = index(issue)
  request = {
    issue: issue,
    card: "sip",
    expected_generation: current.fetch("generation"),
    expected_digest: current.fetch("digest"),
    actor: "codex:5821-wp04-child-authority-repair",
    reason: "Align the executable child contract with the reviewed distributed authority architecture.",
    operation: {
      operation: "set_field",
      field: "required_outcome",
      value: contract.fetch(:outcome)
    }
  }
  run_edit(write_request(issue, "sip-required-outcome", request), "apply")

  current = index(issue)
  stp = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues", issue.to_s, "cards/stp.values.json")))
  deliverables = stp.dig("content", "values", "deliverables")
  deliverables = deliverables.reject { |value| value.start_with?("Authority safety:") }
  deliverables << "Authority safety: #{contract.fetch(:safety)}"
  request = {
    issue: issue,
    card: "stp",
    expected_generation: current.fetch("generation"),
    expected_digest: current.fetch("digest"),
    actor: "codex:5821-wp04-child-authority-repair",
    reason: "Align the executable child contract with the reviewed distributed authority architecture.",
    operation: {
      operation: "replace_planning_collection",
      field: "deliverables",
      values: deliverables
    }
  }
  run_edit(write_request(issue, "stp-acceptance", request), "apply")

  current = index(issue)
  vpp = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues", issue.to_s, "cards/vpp.values.json")))
  lanes = vpp.dig("content", "values", "lanes")
  lanes.fetch(0)["proof_role"] = contract.fetch(:proof)
  request = {
    issue: issue,
    card: "vpp",
    expected_generation: current.fetch("generation"),
    expected_digest: current.fetch("digest"),
    actor: "codex:5821-wp04-child-authority-repair",
    reason: "Align the proving lane with the reviewed distributed authority architecture.",
    operation: {
      operation: "replace_validation_lanes",
      lanes: lanes
    }
  }
  run_edit(write_request(issue, "vpp-lanes", request), "apply")

  puts "repaired child authority contract ##{issue}"
end
