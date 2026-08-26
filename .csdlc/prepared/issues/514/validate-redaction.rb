#!/usr/bin/env ruby
root = File.expand_path("../../../..", __dir__)
profiles = File.read(File.join(root, "adl/src/provider/profiles.rs"))
tests = File.read(File.join(root, "adl/src/provider/mod.rs"))
doc = File.read(File.join(root, "docs/provider/inference-profiles.md"))

required = [
  "PROFILE_REDACTION_SCHEMA",
  "adl.provider_profile_redacted_projection.v1",
  "redacted_provider_profile_projection",
  "is_private_config_key",
  "redacted_value_for_key",
  "provider_mod_redacted_profile_projection_excludes_private_payloads",
  "recovery_code"
]

missing = required.reject { |needle| profiles.include?(needle) || tests.include?(needle) || doc.include?(needle) }
abort "missing PROV-A redaction markers: #{missing.join(", ")}" unless missing.empty?
forbidden = ["OPENAI_API_KEY", "raw prompt"]
leaks = forbidden.select { |needle| doc.include?(needle) }
abort "redaction doc leaked private marker(s): #{leaks.join(", ")}" unless leaks.empty?
puts "PROV-A redaction: pass"
