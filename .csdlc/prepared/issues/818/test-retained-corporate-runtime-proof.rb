#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"

PLAN = ".csdlc/prepared/issues/818/retained-corporate-runtime-resolution-plan.json"
SOURCE = ".csdlc/evidence/818/retained-corporate-runtime/reconciliation.json"
TMP = ".csdlc/evidence/818/retained-corporate-runtime-negative-tmp"
VALIDATOR = ".csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb"

def reject_mutation(plan, receipt, name)
  plan_copy = Marshal.load(Marshal.dump(plan))
  receipt_copy = Marshal.load(Marshal.dump(receipt))
  yield plan_copy, receipt_copy
  plan_path = File.join(TMP, "#{name}-plan.json")
  receipt_path = File.join(TMP, "#{name}-receipt.json")
  File.write(plan_path, JSON.pretty_generate(plan_copy) + "\n")
  File.write(receipt_path, JSON.pretty_generate(receipt_copy) + "\n")
  _output, status = Open3.capture2e(
    {"ISSUE818_PLAN" => plan_path, "ISSUE818_RECEIPT" => receipt_path},
    "ruby", VALIDATOR
  )
  raise "negative fixture passed: #{name}" if status.success?
end

FileUtils.rm_rf(TMP)
FileUtils.mkdir_p(TMP)
plan = JSON.parse(File.read(PLAN))
receipt = JSON.parse(File.read(SOURCE))
reference_index = plan.fetch("rows").index { |row| !row.fetch("reference_only_evidence").empty? }
raise "reference-only fixture row missing" unless reference_index

reject_mutation(plan, receipt, "missing-row") { |_, copy| copy.fetch("rows").pop }
reject_mutation(plan, receipt, "duplicate-row") { |_, copy| copy.fetch("rows")[-1] = copy.fetch("rows").first }
reject_mutation(plan, receipt, "wrong-candidate") { |_, copy| copy["candidate"] = "0" * 40 }
reject_mutation(plan, receipt, "stale-evidence") { |_, copy| copy.fetch("rows").first.fetch("candidate_evidence").first["sha256"] = "0" * 64 }
reject_mutation(plan, receipt, "source-reference-drift") { |copy, _| copy.fetch("rows")[reference_index]["reference_only_evidence"] = [] }
reject_mutation(plan, receipt, "source-assessment-promotion") do |copy, observed|
  [copy, observed].each { |document| document.fetch("rows").first["source_assessment"] = "proven" }
end
reject_mutation(plan, receipt, "synthetic-behavioral-pass") do |copy, observed|
  [copy, observed].each { |document| document.fetch("rows").first.fetch("resolution")["behavioral_pass_claim"] = true }
end
reject_mutation(plan, receipt, "fabricated-approval") { |_, copy| copy.fetch("rows").first.fetch("resolution")["status"] = "approved" }
reject_mutation(plan, receipt, "premature-release") { |_, copy| copy["release_ready"] = true }
reject_mutation(plan, receipt, "proposal-digest") do |copy, observed|
  [copy, observed].each { |document| document.fetch("rows").first.fetch("resolution")["proposal_digest"] = "0" * 64 }
end
reject_mutation(plan, receipt, "removal-scope") do |copy, observed|
  [copy, observed].each { |document| document.fetch("rows").first.fetch("resolution")["removal_scope"] = "remove all corporate requirements" }
end
reject_mutation(plan, receipt, "reference-promoted-to-proof") do |copy, observed|
  [copy, observed].each { |document| document.fetch("rows").first.fetch("resolution")["preexisting_evidence_context"] = "issue closure proves execution" }
end
reject_mutation(plan, receipt, "private-proof-claim") { |_, copy| copy.fetch("non_claims")[1] = "Private instruments were inspected." }
reject_mutation(plan, receipt, "sibling-bucket-row") { |_, copy| copy.fetch("rows").first["row_id"] = "DRT-A:retained-181-ac-4" }
reject_mutation(plan, receipt, "unclassified") { |_, copy| copy["unclassified"] = 1 }

FileUtils.rm_rf(TMP)
puts "PASS issue #818 negative matrix: 15/15 invalid packets rejected"
