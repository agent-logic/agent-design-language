#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ISSUES = [5340, 5341].freeze
EXPECTED_REPOSITORY = "danielbaustin/agent-design-language"
ISSUE = 5349
PREPARATION_PATHS = [
  ".csdlc/issues/5349",
  ".csdlc/locks/5349.lock",
  ".csdlc/prepared/issues/5349",
  ".csdlc/evidence/5349"
].freeze
FUTURE_PRODUCT_PATH = "adl-v2/crates/adl-adapters"

def git(*argv)
  stdout, stderr, status = Open3.capture3("git", *argv)
  [stdout.strip, stderr.strip, status]
end

def paths_overlap?(left, right)
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end

common_dir_text, common_dir_error, common_dir_status = git(
  "rev-parse", "--path-format=absolute", "--git-common-dir"
)
unless common_dir_status.success?
  warn common_dir_error
  exit 3
end

common_dir = Pathname(common_dir_text)
results = ISSUES.map do |issue|
  receipt_path = common_dir.join("csdlc-v2", "closeout", "#{issue}.json")
  result = {
    "issue" => issue,
    "receipt" => receipt_path.to_s,
    "github_merged" => false,
    "typed_closed_out" => false,
    "receipt_retained" => false,
    "merged_sha_ancestral" => false,
    "reasons" => []
  }

  unless receipt_path.file?
    result["reasons"] << "missing_terminal_receipt"
    next result
  end

  begin
    receipt = JSON.parse(receipt_path.read)
  rescue JSON::ParserError => error
    result["reasons"] << "malformed_terminal_receipt:#{error.message}"
    next result
  end

  record = receipt.fetch("record", {})
  terminal = record.fetch("terminal", {})
  result["receipt_retained"] =
    receipt["schema"] == "csdlc.terminal_receipt.v1" &&
    receipt["issue"] == issue &&
    receipt["repository"] == EXPECTED_REPOSITORY &&
    receipt["receipt_ref"] == "csdlc-v2/closeout/#{issue}.json"
  result["typed_closed_out"] = record["phase"] == "closed_out"
  result["github_merged"] =
    terminal["disposition"] == "merged" &&
    terminal["observed_state"] == "merged"

  sha = terminal["observed_sha"]
  if sha.is_a?(String) && sha.match?(/\A[0-9a-f]{40}\z/)
    _stdout, _stderr, ancestry_status = git(
      "merge-base", "--is-ancestor", sha, "origin/main"
    )
    result["merged_sha"] = sha
    result["merged_sha_ancestral"] = ancestry_status.success?
  else
    result["reasons"] << "missing_or_invalid_merged_sha"
  end

  result["reasons"] << "receipt_identity_mismatch" unless result["receipt_retained"]
  result["reasons"] << "typed_phase_not_closed_out" unless result["typed_closed_out"]
  result["reasons"] << "github_terminal_state_not_merged" unless result["github_merged"]
  result["reasons"] << "merged_sha_not_ancestral_to_origin_main" unless result["merged_sha_ancestral"]
  result
end

ready = results.all? do |result|
  result.values_at(
    "github_merged",
    "typed_closed_out",
    "receipt_retained",
    "merged_sha_ancestral"
  ).all?
end

claim_collisions = Dir.glob(".csdlc/issues/*/index.json").sort.each_with_object([]) do |path, collisions|
  record = JSON.parse(File.read(path))
  claim = record["claim"]
  next unless claim.is_a?(Hash) && record["issue"] != ISSUE

  overlaps = Array(claim["protected_paths"]).product(
    PREPARATION_PATHS + [FUTURE_PRODUCT_PATH]
  ).select { |claimed, target| paths_overlap?(claimed, target) }
  next if overlaps.empty?

  collisions << {
    "issue" => record["issue"],
    "claim_id" => claim["id"],
    "overlaps" => overlaps
  }
rescue JSON::ParserError => error
  collisions << {
    "issue" => File.basename(File.dirname(path)),
    "claim_id" => nil,
    "overlaps" => [],
    "error" => "malformed_issue_record:#{error.message}"
  }
end

ready &&= claim_collisions.empty?

puts JSON.pretty_generate(
  "schema" => "adl.csdlc.issue_5349_dependency_gate.v1",
  "status" => ready ? "ready" : "waiting",
  "origin_main" => git("rev-parse", "origin/main").first,
  "snapshot_boundary" => "local fetched origin/main and tracked typed issue records; refresh read-only GitHub truth before product claim amendment",
  "claim_collisions" => claim_collisions,
  "results" => results
)
exit(ready ? 0 : 2)
