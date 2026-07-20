#!/usr/bin/env ruby

require "digest"
require "json"
require_relative "feature_decisions_5594"

root = File.expand_path("../../../..", __dir__)
source = File.join(root, "docs/planning/ADL_FEATURE_LIST.md")
rows = []

File.readlines(source).each_with_index do |line, index|
  next unless line.start_with?("|")

  columns = line.split("|").map(&:strip)[1..-2]
  next unless columns && columns.length == 4
  next if columns[0] == "Feature band" || columns[0].match?(/^-+$/)

  rows << [index + 1, columns]
end

entries = rows.each_with_index.map do |(source_line, row), index|
  feature, status, evidence, next_target = row
  code = FeatureDecisions5594::BY_SOURCE_LINE.fetch(source_line)
  decision = FeatureDecisions5594::GROUPS.fetch(code)

  {
    "row" => index + 1,
    "source_line" => source_line,
    "feature" => feature,
    "canonical_status" => status,
    "canonical_evidence" => evidence,
    "canonical_next_target" => next_target,
    "classification" => decision.fetch(:classification),
    "owner_issues" => decision.fetch(:owner_issues),
    "cutover_disposition" => decision.fetch(:disposition),
    "decision_basis" => decision.fetch(:basis)
  }
end

digest = Digest::SHA256.hexdigest(rows.map { |_, row| row.join("\u001f") }.join("\n"))
puts JSON.pretty_generate({
  "schema" => "adl.v0918.feature_preservation_crosswalk.v1",
  "source" => "docs/planning/ADL_FEATURE_LIST.md",
  "source_row_count" => rows.length,
  "source_row_digest" => digest,
  "entries" => entries
})
