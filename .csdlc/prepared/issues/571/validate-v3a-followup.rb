#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

ROOT = File.expand_path("../../../..", __dir__)

class ValidationFailure < StandardError; end

def read_json(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => e
  abort "invalid JSON in #{path}: #{e.message}"
end

def require_file(path)
  return path if File.file?(path)

  abort "missing required file: #{path}"
end

def assert(condition, message)
  raise ValidationFailure, message unless condition
end

def blank?(value)
  value.nil? || (value.respond_to?(:empty?) && value.empty?)
end

def validate_predecessor_coverage(path)
  doc = read_json(path)
  assert doc["schema"] == "adl.csdlc_v3.predecessor_coverage.v1",
         "predecessor coverage schema mismatch"

  denominator = doc.fetch("denominator", [])
  [161, 162, 163].each do |issue|
    assert denominator.include?(issue),
           "predecessor denominator missing issue ##{issue}"
  end

  seen_ids = Set.new
  seen_rows = Set.new
  doc.fetch("entries", []).each do |entry|
    issue = entry.fetch("issue")
    entry.fetch("requirements", []).each do |requirement|
      id = requirement["id"]
      source_acceptance = requirement["source_acceptance"]
      disposition = requirement["disposition"]
      assert !blank?(id), "requirement under issue ##{issue} is missing id"
      assert !blank?(source_acceptance), "#{id} is missing source_acceptance"
      assert !blank?(disposition), "#{id} is missing disposition"

      assert seen_ids.add?(id), "duplicate retained requirement id: #{id}"
      row_key = [issue, source_acceptance]
      assert seen_rows.add?(row_key),
             "duplicate retained requirement row: issue ##{issue} #{source_acceptance}"

      next unless disposition == "retained"

      assert !blank?(requirement["owner_issue"]),
             "#{id} is retained but has no owner_issue"
      assert !blank?(requirement["proof_lane"]),
             "#{id} is retained but has no proof_lane"
      assert !(blank?(requirement["owner_issue"]) && !blank?(requirement["maps_to"])),
             "#{id} maps only to broad document sections"
    end
  end
end

def validate_lifecycle(path)
  doc = read_json(path)
  assert doc["schema"] == "adl.csdlc_v3.proportional_lifecycle.v1",
         "proportional lifecycle schema mismatch"

  default_path = doc.fetch("default_path")
  required_gates = %w[bind publication finish cleanup]

  gate_refs = default_path["required_gates"] || default_path["gates"] || []
  gate_refs = gate_refs.keys if gate_refs.is_a?(Hash)
  gate_refs = gate_refs.map(&:to_s)

  required_gates.each do |gate|
    assert gate_refs.include?(gate),
           "default_path must explicitly include retained #{gate} gate"
  end

  retained = doc.fetch("surfaces", [])
                .select { |surface| surface["disposition"] == "retained" }
                .map { |surface| surface["id"] }
  required_gates.each do |gate|
    assert retained.include?(gate),
           "retained-gate matrix missing #{gate}"
  end
end

def validate_contract(path)
  text = File.read(path)
  [
    [/#162/, "CONTRACT.md must cite #162 construction-slice evidence"],
    [/#163/, "CONTRACT.md must cite #163 approval evidence"],
    [/Decision 11/i, "CONTRACT.md must bind the decision to Decision 11"],
    [/threshold/i, "CONTRACT.md must cite construction decision thresholds"],
    [/\b(promoted|promotion|discarded|discard)\b/i,
     "CONTRACT.md must state whether the measured construction slice was promoted or discarded"]
  ].each do |pattern, message|
    assert text.match?(pattern), message
  end
end

def validate_diff_hygiene(path)
  text = File.read(path)
  assert text.match?(/git["']?,\s*["']-C["']?,\s*root,\s*["']diff["']?,\s*["']--check["']?,\s*["']#\{diff_base\}\.\.\.#\{diff_head\}["']/m) ||
         text.match?(/git\s+diff\s+--check\s+\S+\.\.\.\S+/) ||
         text.match?(/diff_base.*diff_head.*diff.*--check.*\.\.\./im),
         "implementation validator must run git diff --check over an explicit base...head range"
end

def expect_failure(label)
  failed = false
  begin
    yield
  rescue SystemExit
    failed = true
  rescue StandardError
    failed = true
  end
  assert failed, "negative fixture unexpectedly passed: #{label}"
end

def negative_fixtures
  expect_failure("retained requirement without owner/proof") do
    requirement = {
      "id" => "fixture-ac-1",
      "source_acceptance" => "AC-1",
      "disposition" => "retained",
      "maps_to" => ["broad-section"]
    }
    assert !blank?(requirement["owner_issue"]),
           "#{requirement["id"]} is retained but has no owner_issue"
  end

  expect_failure("default lifecycle without retained gates") do
    gate_refs = %w[design validation implementation_review closeout]
    %w[bind publication finish cleanup].each do |gate|
      assert gate_refs.include?(gate),
             "default_path must explicitly include retained #{gate} gate"
    end
  end

  expect_failure("vacuous diff hygiene") do
    text = "system('git', 'diff', '--check')"
    assert text.match?(/git\s+diff\s+--check\s+\S+\.\.\.\S+/) ||
           text.match?(/git\s+diff\s+--check.*base.*head/im),
           "implementation validator must run git diff --check over an explicit base...head range"
  end
end

predecessor = require_file(File.join(ROOT, "docs/csdlc-v3/predecessor-coverage.json"))
lifecycle = require_file(File.join(ROOT, "docs/csdlc-v3/proportional-lifecycle.json"))
contract = require_file(File.join(ROOT, "docs/csdlc-v3/CONTRACT.md"))
implementation_validator =
  require_file(File.join(ROOT, ".csdlc/prepared/issues/500/validate-implementation.rb"))

negative_fixtures
validate_predecessor_coverage(predecessor)
validate_lifecycle(lifecycle)
validate_contract(contract)
validate_diff_hygiene(implementation_validator)

puts "V3-A corrective follow-up validation passed"
