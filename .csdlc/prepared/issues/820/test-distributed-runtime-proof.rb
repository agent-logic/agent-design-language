#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "tmpdir"

VALIDATOR = ".csdlc/prepared/issues/820/validate-distributed-runtime-proof.rb"
PLAN = ".csdlc/prepared/issues/820/distributed-runtime-resolution-plan.json"
RECEIPT = ".csdlc/evidence/820/distributed-runtime/reconciliation.json"

def run_validator(plan, receipt)
  Open3.capture3({"ISSUE820_PLAN" => plan, "ISSUE820_RECEIPT" => receipt}, "ruby", VALIDATOR)
end

_out, err, status = run_validator(PLAN, RECEIPT)
raise "baseline validator failed: #{err}" unless status.success?

mutations = {
  "missing-row" => ->(doc) { doc["rows"].pop; doc["row_count"] = 24 },
  "duplicate-row" => ->(doc) { doc["rows"][1] = doc["rows"][0] },
  "behavioral-pass" => ->(doc) { doc["behavioral_pass_count"] = 1 },
  "approval-bypass" => ->(doc) { doc["rows"][0]["resolution"]["status"] = "passed" },
  "release-ready" => ->(doc) { doc["release_ready"] = true },
  "proposal-tamper" => ->(doc) { doc["rows"][0]["resolution"]["removal_scope"] = "remove everything" },
  "owner-tamper" => ->(doc) { doc["rows"][0]["owner"] = "#999" },
  "evidence-tamper" => ->(doc) { doc["rows"][0]["candidate_evidence"][0]["sha256"] = "0" * 64 },
  "surface-tamper" => ->(doc) { doc["candidate_surface_tree_sha256"] = "0" * 64 }
}

Dir.mktmpdir("issue820-negative-") do |dir|
  mutations.each do |name, mutation|
    doc = JSON.parse(File.read(RECEIPT))
    mutation.call(doc)
    path = File.join(dir, "#{name}.json")
    File.write(path, JSON.pretty_generate(doc) + "\n")
    _stdout, _stderr, result = run_validator(PLAN, path)
    raise "validator accepted #{name}" if result.success?
  end
end

puts "validated issue 820 negative matrix: #{mutations.length} fail-closed mutations rejected"
