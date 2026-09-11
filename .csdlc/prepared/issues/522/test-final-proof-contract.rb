#!/usr/bin/env ruby
require_relative "run-candidate-validation"

samples = {
  "negative_cases_plus_success" => ["PASS: handoff; 7 negative cases rejected\n", 8],
  "fraction" => ["PASS: 8/8 malformed cases rejected\n", 8],
  "json_findings" => ["{\"findings\":14,\"result\":\"pass\"}\n", 14],
  "json_validated" => ["{\"validated\":11,\"result\":\"pass\"}\n", 11],
  "cargo_tests" => ["test result: ok. 22 passed; 0 failed\ntest result: ok. 11 passed; 0 failed\n", 33],
  "shell_assertions" => ["PASS first\nsecond passed\n", 2]
}
samples.each do |parser, (output, expected)|
  actual = derive_validation_denominator(output, parser)
  abort("#{parser} denominator #{actual} != #{expected}") unless actual == expected
end

begin
  derive_validation_denominator("success without a count", "cargo_tests")
  abort("zero denominator was accepted")
rescue SystemExit
end

probe = execute_validation_command(["ruby", "-e", "puts '{\"validated\":3}'"], "json_validated")
abort("valid replay rejected") unless replay_validation_command?(probe)
forged_denominator = probe.merge("denominator" => 9999)
abort("forged denominator accepted") if replay_validation_command?(forged_denominator)
forged_output = probe.merge("semantic_output_sha256" => "0" * 64)
abort("forged output digest accepted") if replay_validation_command?(forged_output)

required_856_commands = [
  ["ruby", ".csdlc/prepared/issues/522/validate-release-lock-matrix.rb"],
  ["cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets"],
  ["bash", "adl/tools/test_release_ceremony.sh"],
  ["bash", "adl/tools/test_owner_binary_install.sh"],
  ["cargo", "test", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "card_identity"],
  ["cargo", "test", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "gate9"]
]
missing_commands = required_856_commands.reject { |argv| COMMANDS.any? { |actual, _| actual == argv } }
abort("#856 validation commands omitted: #{missing_commands.inspect}") unless missing_commands.empty?

validator = File.read(File.join(__dir__, "validate-remediation-ledger.rb"))
required_guards = [
  "reviewed_subject_digest", "executed_candidate", "fixed_with_deferred_proof",
  "operator_authorization", "returned finding projection omits or invents",
  "declared post-dependency authority", "executed validation receipt is not reproducible"
]
missing = required_guards.reject { |guard| validator.include?(guard) }
abort("validator omits repaired proof branches: #{missing.join(', ')}") unless missing.empty?

puts({status: "passed", denominator_parsers: samples.length, replay_tamper_cases: 2, required_856_commands: required_856_commands.length, guarded_repair_branches: required_guards.length}.to_json)
