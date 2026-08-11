#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
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

def fail!(message)
  warn "history proof invalid: #{message}"
  exit 1
end

options = {}
OptionParser.new do |parser|
  parser.on("--manifest PATH") { |path| options[:manifest] = path }
  parser.on("--results PATH") { |path| options[:results] = path }
end.parse!

fail!("--manifest is required") unless options[:manifest]
fail!("--results is required") unless options[:results]

begin
  manifest = JSON.parse(File.read(options[:manifest]))
  results = JSON.parse(File.read(options[:results]))
rescue Errno::ENOENT => e
  fail!("required input is missing: #{e.message}")
rescue JSON::ParserError => e
  fail!("input is not valid JSON: #{e.message}")
end

fail!("manifest schema mismatch") unless manifest["schema"] == "adl.conversation_history.case_manifest.v1"
fail!("manifest issue mismatch") unless manifest["issue"] == 114
fail!("manifest must contain exactly the canonical 42 ordered names") unless manifest["cases"] == EXPECTED_CASES
fail!("results schema mismatch") unless results["schema"] == "adl.conversation_history.case_results.v1"
fail!("results issue mismatch") unless results["issue"] == 114

cases = results["cases"]
fail!("results cases must be an array") unless cases.is_a?(Array)
fail!("results must contain exactly 42 cases") unless cases.length == EXPECTED_CASES.length
fail!("each case must contain only name and status") unless cases.all? { |entry| entry.is_a?(Hash) && entry.keys.sort == %w[name status] }

names = cases.map { |entry| entry["name"] }
fail!("case names contain duplicates") unless names.uniq.length == names.length
fail!("case names are missing, extra, renamed, or reordered") unless names == EXPECTED_CASES

nonpassing = cases.reject { |entry| entry["status"] == "passed" }
fail!("every canonical case must pass") unless nonpassing.empty?

puts JSON.generate(
  schema: "adl.conversation_history.denominator_validation.v1",
  issue: 114,
  case_count: EXPECTED_CASES.length,
  outcome: "passed"
)
