#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "pathname"

ROOT_FOR_TEST = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
VALIDATOR = ROOT_FOR_TEST / ".csdlc/prepared/issues/467/validate-quality-gate.rb"
WORK = ROOT_FOR_TEST / ".csdlc/evidence/467/adversarial-fixtures"
load VALIDATOR

def copy(value)
  Marshal.load(Marshal.dump(value))
end

def errors_for(name, matrix)
  path = WORK / "#{name}.json"
  path.write(JSON.pretty_generate(matrix) + "\n")
  validate_matrix(path, canonical: false).last
end

def reject!(name, matrix, expected)
  errors = errors_for(name, matrix)
  raise "#{name} unexpectedly passed" if errors.empty?
  raise "#{name} did not report #{expected}: #{errors.inspect}" unless errors.any? { |error| error.include?(expected) }
end

FileUtils.rm_rf(WORK)
FileUtils.mkdir_p(WORK)
base = build_matrix
raise "base fixture failed" unless errors_for("base", base).empty?

tampered = copy(base)
tampered["rows"].shift
reject!("missing-row", tampered, "denominator_missing")

tampered = copy(base)
tampered["rows"] << copy(tampered["rows"].first)
reject!("duplicate-row", tampered, "denominator_duplicate")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
row["disposition"] = "blocked"
reject!("accepted-suppressed", tampered, "accepted_mapping_missing")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
row["evidence"]["deliveries"].first["merge_sha"] = "0" * 40
reject!("merge-identity-tamper", tampered, "delivery_mapping_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "accepted" }
row["evidence"]["deliveries"].first["evidence"]["sha256"] = "0" * 64
reject!("proof-digest-tamper", tampered, "delivery_mapping_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["disposition"] == "scoped_out" }
row["evidence"]["scope"]["target"] = "invented"
reject!("scope-tamper", tampered, "scope_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["id"] == "critical:AEE-018" }
row["source_status"] = "blocked_with_evidence"
reject!("implemented-status-tamper", tampered, "source_status_mismatch")

tampered = copy(base)
row = tampered["rows"].find { |item| item["id"] == "critical:AEE-020" }
row["disposition"] = "accepted"
reject!("circular-tail-tamper", tampered, "scope_mismatch")

FileUtils.rm_rf(WORK)
puts JSON.generate(schema: "adl.v0.92.quality_gate_adversarial_suite.v3", status: "passed", cases: 8, accepted: 30, scoped_out: 3, blocked: 0)
