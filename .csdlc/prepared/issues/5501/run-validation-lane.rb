#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb <lane>") }
allowed = %w[live-manifest live-two-shard baseline-comparison post-merge-exact].freeze
abort("unsupported lane: #{lane}") unless allowed.include?(lane)

if lane == "live-manifest"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  manifest = root.join(".csdlc/evidence/5501/live-run-manifest.json")
  validator = Pathname.new(__dir__).join("validate-live-run-manifest.rb")
  exec("ruby", validator.to_s, manifest.to_s)
end

if lane == "live-two-shard"
  validator = Pathname.new(__dir__).join("validate-retained-live-proof.rb")
  exec("ruby", validator.to_s)
end

if lane == "baseline-comparison"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  baseline = root.join(".csdlc/evidence/5501/single-agent-comparison.json")
  proof = root.join(".csdlc/evidence/5501/retained-live-proof.json")
  fail "single-agent comparison evidence is absent" unless baseline.file?
  fail "retained proof evidence is absent" unless proof.file?

  data = JSON.parse(baseline.read)
  unless data["baseline_status"] == "comparison_only_not_executed_as_substitute"
    abort("baseline comparison must not claim an unobserved substitute run")
  end
  unless data.dig("comparison", "fairness_result") == "bounded_comparison_without_speedup_claim"
    abort("baseline comparison must avoid numeric speedup claims")
  end
  puts JSON.pretty_generate(status: "pass", lane: lane, baseline: data["baseline_status"])
  exit 0
end

if lane == "post-merge-exact"
  root = Pathname.new(__dir__).join("../../../..").expand_path
  dependency_gate = Pathname.new(__dir__).join("check-dependencies.rb")
  manifest = root.join(".csdlc/evidence/5501/live-run-manifest.json")
  manifest_validator = Pathname.new(__dir__).join("validate-live-run-manifest.rb")
  retained_validator = Pathname.new(__dir__).join("validate-retained-live-proof.rb")
  system("ruby", dependency_gate.to_s) || exit(2)
  system("ruby", manifest_validator.to_s, manifest.to_s) || exit(2)
  system("ruby", retained_validator.to_s) || exit(2)
  puts JSON.pretty_generate(status: "pass", lane: lane)
  exit 0
end
