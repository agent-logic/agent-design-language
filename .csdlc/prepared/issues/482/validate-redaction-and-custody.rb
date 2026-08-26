#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "set"

ROOT = File.expand_path("../../../..", __dir__)
SCHEDULE_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json")
RECEIPTS_PATH = File.join(ROOT, "docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json")
SCHEDULE_MD_PATH = File.join(ROOT, "docs/operations/corporate/asset-register/critical-asset-schedule.md")

FORBIDDEN_KEYS = /\A(password|passwd|token|secret|private_key|access_key|session_key|credential|signature|tax_id|ssn|account_number)\z/i
FORBIDDEN_VALUE_PATTERNS = [
  /-----BEGIN [A-Z ]*PRIVATE KEY-----/,
  /AKIA[0-9A-Z]{16}/,
  /gh[pousr]_[A-Za-z0-9_]{36,}/,
  /xox[baprs]-[A-Za-z0-9-]+/,
  /sk-[A-Za-z0-9]{20,}/
].freeze

def fail_with(message)
  warn "CORP-A redaction/custody validation failed: #{message}"
  exit 1
end

def walk_json(value, path = "$", &block)
  case value
  when Hash
    value.each do |key, nested|
      yield("#{path}.#{key}", key, nested)
      walk_json(nested, "#{path}.#{key}", &block)
    end
  when Array
    value.each_with_index { |nested, index| walk_json(nested, "#{path}[#{index}]", &block) }
  end
end

fail_with("missing schedule #{SCHEDULE_PATH}") unless File.file?(SCHEDULE_PATH)
fail_with("missing receipts #{RECEIPTS_PATH}") unless File.file?(RECEIPTS_PATH)
fail_with("missing Markdown schedule #{SCHEDULE_MD_PATH}") unless File.file?(SCHEDULE_MD_PATH)

schedule = JSON.parse(File.read(SCHEDULE_PATH))
receipts = JSON.parse(File.read(RECEIPTS_PATH))
markdown = File.read(SCHEDULE_MD_PATH)

fail_with("receipt schema mismatch") unless receipts.fetch("schema") == "adl.corporate.redacted_custody_receipts.v1"
fail_with("receipt issue mismatch") unless receipts.fetch("issue") == 482
fail_with("receipt policy must reject private material in git") unless receipts.dig("redaction_policy", "private_material_in_git") == false

asset_ids = schedule.fetch("assets").map { |asset| asset.fetch("id") }.to_set
receipt_rows = receipts.fetch("receipts")
receipt_ids = receipt_rows.map { |receipt| receipt.fetch("receipt_id") }
receipt_asset_ids = receipt_rows.map { |receipt| receipt.fetch("asset_id") }
receipt_binding_ids = receipt_rows.map { |receipt| receipt.fetch("redacted_instrument_binding_id") }
receipt_binding_digests = receipt_rows.map { |receipt| receipt.fetch("redacted_instrument_binding_digest") }
duplicate_receipt_ids = receipt_ids.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys
duplicate_receipt_asset_ids = receipt_asset_ids.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys
duplicate_binding_ids = receipt_binding_ids.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys
duplicate_binding_digests = receipt_binding_digests.each_with_object(Hash.new(0)) { |id, counts| counts[id] += 1 }.select { |_id, count| count > 1 }.keys

fail_with("receipt ids must be unique: #{duplicate_receipt_ids.sort.join(", ")}") unless duplicate_receipt_ids.empty?
fail_with("receipt asset ids must be unique: #{duplicate_receipt_asset_ids.sort.join(", ")}") unless duplicate_receipt_asset_ids.empty?
fail_with("redacted binding ids must be unique: #{duplicate_binding_ids.sort.join(", ")}") unless duplicate_binding_ids.empty?
fail_with("redacted binding digests must be unique: #{duplicate_binding_digests.sort.join(", ")}") unless duplicate_binding_digests.empty?
fail_with("receipt count must equal asset count") unless receipt_rows.length == schedule.fetch("assets").length

receipt_asset_id_set = receipt_asset_ids.to_set
missing_receipts = asset_ids - receipt_asset_id_set
extra_receipts = receipt_asset_id_set - asset_ids
fail_with("missing receipt rows: #{missing_receipts.to_a.sort.join(", ")}") unless missing_receipts.empty?
fail_with("unexpected receipt rows: #{extra_receipts.to_a.sort.join(", ")}") unless extra_receipts.empty?

