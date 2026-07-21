#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCY = 5356
RECEIPT_REF = "csdlc-v2/closeout/5356.json"
HEX40 = /\A[0-9a-f]{40}\z/

def fail_gate(message)
  warn("#5357 WP-18 gate: #{message}")
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

  head = git("rev-parse", "HEAD")
  _out, ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", sha, head)
  fail_gate("##{DEPENDENCY} merged SHA #{sha} is not ancestral to #{head}") unless ancestry.success?

  puts JSON.generate(
    status: "pass",
    issue: 5357,
    dependency: DEPENDENCY,
    dependency_sha: sha,
    receipt_sha256: Digest::SHA256.file(receipt_path).hexdigest,
    revision: head
  )
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid retained receipt: #{e.message}")
end
