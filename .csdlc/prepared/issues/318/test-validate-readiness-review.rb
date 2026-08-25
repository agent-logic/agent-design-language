#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
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
    {"WP29_MILESTONE_ROOT" => milestone, "WP29_EVIDENCE_ROOT" => evidence},
    "ruby", VALIDATOR, "all"
  )
  raise "#{name}: unexpectedly passed" if status.success?
  raise "#{name}: expected #{expected.inspect}, got #{err.inspect}" unless err.include?(expected)
end

FileUtils.rm_rf(WORK)
begin
  run_case("title-variance", "TAIL-10 title variance") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    File.write(path, File.read(path).sub("title: Release ceremony", "title: Release and cleanup ceremony"))
  end
  run_case("missing-primary-result", "CORP-A must define exactly one primary_deliverable") do |milestone, _evidence|
    path = File.join(milestone, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
    File.write(path, File.read(path).sub(/^    primary_deliverable: One accepted critical-asset schedule.*\n/, ""))
  end
  run_case("creation-denominator", "creation-owned denominator mismatch") do |milestone, _evidence|
    path = File.join(milestone, "WP_ISSUE_WAVE_v0.92.1.yaml")
    File.write(path, File.read(path).sub("creation_owner: WP-01", "creation_owner: operator"))
  end
  run_case("issue-denominator", "canonical issue denominator mismatch") do |_milestone, evidence|
    path = File.join(evidence, "issue-universe.json")
    json = JSON.parse(File.read(path))
    json.fetch("issues").delete_at(1)
    File.write(path, JSON.pretty_generate(json) + "\n")
  end
ensure
  FileUtils.rm_rf(WORK)
end

puts JSON.generate(schema: "adl.v092.wp29.readiness-review-negatives.v1", status: "pass", cases: 4)
