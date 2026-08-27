#!/usr/bin/env ruby
# frozen_string_literal: true

ROOT = File.expand_path("../../../..", __dir__)
MANIFEST = File.join(ROOT, "adl/Cargo.toml")
FILTERS = %w[
  provider_fault_classifier
  fault_classification_round_trips_with_snake_case_schema_values
  provider_fault_summary
  phase1_manifest_references_all_required_schema_surfaces
  schema_smoke_contains_manifest_title
]

FILTERS.each do |filter|
  cmd = ["cargo", "test", "--manifest-path", MANIFEST, filter, "--", "--test-threads=1"]
  puts cmd.join(" ")
  abort "RUST-01 fault/trace proof failed for #{filter}" unless system(*cmd, chdir: ROOT)
end

puts "RUST-01 fault/trace proof passed"
