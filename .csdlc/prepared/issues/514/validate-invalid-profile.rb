#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
profiles = File.read(File.join(root, "adl/src/provider/profiles.rs"))
tests = File.read(File.join(root, "adl/src/provider/mod.rs"))

required = [
  "validate_bounded_f64",
  "validate_positive_u64",
  "config.temperature",
  "deterministic_seed must remain 0",
  "provider_mod_profile_expansion_rejects_non_deterministic_ollama_seed",
  "provider_mod_profile_expansion_rejects_malformed_inference_values",
  "provider_mod_profile_expansion_rejects_provider_model_id_conflicts"
]

missing = required.reject { |needle| profiles.include?(needle) || tests.include?(needle) }
abort "missing PROV-A invalid-profile markers: #{missing.join(", ")}" unless missing.empty?
puts "PROV-A invalid-profile: pass"
