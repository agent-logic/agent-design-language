#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

RECEIPT = ENV.fetch("ISSUE818_SUPERSESSION", File.join(__dir__, "issue-818-candidate-current-supersession.json"))
SOURCE_PACKET = ".csdlc/evidence/818/retained-corporate-runtime/reconciliation.json"
SOURCE_PLAN = ".csdlc/prepared/issues/818/retained-corporate-runtime-resolution-plan.json"
APPROVAL_SENTENCE = "Merging this PR approves only the exact proposal objects and digests reviewed here; it does not claim the associated product behavior was implemented or removed."

def assert(condition, message)
  raise message unless condition
end

def command(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  raise "command failed: #{argv.join(' ')}: #{stderr}" unless status.success?
  stdout
end

def git(*argv)
  command("git", *argv).strip
end

def canonical(value)
  case value
  when Hash then value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array then value.map { |entry| canonical(entry) }
  else value
  end
end

receipt = JSON.parse(File.read(RECEIPT))
expected_keys = %w[approval_basis bucket_release_gate_unresolved closing_pr closing_pr_head disposition finding issue merge_commit merged_at merged_by non_claims parent_issue proposal_digest_set_sha256 review_candidate row_count rows schema source_artifacts source_candidate source_issue supersedes]
assert(receipt.keys.sort == expected_keys.sort, "receipt schema drift")
assert(receipt.fetch("schema") == "adl.v0921.issue833.issue818_candidate_current_supersession.v1", "schema mismatch")
assert(receipt.fetch("issue") == 833 && receipt.fetch("source_issue") == 818 && receipt.fetch("parent_issue") == 522, "issue identity mismatch")
assert(receipt.fetch("finding") == "D520-RET-001", "finding mismatch")
assert(receipt.fetch("closing_pr") == 832, "closing PR mismatch")
assert(receipt.fetch("row_count") == 17, "row count mismatch")
assert(receipt.fetch("disposition") == "approved_by_exact_operator_merge", "approval disposition mismatch")
assert(receipt.fetch("bucket_release_gate_unresolved") == 0, "bucket is not reconciled")
assert(receipt.fetch("supersedes").start_with?("Only the 17 nested pending_operator_review"), "supersession scope widened")
assert(receipt.fetch("non_claims").length == 3, "non-claim boundary drift")
assert(receipt.fetch("non_claims").last.include?("does not approve the milestone release or close issues #522 or #833"), "downstream authority widened")

candidate = receipt.fetch("review_candidate")
source_candidate = receipt.fetch("source_candidate")
pr_head = receipt.fetch("closing_pr_head")
merge_commit = receipt.fetch("merge_commit")
[candidate, source_candidate, pr_head, merge_commit].each do |sha|
  assert(sha.match?(/\A[0-9a-f]{40}\z/), "invalid SHA")
  assert(git("cat-file", "-t", sha) == "commit", "missing commit #{sha}")
end
[[source_candidate, pr_head], [pr_head, merge_commit], [merge_commit, candidate]].each do |ancestor, descendant|
  _out, _err, status = Open3.capture3("git", "merge-base", "--is-ancestor", ancestor, descendant)
  assert(status.success?, "ancestry failure #{ancestor} -> #{descendant}")
end
merge_parents = git("show", "-s", "--format=%P", merge_commit).split
assert(merge_parents.include?(pr_head), "merge commit does not contain exact PR head")

packet = JSON.parse(command("git", "show", "#{candidate}:#{SOURCE_PACKET}"))
plan = JSON.parse(command("git", "show", "#{candidate}:#{SOURCE_PLAN}"))
assert(packet.fetch("row_count") == 17 && packet.fetch("rows").length == 17, "source packet denominator mismatch")
assert(packet.fetch("operator_approval_pending") == 17 && packet.fetch("release_ready") == false, "historical proposal state was rewritten")
assert(plan.fetch("rows").length == 17, "source plan denominator mismatch")

receipt.fetch("source_artifacts").each do |artifact|
  path = artifact.fetch("path")
  head_blob = git("rev-parse", "#{pr_head}:#{path}")
  candidate_blob = git("rev-parse", "#{candidate}:#{path}")
  candidate_bytes = command("git", "show", "#{candidate}:#{path}")
  assert(head_blob == artifact.fetch("git_blob_at_pr_head"), "PR-head blob mismatch for #{path}")
  assert(candidate_blob == artifact.fetch("git_blob_at_review_candidate"), "candidate blob mismatch for #{path}")
  assert(head_blob == candidate_blob, "approved artifact changed after merge for #{path}")
  assert(Digest::SHA256.hexdigest(candidate_bytes) == artifact.fetch("sha256_at_review_candidate"), "candidate digest mismatch for #{path}")
end

source_rows = packet.fetch("rows").map { |row| [row.fetch("row_id"), row.fetch("resolution").fetch("proposal_digest")] }
receipt_rows = receipt.fetch("rows").map { |row| [row.fetch("row_id"), row.fetch("proposal_digest")] }
assert(receipt_rows.length == receipt_rows.uniq.length, "duplicate supersession row")
assert(receipt_rows == source_rows, "supersession rows differ from exact approved packet order or digest")
assert(plan.fetch("rows").map { |row| [row.fetch("row_id"), row.fetch("resolution").fetch("proposal_digest")] } == source_rows, "plan and receipt proposals differ")
plan.fetch("rows").each do |row|
  resolution = row.fetch("resolution")
  computed = Digest::SHA256.hexdigest(JSON.generate(canonical(resolution.reject { |key, _value| key == "proposal_digest" })))
  assert(resolution.fetch("proposal_digest") == computed, "proposal digest invalid for #{row.fetch('row_id')}")
end
aggregate = Digest::SHA256.hexdigest(source_rows.map(&:last).join("\0") + "\n")
assert(aggregate == receipt.fetch("proposal_digest_set_sha256"), "proposal digest set mismatch")

pr = JSON.parse(command("gh", "pr", "view", "832", "--repo", "agent-logic/agent-design-language", "--json", "state,headRefOid,mergeCommit,mergedAt,mergedBy,body"))
assert(pr.fetch("state") == "MERGED", "closing PR is not merged")
assert(pr.fetch("headRefOid") == pr_head, "live closing PR head mismatch")
assert(pr.fetch("mergeCommit").fetch("oid") == merge_commit, "live merge commit mismatch")
assert(pr.fetch("mergedBy").fetch("login") == receipt.fetch("merged_by"), "live merge actor mismatch")
assert(pr.fetch("mergedAt") == receipt.fetch("merged_at"), "live merge time mismatch")
assert(pr.fetch("body").include?(APPROVAL_SENTENCE), "closing PR lacks exact bounded approval authority")

issue = JSON.parse(command("gh", "issue", "view", "818", "--repo", "agent-logic/agent-design-language", "--json", "state,closedByPullRequestsReferences"))
assert(issue.fetch("state") == "CLOSED", "source issue is not closed")
assert(issue.fetch("closedByPullRequestsReferences").any? { |entry| entry.fetch("number") == 832 }, "issue was not closed by PR #832")

puts "PASS: #818 exact 17-row proposal set was approved by operator merge at PR #832, is unchanged at #{candidate}, and is narrowly superseded; #522/#833 remain open"
