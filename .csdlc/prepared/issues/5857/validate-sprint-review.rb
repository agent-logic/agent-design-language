#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
packet = File.join(root, ".csdlc/evidence/5857/sprint-review.json")
abort("sprint_review_missing: #{packet.delete_prefix(root + "/")}") unless File.file?(packet)

document = JSON.parse(File.read(packet))
abort("sprint_review_schema_invalid") unless document["schema"] == "adl.sprint_review.v1"
children = document["children"]
expected = [5825, 5826, 5827, 5828, 5829, 5830, 5831, 5833, 5834]
abort("sprint_review_children_mismatch") unless children.is_a?(Array) && children.map { |entry| entry["issue"] } == expected

top_keys = %w[children findings generated_from_head non_claims repairs schema sprint umbrella]
abort("sprint_review_unknown_field") unless document.keys.sort == top_keys
abort("sprint_review_wrong_umbrella") unless document["umbrella"] == {
  "issue" => 5857,
  "issue_repository" => "danielbaustin/agent-design-language",
  "code_repository" => "agent-logic/agent-design-language",
  "state" => "open_pending_review"
}

children.each do |entry|
  required = %w[code_repository head_sha issue issue_repository issue_state merge_sha pr pr_state reviewed_revision]
  abort("child_shape_invalid:#{entry["issue"]}") unless entry.keys.sort == required
  issue = entry.fetch("issue")
  abort("child_repository_invalid:#{issue}") unless entry["issue_repository"] == "danielbaustin/agent-design-language" && entry["code_repository"] == "agent-logic/agent-design-language"
  abort("child_terminal_invalid:#{issue}") unless entry["issue_state"] == "closed" && entry["pr_state"] == "merged"
  abort("child_revision_invalid:#{issue}") unless entry["reviewed_revision"].match?(/\Agit-blake3:[0-9a-f]{40}:[0-9a-f]{64}\z/)
  index = JSON.parse(File.read(File.join(root, ".csdlc/issues/#{issue}/index.json")))
  abort("child_review_mismatch:#{issue}") unless index.dig("review", "completed") == true && index.dig("review", "findings") == [] && index.dig("review", "reviewed_revision") == entry["reviewed_revision"]
  [entry["head_sha"], entry["merge_sha"]].each { |sha| abort("child_sha_invalid:#{issue}") unless sha.match?(/\A[0-9a-f]{40}\z/) }
  system("git", "-C", root, "cat-file", "-e", "#{entry["head_sha"]}^{commit}", out: File::NULL, err: File::NULL) or abort("child_head_missing:#{issue}")
  system("git", "-C", root, "merge-base", "--is-ancestor", entry["merge_sha"], "HEAD", out: File::NULL, err: File::NULL) or abort("child_merge_not_ancestral:#{issue}")
end

repairs = document["repairs"]
abort("repair_roster_invalid") unless repairs.map { |entry| [entry["issue_repository"], entry["issue"], entry["pr"]] } == [["agent-logic/agent-design-language", 144, 147], ["agent-logic/agent-design-language", 209, 215]]
repairs.each do |entry|
  abort("repair_terminal_invalid:#{entry["issue"]}") unless entry["issue_state"] == "closed" && entry["pr_state"] == "merged"
  system("git", "-C", root, "merge-base", "--is-ancestor", entry["merge_sha"], "HEAD", out: File::NULL, err: File::NULL) or abort("repair_merge_not_ancestral:#{entry["issue"]}")
end

abort("sprint_review_findings_not_empty") unless document["findings"] == []
forbidden = /personhood|citizenship|consciousness|governance approval|public release/i
document.fetch("non_claims").each { |claim| abort("non_claim_invalid") unless claim.is_a?(String) && claim.match?(forbidden) }

wp16 = File.join(root, ".csdlc/prepared/issues/5834/validate-review-packet.rb")
stdout, stderr, status = Open3.capture3("ruby", wp16, "--packet", "docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md", "--manifest", "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json", "--schema", "docs/milestones/v0.92/review/first-birthday-review-packet.schema.json", chdir: root)
abort("wp16_packet_invalid:#{stderr.strip}") unless status.success?

puts JSON.generate({ schema: "adl.sprint_review_validation.v1", status: "passed", children: children.length, repairs: repairs.length, wp16: stdout.strip })
