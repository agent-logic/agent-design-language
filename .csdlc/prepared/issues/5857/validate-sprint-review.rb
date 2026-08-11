#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

class ValidationFailure < StandardError; end

ROOT = File.expand_path("../../../..", __dir__)
PACKET = File.join(ROOT, ".csdlc/evidence/5857/sprint-review.json")
MAPPINGS = File.join(ROOT, ".csdlc/evidence/5857/terminal-mappings.json")
CHILD_ISSUES = [5825, 5826, 5827, 5828, 5829, 5830, 5831, 5833, 5834].freeze

def reject!(code)
  raise ValidationFailure, code
end

def git_success?(*argv)
  system("git", "-C", ROOT, *argv, out: File::NULL, err: File::NULL)
end

def merge_tree_index(merge_sha, issue)
  stdout, = Open3.capture2("git", "-C", ROOT, "show", "#{merge_sha}:.csdlc/issues/#{issue}/index.json")
  JSON.parse(stdout)
rescue StandardError
  reject!("merge_tree_review_missing:#{issue}")
end

def validate_review!(entry, kind)
  issue = entry.fetch("issue")
  index = merge_tree_index(entry.fetch("merge_sha"), issue)
  review = index["review"]
  reject!("#{kind}_review_mismatch:#{issue}") unless review.is_a?(Hash) && review["completed"] == true && review["findings"] == [] && review["reviewed_revision"] == entry["reviewed_revision"]
end

def validate_document!(document, mappings, run_wp16: true)
  reject!("sprint_review_schema_invalid") unless document["schema"] == "adl.sprint_review.v1"
  reject!("sprint_review_unknown_field") unless document.keys.sort == %w[children findings generated_from_head non_claims repairs schema sprint umbrella]
  reject!("sprint_review_wrong_umbrella") unless document["umbrella"] == {"issue" => 5857, "issue_repository" => "danielbaustin/agent-design-language", "code_repository" => "agent-logic/agent-design-language", "state" => "open_pending_review"}

  children = document["children"]
  reject!("sprint_review_children_mismatch") unless children.is_a?(Array) && children.map { |entry| entry["issue"] } == CHILD_ISSUES
  expected_children = mappings.fetch("children")
  reject!("terminal_mapping_roster_invalid") unless expected_children.map { |entry| entry["issue"] } == CHILD_ISSUES
  children.each_with_index do |entry, index|
    issue = entry["issue"]
    required = %w[code_repository head_sha issue issue_repository issue_state merge_sha pr pr_state reviewed_revision]
    reject!("child_shape_invalid:#{issue}") unless entry.keys.sort == required
    reject!("child_terminal_mapping_mismatch:#{issue}") unless entry == expected_children[index]
    reject!("child_sha_invalid:#{issue}") unless [entry["head_sha"], entry["merge_sha"]].all? { |sha| sha.match?(/\A[0-9a-f]{40}\z/) }
    reject!("child_head_missing:#{issue}") unless git_success?("cat-file", "-e", "#{entry["head_sha"]}^{commit}")
    reject!("child_merge_not_ancestral:#{issue}") unless git_success?("merge-base", "--is-ancestor", entry["merge_sha"], "HEAD")
    validate_review!(entry, "child")
  end

  repairs = document["repairs"]
  expected_repairs = mappings.fetch("repairs")
  reject!("repair_roster_invalid") unless repairs.is_a?(Array) && repairs.map { |entry| [entry["issue_repository"], entry["issue"], entry["pr"]] } == [["agent-logic/agent-design-language", 144, 147], ["agent-logic/agent-design-language", 209, 215]]
  repairs.each_with_index do |entry, index|
    issue = entry["issue"]
    required = %w[code_repository head_sha issue issue_repository issue_state merge_sha pr pr_state purpose review_completed review_findings reviewed_revision]
    reject!("repair_shape_invalid:#{issue}") unless entry.keys.sort == required
    reject!("repair_terminal_mapping_mismatch:#{issue}") unless entry == expected_repairs[index]
    reject!("repair_review_claim_invalid:#{issue}") unless entry["review_completed"] == true && entry["review_findings"] == []
    reject!("repair_head_missing:#{issue}") unless git_success?("cat-file", "-e", "#{entry["head_sha"]}^{commit}")
    reject!("repair_merge_not_ancestral:#{issue}") unless git_success?("merge-base", "--is-ancestor", entry["merge_sha"], "HEAD")
    validate_review!(entry, "repair")
  end

  reject!("sprint_review_findings_not_empty") unless document["findings"] == []
  forbidden = /personhood|citizenship|consciousness|governance approval|public release/i
  document.fetch("non_claims").each { |claim| reject!("non_claim_invalid") unless claim.is_a?(String) && claim.match?(forbidden) }

  wp16_stdout = "not_run"
  if run_wp16
    wp16 = File.join(ROOT, ".csdlc/prepared/issues/5834/validate-review-packet.rb")
    wp16_stdout, stderr, status = Open3.capture3("ruby", wp16, "--packet", "docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md", "--manifest", "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json", "--schema", "docs/milestones/v0.92/review/first-birthday-review-packet.schema.json", chdir: ROOT)
    reject!("wp16_packet_invalid:#{stderr.strip}") unless status.success?
  end
  {children: children.length, repairs: repairs.length, wp16: wp16_stdout.strip}
end

def self_test!(document, mappings)
  cases = {
    "wrong-child-pr" => ->(copy) { copy["children"][0]["pr"] = 9999 },
    "mismatched-child-head-merge" => ->(copy) { copy["children"][0]["head_sha"] = copy["children"][1]["head_sha"] },
    "missing-repair-review" => ->(copy) { copy["repairs"][0].delete("reviewed_revision") },
    "failed-repair-review" => ->(copy) { copy["repairs"][1]["review_completed"] = false; copy["repairs"][1]["review_findings"] = ["unresolved"] }
  }
  cases.each do |name, mutate|
    copy = Marshal.load(Marshal.dump(document))
    mutate.call(copy)
    begin
      validate_document!(copy, mappings, run_wp16: false)
    rescue ValidationFailure
      next
    end
    reject!("negative_case_accepted:#{name}")
  end
  cases.keys
end

abort("sprint_review_missing") unless File.file?(PACKET) && File.file?(MAPPINGS)
document = JSON.parse(File.read(PACKET))
mappings = JSON.parse(File.read(MAPPINGS))
begin
  negative_cases = ARGV.include?("--self-test") ? self_test!(document, mappings) : []
  result = validate_document!(document, mappings)
  puts JSON.generate({schema: "adl.sprint_review_validation.v1", status: "passed", children: result[:children], repairs: result[:repairs], negative_cases: negative_cases, wp16: result[:wp16]})
rescue ValidationFailure => error
  abort(error.message)
end
