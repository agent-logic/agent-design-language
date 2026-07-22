#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5349 5499 5498 5500 5502].freeze

def fail_closed(message)
  warn(message)
  exit 2
end

common_dir, status = Open3.capture2("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common).cleanpath unless common.absolute?

def audit_receipt(common, issue)
  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  return { present: false } unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || receipt
  terminal = record["terminal"] || {}
  {
    present: true,
    phase: record["phase"],
    claim_active: !record["claim"].nil?,
    disposition: terminal["disposition"],
    observed_sha: terminal["observed_sha"] || record["observed_sha"] || record["merge_sha"]
  }
rescue JSON::ParserError => e
  { present: true, malformed: e.message }
end

def audit_projection(issue)
  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  return { present: false } unless index_path.file?

  index = JSON.parse(index_path.read)
  {
    present: true,
    phase: index["phase"],
    claim_active: !index["claim"].nil?
  }
rescue JSON::ParserError => e
  { present: true, malformed: e.message }
end

origin_main = "origin/main"
unless system("git", "-C", ROOT.to_s, "rev-parse", "--verify", origin_main, out: File::NULL, err: File::NULL)
  fail_closed("origin/main is unavailable; refresh live repository state before dependency admission")
end

log, log_status = Open3.capture2("git", "-C", ROOT.to_s, "log", "--format=%H%x00%B%x00END", "--max-count=500", origin_main)
fail_closed("cannot inspect origin/main dependency history") unless log_status.success?

results = []
blockers = []
DEPENDENCIES.each do |issue|
  marker = /(?:#|issue[ -])#{Regexp.escape(issue)}\b/i
  merged = log.split("\u0000END\n").find { |entry| entry.match?(marker) }
  unless merged
    blockers << { issue: issue.to_i, reason: "no_live_merged_commit_evidence_on_origin_main" }
    next
  end

  merge_sha = merged.split("\u0000", 2).first
  unless merge_sha.to_s.match?(/\A[0-9a-f]{40}\z/)
    blockers << { issue: issue.to_i, reason: "live_merge_sha_malformed" }
    next
  end
  ancestral = system("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, "HEAD",
                     out: File::NULL, err: File::NULL)
  unless ancestral
    blockers << {
      issue: issue.to_i,
      live_merge_sha: merge_sha,
      reason: "live_merge_sha_not_ancestral_to_5501_head"
    }
    next
  end

  results << {
    issue: issue.to_i,
    live_merge_sha: merge_sha,
    ancestral_to_head: true,
    receipt_audit: audit_receipt(common, issue),
    projection_audit: audit_projection(issue)
  }
end

status = blockers.empty? ? "ready" : "blocked"
puts JSON.pretty_generate(
  status: status,
  dependency_rule: "live_merge_plus_ancestry",
  audit_only: ["typed_closeout", "retained_receipt", "claim_release_projection"],
  blockers: blockers,
  dependencies: results
)
exit 2 unless blockers.empty?
