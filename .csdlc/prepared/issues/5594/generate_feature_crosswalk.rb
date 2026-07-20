#!/usr/bin/env ruby

require "digest"
require "json"

root = File.expand_path("../../../..", __dir__)
source = File.join(root, "docs/planning/ADL_FEATURE_LIST.md")
rows = []

File.readlines(source).each do |line|
  next unless line.start_with?("|")

  columns = line.split("|").map(&:strip)[1..-2]
  next unless columns && columns.length == 4
  next if columns[0] == "Feature" || columns[0].match?(/^-+$/)

  rows << columns
end


rules = [
  ["secure_access_observatory", [5590], /(access|remote|communication|a2a|acip|transport|observatory|telemetry|guardian)/i],
  ["reasoning_adaptive_cognition", [5592], /(reason|loop|adapt|learning|affect|cognitive|curiosity|constructability|godel|theory of mind|skill|guild|economic)/i],
  ["governed_operations", [5589], /(govern|freedom gate|delegation|agent lifecycle|provider|scheduler|tool|identity|memory|chronosense|checkpoint|lifelog|resilien|shepherd|private state)/i],
  ["kernel_continuity_ingress", [5591], /(runtime|execution|replay|continuity|backpressure|lifecycle|bounded concurrency)/i],
  ["csdlc_external_owner", [5358], /(c-sdlc|review|issue|pull request|prompt card|workflow control)/i]
]

entries = rows.each_with_index.map do |row, index|
  feature, status, evidence, next_target = row
  matched = rules.find { |_, _, pattern| row.join(" ").match?(pattern) }
  classification, owner_issues = matched ? matched[0, 2] : ["retained_or_external", [5336, 5347]]
  disposition = case classification
  when "secure_access_observatory", "reasoning_adaptive_cognition", "governed_operations", "kernel_continuity_ingress"
    "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition"
  when "csdlc_external_owner"
    "external_owner_acceptance_required"
  when "retained_or_external"
    status.match?(/planned|mvp|post-mvp|required .*under development/i) ?
      "deferred_to_canonical_next_target" :
      "retained_existing_evidence_pending_deletion_eligibility"
  end

  {
    "row" => index + 1,
    "feature" => feature,
    "canonical_status" => status,
    "canonical_evidence" => evidence,
    "canonical_next_target" => next_target,
    "classification" => classification,
    "owner_issues" => owner_issues,
    "cutover_disposition" => disposition
  }
end

digest = Digest::SHA256.hexdigest(rows.map { |row| row.join("\u001f") }.join("\n"))
puts JSON.pretty_generate({
  "schema" => "adl.v0918.feature_preservation_crosswalk.v1",
  "source" => "docs/planning/ADL_FEATURE_LIST.md",
  "source_row_count" => rows.length,
  "source_row_digest" => digest,
  "entries" => entries
})
