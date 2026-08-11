#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "rbconfig"

require_relative "../5862/proof-receipt-contract"

EXPECTED_NEGATIVE_CASES = {
  "fence_without_old_holder_activation_proof" => "fenced",
  "revoke_without_old_holder_activation_proof" => "fenced",
  "fence_same_epoch" => "denied",
  "fence_epoch_gap" => "denied",
  "fence_uncommitted_next_epoch" => "denied",
  "stale_authority_membership" => "denied",
  "no_current_authority_membership" => "denied",
  "unauthorized_operation" => "denied",
  "fenced_mutation" => "denied",
  "replay_receipt_mismatch" => "denied",
  "atomic_receipt_failure" => "fail_closed",
  "restart_floor_retained" => "fenced",
  "rollback_below_floor" => "denied",
  "unsafe_state_path" => "denied",
  "symlink_state_path" => "denied",
  "state_capacity" => "denied"
}.freeze
ISSUE_EVIDENCE_PREFIX = ".csdlc/evidence/5870/"
MACHINE_MARKER = "ADL_ISSUE_5870_NEGATIVE_CASE_V1 "
PRODUCER_PATH = ".csdlc/evidence/5870/derive-negative-cases.rb"
CLIPPY = ["cargo", "clippy", "--manifest-path", "adl-runtime/Cargo.toml", "--test", "distributed_fencing", "--", "-D", "warnings"].freeze
REPO_ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path

def fail_receipt(message)
  abort "issue 5870 receipt: #{message}"
end

def issue_evidence_file(path, digest, label)
  fail_receipt("#{label} path is not normalized") unless path.is_a?(String) &&
    Pathname.new(path).cleanpath.to_s == path
  fail_receipt("#{label} escapes issue evidence") unless path.start_with?(ISSUE_EVIDENCE_PREFIX)

  current = REPO_ROOT
  components = path.split("/")
  components.each_with_index do |component, index|
    current = current.join(component)
    metadata = File.lstat(current)
    fail_receipt("#{label} path contains a symlink component") if metadata.symlink?
    if index < components.length - 1 && !metadata.directory?
      fail_receipt("#{label} path contains a non-directory ancestor")
    end
  rescue Errno::ENOENT
    fail_receipt("#{label} missing")
  end

  metadata = File.lstat(current)
  fail_receipt("#{label} must be an ordinary file") unless metadata.file? && !metadata.symlink?
  unless digest.nil?
    fail_receipt("#{label} digest malformed") unless digest.to_s.match?(/\A[0-9a-f]{64}\z/)
    fail_receipt("#{label} digest mismatch") unless Digest::SHA256.file(current).hexdigest == digest
  end
  current
end

evidence_path = ARGV.fetch(0, ".csdlc/evidence/5870/execution-proof.json")
evidence_file = issue_evidence_file(evidence_path, nil, "execution proof")
proof = JSON.parse(File.read(evidence_file))
fail_receipt("execution proof issue mismatch") unless proof["issue"] == 5870

outer_cases = Array(proof["negative_cases"])
outer_map = outer_cases.to_h { |entry| [entry["case"], entry["result"]] }
fail_receipt("outer negative-case denominator, names, or expected results mismatch") unless
  outer_cases.length == EXPECTED_NEGATIVE_CASES.length &&
  outer_map.length == EXPECTED_NEGATIVE_CASES.length &&
  outer_map == EXPECTED_NEGATIVE_CASES

machine_paths = outer_cases.map { |entry| entry["evidence_path"] }.uniq
machine_digests = outer_cases.map { |entry| entry["evidence_sha256"] }.uniq
fail_receipt("outer cases do not bind one machine artifact") unless
  machine_paths.length == 1 && machine_digests.length == 1
machine_path = machine_paths.fetch(0)
issue_evidence_file(machine_path, machine_digests.fetch(0), "machine negative cases")
machine = JSON.parse(File.read(machine_path))
fail_receipt("machine evidence schema mismatch") unless
  machine["schema"] == "adl.wp04.negative_cases.machine.v1"
fail_receipt("machine evidence issue mismatch") unless machine["issue"] == 5870
fail_receipt("machine evidence source mismatch") unless
  machine["source_revision"] == proof["source_revision"]

fail_receipt("producer path mismatch") unless machine["producer_path"] == PRODUCER_PATH
issue_evidence_file(PRODUCER_PATH, machine["producer_sha256"], "machine producer")
stdout_path = machine["stdout_path"]
stderr_path = machine["stderr_path"]
issue_evidence_file(stdout_path, machine["stdout_sha256"], "machine stdout")
issue_evidence_file(stderr_path, machine["stderr_sha256"], "machine stderr")

observed = File.readlines(stdout_path, chomp: true).each_with_object([]) do |line, entries|
  next unless line.start_with?(MACHINE_MARKER)

  payload = JSON.parse(line.delete_prefix(MACHINE_MARKER))
  entries << {
    "case" => payload.fetch("case"),
    "result" => payload.fetch("result"),
    "observed_line_sha256" => Digest::SHA256.hexdigest(line)
  }
end
observed_map = observed.to_h { |entry| [entry["case"], entry["result"]] }
fail_receipt("executed negative-case denominator, names, or results mismatch") unless
  observed.length == EXPECTED_NEGATIVE_CASES.length &&
  observed_map.length == EXPECTED_NEGATIVE_CASES.length &&
  observed_map == EXPECTED_NEGATIVE_CASES
fail_receipt("machine case records do not match executed markers") unless
  Array(machine["cases"]) == EXPECTED_NEGATIVE_CASES.keys.map { |name|
    observed.find { |entry| entry["case"] == name }
  }

producer_stdout, producer_stderr, producer_status = Open3.capture3(
  RbConfig.ruby,
  PRODUCER_PATH,
  "verify",
  machine_path,
  proof.fetch("source_revision"),
  chdir: REPO_ROOT.to_s
)
fail_receipt("machine producer verification failed: #{producer_stderr.strip}") unless
  producer_status.success? && producer_stdout.start_with?("PASS:")

ARGV[0] = evidence_path
Wp04ProofReceiptContract.validate(
  issue: 5870,
  wp: "WP-04.08",
  paths: ["adl-runtime/src/distributed/fencing.rs", "adl-runtime/tests/distributed_fencing.rs"],
  test: "distributed_fencing",
  platforms: [],
  required_commands: [CLIPPY]
)
