#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "json"
require "open3"

SOURCE = ".csdlc/evidence/819/retained-v3/reconciliation.json"
TMP = ".csdlc/evidence/819/retained-v3-negative-tmp"
VALIDATOR = ".csdlc/prepared/issues/819/validate-retained-v3-proof.rb"

def mutate(document, name)
  copy = Marshal.load(Marshal.dump(document))
  yield copy
  path = File.join(TMP, "#{name}.json")
  File.write(path, JSON.pretty_generate(copy) + "\n")
  _output, status = Open3.capture2e({"ISSUE819_RECEIPT" => path}, "ruby", VALIDATOR)
  raise "negative fixture passed: #{name}" if status.success?
end

def mutate_pair(plan, receipt, name)
  plan_copy = Marshal.load(Marshal.dump(plan))
  receipt_copy = Marshal.load(Marshal.dump(receipt))
  yield plan_copy, receipt_copy
  plan_path = File.join(TMP, "#{name}-plan.json")
  receipt_path = File.join(TMP, "#{name}-receipt.json")
  File.write(plan_path, JSON.pretty_generate(plan_copy) + "\n")
  File.write(receipt_path, JSON.pretty_generate(receipt_copy) + "\n")
  _output, status = Open3.capture2e({"ISSUE819_PLAN" => plan_path, "ISSUE819_RECEIPT" => receipt_path}, "ruby", VALIDATOR)
  raise "coordinated negative fixture passed: #{name}" if status.success?
end

FileUtils.rm_rf(TMP)
FileUtils.mkdir_p(TMP)
document = JSON.parse(File.read(SOURCE))
plan = JSON.parse(File.read(".csdlc/prepared/issues/819/retained-v3-resolution-plan.json"))

mutate(document, "missing-row") { |copy| copy.fetch("rows").pop }
mutate(document, "duplicate-row") { |copy| copy.fetch("rows")[-1] = copy.fetch("rows").first }
mutate(document, "unclassified") { |copy| copy["unclassified"] = 1 }
mutate(document, "failed-command") { |copy| copy.fetch("commands").first["status"] = "failed" }
mutate(document, "command-argv") { |copy| copy.fetch("commands").first["argv"] << "--ignored" }
mutate(document, "stale-evidence") { |copy| copy.fetch("rows").first.fetch("candidate_evidence").first["sha256"] = "0" * 64 }
mutate(document, "empty-evidence") { |copy| copy.fetch("rows").first["candidate_evidence"] = [] }
mutate(document, "wrong-candidate") { |copy| copy["candidate"] = "0" * 40 }
mutate(document, "surface-digest") { |copy| copy["candidate_surface_tree_sha256"] = "0" * 64 }
mutate(document, "resolution-drift") { |copy| copy.fetch("rows").first.fetch("resolution")["criterion_specific_basis"] = "unrelated claim" }
amendment_index = document.fetch("rows").index { |row| row.dig("resolution", "type") == "governed_disposition_proposal" }
mutate(document, "synthetic-pass-amendment") { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["behavioral_pass_claim"] = true }
mutate(document, "fabricated-amendment-approval") { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["status"] = "approved" }
mutate(document, "premature-release") { |copy| copy["release_ready"] = true }
mutate_pair(plan, document, "coordinated-assessment-promotion") do |plan_copy, receipt_copy|
  [plan_copy, receipt_copy].each { |copy| copy.fetch("rows")[amendment_index]["source_assessment"] = "proven" }
end
mutate_pair(plan, document, "coordinated-semantic-drift") do |plan_copy, receipt_copy|
  [plan_copy, receipt_copy].each { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["criterion_specific_basis"] = "rewritten basis" }
end
mutate_pair(plan, document, "coordinated-disposition-drift") do |plan_copy, receipt_copy|
  [plan_copy, receipt_copy].each { |copy| copy.fetch("rows")[amendment_index].fetch("resolution")["proposed_disposition"] = "amend_without_replacement" }
end
mutate_pair(plan, document, "forged-source-pointer") do |plan_copy, receipt_copy|
  forged = JSON.parse(File.read("docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/retained-v3.json"))
  forged.fetch("rows").find { |row| row.fetch("row_id") == receipt_copy.fetch("rows")[amendment_index].fetch("row_id") }["proposed_result"] = "proven"
  forged_path = File.join(TMP, "forged-source.json")
  File.write(forged_path, JSON.pretty_generate(forged) + "\n")
  plan_copy["source_mapping"] = forged_path
  [plan_copy, receipt_copy].each { |copy| copy.fetch("rows")[amendment_index]["source_assessment"] = "proven" }
end
mutate_pair(plan, document, "owner-drift") do |plan_copy, receipt_copy|
  [plan_copy, receipt_copy].each { |copy| copy.fetch("rows").first["owner"] = "#forged-owner" }
end

FileUtils.rm_rf(TMP)
puts "PASS issue #819 negative matrix: 18/18 invalid receipts rejected"
