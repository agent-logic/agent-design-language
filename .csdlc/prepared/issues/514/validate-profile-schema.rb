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
rust_tests = [
  "provider_mod_profile_expansion_materializes_bounded_inference_defaults",
  "provider_mod_profile_expansion_rejects_malformed_inference_values",
  "provider_mod_profile_expansion_rejects_provider_model_id_conflicts"
]
Dir.chdir(root) do
  rust_tests.each do |test|
    ok = system("cargo", "test", "--manifest-path", "adl/Cargo.toml", "--lib", test)
    abort "PROV-A profile-schema Rust test failed: #{test}" unless ok
  end
end
puts "PROV-A profile-schema: pass"
