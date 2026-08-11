#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/200/v5/"
PROOF_RELATIVE = "#{PREFIX}execution-proof.json"
EXPECTED_PROTECTED = [
  "adl-runtime/src/distributed/mod.rs",
  "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/authority_reconciliation.rs",
  "adl-runtime/src/distributed/authority_reconciliation/tests.rs",
  "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/tests/distributed_authority_reconciliation.rs",
  ".csdlc/prepared/issues/200/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/200/validate-proof-receipt.rb"
].freeze
PORTABLE_TEST_PATHS = [
  "adl-runtime/src/distributed/authority_reconciliation/tests.rs",
  "adl-runtime/tests/distributed_authority_reconciliation.rs"
].freeze
EXPECTED_CASES = %w[
  happy_single_step happy_multi_step exact_retry_cached_result pending_blocks_read
  pending_blocks_mutation published_permit_current missing_201_token
  public_token_forgery_denied legacy_command_denied wrong_domain wrong_polis wrong_node
  wrong_guardian wrong_boot wrong_protocol_instance wrong_membership wrong_operation_kind
  wrong_adapter_version wrong_time_digest conflicting_retry reordered_step duplicate_step
  missing_step forged_step_receipt crash_after_journal crash_each_step crash_after_result
  crash_before_checkpoint crash_after_checkpoint coherent_rollback capacity_n_plus_one_no_partial
  state_or_lock_symlink_rejected corrupt_journal_rejected noncanonical_state_rejected
  opened_handle_growth_rejected checkpoint_object_collision
].freeze
PASSED = %w[happy_single_step happy_multi_step exact_retry_cached_result published_permit_current].freeze
RECONCILED = %w[crash_after_journal crash_each_step crash_after_result crash_before_checkpoint crash_after_checkpoint].freeze
MARKER = "ADL_ISSUE_200_CASE_V1 "
ASSERTION_MARKER = "ADL_ISSUE_200_ASSERTION_V1 "
EXPECTED_ASSERTIONS = [
  %w[exact_retry_cached_result cached_result_no_reexecution],
  %w[exact_retry_cached_result conflicting_view_rejected],
  %w[exact_retry_cached_result corrupt_view_rejected],
  %w[published_permit_current current_read_valid],
  %w[published_permit_current current_mutation_valid],
  %w[published_permit_current read_escalation_denied],
  %w[published_permit_current wrong_lineage_denied],
  %w[published_permit_current wrong_mutation_action_denied],
  %w[published_permit_current retained_read_denied_after_pending],
  %w[published_permit_current retained_mutation_denied_after_pending],
  %w[crash_after_checkpoint missing_marker_and_view_retry],
  %w[crash_after_checkpoint committed_marker_missing_view_retry],
  %w[crash_after_checkpoint published_view_exact_retry]
].freeze

def fail_receipt(message)
  abort("issue 200 receipt: #{message}")
end

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git #{args.join(' ')} failed: #{err.strip}") unless status.success?
  out
end

def ordinary(relative)
  fail_receipt("unsafe path: #{relative}") if Pathname.new(relative).absolute? || Pathname.new(relative).cleanpath.to_s != relative
  current = ROOT
  relative.split("/").each_with_index do |part, index|
    current = current.join(part)
    metadata = File.lstat(current)
    fail_receipt("symlink path: #{relative}") if metadata.symlink?
    fail_receipt("non-directory ancestor: #{relative}") if index < relative.split("/").length - 1 && !metadata.directory?
  end
  fail_receipt("not ordinary file: #{relative}") unless current.file? && !current.symlink?
  current
rescue Errno::ENOENT
  fail_receipt("missing file: #{relative}")
end

proof = JSON.parse(File.binread(ordinary(PROOF_RELATIVE)))
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue200.authority_reconciliation_proof.v2" && proof["issue"] == 200
source = proof.fetch("source_revision")
source_tree = proof.fetch("source_tree")
fail_receipt("source malformed") unless source.match?(/\A[0-9a-f]{40}\z/) && source_tree.match?(/\A[0-9a-f]{40}\z/)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  fail_receipt("protected digest drift: #{entry['path']}") unless Digest::SHA256.file(ordinary(entry.fetch("path"))).hexdigest == entry.fetch("sha256")
end
PORTABLE_TEST_PATHS.each do |relative|
  fail_receipt("machine-local /private/tmp fixture remains: #{relative}") if File.binread(ordinary(relative)).include?("/private/tmp")
