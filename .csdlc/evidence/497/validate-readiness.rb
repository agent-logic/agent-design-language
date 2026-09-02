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
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-identity-readback-redacted.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/account-authority-readback.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/external-action-classification.v1.json",
  "docs/milestones/v0.92.1/evidence/corporate/corp-c/control-plane-denominator.v1.json",
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
assert(aws["classification"] == "partial_readback", "AWS readback classification must remain partial until account-control rows pass")
assert(aws["evidence_source"] == "docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-identity-readback-redacted.v1.json", "AWS readback must bind to redacted STS receipt")
assert(aws["evidence_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/), "AWS readback must retain evidence sha256")

aws_identity = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-identity-readback-redacted.v1.json")
assert(aws_identity["schema"] == "adl.corporate.corp_c.aws_identity_readback_redacted.v1", "unexpected AWS identity receipt schema")
assert(aws_identity["profile"] == "agent-logic-admin", "AWS identity receipt profile mismatch")
assert(aws_identity["mutation"] == false, "AWS identity receipt must be non-mutating")
assert(aws_identity["credential_material_captured"] == false, "AWS identity receipt must not capture credential material")
assert(aws_identity["account_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/), "AWS identity receipt must retain account hash")
assert(aws_identity["account_id_redacted"] == true, "AWS account id must be redacted")
assert(aws_identity["arn_redacted"] == true, "AWS arn must be redacted")
assert(aws_identity.fetch("retained_hash_matches").any? { |row| row["field"] == "account_id_sha256" && row["sha256"].to_s.match?(/\A[0-9a-f]{64}\z/) }, "AWS identity receipt must bind to retained hashed evidence")

classification = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/external-action-classification.v1.json")
assert(classification["schema"] == "adl.corporate.corp_c.external_action_classification.v1", "unexpected classification schema")
assert(classification["issue"] == 497, "classification issue mismatch")
assert(classification["authorized_actions"] == [], "authorized actions must remain empty without explicit operator authorization")
assert(!classification.fetch("blocked_actions").empty?, "blocked actions must be explicit until live #497 denominator passes")
assert(classification.fetch("rows").all? { |row| row["mutation_performed_by_497"] == false }, "no external mutation may be recorded for #497")
assert(classification.fetch("rows").any? { |row| row["classification"].to_s.start_with?("blocked_") }, "expected blocked external actions")

denominator = read_json("docs/milestones/v0.92.1/evidence/corporate/corp-c/control-plane-denominator.v1.json")
assert(denominator["schema"] == "adl.corporate.corp_c.control_plane_denominator.v1", "unexpected denominator schema")
assert(denominator["status"] == "blocked_missing_required_readbacks", "denominator must remain blocked until all required #497 rows pass")
assert(denominator.fetch("live_issue_acceptance").size == 4, "denominator must preserve all four live #497 acceptance criteria")
assert(denominator.fetch("rows").all? { |row| row["required_by_497"] == true }, "all denominator rows must be required by #497")
assert(denominator.fetch("rows").any? { |row| row["classification"].to_s.start_with?("blocked_") }, "denominator must identify blocking rows")
assert(denominator.dig("mutation_authority", "new_mutation_performed_by_497") == false, "#497 must not record new mutation")
assert(denominator.dig("mutation_authority", "blocked_if_mutation_required") == true, "mutation-required rows must block without authority")

packet = read_json("docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json")
assert(packet["schema"] == "adl.corporate.operational_control_transfer_acceptance.v1", "unexpected packet schema")
assert(packet["issue"] == 497, "packet issue mismatch")
assert(packet["status"] == "blocked_missing_required_readbacks", "packet must not accept CORP-C with missing required readbacks")
assert(packet.dig("prerequisite_gate", "status") == "pass", "packet prerequisite gate not pass")
assert(packet["authorized_actions"] == [], "packet authorized actions must remain empty")
assert(!packet.fetch("blocked_actions").empty?, "packet must record blocking actions")
assert(packet.dig("mutation_authority", "new_mutation_performed_by_497") == false, "packet must not claim new mutation")
assert(packet.dig("mutation_authority", "blocked_if_mutation_required") == true, "packet must block if mutation authority is required")

acceptance_statuses = packet.fetch("acceptance").to_h { |row| [row.fetch("id"), row.fetch("status")] }
assert(acceptance_statuses["AC-1"] == "blocked", "AC-1 status must be blocked")
assert(acceptance_statuses["AC-2"] == "partial", "AC-2 status must be partial")
assert(acceptance_statuses["AC-3"] == "blocked", "AC-3 status must be blocked")
assert(acceptance_statuses["AC-4"] == "blocked", "AC-4 status must be blocked")

markdown = read_text("docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.md")
[
  "CORP-C is blocked, not accepted.",
  "This PR must not close #497 while they remain missing.",
  "No production/provider mutation",
  "This packet does not mean:",
  "#497 is ready to close",
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
  blocked_actions: packet.fetch("blocked_actions").size,
  issue_ready_to_close: false,
  external_mutations_performed: false
})
