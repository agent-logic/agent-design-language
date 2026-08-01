#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCY = 5354
RECEIPT_REF = "csdlc-v2/closeout/5354.json"
WP14A_LEDGER = ".csdlc/evidence/5384/platform-acceptance-ledger.v1.json"
WP15_PACKET = ".csdlc/evidence/5354/convergence-proof.v1.json"
WP15_RECONCILIATION = "docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md"
WP15_REQUIRED_MERGES = %w[
  97427f324c87d97cb1b36c7804c50bf80c9389d8
  ab4e9e2217c152df47b1754b66b01febb4a59549
].freeze
HEX40 = /\A[0-9a-f]{40}\z/

def fail_gate(message)
  warn("#5351 WP-15 gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

def installed_binary(name)
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  binary = common.parent.join(".adl/bin/csdlc-v2", name)
  fail_gate("missing installed typed binary #{name}") unless binary.file? && binary.executable?
  binary.to_s
end

begin
  [WP14A_LEDGER, WP15_PACKET, WP15_RECONCILIATION].each do |path|
    fail_gate("missing required retained dependency evidence #{path}") unless ROOT.join(path).file?
  end
  head = git("rev-parse", "HEAD")
  WP15_REQUIRED_MERGES.each do |sha|
    _out, ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", sha, head)
    fail_gate("required WP-15 merge #{sha} is not ancestral to #{head}") unless ancestry.success?
  end

  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  receipt_path = common.join(RECEIPT_REF)
  fail_gate("missing retained terminal receipt #{RECEIPT_REF}") unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt.fetch("record") { fail_gate("receipt has no typed record") }
  current_path = ROOT.join(".csdlc/issues/#{DEPENDENCY}/index.json")
  fail_gate("missing current typed record for ##{DEPENDENCY}") unless current_path.file?
  current = JSON.parse(current_path.read)
  fail_gate("current typed record differs from retained receipt") unless current == record
  fail_gate("##{DEPENDENCY} is not typed closed_out") unless record["phase"] == "closed_out"
  fail_gate("##{DEPENDENCY} still has an active claim") unless record["claim"].nil?

  doctor_out, doctor_status = Open3.capture2e(
    installed_binary("csdlc-doctor"), "--repo", ROOT.to_s, "--issue", DEPENDENCY.to_s
  )
  fail_gate("typed doctor rejected ##{DEPENDENCY}: #{doctor_out.strip}") unless doctor_status.success?
  doctor = JSON.parse(doctor_out)
  unless doctor["status"] == "pass" && doctor["phase"] == "closed_out" && Array(doctor["findings"]).empty?
    fail_gate("typed doctor does not report clean closed_out truth")
  end

  terminal = record.fetch("terminal") { fail_gate("receipt has no terminal evidence") }
  unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
    fail_gate("##{DEPENDENCY} terminal disposition is not merged")
  end
  pull_request = terminal["pull_request"]
  fail_gate("##{DEPENDENCY} terminal record has no PR identity") unless pull_request.is_a?(Integer) && pull_request.positive?
  sha = terminal["observed_sha"]
  fail_gate("##{DEPENDENCY} merged SHA is invalid") unless sha&.match?(HEX40)

  _out, ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", sha, head)
  fail_gate("##{DEPENDENCY} merged SHA #{sha} is not ancestral to #{head}") unless ancestry.success?

  puts JSON.generate(
    status: "pass",
    issue: 5351,
    dependency: DEPENDENCY,
    dependency_sha: sha,
    wp15_required_merges: WP15_REQUIRED_MERGES,
    receipt_sha256: Digest::SHA256.file(receipt_path).hexdigest,
    revision: head
  )
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid retained receipt: #{e.message}")
end