end
fail_receipt("test summary mismatch") unless proof["test_summary"] == { "selected" => 36, "passed" => 36, "skipped" => 0 }
fail_receipt("result summary mismatch") unless proof["result_summary"] == { "passed" => 4, "reconciled" => 5, "rejected" => 27 }
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == %w[clippy machine_cases nextest]
commands.each do |name, command|
  fail_receipt("#{name} failed") unless command.fetch("exit_code") == 0
  fail_receipt("#{name} time inverted") if Time.iso8601(command.fetch("finished_at")) < Time.iso8601(command.fetch("started_at"))
  %w[stdout stderr].each do |stream|
    relative = command.fetch("#{stream}_path")
    fail_receipt("stream escapes evidence") unless relative.start_with?(PREFIX)
    fail_receipt("#{name} #{stream} digest mismatch") unless Digest::SHA256.file(ordinary(relative)).hexdigest == command.fetch("#{stream}_sha256")
  end
end
machine = commands.fetch("machine_cases")
text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(machine.fetch("#{stream}_path"))) }.join
observed = text.lines.map do |line|
  next unless line.include?(MARKER)
  name, result = line.split(MARKER, 2).fetch(1).strip.split(" ", 2)
  [name, result, Digest::SHA256.hexdigest("#{MARKER}#{name} #{result}")]
end.compact
cases = proof.fetch("cases")
fail_receipt("case order mismatch") unless cases.map { |entry| entry["case"] } == EXPECTED_CASES
fail_receipt("marker denominator mismatch") unless observed.length == 36 && observed.map(&:first).uniq.length == 36
observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
EXPECTED_CASES.each_with_index do |name, index|
  result = PASSED.include?(name) ? "passed" : (RECONCILED.include?(name) ? "reconciled" : "rejected")
  digest = Digest::SHA256.hexdigest("#{MARKER}#{name} #{result}")
  fail_receipt("case substitution: #{name}") unless cases.fetch(index) == { "case" => name, "result" => result, "observed_line_sha256" => digest }
  fail_receipt("observed substitution: #{name}") unless observed_by_name[name] == [result, digest]
end
observed_assertions = text.lines.map do |line|
  next unless line.include?(ASSERTION_MARKER)
  case_name, assertion_name = line.split(ASSERTION_MARKER, 2).fetch(1).strip.split(" ", 2)
  key = [case_name, assertion_name]
  [key, Digest::SHA256.hexdigest("#{ASSERTION_MARKER}#{case_name} #{assertion_name}")]
end.compact
fail_receipt("subassertion summary mismatch") unless proof["subassertion_summary"] == {
  "selected" => EXPECTED_ASSERTIONS.length,
  "observed" => EXPECTED_ASSERTIONS.length
}
subassertions = proof.fetch("subassertions")
fail_receipt("subassertion order mismatch") unless subassertions.map { |entry| [entry["case"], entry["assertion"]] } == EXPECTED_ASSERTIONS
fail_receipt("subassertion denominator mismatch") unless observed_assertions.length == EXPECTED_ASSERTIONS.length && observed_assertions.map(&:first).uniq.length == EXPECTED_ASSERTIONS.length
observed_assertions_by_key = observed_assertions.to_h
EXPECTED_ASSERTIONS.each_with_index do |(case_name, assertion_name), index|
  digest = Digest::SHA256.hexdigest("#{ASSERTION_MARKER}#{case_name} #{assertion_name}")
  expected = { "case" => case_name, "assertion" => assertion_name, "observed_line_sha256" => digest }
  fail_receipt("subassertion substitution: #{case_name}/#{assertion_name}") unless subassertions.fetch(index) == expected
  fail_receipt("observed subassertion substitution: #{case_name}/#{assertion_name}") unless observed_assertions_by_key[[case_name, assertion_name]] == digest
end
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires immutable introduction") if introductions.empty?
introduction = introductions.first
fail_receipt("proof not introduced from absence") if system("git", "cat-file", "-e", "#{introduction}^:#{PROOF_RELATIVE}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
if system("git", "cat-file", "-e", "#{source}^{commit}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
  fail_receipt("source not ancestral") unless system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s)
  fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == source_tree
  protected.each do |entry|
    fail_receipt("source object mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(git("show", "#{source}:#{entry.fetch('path')}")) == entry.fetch("sha256")
  end
else
  protected.each do |entry|
    fail_receipt("introduced object mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(git("show", "#{introduction}:#{entry.fetch('path')}")) == entry.fetch("sha256")
  end
end
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed after introduction") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PREFIX).empty?
fail_receipt("protected/proof worktree dirty") unless git("status", "--porcelain=v1", "--untracked-files=all", "--", *EXPECTED_PROTECTED, PREFIX).empty?
puts "PASS: issue #200 merge-safe proof binds exact 36/36 cases, 13/13 subassertions, and strict Clippy"
