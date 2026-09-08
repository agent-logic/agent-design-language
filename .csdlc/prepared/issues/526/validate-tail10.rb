#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "digest"
ROOT = File.expand_path("../../../..", __dir__)
NOTES = File.join(ROOT, "docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md")
RECEIPT = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-10/ceremony-receipt.json")
REVIEW = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-09/planning-review.json")

def receipt_errors(receipt, notes_digest)
  errors = []
  required = %w[candidate_sha tag release_id release_url authorized_by authorized_at ceremony_argv tag_target_sha release_target_sha notes_sha256 readback_at]
  errors << "receipt fields missing" unless required.all? { |key| !receipt[key].to_s.strip.empty? }
  %w[candidate_sha tag_target_sha release_target_sha].each { |key| errors << "invalid #{key}" unless receipt[key].to_s.match?(/\A[0-9a-f]{40}\z/) }
  errors << "tag does not target candidate" unless receipt["tag_target_sha"] == receipt["candidate_sha"]
  errors << "release does not target candidate" unless receipt["release_target_sha"] == receipt["candidate_sha"]
  errors << "notes digest mismatch" unless receipt["notes_sha256"] == notes_digest
  errors << "check-only ceremony preflight absent" unless receipt["preflight_argv"].to_s.include?("adl/tools/release_ceremony.sh") && receipt["preflight_status"] == "passed"
  errors << "ceremony tests absent" unless receipt["ceremony_test_status"] == "passed"
  merges = receipt.fetch("tail_merges", [])
  errors << "tail merge census incomplete" unless merges.map { |row| row["issue"] } == (516..525).to_a
  errors << "tail merge proof incomplete" if merges.any? { |row| row["reviewed"] != true || row["green"] != true || row["ancestral"] != true || !row["merge_sha"].to_s.match?(/\A[0-9a-f]{40}\z/) }
  errors
end

if ARGV == ["--negative"]
  digest = "b" * 64
  base = {"candidate_sha" => "a" * 40, "tag" => "v0.92.1", "release_id" => "1", "release_url" => "https://example.invalid/1",
          "authorized_by" => "operator", "authorized_at" => "2026-01-01T00:00:00Z", "ceremony_argv" => "release ceremony",
          "tag_target_sha" => "a" * 40, "release_target_sha" => "a" * 40, "notes_sha256" => digest,
          "readback_at" => "2026-01-01T00:01:00Z", "preflight_argv" => "bash adl/tools/release_ceremony.sh --version v0.92.1",
          "preflight_status" => "passed", "ceremony_test_status" => "passed",
          "tail_merges" => (516..525).map { |n| {"issue" => n, "merge_sha" => "c" * 40, "reviewed" => true, "green" => true, "ancestral" => true} }}
  mutations = [base.merge("authorized_by" => ""), base.merge("tag_target_sha" => "d" * 40), base.merge("notes_sha256" => "e" * 64),
               base.merge("preflight_status" => "skipped"), base.merge("tail_merges" => []),
               base.merge("tail_merges" => base["tail_merges"].map(&:dup).tap { |rows| rows.first["ancestral"] = false })]
  abort "negative mutation escaped" unless mutations.all? { |row| !receipt_errors(row, digest).empty? }
  puts "issue 526 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

abort "missing release notes" unless File.file?(NOTES) && !File.zero?(NOTES)
abort "missing current #525 review" unless File.file?(REVIEW) && !File.zero?(REVIEW)
review = JSON.parse(File.read(REVIEW))
abort "#525 has unresolved release blockers" unless review["outcome"] == "pass" && review["unresolved_release_blockers"] == 0
abort "missing ceremony receipt" unless File.file?(RECEIPT) && !File.zero?(RECEIPT)
receipt = JSON.parse(File.read(RECEIPT))
errors = receipt_errors(receipt, Digest::SHA256.file(NOTES).hexdigest)
abort errors.join("\n") unless errors.empty?
receipt.fetch("tail_merges").each do |row|
  errors << "merge #{row['issue']} is not ancestral" unless system("git", "merge-base", "--is-ancestor", row["merge_sha"], receipt["candidate_sha"], out: File::NULL, err: File::NULL)
end
abort errors.join("\n") unless errors.empty?
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 526 ceremony receipt semantics passed"
