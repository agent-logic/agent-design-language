#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"

ROOT = File.expand_path("../../..", __dir__)
RECONCILIATION_ROOT =
  "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation"
OUTPUT_ROOT =
  "docs/milestones/v0.92.1/evidence/release/tail-06/issue-764"

SOURCE_CANDIDATE_REVIEW_SHA =
  "c24f8fa65ce445b03ce6cd69007307291d78b60c"
HISTORICAL_CANDIDATE_SHA =
  "bf159eb416950dfa3399933829726a7b7e71f897"

RETAINED_MAPPING_FILES = [
  "retained-corporate-runtime.json",
  "retained-v3.json"
].freeze

LATER_STAGE_MAPPING_FILES = [
  "release-stage-mapping.json"
].freeze

REMEDIATION_CLASSES = [
  "non_proving",
  "source_supported_not_execution_proof",
  "current_gate_obligation"
].freeze

PRESERVED_CLASSES = [
  "proven",
  "accepted_amendment",
  "not_applicable_at_quality_gate"
].freeze

DEFAULT_REMEDIATION_COUNTS = {
  "non_proving" => 143,
  "source_supported_not_execution_proof" => 51,
  "current_gate_obligation" => 4
}.freeze

DEFAULT_PRESERVED_COUNTS = {
  "proven" => 10,
  "accepted_amendment" => 8,
  "not_applicable_at_quality_gate" => 11
}.freeze

CHECK_ONLY = ARGV.include?("--check")

def expected_counts(defaults, env_prefix)
  defaults.transform_values.with_index do |value, index|
    key = defaults.keys[index]
    Integer(ENV.fetch("#{env_prefix}_#{key.upcase}", value))
  end
end

EXPECTED_REMEDIATION_COUNTS =
  expected_counts(DEFAULT_REMEDIATION_COUNTS, "ADL_764_EXPECT_REMEDIATION")
EXPECTED_PRESERVED_COUNTS =
  expected_counts(DEFAULT_PRESERVED_COUNTS, "ADL_764_EXPECT_PRESERVED")

def read_json(repo_relative)
  path = File.join(ROOT, repo_relative)
  JSON.parse(File.read(path))
end

def sha256(repo_relative)
  Digest::SHA256.file(File.join(ROOT, repo_relative)).hexdigest
end

def count_by(rows, key)
  rows.each_with_object(Hash.new(0)) { |row, counts| counts[row.fetch(key)] += 1 }
end

def stable_row(row)
  {
    "row_id" => row.fetch("row_id"),
    "criterion_id" => row.fetch("criterion_id", row.fetch("row_id")),
    "criterion_text_digest" => row.fetch("criterion_text_digest", nil),
    "mapping_file" => row.fetch("mapping_file"),
    "reconciliation_class" => row.fetch("reconciliation_class"),
    "proof_requirement" => proof_requirement_for(row),
    "successor_authority" => successor_authority_for(row),
    "owner" => owner_for(row),
    "disposition" => disposition_for(row)
  }
end

def proof_requirement_for(row)
  case row.fetch("reconciliation_class")
  when "non_proving"
    "candidate-bound behavioral proof or governed amendment/removal required"
  when "source_supported_not_execution_proof"
    "execution receipt required; source support alone is not release proof"
  when "current_gate_obligation"
    "current release gate obligation requires candidate-bound proof"
  when "proven"
    "preserved as reviewed proof disposition"
  when "accepted_amendment"
    "preserved as governed amendment disposition"
  when "not_applicable_at_quality_gate"
    "preserved as later-stage or not-applicable quality-gate disposition"
  else
    "not part of #764 retained proof-gap partition"
  end
end

def successor_authority_for(row)
  row["successor_issue"] ||
    row["owner_issue"] ||
    row["authority_refs"] ||
    row["mapping_file"]
end

def owner_for(row)
  row["owner_issue"] ||
    row["successor_issue"] ||
    owner_from_row_id(row.fetch("row_id"))
end

def owner_from_row_id(row_id)
  prefix = row_id.split(":").first
  case prefix
  when "CORP-A" then "#482"
  when "CORP-B" then "#483"
  when "CORP-C" then "#497"
  when "CORP-D" then "#498"
  when "V3-A" then "#500"
  when "V3-B" then "#501"
  when "V3-C" then "#502"
  when "V3-D" then "#503"
  when "V3-E" then "#504"
  when "V3-F" then "#505"
  when "DRT-A" then "#506"
  when "DRT-B" then "#507"
  when "DRT-C" then "#508"
  when "TAIL-01" then "#517"
  else prefix
  end
end

def disposition_for(row)
  case row.fetch("reconciliation_class")
  when *REMEDIATION_CLASSES
    "unproved"
  when *PRESERVED_CLASSES
    "preserved"
  else
    "outside_issue_764_partition"
  end
