#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "tmpdir"
require "digest"

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


  paired_mutations = {
    "plan-candidate" => lambda do |plan, _receipt|
      plan["candidate"] = "0" * 40
    end,
    "coordinated-approval" => lambda do |plan, receipt|
      [plan, receipt].each do |doc|
        resolution = doc["rows"][0]["resolution"]
        resolution["approval_state"] = "approved"
        resolution["proposal_digest"] = Digest::SHA256.hexdigest([
          doc["rows"][0]["row_id"], doc["rows"][0]["criterion_text_digest"], resolution["type"], resolution["category"],
          resolution["proposed_disposition"], resolution["removal_scope"], resolution["replacement_text"], resolution["behavioral_pass_claim"], resolution["criterion_specific_basis"], resolution["proof_boundary"],
          resolution["approval_state"], resolution["operator_review_target"], resolution["operator_review_effect"]
        ].join("\0"))
      end
    end,
    "coordinated-operator-effect" => lambda do |plan, receipt|
      [plan, receipt].each do |doc|
        resolution = doc["rows"][0]["resolution"]
        resolution["operator_review_effect"] = "Approval is automatic."
        resolution["proposal_digest"] = Digest::SHA256.hexdigest([
          doc["rows"][0]["row_id"], doc["rows"][0]["criterion_text_digest"], resolution["type"], resolution["category"],
          resolution["proposed_disposition"], resolution["removal_scope"], resolution["replacement_text"], resolution["behavioral_pass_claim"], resolution["criterion_specific_basis"], resolution["proof_boundary"],
          resolution["approval_state"], resolution["operator_review_target"], resolution["operator_review_effect"]
        ].join("\0"))
      end
    end,
    "coordinated-governance-semantics" => lambda do |plan, receipt|
      [plan, receipt].each do |doc|
        resolution = doc["rows"][0]["resolution"]
        resolution["category"] = "behavioral_proof"
        resolution["replacement_text"] = "Criterion is satisfied by production behavior."
        resolution["proposal_digest"] = Digest::SHA256.hexdigest([
          doc["rows"][0]["row_id"], doc["rows"][0]["criterion_text_digest"], resolution["type"], resolution["category"],
          resolution["proposed_disposition"], resolution["removal_scope"], resolution["replacement_text"], resolution["behavioral_pass_claim"], resolution["criterion_specific_basis"], resolution["proof_boundary"],
          resolution["approval_state"], resolution["operator_review_target"], resolution["operator_review_effect"]
        ].join("\0"))
      end
    end
  }
  paired_mutations.each do |name, mutation|
    plan = JSON.parse(File.read(PLAN))
    receipt = JSON.parse(File.read(RECEIPT))
    mutation.call(plan, receipt)
    plan_path = File.join(dir, "#{name}-plan.json")
    receipt_path = File.join(dir, "#{name}-receipt.json")
    File.write(plan_path, JSON.pretty_generate(plan) + "\n")
    File.write(receipt_path, JSON.pretty_generate(receipt) + "\n")
    _stdout, _stderr, result = run_validator(plan_path, receipt_path)
    raise "validator accepted #{name}" if result.success?
  end

  puts "validated issue 820 negative matrix: #{mutations.length + paired_mutations.length} fail-closed mutations rejected"
end
