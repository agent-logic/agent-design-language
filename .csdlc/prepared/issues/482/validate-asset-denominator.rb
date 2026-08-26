#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

ROOT = File.expand_path("../../../..", __dir__)
SCHEDULE_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json")
RECEIPTS_PATH = File.join(ROOT, "docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json")

def fail_with(message)
  warn "CORP-A asset-denominator validation failed: #{message}"
  exit 1
end

fail_with("missing schedule #{SCHEDULE_PATH}") unless File.file?(SCHEDULE_PATH)
fail_with("missing custody receipts #{RECEIPTS_PATH}") unless File.file?(RECEIPTS_PATH)

schedule = JSON.parse(File.read(SCHEDULE_PATH))
receipts = JSON.parse(File.read(RECEIPTS_PATH))

fail_with("unexpected schema #{schedule["schema"].inspect}") unless schedule["schema"] == "adl.corporate.critical_asset_schedule.v1"
fail_with("schedule is not accepted") unless schedule["accepted"] == true
fail_with("issue mismatch") unless schedule["issue"] == 482
fail_with("umbrella mismatch") unless schedule["umbrella_issue"] == 529

classes = schedule.fetch("critical_asset_classes")
assets = schedule.fetch("assets")
fail_with("critical asset classes must be unique") unless classes.length == classes.to_set.length
fail_with("asset ids must be unique") unless assets.map { |asset| asset.fetch("id") }.length == assets.map { |asset| asset.fetch("id") }.to_set.length

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

receipt_asset_ids = receipts.fetch("receipts").map { |receipt| receipt.fetch("asset_id") }.to_set
missing_receipts = assets.map { |asset| asset.fetch("id") }.to_set - receipt_asset_ids
fail_with("assets without custody receipts: #{missing_receipts.to_a.sort.join(", ")}") unless missing_receipts.empty?

puts "CORP-A asset denominator ok: #{assets.length} assets cover #{classes.length} classes with matching custody receipts"