end

def bucket_key(row)
  row_id = row.fetch("row_id")
  prefix = row_id.split(":").first
  if row.fetch("mapping_file") == "release-stage-mapping.json"
    "TAIL-01 current quality gate"
  elsif prefix.start_with?("CORP")
    "Corporate/runtime retained proof"
  elsif prefix.start_with?("V3")
    "C-SDLC v3 retained proof"
  elsif prefix.start_with?("DRT")
    "Distributed Runtime retained proof"
  else
    "#{prefix} retained proof"
  end
end

def source_file_for_bucket(rows)
  rows.map { |row| row.fetch("mapping_file") }.uniq.sort
end

def child_title_for(bucket_name)
  case bucket_name
  when "Corporate/runtime retained proof"
    "[v0.92.1][TAIL-06.08a][quality] Close corporate Runtime retained proof gaps"
  when "C-SDLC v3 retained proof"
    "[v0.92.1][TAIL-06.08b][quality] Close C-SDLC v3 retained proof gaps"
  when "Distributed Runtime retained proof"
    "[v0.92.1][TAIL-06.08c][quality] Close distributed Runtime retained proof gaps"
  when "TAIL-01 current quality gate"
    "[v0.92.1][TAIL-06.08d][quality] Reprove current TAIL-01 quality-gate obligations"
  else
    "[v0.92.1][TAIL-06.08x][quality] Close retained proof gaps"
  end
end

def child_dependency_for(bucket_name)
  case bucket_name
  when "TAIL-01 current quality gate"
    ["#764 denominator packet", "all proof-bearing retained child buckets it consumes"]
  else
    ["#764 denominator packet"]
  end
end

census_path = File.join(RECONCILIATION_ROOT, "census.json")
census = read_json(census_path)
rows = census.fetch("rows")

eligible_rows = rows.select do |row|
  mapping_file = row.fetch("mapping_file")
  RETAINED_MAPPING_FILES.include?(mapping_file) ||
    LATER_STAGE_MAPPING_FILES.include?(mapping_file)
end

remediation_rows = eligible_rows.select do |row|
  REMEDIATION_CLASSES.include?(row.fetch("reconciliation_class"))
end

preserved_rows = eligible_rows.select do |row|
  PRESERVED_CLASSES.include?(row.fetch("reconciliation_class"))
end

remediation_counts = count_by(remediation_rows, "reconciliation_class")
preserved_counts = count_by(preserved_rows, "reconciliation_class")

expected_remediation_total = EXPECTED_REMEDIATION_COUNTS.values.sum
expected_preserved_total = EXPECTED_PRESERVED_COUNTS.values.sum

count_findings = []
EXPECTED_REMEDIATION_COUNTS.each do |classification, expected|
  actual = remediation_counts[classification]
  next if actual == expected

  count_findings << {
    "severity" => "P1",
    "classification" => classification,
    "expected" => expected,
    "actual" => actual,
    "message" => "remediation count drift"
  }
end

EXPECTED_PRESERVED_COUNTS.each do |classification, expected|
  actual = preserved_counts[classification]
  next if actual == expected

  count_findings << {
    "severity" => "P1",
    "classification" => classification,
    "expected" => expected,
    "actual" => actual,
    "message" => "preserved count drift"
  }
end

stable_remediation_rows = remediation_rows.map { |row| stable_row(row) }
stable_preserved_rows = preserved_rows.map { |row| stable_row(row) }

child_buckets = stable_remediation_rows.group_by { |row| bucket_key(row) }.map do |name, bucket_rows|
  {
    "bucket" => name,
    "candidate_title" => child_title_for(name),
    "row_count" => bucket_rows.length,
    "classes" => count_by(bucket_rows, "reconciliation_class"),
    "source_mapping_files" => source_file_for_bucket(bucket_rows),
    "dependencies" => child_dependency_for(name),
    "candidate_child_scope" =>
      case name
      when "Corporate/runtime retained proof"
        "candidate-bound proof or governed amendments for corporate/runtime retained criteria"
      when "C-SDLC v3 retained proof"
        "candidate-bound execution receipts for C-SDLC v3 retained criteria"
      when "Distributed Runtime retained proof"
        "candidate-bound Runtime/DRT proof for distributed retained criteria"
      when "TAIL-01 current quality gate"
        "current TAIL-01 quality-gate proof obligations"
      else
        "candidate-bound retained proof remediation"
      end,
    "non_goals" => [
      "do not relabel ownership, closure, accounting, or documentation presence as behavioral proof",
      "do not silently remove retained requirements",
      "do not absorb product fixes into the #764 umbrella"
    ]
  }
end.sort_by { |bucket| bucket.fetch("bucket") }

source_inputs = [
  census_path,
  File.join(RECONCILIATION_ROOT, "retained-corporate-runtime.json"),
  File.join(RECONCILIATION_ROOT, "retained-v3.json"),
  File.join(RECONCILIATION_ROOT, "release-stage-mapping.json")
]

