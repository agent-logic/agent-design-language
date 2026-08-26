#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
profiles = File.read(File.join(root, "adl/src/provider/profiles.rs"))
tests = File.read(File.join(root, "adl/src/provider/mod.rs"))
doc = File.read(File.join(root, "docs/provider/inference-profiles.md"))

required = [
  "ProviderInferenceProfilePreset",
  "provider_model_id",
  "temperature",
  "top_p",
  "max_output_tokens",
  "timeout_secs",
  "provider_mod_profile_expansion_materializes_bounded_inference_defaults"
]

missing = required.reject { |needle| profiles.include?(needle) || tests.include?(needle) || doc.include?(needle) }
abort "missing PROV-A schema proof markers: #{missing.join(", ")}" unless missing.empty?
puts "PROV-A profile-schema: pass"
