#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

ROOT = File.expand_path("../../../..", __dir__)
SCHEDULE_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json")
RECEIPTS_PATH = File.join(ROOT, "docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json")
DENOMINATOR_PATH = File.join(ROOT, ".csdlc/prepared/issues/482/canonical-critical-asset-denominator.v1.json")

def fail_with(message)
  warn "CORP-A asset-denominator validation failed: #{message}"
  exit 1
end

fail_with("missing schedule #{SCHEDULE_PATH}") unless File.file?(SCHEDULE_PATH)
fail_with("missing custody receipts #{RECEIPTS_PATH}") unless File.file?(RECEIPTS_PATH)
fail_with("missing canonical denominator #{DENOMINATOR_PATH}") unless File.file?(DENOMINATOR_PATH)

schedule = JSON.parse(File.read(SCHEDULE_PATH))
receipts = JSON.parse(File.read(RECEIPTS_PATH))
denominator = JSON.parse(File.read(DENOMINATOR_PATH))

fail_with("unexpected schema #{schedule["schema"].inspect}") unless schedule["schema"] == "adl.corporate.critical_asset_schedule.v1"
fail_with("schedule is not accepted") unless schedule["accepted"] == true
fail_with("issue mismatch") unless schedule["issue"] == 482
fail_with("umbrella mismatch") unless schedule["umbrella_issue"] == 529
fail_with("denominator schema mismatch") unless denominator.fetch("schema") == "adl.corporate.corp_a_critical_asset_denominator.v1"
fail_with("denominator issue mismatch") unless denominator.fetch("issue") == 482
fail_with("denominator umbrella mismatch") unless denominator.fetch("umbrella_issue") == 529
fail_with("denominator must declare exactly 14 classes") unless denominator.fetch("class_count") == 14

classes = schedule.fetch("critical_asset_classes")
assets = schedule.fetch("assets")
canonical_rows = denominator.fetch("classes")
canonical_classes = canonical_rows.map { |row| row.fetch("class_id") }
canonical_asset_ids = canonical_rows.map { |row| row.fetch("required_asset_id") }

fail_with("critical asset classes must be unique") unless classes.length == classes.to_set.length
fail_with("asset ids must be unique") unless assets.map { |asset| asset.fetch("id") }.length == assets.map { |asset| asset.fetch("id") }.to_set.length
fail_with("canonical classes must be unique") unless canonical_classes.length == canonical_classes.to_set.length
fail_with("canonical asset ids must be unique") unless canonical_asset_ids.length == canonical_asset_ids.to_set.length
fail_with("canonical class count mismatch") unless canonical_classes.length == denominator.fetch("class_count")
fail_with("schedule must contain exactly 14 critical classes") unless classes.length == 14
fail_with("schedule class denominator differs from canonical manifest") unless classes == canonical_classes
fail_with("asset id denominator differs from canonical manifest") unless assets.map { |asset| asset.fetch("id") } == canonical_asset_ids

asset_classes = assets.map { |asset| asset.fetch("asset_class") }
missing_classes = classes - asset_classes
extra_classes = asset_classes - classes
duplicate_classes = asset_classes.each_with_object(Hash.new(0)) { |klass, counts| counts[klass] += 1 }.select { |_klass, count| count > 1 }.keys

fail_with("missing classes: #{missing_classes.join(", ")}") unless missing_classes.empty?
fail_with("unexpected classes: #{extra_classes.join(", ")}") unless extra_classes.empty?
fail_with("duplicate classes: #{duplicate_classes.join(", ")}") unless duplicate_classes.empty?

required_asset_fields = %w[
  id
  asset_class
  name
  business_owner
  custodian
  disposition
  provenance
  licensing
  trademark
  assignment
  custody_receipt_ref
  validation_surfaces
]

assets.each do |asset|
  missing = required_asset_fields.reject { |field| asset.key?(field) && !asset[field].nil? && asset[field] != "" }
  fail_with("#{asset["id"] || "(missing id)"} missing fields: #{missing.join(", ")}") unless missing.empty?
  fail_with("#{asset["id"]} is not accepted") unless asset.fetch("disposition").start_with?("accepted")
  fail_with("#{asset["id"]} missing all validation lanes") unless asset.fetch("validation_surfaces").sort == %w[asset-denominator provenance-and-license redaction-and-custody]
end

receipt_rows = receipts.fetch("receipts")
receipt_ids = receipt_rows.map { |receipt| receipt.fetch("receipt_id") }
receipt_asset_ids = receipt_rows.map { |receipt| receipt.fetch("asset_id") }
duplicate_receipt_ids = receipt_ids.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys
duplicate_receipt_asset_ids = receipt_asset_ids.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys

fail_with("receipt ids must be unique: #{duplicate_receipt_ids.sort.join(", ")}") unless duplicate_receipt_ids.empty?
fail_with("receipt asset ids must be unique: #{duplicate_receipt_asset_ids.sort.join(", ")}") unless duplicate_receipt_asset_ids.empty?
fail_with("receipt count must equal asset count") unless receipt_rows.length == assets.length

missing_receipts = assets.map { |asset| asset.fetch("id") }.to_set - receipt_asset_ids.to_set
extra_receipts = receipt_asset_ids.to_set - assets.map { |asset| asset.fetch("id") }.to_set
fail_with("assets without custody receipts: #{missing_receipts.to_a.sort.join(", ")}") unless missing_receipts.empty?
fail_with("receipts without schedule assets: #{extra_receipts.to_a.sort.join(", ")}") unless extra_receipts.empty?

receipt_ids_by_asset = receipt_rows.group_by { |receipt| receipt.fetch("asset_id") }
assets.each do |asset|
  id = asset.fetch("id")
  referenced = receipt_ids_by_asset.fetch(id, [])
  fail_with("#{id} must have exactly one custody receipt") unless referenced.length == 1
  fail_with("#{id} receipt reference must match the only receipt id") unless asset.fetch("custody_receipt_ref") == referenced.first.fetch("receipt_id")
end

puts "CORP-A asset denominator ok: #{assets.length} assets cover #{classes.length} classes with matching custody receipts"
