#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5350 5361].freeze

def fail_gate(message)
  warn("dependency gate failed: #{message}")
  exit(1)
end

common_dir, status = Open3.capture2("git", "rev-parse", "--git-common-dir", chdir: ROOT.to_s)
fail_gate("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common) unless common.absolute?
primary_root = common.parent
doctor = primary_root.join(".adl/bin/csdlc-v2/csdlc-doctor")
fail_gate("stable typed csdlc-doctor is absent") unless doctor.executable?

head, head_status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
fail_gate("cannot resolve exact execution revision") unless head_status.success?
execution_revision = (ARGV[0] || head.strip).strip
fail_gate("execution revision is not an exact SHA") unless execution_revision.match?(/\A[0-9a-f]{40}\z/)
fail_gate("checkout moved from exact execution revision #{execution_revision}") unless head.strip == execution_revision

DEPENDENCIES.each do |issue|
  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  fail_gate("##{issue} typed index is absent") unless index_path.file?
  fail_gate("##{issue} retained closeout receipt is absent") unless receipt_path.file?

  index = JSON.parse(index_path.read)
  receipt = JSON.parse(receipt_path.read)
  doctor_out, doctor_err, doctor_status = Open3.capture3(
    doctor.to_s,
    "--repo",
    ROOT.to_s,
    "--issue",
    issue,
    chdir: ROOT.to_s
  )
  fail_gate("##{issue} typed doctor failed: #{doctor_err.strip}") unless doctor_status.success?
  doctor_report = JSON.parse(doctor_out)
  fail_gate("##{issue} typed doctor status is not pass") unless doctor_report["status"] == "pass"
  fail_gate("##{issue} typed doctor phase is not closed_out") unless doctor_report["phase"] == "closed_out"
  fail_gate("##{issue} typed doctor retains findings") unless doctor_report.fetch("findings", []).empty?
  fail_gate("##{issue} phase is #{index.fetch("phase", "missing")}, not closed_out") unless index["phase"] == "closed_out"
  fail_gate("##{issue} claim remains active") unless index["claim"].nil?

  terminal = receipt["terminal"] || receipt.dig("record", "terminal") || {}
  disposition = terminal["disposition"] || terminal["state"]
  fail_gate("##{issue} receipt is not merged") unless disposition == "merged"
  merge_sha = terminal["observed_sha"] || receipt["observed_sha"] || receipt.dig("record", "observed_sha")
  fail_gate("##{issue} receipt lacks an exact merge SHA") unless merge_sha&.match?(/\A[0-9a-f]{40}\z/)

  _out, ancestor = Open3.capture2("git", "merge-base", "--is-ancestor", merge_sha, execution_revision, chdir: ROOT.to_s)
  fail_gate("##{issue} merge #{merge_sha} is not ancestral to #{execution_revision}") unless ancestor.success?
end

puts(JSON.generate(status: "pass", dependencies: DEPENDENCIES, execution_revision: execution_revision))
