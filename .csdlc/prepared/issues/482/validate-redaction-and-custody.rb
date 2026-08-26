#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
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
receipt_ids = receipts.fetch("receipts").map { |receipt| receipt.fetch("asset_id") }.to_set
missing_receipts = asset_ids - receipt_ids
extra_receipts = receipt_ids - asset_ids
fail_with("missing receipt rows: #{missing_receipts.to_a.sort.join(", ")}") unless missing_receipts.empty?
fail_with("unexpected receipt rows: #{extra_receipts.to_a.sort.join(", ")}") unless extra_receipts.empty?

receipt_by_id = receipts.fetch("receipts").to_h { |receipt| [receipt.fetch("receipt_id"), receipt] }
schedule.fetch("assets").each do |asset|
  receipt_ref = asset.fetch("custody_receipt_ref")
  receipt = receipt_by_id[receipt_ref]
  fail_with("#{asset.fetch("id")} references missing receipt #{receipt_ref}") unless receipt
  fail_with("#{asset.fetch("id")} receipt asset mismatch") unless receipt.fetch("asset_id") == asset.fetch("id")
  fail_with("#{asset.fetch("id")} receipt class mismatch") unless receipt.fetch("asset_class") == asset.fetch("asset_class")
  fail_with("#{asset.fetch("id")} receipt is not accepted") unless receipt.fetch("verification_result") == "accepted"
  fail_with("#{asset.fetch("id")} receipt lacks redaction statement") if receipt.fetch("redaction").strip.empty?
  fail_with("#{asset.fetch("id")} missing Markdown receipt reference") unless markdown.include?(receipt_ref)
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
