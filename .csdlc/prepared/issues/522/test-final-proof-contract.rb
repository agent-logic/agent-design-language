#!/usr/bin/env ruby
require_relative "run-candidate-validation"

samples = {
  "negative_cases_plus_success" => ["PASS: handoff; 7 negative cases rejected\n", 8],
  "fraction" => ["PASS: 8/8 malformed cases rejected\n", 8],
  "json_findings" => ["{\"findings\":14,\"result\":\"pass\"}\n", 14],
  "cargo_tests" => ["test result: ok. 22 passed; 0 failed\ntest result: ok. 11 passed; 0 failed\n", 33],
  "shell_assertions" => ["PASS first\nsecond passed\n", 2]
}
samples.each do |parser, (output, expected)|
  actual = denominator(output, parser)
  abort("#{parser} denominator #{actual} != #{expected}") unless actual == expected
end

begin
  denominator("success without a count", "cargo_tests")
  abort("zero denominator was accepted")
rescue SystemExit
end

validator = File.read(File.join(__dir__, "validate-remediation-ledger.rb"))
required_guards = [
  "reviewed_subject_digest", "executed_candidate", "fixed_with_deferred_proof",
  "operator_authorization", "returned finding projection omits or invents",
  "declared post-dependency authority", "executed validation receipt is not reproducible"
]
missing = required_guards.reject { |guard| validator.include?(guard) }
abort("validator omits repaired proof branches: #{missing.join(', ')}") unless missing.empty?

puts({status: "passed", denominator_parsers: samples.length, guarded_repair_branches: required_guards.length}.to_json)
