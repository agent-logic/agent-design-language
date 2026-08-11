#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/202/v9/"
PROOF_RELATIVE = "#{PREFIX}execution-proof.json"
EXPECTED_PROTECTED = %w[
  adl-runtime/src/distributed/mod.rs adl-runtime/src/distributed/authority_protocol.rs
  adl-runtime/src/distributed/transport.rs adl-runtime/src/distributed/transport/core.rs
  adl-runtime/src/distributed/transport/root.rs
  adl-runtime/src/distributed/transport/governed/learner_transport.rs
  adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
  adl-runtime/src/distributed/transport/governed/polis_runtime.rs
  adl-runtime/tests/distributed_authorized_learner_transport.rs
  adl-runtime/tests/distributed_transport.rs adl-runtime/tests/distributed_discovery.rs
  adl-runtime/tests/distributed_runtime_transport.rs
  adl/tools/check_coverage_impact.sh adl/tools/test_check_coverage_impact.sh
  .csdlc/prepared/issues/202/produce-proof-receipt.rb .csdlc/prepared/issues/202/validate-proof-receipt.rb
].freeze
EXPECTED_COMMANDS = {
  "private_cases" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests -- --nocapture --test-threads=1],
  "public_cases" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- --test-threads=1],
  "transport_compile" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_transport --no-run],
  "discovery_compile" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_discovery --no-run],
  "runtime_compile" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport --no-run],
  "runtime_route_rotation" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport route_replacement_retries_exact_sequence_after_peer_restart_and_certificate_rotation -- --exact --nocapture],
  "coverage_contract" => %w[bash adl/tools/test_check_coverage_impact.sh],
  "clippy_lib" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings],
  "clippy_public" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- -D warnings]
}.freeze

def fail_receipt(message)
  abort("issue 202 receipt: #{message}")
end
def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git failed: #{err.strip}") unless status.success?
  out
end

proof_path = ROOT.join(PROOF_RELATIVE)
fail_receipt("missing or unsafe proof") unless proof_path.file? && !proof_path.symlink?
proof = JSON.parse(File.binread(proof_path))
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue202.authorized_learner_transport_proof.v9" && proof["issue"] == 202
source = proof.fetch("source_revision")
fail_receipt("source malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
fail_receipt("proof is not bound to exact current origin/main") unless git("rev-parse", "refs/remotes/origin/main").strip == proof.fetch("required_main_ancestor")
fail_receipt("ancestry missing") unless system("git", "merge-base", "--is-ancestor", proof.fetch("required_main_ancestor"), source, chdir: ROOT.to_s)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  path = ROOT.join(entry.fetch("path"))
  fail_receipt("unsafe protected path") unless path.file? && !path.symlink?
  fail_receipt("protected digest drift") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
fail_receipt("test summary mismatch") unless proof.fetch("test_summary") == { "semantic_cases" => 36, "private_runner_selected" => 42, "private_runner_passed" => 42, "public_selected" => 13, "public_passed" => 13, "named_subassertions" => 31 }
fail_receipt("runner marker denominator mismatch") unless proof.fetch("cases").length == 42 && proof.fetch("cases").map { |entry| entry["case"] }.uniq.length == 42 && proof.fetch("cases").all? { |entry| entry["result"] == "passed" }
fail_receipt("subassertion denominator mismatch") unless proof.fetch("subassertions").length == 31 && proof.fetch("subassertions").map { |entry| [entry["case"], entry["assertion"]] }.uniq.length == 31
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == EXPECTED_COMMANDS.keys.sort
commands.each do |name, command|
  fail_receipt("command argv mismatch #{name}") unless command.fetch("argv") == EXPECTED_COMMANDS.fetch(name)
  fail_receipt("command failed") unless command.fetch("exit_code") == 0
  %w[stdout stderr].each do |stream|
    relative = command.fetch("#{stream}_path")
    fail_receipt("stream escapes evidence") unless relative.start_with?(PREFIX)
    fail_receipt("stream digest mismatch") unless Digest::SHA256.file(ROOT.join(relative)).hexdigest == command.fetch("#{stream}_sha256")
  end
end
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires immutable introduction") if introductions.empty?
introduction = introductions.first
fail_receipt("source not ancestral") unless system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s)
fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == proof.fetch("source_tree")
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PREFIX).empty?
puts "PASS: issue #202 v9 proof binds exact command argv, 36 semantic / 42 runner + 13 public / 31 assertions, three integration compiles, coverage routing, route rotation, strict library/public Clippy, and exact current origin/main ancestry"
