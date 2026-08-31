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

def validate_owner_lane_registry(path)
  doc = read_json(path)
  assert doc["schema"] == "adl.csdlc_v3.owner_proof_lanes.v1",
         "owner proof lane registry schema mismatch"

  lanes_by_issue = {}
  doc.fetch("sources", []).each do |source|
    owner_issue = source.fetch("owner_issue")
    source_path = source["source_path"]
    source_revision = source["source_revision"]
    assert !blank?(source_path), "owner ##{owner_issue} lane source missing path"
    assert !blank?(source_revision), "owner ##{owner_issue} lane source missing revision"

    lanes = {}
    source.fetch("lanes", []).each do |lane|
      lane_name = lane["lane"]
      argv = lane["argv"]
      assert !blank?(lane_name), "owner ##{owner_issue} lane has blank name"
      assert argv.is_a?(Array) && argv.all? { |part| part.is_a?(String) && !part.empty? },
             "owner ##{owner_issue} lane #{lane_name} must declare executable argv"
      assert !lanes.key?(lane_name), "duplicate owner ##{owner_issue} lane #{lane_name}"
      lanes[lane_name] = argv
    end
    assert !lanes.empty?, "owner ##{owner_issue} has no declared proof lanes"
    assert !lanes_by_issue.key?(owner_issue), "duplicate owner issue lane registry ##{owner_issue}"
    lanes_by_issue[owner_issue] = lanes
  end
  lanes_by_issue
end

