#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5344 5345].freeze

def fail_closed(message)
  warn(message)
  exit 2
end

common_dir, status = Open3.capture2("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common).cleanpath unless common.absolute?

def terminal_dependency(issue, common)
  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  fail_closed("##{issue} retained closeout receipt is absent") unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || receipt
  terminal = record["terminal"] || {}
  merge_sha = terminal["observed_sha"] || record["observed_sha"] || record["merge_sha"]

  fail_closed("##{issue} receipt is not closed_out") unless record["phase"] == "closed_out"
  fail_closed("##{issue} receipt retained an active claim") unless record["claim"].nil?
  fail_closed("##{issue} receipt is not merged") unless terminal["disposition"] == "merged"
  fail_closed("##{issue} receipt omits observed merge SHA") unless merge_sha.is_a?(String) && merge_sha.match?(/\A[0-9a-f]{40}\z/)

  ancestral = system(
    "git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, "HEAD",
    out: File::NULL, err: File::NULL
  )
  fail_closed("##{issue} merge SHA is not ancestral to #5343") unless ancestral

  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  fail_closed("##{issue} typed projection is absent") unless index_path.file?
  index = JSON.parse(index_path.read)
  fail_closed("##{issue} typed projection is not closed_out") unless index["phase"] == "closed_out"
  fail_closed("##{issue} claim remains active") unless index["claim"].nil?

  { issue: issue.to_i, receipt: receipt_path, merge_sha: merge_sha }
rescue JSON::ParserError => e
  fail_closed("##{issue} terminal evidence is malformed: #{e.message}")
end

dependencies = DEPENDENCIES.map { |issue| terminal_dependency(issue, common) }

handoff_candidates = [
  ROOT.join("docs/milestones/v0.91.8/evidence/wp12/cutover-handoff-5344.v1.json"),
  ROOT.join("docs/milestones/v0.91.8/evidence/wp12/soak-rollback-5344.v1.json")
]
handoff_path = handoff_candidates.find(&:file?)
fail_closed("#5344 accepted soak/rollback handoff is absent") unless handoff_path
handoff = JSON.parse(handoff_path.read)

required = %w[status reviewed_revision manifest_digest prior_selector_digest restored_selector_digest fresh_install_receipt rollback_window]
missing = required.reject { |key| handoff.key?(key) }
fail_closed("#5344 handoff omits #{missing.join(', ')}") unless missing.empty?
fail_closed("#5344 handoff is not accepted") unless handoff["status"] == "accepted"
fail_closed("#5344 selector restoration is not exact") unless handoff["prior_selector_digest"] == handoff["restored_selector_digest"]
fail_closed("#5344 handoff contains unresolved rows") unless Array(handoff["unresolved_rows"]).empty?

puts JSON.pretty_generate(
  status: "pass",
  dependencies: dependencies.map do |dependency|
    {
      issue: dependency.fetch(:issue),
      receipt: dependency.fetch(:receipt).relative_path_from(ROOT).to_s,
      merge_sha: dependency.fetch(:merge_sha)
    }
  end,
  handoff: handoff_path.relative_path_from(ROOT).to_s
)
