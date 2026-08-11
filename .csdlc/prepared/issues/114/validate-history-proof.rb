#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "optparse"

EXPECTED_CASES = %w[
  append_first_turn
  append_ordered_turn
  outcome_monotonic
  restart_continuity
  browser_reconnect_page
  exact_duplicate_cached
  conflicting_duplicate_denied
  sequence_gap_denied
  reorder_denied
  terminal_rewrite_denied
  unauthorized_read_denied
  revoked_read_denied
  expired_identity_denied
  policy_epoch_drift_denied
  cross_polis_denied
  cross_principal_cursor_denied
  tampered_cursor_denied
  stale_cursor_denied
  stable_snapshot_paging
  bounded_search
  search_private_state_absent
  bounded_export
  export_reauthorization_denied
  retention_expiry
  deletion_tombstone
  deletion_exact_retry
  deletion_residue_absent
  partial_write_recovery
  reply_loss_cached
  disk_full_no_false_success
  read_only_no_effect
  lock_contention_bounded
  corrupt_record_quarantined
  receipt_chain_break_quarantined
  watermark_drift_quarantined
  unknown_newer_schema_denied
  unsupported_older_schema_denied
  migration_resume_before_publish
  migration_reopen_after_publish
  lossy_migration_denied
  rollback_after_new_write_denied
  forbidden_field_redaction
].freeze

RECEIPT_KEYS = {
  "rust_store" => %w[candidate_sha command evidence_ref kind status test_count],
  "runtime_api" => %w[candidate_sha command evidence_ref kind status test_count],
  "browser" => %w[browser_check_count candidate_sha command evidence_ref kind runtime_backed status],
  "strict_clippy" => %w[candidate_sha command deny_warnings evidence_ref kind status],
  "diff_hygiene" => %w[candidate_sha clean command evidence_ref kind status],
  "independent_review" => %w[candidate_sha evidence_ref findings_status independent kind reviewer status]
}.freeze
EXPECTED_RECEIPTS = RECEIPT_KEYS.keys.freeze
EXPECTED_ACCEPTANCE = (1..9).map { |number| "AC-#{number}" }.freeze
REQUIRED_COMMANDS = {
  "rust_store" => %w[cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history --no-tests=fail],
  "runtime_api" => %w[cargo nextest run --locked --manifest-path adl/Cargo.toml --test conversation_history_runtime_api --no-tests=fail],
  "strict_clippy" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history -- -D warnings],
  "diff_hygiene" => %w[git diff --check]
}.freeze
BROWSER_COMMAND_PREFIX = %w[node adl/tools/validate_v092_html_observatory_history.mjs].freeze
SHA_PATTERN = /\A[0-9a-f]{40}\z/

def fail!(message)
  warn "history proof invalid: #{message}"
  exit 1
end

def read_json(path)
  JSON.parse(File.read(path))
rescue Errno::ENOENT => e
  fail!("required input is missing: #{e.message}")
rescue JSON::ParserError => e
  fail!("input is not valid JSON: #{e.message}")
end

def nonempty_string?(value)
  value.is_a?(String) && !value.empty?
end

def valid_command?(value)
  value.is_a?(Array) && !value.empty? && value.all? { |argument| nonempty_string?(argument) }
end

options = { repo: "." }
OptionParser.new do |parser|
  parser.on("--manifest PATH") { |path| options[:manifest] = path }
  parser.on("--schema PATH") { |path| options[:schema] = path }
  parser.on("--results PATH") { |path| options[:results] = path }
  parser.on("--candidate-sha SHA") { |sha| options[:candidate_sha] = sha }
  parser.on("--repo PATH") { |path| options[:repo] = path }
end.parse!

%i[manifest schema results candidate_sha].each do |name|
  fail!("--#{name.to_s.tr('_', '-')} is required") unless options[name]
end

head_sha, git_error, git_status = Open3.capture3("git", "-C", options[:repo], "rev-parse", "HEAD")
fail!("cannot resolve repository HEAD: #{git_error.strip}") unless git_status.success?
head_sha = head_sha.strip

candidate_selector = options[:candidate_sha]
candidate_sha = candidate_selector == "HEAD" ? head_sha : candidate_selector
fail!("candidate SHA must be HEAD or an exact lowercase 40-hex commit") unless SHA_PATTERN.match?(candidate_sha)
fail!("candidate SHA is stale or does not match repository HEAD") unless head_sha == candidate_sha

manifest = read_json(options[:manifest])
schema = read_json(options[:schema])
results = read_json(options[:results])

fail!("manifest must be an object") unless manifest.is_a?(Hash)
fail!("manifest contains unknown or missing fields") unless manifest.keys.sort == %w[cases issue schema]
fail!("manifest schema mismatch") unless manifest["schema"] == "adl.conversation_history.case_manifest.v1"
fail!("manifest issue mismatch") unless manifest["issue"] == 114
fail!("manifest must contain exactly the canonical 42 ordered names") unless manifest["cases"] == EXPECTED_CASES

