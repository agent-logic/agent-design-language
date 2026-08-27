#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
profiles = File.read(File.join(root, "adl/src/provider/profiles.rs"))
tests = File.read(File.join(root, "adl/src/provider/mod.rs"))

required = [
  "DETERMINISTIC_OLLAMA_INFERENCE_PROFILE",
  "materialization_policy",
  "PROFILE_MATERIALIZATION_SCHEMA",
  "provider_profile_materialization_projection",
  "provider_mod_profile_materialization_projection_is_stable_and_redacted",
  "deterministic_ollama_v1",
  "deterministic_seed",
  "provider_mod_profile_expansion_materializes_bounded_inference_defaults"
]

missing = required.reject { |needle| profiles.include?(needle) || tests.include?(needle) }
abort "missing PROV-A ollama materialization markers: #{missing.join(", ")}" unless missing.empty?
rust_tests = [
  "provider_mod_profile_expansion_materializes_bounded_inference_defaults",
  "provider_mod_profile_materialization_projection_is_stable_and_redacted"
]
Dir.chdir(root) do
  rust_tests.each do |test|
    ok = system("cargo", "test", "--manifest-path", "adl/Cargo.toml", "--lib", test)
    abort "PROV-A ollama-materialization Rust test failed: #{test}" unless ok
  end
end
puts "PROV-A ollama-materialization: pass"