acceptance_authority = schedule.fetch("acceptance_authority")
authority_receipts = receipts.fetch("authority_receipts")
fail_with("exactly one authority receipt is required") unless authority_receipts.length == 1
authority_receipt = authority_receipts.first
authority_receipt_id = authority_receipt.fetch("receipt_id")
fail_with("authority receipt reference mismatch") unless acceptance_authority.fetch("authority_receipt_ref").end_with?("##{authority_receipt_id}")
fail_with("authority accepted_by mismatch") unless acceptance_authority.fetch("accepted_by") == authority_receipt.fetch("accepted_by")
fail_with("authority accepted_at mismatch") unless acceptance_authority.fetch("accepted_at") == authority_receipt.fetch("accepted_at")
fail_with("authority receipt is not accepted") unless authority_receipt.fetch("verification_result") == "accepted"
fail_with("authority binding id mismatch") unless acceptance_authority.fetch("redacted_authority_binding_id") == authority_receipt.fetch("redacted_authority_binding_id")

authority_tuple = [
  schedule.fetch("issue"),
  schedule.fetch("schedule_id"),
  authority_receipt_id,
  authority_receipt.fetch("redacted_authority_binding_id"),
  authority_receipt.fetch("accepted_by"),
  authority_receipt.fetch("accepted_at")
].join("|")
expected_authority_digest = "sha256:#{Digest::SHA256.hexdigest(authority_tuple)}"
fail_with("authority digest mismatch") unless authority_receipt.fetch("redacted_authority_binding_digest") == expected_authority_digest
fail_with("schedule authority digest mismatch") unless acceptance_authority.fetch("redacted_authority_binding_digest") == expected_authority_digest

receipt_by_id = receipt_rows.to_h { |receipt| [receipt.fetch("receipt_id"), receipt] }
schedule.fetch("assets").each do |asset|
  receipt_ref = asset.fetch("custody_receipt_ref")
  receipt = receipt_by_id[receipt_ref]
  fail_with("#{asset.fetch("id")} references missing receipt #{receipt_ref}") unless receipt
  fail_with("#{asset.fetch("id")} receipt asset mismatch") unless receipt.fetch("asset_id") == asset.fetch("id")
  fail_with("#{asset.fetch("id")} receipt class mismatch") unless receipt.fetch("asset_class") == asset.fetch("asset_class")
  fail_with("#{asset.fetch("id")} receipt is not accepted") unless receipt.fetch("verification_result") == "accepted"
  fail_with("#{asset.fetch("id")} receipt lacks redaction statement") if receipt.fetch("redaction").strip.empty?
  fail_with("#{asset.fetch("id")} missing Markdown receipt reference") unless markdown.include?(receipt_ref)
  fail_with("#{asset.fetch("id")} receipt accepted_by mismatch") unless receipt.fetch("accepted_by") == authority_receipt.fetch("accepted_by")
  fail_with("#{asset.fetch("id")} receipt accepted_at mismatch") unless receipt.fetch("accepted_at") == authority_receipt.fetch("accepted_at")
  fail_with("#{asset.fetch("id")} authority receipt mismatch") unless receipt.fetch("authority_receipt_ref") == authority_receipt_id

  expected_binding_id = "corp-a-instrument-binding-#{receipt.fetch("asset_class").tr("_", "-")}-v1"
  fail_with("#{asset.fetch("id")} redacted binding id mismatch") unless receipt.fetch("redacted_instrument_binding_id") == expected_binding_id

  binding_tuple = [
    receipts.fetch("issue"),
    schedule.fetch("schedule_id"),
    receipt.fetch("receipt_id"),
    receipt.fetch("asset_id"),
    receipt.fetch("asset_class"),
    receipt.fetch("custodian_role"),
    receipt.fetch("accepted_by"),
    receipt.fetch("accepted_at"),
    receipt.fetch("authority_receipt_ref")
  ].join("|")
  expected_digest = "sha256:#{Digest::SHA256.hexdigest(binding_tuple)}"
  fail_with("#{asset.fetch("id")} redacted binding digest mismatch") unless receipt.fetch("redacted_instrument_binding_digest") == expected_digest
end

[schedule, receipts].each do |json|
  walk_json(json) do |path, key, nested|
    fail_with("forbidden sensitive key at #{path}") if key.match?(FORBIDDEN_KEYS)
    next unless nested.is_a?(String)

    FORBIDDEN_VALUE_PATTERNS.each do |pattern|
      fail_with("forbidden sensitive value pattern at #{path}") if nested.match?(pattern)
    end
  end
end

fail_with("Markdown claims credential/private instruments are included") if markdown.match?(/credentials?.*stored|private instruments?.*stored/i)

puts "CORP-A redaction/custody ok: #{receipts.fetch("receipts").length} redacted receipts checked"
