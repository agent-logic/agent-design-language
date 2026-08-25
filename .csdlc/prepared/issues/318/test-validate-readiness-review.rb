#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"
require_relative "validate-readiness-review"

ROOT = File.expand_path("../../../..", __dir__) unless defined?(ROOT)
SOURCE_MILESTONE = File.join(ROOT, "docs/milestones/v0.92.1")
SOURCE_EVIDENCE = File.join(ROOT, ".csdlc/evidence/318")
WORK = File.join(__dir__, ".negative-work")
VALIDATOR = File.join(__dir__, "validate-readiness-review.rb")

def run_case(name, expected)
  milestone = File.join(WORK, name, "milestone")
  evidence = File.join(WORK, name, "evidence")
  FileUtils.mkdir_p(File.dirname(milestone))
  FileUtils.cp_r(SOURCE_MILESTONE, milestone)
  FileUtils.cp_r(SOURCE_EVIDENCE, evidence)
  yield milestone, evidence
  _out, err, status = Open3.capture3(
    {"WP29_MILESTONE_ROOT" => milestone, "WP29_EVIDENCE_ROOT" => evidence, "WP29_SKIP_LIVE_GITHUB" => "1"},
    "ruby", VALIDATOR, "all"
  )
  raise "#{name}: unexpectedly passed" if status.success?
  raise "#{name}: expected #{expected.inspect}, got #{err.inspect}" unless err.include?(expected)
end

FileUtils.rm_rf(WORK)
begin
  run_case("title-variance", "README.md canonical tail summary mismatch") do |milestone, _evidence|
    path = File.join(milestone, "README.md")
    File.write(path, File.read(path).sub("TAIL-06 Review findings remediation;", "TAIL-06 Review findings remediation and preflight;"))
  end
  run_case("missing-primary-result", "CORP-A must define exactly one primary_deliverable") do |milestone, _evidence|
    path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
    File.write(path, File.read(path).sub(/^    primary_deliverable: One accepted critical-asset schedule.*\n/, ""))
  end
  run_case("creation-denominator", "catalog creation-title denominator mismatch") do |milestone, _evidence|
    path = File.join(milestone, "PLANNED_ISSUE_CATALOG_v0.92.1.md")
    row = File.read(path).lines.find { |line| line.start_with?("| CORP-A |") }
    File.write(path, File.read(path).sub(row, row + row))
  end
  run_case("unit-contract-denominator", "unit-contract denominator mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
    File.write(path, File.read(path).sub(/^  AWS-A: \{primary_result:.*\n/, ""))
  end
  run_case("supporting-work-closeable", "AWS-A permits supporting work to close independently") do |milestone, _evidence|
    path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
    File.write(path, File.read(path).sub(/(AWS-A: \{primary_result: [^,]+, supporting_work_closeable:) false/, "\\1 true"))
  end
  run_case("issue-denominator", "canonical issue denominator mismatch") do |_milestone, evidence|
    path = File.join(evidence, "issue-universe.json")
    json = JSON.parse(File.read(path))
    json.fetch("issues").delete_at(1)
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
  run_case("tail-order", "release-tail order mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    text = File.read(path)
    first = text.index("  - id: TAIL-01")
    second = text.index("  - id: TAIL-02")
    third = text.index("  - id: TAIL-03")
    one = text[first...second]
    two = text[second...third]
    File.write(path, text[0...first] + two + one + text[third..])
  end
  run_case("tail-dependency", "TAIL-10 dependency mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    File.write(path, File.read(path).sub(/(id: TAIL-10.*?depends_on:) \[TAIL-09\]/m, "\\1 [TAIL-08]"))
  end
  run_case("v093-activation", "v0.93 activation claim changed") do |_milestone, evidence|
    path = File.join(evidence, "readiness-review.json")
    json = JSON.parse(File.read(path))
    json.fetch("v0_93")["activated"] = true
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
  run_case("planning-source-addendum", "planning source addendum denominator mismatch") do |_milestone, evidence|
    path = File.join(evidence, "planning-source-addendum.json")
    json = JSON.parse(File.read(path))
    json.fetch("sources").delete_at(0)
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
  run_case("wp01-creation-denominator", "WP-01 creation denominator mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
    File.write(path, File.read(path).sub("creation_denominator: [CORP-A, CORP-B,", "creation_denominator: [CORP-A,"))
  end
  run_case("aggregate-dependency", "INT-01 depends on non-issue aggregate CORP-01") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    File.write(path, File.read(path).sub("depends_on: [CORP-D, AWS-G, GCP-E,", "depends_on: [CORP-01, AWS-G, GCP-E,"))
  end
  run_case("open-pr-state", "issue 318 invalid merge") do |_milestone, evidence|
    path = File.join(evidence, "issue-universe.json")
    json = JSON.parse(File.read(path))
    json.fetch("issues").find { |row| row["issue"] == 318 }["pr_state"] = "MERGED"
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
  run_case("quality-lane-denominator", "quality gate missing development lane Cross-cloud Runtime Terraform") do |milestone, _evidence|
    path = File.join(milestone, "QUALITY_GATE_v0.92.1.md")
    File.write(path, File.read(path).sub("- Cross-cloud Runtime Terraform conversion\n", ""))
  end
  run_case("rust-recommendation-denominator", "Rust source recommendation denominator mismatch") do |_milestone, evidence|
    path = File.join(evidence, "planning-source-addendum.json")
    json = JSON.parse(File.read(path))
    rust = json.fetch("sources").find { |row| row["source_id"] == "TBD-RUST-SIMPLIFICATION" }
    rust.fetch("excluded_recommendations").pop
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
  run_case("observatory-prerequisite-denominator", "#84 prerequisite denominator mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    File.write(path, File.read(path).sub("depends_on: [251, 122, 340, 256]", "depends_on: [251, 122]"))
  end
  run_case("open-pr-head-policy", "issue 318 invalid open-head policy") do |_milestone, evidence|
    path = File.join(evidence, "issue-universe.json")
    json = JSON.parse(File.read(path))
    json.fetch("issues").find { |row| row["issue"] == 318 }["head_policy"] = "tracked_self_reference"
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
ensure
  FileUtils.rm_rf(WORK)
end

local_head, = Open3.capture2("git", "-C", ROOT, "rev-parse", "HEAD")
local_head = local_head.strip
raise "post-push OPEN equality rejected" unless open_pr_head_valid?(ROOT, local_head, local_head, "0" * 40)
raise "pre-push published ancestry rejected" unless open_pr_head_valid?(ROOT, "769703eb145a6ce63ca4e49c04e393f05f5cc068", local_head, "769703eb145a6ce63ca4e49c04e393f05f5cc068")
raise "divergent OPEN head accepted" if open_pr_head_valid?(ROOT, "0" * 40, local_head, "1" * 40)

puts JSON.generate(schema: "adl.v092.wp29.readiness-review-negatives.v1", status: "pass", cases: 17, open_pr_head_cases: 3)
