#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
evidence = File.join(root, ".csdlc/evidence/5896")

dry = JSON.parse(File.read(File.join(evidence, "dry-run.json")))
applied = JSON.parse(File.read(File.join(evidence, "apply-report.json")))
second = JSON.parse(File.read(File.join(evidence, "second-run.json")))
doctors = JSON.parse(File.read(File.join(evidence, "doctor-results.json")))

raise "dry-run cohort mismatch" unless dry.fetch("results").length == 58 && !dry.fetch("applied")
raise "apply count mismatch" unless applied.fetch("changed") == 58
raise "second run was not idempotent" unless second.fetch("changed").zero?
raise "doctor denominator mismatch" unless doctors.length == 58
raise "corrupt migrated record" if doctors.any? { |report| report.fetch("status") == "corrupt" }

dispositions = applied.fetch("results")
                      .group_by { |result| result.fetch("disposition") }
                      .transform_values(&:length)
raise "open disposition mismatch" unless dispositions["reset_initialized"] == 56
raise "terminal disposition mismatch" unless dispositions["closed_out"] == 2

legacy = Dir.glob(File.join(root, ".csdlc/issues/*/index.json")).each_with_object([]) do |path, found|
  record = JSON.parse(File.read(path))
  if record.fetch("phase") == "bound" &&
     (record["branch"].nil? || record["worktree"].nil?)
    found << record.fetch("issue")
  end
end
raise "legacy topology records remain: #{legacy.join(',')}" unless legacy.empty?

wp24 = doctors.find { |report| report.fetch("issue") == 5844 }
raise "WP-24 doctor result missing" unless wp24
raise "WP-24 is not execution-ready: #{wp24.fetch('findings')}" unless wp24.fetch("status") == "pass"

puts JSON.generate(
  schema: "csdlc.bound_topology_migration_validation.v1",
  migrated: applied.fetch("changed"),
  second_run_changes: second.fetch("changed"),
  doctor_count: doctors.length,
  corrupt_count: doctors.count { |report| report.fetch("status") == "corrupt" },
  wp24_status: wp24.fetch("status")
)