def validate_predecessor_coverage(path, lanes_by_issue)
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
      owner_issue = requirement["owner_issue"]
      proof_lane = requirement["proof_lane"]
      assert owner_issue.is_a?(Integer), "#{id} owner_issue must be an integer"
      assert lanes_by_issue.key?(owner_issue),
             "#{id} owner_issue ##{owner_issue} has no executable lane registry"
      assert lanes_by_issue.fetch(owner_issue).key?(proof_lane),
             "#{id} proof_lane #{proof_lane.inspect} is not declared by owner ##{owner_issue}"
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
    [/construction-decision\.json/,
     "CONTRACT.md must cite the machine-readable construction decision artifact"],
    [/\bnot promoted\b/i,
     "CONTRACT.md must state the measured construction slice is not promoted when evidence is missing"],
    [/exact-revision evidence is missing/i,
     "CONTRACT.md must record missing exact-revision construction evidence"]
  ].each do |pattern, message|
    assert text.match?(pattern), message
  end

  default_section = text.split("Default V3 path", 2).last || ""
  %w[bind publication finish cleanup].each do |gate|
    assert default_section.match?(/\b#{Regexp.escape(gate)}\b/i),
           "CONTRACT.md default path must mention retained #{gate} gate"
  end
end

def validate_construction_decision(path)
  doc = read_json(path)
  assert doc["schema"] == "adl.csdlc_v3.construction_decision.v1",
         "construction decision schema mismatch"
  assert doc["predecessor_issue"] == 162,
         "construction decision must bind to predecessor issue #162"
  assert doc["decision_11_binding_issue"] == 163,
         "construction decision must bind to #163 / Decision 11"
  assert doc["decision_owner_issue"] == 505,
         "construction decision must defer live authority disposition to #505"
  assert doc["expected_evidence_artifact"] == ".csdlc/evidence/162/proof.json",
         "construction decision must name the expected #162 evidence artifact"
  assert doc["expected_evidence_revision"].nil?,
         "missing #162 evidence must not invent an exact evidence revision"
  assert doc["slice_disposition"] == "not_promoted_missing_162_measurements",
         "construction slice disposition must fail closed while #162 measurements are missing"

  expected = {
    "stripped_release_binary_size" => [35, "MiB"],
    "direct_dependency_count" => [30, "dependencies"],
    "locked_transitive_package_count" => [300, "packages"],
    "clean_build_seconds" => [300, "seconds"],
    "warm_incremental_build_seconds" => [60, "seconds"],
    "startup_version_schema_completion_p95_ms" => [50, "milliseconds"],
    "local_issue_show_p95_ms" => [250, "milliseconds"],
    "local_doctor_p95_seconds" => [1, "seconds"],
    "deterministic_spike_test_suite_seconds" => [30, "seconds"],
    "authored_production_rust_lines" => [2500, "lines"]
  }

  seen = Set.new
  rows = doc.fetch("measurements", [])
  assert rows.length == expected.length, "construction decision must record ten #162 measurements"
  rows.each do |row|
    id = row["id"]
    assert expected.key?(id), "unexpected construction measurement #{id.inspect}"
    assert seen.add?(id), "duplicate construction measurement #{id}"
    threshold_value, threshold_unit = expected.fetch(id)
    assert row["operator"] == "lte", "#{id} must preserve less-than-or-equal threshold semantics"
    assert row["threshold_value"] == threshold_value, "#{id} threshold value mismatch"
    assert row["threshold_unit"] == threshold_unit, "#{id} threshold unit mismatch"
    assert row["observed_value"].nil?, "#{id} must not invent a measured value"
    assert row["evidence_artifact"] == ".csdlc/evidence/162/proof.json",
           "#{id} must name the #162 evidence artifact"
    assert row["evidence_revision"].nil?, "#{id} must not invent an evidence revision"
    assert row["evidence_status"] == "missing", "#{id} must classify evidence as missing"
    assert row["disposition"] == "blocked_missing_measurement",
           "#{id} must fail closed while measurement evidence is missing"
  end
  assert seen == expected.keys.to_set, "construction decision missing expected measurement rows"
end

def validate_vpp_lane_declarations(path)
  doc = read_json(path)
  lanes = doc.fetch("content").fetch("values").fetch("lanes")
  diff_lane = lanes.find { |lane| lane["lane"] == "exact-range-diff-hygiene" }
  assert diff_lane, "VPP missing exact-range-diff-hygiene lane"
  assert diff_lane["argv"] == ["git", "diff", "--check", "origin/main...HEAD"],
         "exact-range-diff-hygiene VPP lane must execute git diff --check origin/main...HEAD directly"
end

def validate_owner_lane_mapping_negative(lanes_by_issue)
  expect_failure("retained requirement with invented proof lane") do
    owner_issue = 501
    proof_lane = "v3-foundation-invented-lane"
    assert lanes_by_issue.fetch(owner_issue).key?(proof_lane),
           "fixture proof lane is not declared by owner"
  end
end

def validate_construction_decision_negative
  expect_failure("construction decision with invented measurements") do
    row = {
      "id" => "clean_build_seconds",
      "threshold_value" => 300,
      "threshold_unit" => "seconds",
      "observed_value" => 123,
      "evidence_revision" => "pretend-sha"
    }
    assert row["observed_value"].nil? && row["evidence_revision"].nil?,
           "fixture must not invent measured construction evidence"
  end
end

def validate_contract_negative
  expect_failure("contract default path omits finish") do
    default_section = "1. design\n2. bind\n3. validation\n4. publication\n5. cleanup\n"
    assert default_section.match?(/\bfinish\b/i),
           "fixture default path must mention retained finish gate"
  end
end

def validate_vpp_lane_negative
  expect_failure("VPP diff lane delegates to meta-validator") do
    argv = ["ruby", ".csdlc/prepared/issues/571/validate-v3a-followup.rb"]
    assert argv == ["git", "diff", "--check", "origin/main...HEAD"],
           "fixture diff lane must execute exact-range git diff directly"
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

  validate_contract_negative
  validate_construction_decision_negative
  validate_vpp_lane_negative
end

predecessor = require_file(File.join(ROOT, "docs/csdlc-v3/predecessor-coverage.json"))
owner_lanes = require_file(File.join(ROOT, "docs/csdlc-v3/owner-proof-lanes.json"))
lifecycle = require_file(File.join(ROOT, "docs/csdlc-v3/proportional-lifecycle.json"))
contract = require_file(File.join(ROOT, "docs/csdlc-v3/CONTRACT.md"))
construction_decision = require_file(File.join(ROOT, "docs/csdlc-v3/construction-decision.json"))
vpp = require_file(File.join(ROOT, ".csdlc/issues/571/cards/vpp.values.json"))
implementation_validator =
  require_file(File.join(ROOT, ".csdlc/prepared/issues/500/validate-implementation.rb"))

negative_fixtures
lanes_by_issue = validate_owner_lane_registry(owner_lanes)
validate_owner_lane_mapping_negative(lanes_by_issue)
validate_predecessor_coverage(predecessor, lanes_by_issue)
validate_lifecycle(lifecycle)
validate_contract(contract)
validate_construction_decision(construction_decision)
validate_diff_hygiene(implementation_validator)
validate_vpp_lane_declarations(vpp)

puts "V3-A corrective follow-up validation passed"