fail!("receipt schema must be an object") unless schema.is_a?(Hash)
fail!("receipt schema identity mismatch") unless schema["$id"] == "adl.conversation_history.proof_receipts.v2"
fail!("receipt schema issue mismatch") unless schema["x-adl-issue"] == 114
fail!("receipt schema canonical case count mismatch") unless schema["x-adl-canonical-case-count"] == 42
fail!("receipt schema receipt order mismatch") unless schema["x-adl-receipt-order"] == EXPECTED_RECEIPTS
fail!("receipt schema acceptance coverage mismatch") unless schema["x-adl-acceptance-coverage"] == EXPECTED_ACCEPTANCE
fail!("receipt schema required commands mismatch") unless schema["x-adl-required-commands"] == REQUIRED_COMMANDS
fail!("receipt schema browser command prefix mismatch") unless schema["x-adl-browser-command-prefix"] == BROWSER_COMMAND_PREFIX

fail!("results must be an object") unless results.is_a?(Hash)
fail!("results contain unknown or missing fields") unless results.keys.sort == %w[candidate_sha cases issue receipts schema]
fail!("results schema mismatch") unless results["schema"] == "adl.conversation_history.proof_receipts.v2"
fail!("results issue mismatch") unless results["issue"] == 114
fail!("results candidate SHA mismatch") unless results["candidate_sha"] == candidate_sha

cases = results["cases"]
fail!("results cases must be an array") unless cases.is_a?(Array)
fail!("results must contain exactly 42 cases") unless cases.length == EXPECTED_CASES.length
fail!("each case must contain only name and status") unless cases.all? { |entry| entry.is_a?(Hash) && entry.keys.sort == %w[name status] }

names = cases.map { |entry| entry["name"] }
fail!("case names contain duplicates") unless names.uniq.length == names.length
fail!("case names are missing, extra, renamed, or reordered") unless names == EXPECTED_CASES
fail!("every canonical case must pass") unless cases.all? { |entry| entry["status"] == "passed" }

receipts = results["receipts"]
fail!("receipts must be an array") unless receipts.is_a?(Array)
fail!("results must contain exactly six receipts") unless receipts.length == EXPECTED_RECEIPTS.length
fail!("each receipt must be an object") unless receipts.all? { |receipt| receipt.is_a?(Hash) }

kinds = receipts.map { |receipt| receipt["kind"] }
fail!("receipt kinds contain duplicates") unless kinds.uniq.length == kinds.length
fail!("receipts are missing, extra, renamed, or reordered") unless kinds == EXPECTED_RECEIPTS

receipts.each do |receipt|
  kind = receipt["kind"]
  fail!("#{kind} receipt is incomplete or contains unknown fields") unless receipt.keys.sort == RECEIPT_KEYS.fetch(kind)
  fail!("#{kind} receipt candidate SHA mismatch") unless receipt["candidate_sha"] == candidate_sha
  fail!("#{kind} receipt status must be passed") unless receipt["status"] == "passed"
  fail!("#{kind} receipt evidence_ref must be nonempty") unless nonempty_string?(receipt["evidence_ref"])
end

evidence_refs = receipts.map { |receipt| receipt["evidence_ref"] }
fail!("receipt evidence refs must be unique") unless evidence_refs.uniq.length == evidence_refs.length
fail!("receipt evidence refs must stay under .csdlc/evidence/114") unless evidence_refs.all? do |path|
  path.start_with?(".csdlc/evidence/114/") && !path.split("/").include?("..")
end

%w[rust_store runtime_api].each do |kind|
  receipt = receipts.fetch(EXPECTED_RECEIPTS.index(kind))
  fail!("#{kind} receipt command mismatch") unless receipt["command"] == REQUIRED_COMMANDS.fetch(kind)
  fail!("#{kind} receipt must prove a nonzero test count") unless receipt["test_count"].is_a?(Integer) && receipt["test_count"].positive?
end

browser = receipts.fetch(EXPECTED_RECEIPTS.index("browser"))
fail!("browser receipt command mismatch") unless valid_command?(browser["command"]) && browser["command"].first(2) == BROWSER_COMMAND_PREFIX
fail!("browser receipt must prove real Runtime-backed execution") unless browser["runtime_backed"] == true
fail!("browser receipt must prove a nonzero check count") unless browser["browser_check_count"].is_a?(Integer) && browser["browser_check_count"].positive?

clippy = receipts.fetch(EXPECTED_RECEIPTS.index("strict_clippy"))
fail!("strict_clippy receipt command mismatch") unless clippy["command"] == REQUIRED_COMMANDS.fetch("strict_clippy")
fail!("strict_clippy receipt must enforce denied warnings") unless clippy["deny_warnings"] == true

diff = receipts.fetch(EXPECTED_RECEIPTS.index("diff_hygiene"))
fail!("diff_hygiene receipt command mismatch") unless diff["command"] == REQUIRED_COMMANDS.fetch("diff_hygiene")
fail!("diff_hygiene receipt must report a clean diff") unless diff["clean"] == true

review = receipts.fetch(EXPECTED_RECEIPTS.index("independent_review"))
fail!("independent_review receipt reviewer must be nonempty") unless nonempty_string?(review["reviewer"])
fail!("independent_review receipt must be independent") unless review["independent"] == true
fail!("independent_review receipt findings must be resolved") unless review["findings_status"] == "resolved"

puts JSON.generate(
  schema: "adl.conversation_history.denominator_validation.v2",
  issue: 114,
  candidate_sha: candidate_sha,
  case_count: EXPECTED_CASES.length,
  receipt_count: EXPECTED_RECEIPTS.length,
  acceptance_coverage: EXPECTED_ACCEPTANCE,
  outcome: "passed"
)
