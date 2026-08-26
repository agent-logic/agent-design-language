#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
profiles = File.read(File.join(root, "adl/src/provider/profiles.rs"))
tests = File.read(File.join(root, "adl/src/provider/mod.rs"))
evidence = File.read(File.join(root, "docs/milestones/v0.92.1/evidence/provider/prov-a/README.md"))

required = [
  "PROFILE_STATE_SCHEMA",
  "adl.provider_profile_state.v1",
  "last_known_good_profile",
  "retain_last_valid_materialization",
  "validate_before_activation",
  "retained_profile_state",
  "provider_mod_profile_state_retains_previous_last_known_good"
]

missing = required.reject { |needle| profiles.include?(needle) || tests.include?(needle) || evidence.include?(needle) }
abort "missing PROV-A last-known-good markers: #{missing.join(", ")}" unless missing.empty?
puts "PROV-A last-known-good: pass"
