#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

ROOT = File.expand_path("../../../..", __dir__)

def read_json(path)
  JSON.parse(File.read(File.join(ROOT, path)))
end

def fail_with(message)
  warn(message)
  exit 1
end

schedule_path = "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json"
register_path = "docs/operations/corporate/account-custody/corporate-custody-register.v1.json"
receipts_path = "docs/milestones/v0.92.1/evidence/corporate/corp-b/readback-receipts.v1.json"
markdown_path = "docs/operations/corporate/account-custody/corporate-custody-register.md"

[schedule_path, register_path, receipts_path, markdown_path].each do |path|
  fail_with("missing required artifact: #{path}") unless File.file?(File.join(ROOT, path))
end

schedule = read_json(schedule_path)
register = read_json(register_path)
receipts = read_json(receipts_path)
markdown = File.read(File.join(ROOT, markdown_path))

fail_with("register schema mismatch") unless register["schema"] == "adl.corporate.account_custody_register.v1"
fail_with("receipt schema mismatch") unless receipts["schema"] == "adl.corporate.corp_b_readback_receipts.v1"
fail_with("register must be read-only") unless register.dig("authority_boundary", "external_mutations_performed") == false
fail_with("register must not schedule v-*.ai backlog") unless register.dig("authority_boundary", "v_ai_backlog_scheduled") == false

asset_classes = schedule.fetch("critical_asset_classes").to_set
rows = register.fetch("service_rows")
row_class_values = rows.map { |row| row.fetch("source_asset_class") }
counts = Hash.new(0)
row_class_values.each { |value| counts[value] += 1 }
duplicates = counts.select { |_klass, count| count > 1 }.keys
fail_with("duplicate service rows for asset classes: #{duplicates.sort.join(", ")}") unless duplicates.empty?
row_classes = row_class_values.to_set
missing = asset_classes - row_classes
extra = row_classes - asset_classes
fail_with("missing service rows for asset classes: #{missing.to_a.sort.join(", ")}") unless missing.empty?
fail_with("unexpected service rows for asset classes: #{extra.to_a.sort.join(", ")}") unless extra.empty?
fail_with("source asset denominator must be exactly once") unless row_class_values.length == asset_classes.length

allowed_statuses = %w[accepted_readback follow_up_required supporting_register_row]
allowed_owners = %w[CORP-C AWS-G GCP-D service-lane business-operations-role engineering-maintainer-role infrastructure-maintainer-role release-maintainer-role]
rows.each do |row|
  fail_with("row missing id") unless row["id"].to_s.match?(/\Acorp-b-[a-z0-9-]+\z/)
  fail_with("row #{row["id"]} has invalid status") unless allowed_statuses.include?(row["status"])
  fail_with("row #{row["id"]} missing later_owner") unless allowed_owners.include?(row["later_owner"])
  %w[admin billing mfa recovery vault break_glass].each do |field|
    fail_with("row #{row["id"]} missing #{field}") unless row[field].is_a?(Hash) && row[field]["posture"]
  end
  actions = row["actions"]
  fail_with("row #{row["id"]} missing actions array") unless actions.is_a?(Array)
  if row["status"] == "follow_up_required"
    fail_with("row #{row["id"]} follow-up row must have nonempty action list") if actions.empty?
    fail_with("row #{row["id"]} follow-up row must have later owner") if row["later_owner"].to_s.empty?
  end
  if row["status"] == "accepted_readback"
    fail_with("row #{row["id"]} overclaims complete custody") if row["follow_up_required"] == true
    fail_with("row #{row["id"]} accepted row must not carry follow-up actions") unless actions.empty?
  end
end

completed_domains = receipts.fetch("domain_registration_transfer_receipts")
expected_domains = %w[
  agent-logic.ai
  codefriend.ai
  agent-logic.net
  aptitude-atlas.com
  cognitivespacetimemanifold.com
]
observed_domains = completed_domains.map { |row| row.fetch("domain") }.sort
fail_with("domain receipt denominator mismatch") unless observed_domains == expected_domains.sort
completed_domains.each do |row|
  fail_with("domain #{row["domain"]} not successful") unless row["destination_ownership_observed"] == true && row["contains_transfer_password"] == false
  fail_with("domain #{row["domain"]} must not claim hosted-zone move") unless row["hosted_zone_moved"] == false
end

backlog = register.fetch("unscheduled_backlog")
fail_with("v-dev.ai backlog missing") unless backlog.any? { |row| row["name"] == "v-dev.ai" && row["scheduled"] == false && row["milestone_gate"] == false }
backlog.each do |row|
  next unless row["name"].to_s.match?(/\Av-.*\.ai\z/)
  fail_with("#{row["name"]} must remain unscheduled") unless row["scheduled"] == false
  fail_with("#{row["name"]} must not gate the milestone") unless row["milestone_gate"] == false
end
register.fetch("completed_registration_transfers").each do |domain|
  fail_with("#{domain} must not be in completed/actionable transfers") if domain.match?(/\Av-.*\.ai\z/)
end
rows.each do |row|
  row.fetch("actions").each do |action|
    fail_with("v-*.ai transfer action must remain backlog-only") if action.match?(/transfer.*\bv-.*\.ai/i)
  end
end

forbidden = /(BEGIN [A-Z ]*PRIVATE KEY|AKIA[0-9A-Z]{16}|aws_secret_access_key|password\s*[:=]|token\s*[:=]|secret\s*[:=]|credit.?card|tax.?id|ssn|recovery code|seed phrase|private instrument)/i
[register_path, receipts_path, markdown_path].each do |path|
  text = File.read(File.join(ROOT, path))
  fail_with("forbidden sensitive pattern in #{path}") if text.match?(forbidden)
end

fail_with("markdown missing read-only boundary") unless markdown.include?("No external service mutation")
fail_with("markdown overclaims live recovery exercise") if markdown.include?("recovery exercised")

puts JSON.pretty_generate(
  schema: "adl.corporate.corp_b_validation.v1",
  outcome: "passed",
  rows: rows.length,
  completed_domain_registration_transfers: observed_domains,
  backlog_non_gating: true
)