report = {
  "schema" => "adl.v0921.tail06.issue764.retained_proof_gap_denominator.v1",
  "issue" => 764,
  "source_review_issue" => 520,
  "parent_issue" => 522,
  "source_candidate_review_sha" => SOURCE_CANDIDATE_REVIEW_SHA,
  "historical_candidate_sha" => HISTORICAL_CANDIDATE_SHA,
  "source_inputs" => source_inputs.map do |path|
    {
      "path" => path,
      "sha256" => sha256(path)
    }
  end,
  "partition" => {
    "remediation_total" => remediation_rows.length,
    "expected_remediation_total" => expected_remediation_total,
    "remediation_counts" => EXPECTED_REMEDIATION_COUNTS.keys.to_h do |key|
      [key, remediation_counts[key]]
    end,
    "preserved_total" => preserved_rows.length,
    "expected_preserved_total" => expected_preserved_total,
    "preserved_counts" => EXPECTED_PRESERVED_COUNTS.keys.to_h do |key|
      [key, preserved_counts[key]]
    end,
    "excluded_accounting_rows" => rows.length - eligible_rows.length,
    "excluded_accounting_classes" => count_by(rows - eligible_rows, "reconciliation_class")
  },
  "validation" => {
    "status" => count_findings.empty? ? "pass" : "fail",
    "findings" => count_findings
  },
  "remediation_rows" => stable_remediation_rows,
  "preserved_rows" => stable_preserved_rows,
  "child_buckets" => child_buckets,
  "release_decision" => {
    "status" => "blocked",
    "reason" => "The #764 denominator is now stable, but candidate-bound behavioral proof or governed disposition remains required for all remediation rows before release authorization."
  }
}

FileUtils.mkdir_p(File.join(ROOT, OUTPUT_ROOT))

unless CHECK_ONLY
  json_path = File.join(ROOT, OUTPUT_ROOT, "retained-proof-gap-denominator.json")
  File.write(json_path, JSON.pretty_generate(report))

  summary_path = File.join(ROOT, OUTPUT_ROOT, "retained-proof-gap-denominator.md")
  File.write(summary_path, <<~MD)
    # Issue #764 retained proof-gap denominator

    This packet materializes the #764 retained-predecessor denominator from the
    current tracked reconciliation census.

    - Source review issue: #520
    - Parent remediation issue: #522
    - Source review candidate: `#{SOURCE_CANDIDATE_REVIEW_SHA}`
    - Historical candidate preserved by the reconciliation census: `#{HISTORICAL_CANDIDATE_SHA}`
    - Validation status: `#{report.fetch("validation").fetch("status")}`

    ## Partition

    | Partition | Count |
    | --- | ---: |
    | Remediation rows | #{remediation_rows.length} |
    | Preserved rows | #{preserved_rows.length} |
    | Excluded non-#764 accounting rows | #{rows.length - eligible_rows.length} |

    ## Remediation counts

    | Class | Expected | Actual |
    | --- | ---: | ---: |
    #{EXPECTED_REMEDIATION_COUNTS.map { |classification, expected| "| `#{classification}` | #{expected} | #{remediation_counts[classification]} |" }.join("\n")}

    ## Preserved counts

    | Class | Expected | Actual |
    | --- | ---: | ---: |
    #{EXPECTED_PRESERVED_COUNTS.map { |classification, expected| "| `#{classification}` | #{expected} | #{preserved_counts[classification]} |" }.join("\n")}

    ## Child bucket candidates

    | Bucket | Rows | Classes |
    | --- | ---: | --- |
  #{child_buckets.map { |bucket| "| #{bucket.fetch("bucket")} | #{bucket.fetch("row_count")} | #{bucket.fetch("classes").map { |key, value| "`#{key}` #{value}" }.join(", ")} |" }.join("\n")}

    ## Proposed child issues

    #{child_buckets.map { |bucket| "- #{bucket.fetch("candidate_title")}: #{bucket.fetch("candidate_child_scope")} Dependencies: #{bucket.fetch("dependencies").join("; ")}." }.join("\n")}

    ## Boundary

    This packet is denominator and routing evidence only. It does not convert
    source support, issue ownership, issue closure, documentation presence, or
    lifecycle accounting into behavioral proof. Product or live-proof remediation
    must stay in criterion-owned child issues.
  MD
end

puts JSON.generate(
  "status" => report.fetch("validation").fetch("status"),
  "remediation_total" => remediation_rows.length,
  "preserved_total" => preserved_rows.length,
  "json" => File.join(OUTPUT_ROOT, "retained-proof-gap-denominator.json"),
  "markdown" => File.join(OUTPUT_ROOT, "retained-proof-gap-denominator.md")
)
