#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../..", __dir__)

def path(relative)
  File.join(ROOT, relative)
end

def read_json(relative)
  file = path(relative)
  abort("missing required JSON: #{relative}") unless File.file?(file)
  JSON.parse(File.read(file))
rescue JSON::ParserError => e
  abort("invalid JSON #{relative}: #{e.message}")
end

def read_text(relative)
  file = path(relative)
  abort("missing required file: #{relative}") unless File.file?(file)
  File.read(file)
end

def assert(condition, message)
  abort(message) unless condition
end

def assert_ancestor(oid)
  _stdout, stderr, status = Open3.capture3("git", "-C", ROOT, "merge-base", "--is-ancestor", oid, "origin/main")
  assert(status.success?, "merge commit #{oid} is not an ancestor of origin/main: #{stderr.strip}")
end

required_files = [
  ".csdlc/issues/497/index.json",
  ".csdlc/issues/497/cards/stp.md",
  ".csdlc/issues/497/cards/spp.md",
  ".csdlc/issues/497/cards/vpp.md",
  ".csdlc/issues/497/cards/sor.md",
  "docs/operations/corporate/asset-register/critical-asset-schedule.md",
  "docs/operations/corporate/asset-register/critical-asset-schedule.v1.json",
  "docs/operations/corporate/account-custody/corporate-custody-register.md",
  "docs/operations/corporate/account-custody/corporate-custody-register.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-b/readback-receipts.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/prerequisite-ancestry.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/account-authority-readback.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/external-action-classification.v1.json",
  "docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json",
  "docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.md"
]

required_files.each do |relative|
  assert(File.file?(path(relative)), "missing required file: #{relative}")
end

ancestry = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/prerequisite-ancestry.v1.json")
assert(ancestry["schema"] == "adl.corporate.corp_c.prerequisite_ancestry.v1", "unexpected ancestry schema")
assert(ancestry["issue"] == 497, "ancestry issue mismatch")
assert(ancestry["repository"] == "agent-logic/agent-design-language", "ancestry repository mismatch")
assert(ancestry["result"] == "pass", "ancestry result is not pass")

expected_prereqs = {
  482 => [545, "e2c1d1649b0c930a5a1254575a07ef2a4496d48d"],
  483 => [562, "4a0b49c0071bacdaab19d6d9eb8c44380beb51be"],
  493 => [587, "c0bf217934508d6dbc70d78633e6a95d5ddd9d06"],
  496 => [599, "83077ca029d52c9d613ed5a373da30f1dd42d9b3"]
}

rows = ancestry.fetch("live_issue_checks")
assert(rows.size == expected_prereqs.size, "unexpected prerequisite row count")
rows.each do |row|
  issue = row.fetch("issue")
  expected_pr, expected_oid = expected_prereqs.fetch(issue)
  assert(row["state"] == "CLOSED", "issue ##{issue} is not recorded closed")
  assert(row["closing_pr"] == expected_pr, "issue ##{issue} closing PR mismatch")
  assert(row["closing_pr_state"] == "MERGED", "PR ##{expected_pr} is not recorded merged")
  assert(row["merge_commit"] == expected_oid, "PR ##{expected_pr} merge commit mismatch")
  assert(row["ancestor_of_origin_main"] == true, "PR ##{expected_pr} ancestry flag not true")
  assert_ancestor(expected_oid)
end

authority = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/account-authority-readback.v1.json")
assert(authority["schema"] == "adl.corporate.corp_c.account_authority_readback.v1", "unexpected authority schema")
aws = authority.fetch("provider_checks").find { |row| row["provider"] == "aws" }
assert(aws, "missing AWS authority readback")
assert(aws["profile"] == "agent-logic-admin", "AWS readback used unexpected profile")
assert(aws["mutation"] == false, "AWS readback must be non-mutating")
assert(aws["credential_material_captured"] == false, "AWS readback must not capture credential material")
assert(aws["classification"] == "readback_pass", "AWS readback classification mismatch")

classification = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/external-action-classification.v1.json")
assert(classification["schema"] == "adl.corporate.corp_c.external_action_classification.v1", "unexpected classification schema")
assert(classification["issue"] == 497, "classification issue mismatch")
assert(classification["authorized_actions"] == [], "authorized actions must remain empty without explicit operator authorization")
assert(classification["blocked_actions"] == [], "blocked actions must remain empty for CORP-C repository-local acceptance")
assert(classification.fetch("rows").all? { |row| row["mutation_performed_by_497"] == false }, "no external mutation may be recorded for #497")
assert(classification.fetch("rows").any? { |row| row["classification"] == "deferred_action" }, "expected at least one deferred external action")

packet = read_json("docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json")
assert(packet["schema"] == "adl.corporate.operational_control_transfer_acceptance.v1", "unexpected packet schema")
assert(packet["issue"] == 497, "packet issue mismatch")
assert(packet["status"] == "accepted_with_deferred_external_actions", "packet status mismatch")
assert(packet.dig("prerequisite_gate", "status") == "pass", "packet prerequisite gate not pass")
assert(packet["authorized_actions"] == [], "packet authorized actions must remain empty")
assert(packet["blocked_actions"] == [], "packet blocked actions must remain empty")
assert(packet.fetch("deferred_actions").size >= 4, "packet should record deferred external actions")

acceptance_statuses = packet.fetch("acceptance").to_h { |row| [row.fetch("id"), row.fetch("status")] }
assert(acceptance_statuses["AC-1"] == "pass", "AC-1 status mismatch")
assert(acceptance_statuses["AC-2"] == "pass_with_deferred_external_actions", "AC-2 status mismatch")
assert(acceptance_statuses["AC-3"] == "pass", "AC-3 status mismatch")
assert(acceptance_statuses["AC-4"] == "pass", "AC-4 status mismatch")

markdown = read_text("docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.md")
[
  "CORP-C is accepted with deferred external actions.",
  "No production/provider mutation",
  "This acceptance does not mean:",
  "Sprint 7 #345 AWS GPU execution",
  "CORP-D #498 diligence acceptance"
].each do |needle|
  assert(markdown.include?(needle), "acceptance Markdown missing boundary text: #{needle}")
end

scan_files = required_files.select { |relative| relative.include?("corp-c") || relative.include?("control-transfer") || relative.include?(".csdlc/evidence/497") }
credential_markers = [
  /-----BEGIN [A-Z ]*PRIVATE KEY-----/,
  /aws_secret_access_key/i,
  /aws_session_token/i,
  /ghp_[A-Za-z0-9_]{20,}/,
  /github_pat_[A-Za-z0-9_]{20,}/,
  /AKIA[0-9A-Z]{16}/,
  /-----BEGIN OPENSSH PRIVATE KEY-----/
]

scan_files.each do |relative|
  text = read_text(relative)
  credential_markers.each do |pattern|
    assert(!text.match?(pattern), "credential/private-key marker found in #{relative}: #{pattern.inspect}")
  end
end

puts JSON.pretty_generate({
  schema: "adl.corporate.corp_c.validation.v1",
  issue: 497,
  result: "pass",
  validated_files: required_files,
  prerequisite_issues: expected_prereqs.keys.sort,
  deferred_external_actions: packet.fetch("deferred_actions").size,
  external_mutations_performed: false
})
