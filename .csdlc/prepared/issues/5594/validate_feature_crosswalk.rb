#!/usr/bin/env ruby

require "digest"
require "json"

root = File.expand_path("../../../..", __dir__)
path = File.join(root, "docs/planning/ADL_FEATURE_LIST.md")
rows = []

File.readlines(path).each do |line|
  next unless line.start_with?("|")

  columns = line.split("|").map(&:strip)[1..-2]
  next unless columns && columns.length == 4
  next if columns[0] == "Feature" || columns[0].match?(/^-+$/)

  rows << columns
end

expected_count = 123
expected_digest = "5ecc0649f731c7b6afc71e33441924266df540a0997e2aa7b7f889db0005df65"
digest = Digest::SHA256.hexdigest(rows.map { |row| row.join("\u001f") }.join("\n"))

abort("feature-row count changed: #{rows.length}") unless rows.length == expected_count
abort("feature-row digest changed: #{digest}") unless digest == expected_digest
abort("feature row has an empty field") if rows.any? { |row| row.any?(&:empty?) }

names = rows.map(&:first)
abort("duplicate feature names") unless names.uniq.length == names.length

rules = [
  ["secure_access_observatory", 5590, /(access|remote|communication|a2a|acip|transport|observatory|telemetry|guardian)/i],
  ["reasoning_adaptive_cognition", 5592, /(reason|loop|adapt|learning|affect|cognitive|curiosity|constructability|godel|theory of mind|skill|guild|economic)/i],
  ["governed_operations", 5589, /(govern|freedom gate|delegation|agent lifecycle|provider|scheduler|tool|identity|memory|chronosense|checkpoint|lifelog|resilien|shepherd|private state)/i],
  ["kernel_continuity_ingress", 5591, /(runtime|execution|replay|continuity|backpressure|lifecycle|bounded concurrency)/i],
  ["csdlc_external_owner", 5358, /(c-sdlc|review|issue|pull request|prompt card|workflow control)/i]
]
fallback = ["retained_or_external", [5336, 5347]]
counts = Hash.new(0)
artifact_path = File.join(root, "docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json")
artifact = JSON.parse(File.read(artifact_path))
abort("wrong feature-crosswalk schema") unless artifact["schema"] == "adl.v0918.feature_preservation_crosswalk.v1"
abort("feature-crosswalk source count mismatch") unless artifact["source_row_count"] == rows.length
abort("feature-crosswalk source digest mismatch") unless artifact["source_row_digest"] == digest
entries = artifact.fetch("entries")
abort("feature-crosswalk entry count mismatch") unless entries.length == rows.length
allowed_dispositions = [
  "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
  "external_owner_acceptance_required",
  "deferred_to_canonical_next_target",
  "retained_existing_evidence_pending_deletion_eligibility"
]

rows.each_with_index do |row, index|
  haystack = row.join(" ")
  classification = rules.find { |_, _, pattern| haystack.match?(pattern) }
  name, owner = classification ? classification[0, 2] : fallback
  abort("feature row has no owner: #{row.first}") if owner.nil? || Array(owner).empty?
  entry = entries.fetch(index)
  abort("feature-crosswalk row order mismatch: #{row.first}") unless entry["row"] == index + 1 && entry["feature"] == row[0]
  abort("feature-crosswalk canonical fields mismatch: #{row.first}") unless entry.values_at("canonical_status", "canonical_evidence", "canonical_next_target") == row[1, 3]
  abort("feature-crosswalk class mismatch: #{row.first}") unless entry["classification"] == name
  abort("feature-crosswalk owner mismatch: #{row.first}") unless entry["owner_issues"] == Array(owner)
  abort("feature-crosswalk disposition missing: #{row.first}") unless allowed_dispositions.include?(entry["cutover_disposition"])
  counts[name] += 1
end

abort("not every feature row was classified") unless counts.values.sum == rows.length
puts "feature crosswalk ok rows=#{rows.length} digest=#{digest} classes=#{counts.sort.to_h}"
