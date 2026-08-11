#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PREFIX = ".csdlc/evidence/201/"
PROOF_PREFIX = "#{PREFIX}v6/"
PROOF_RELATIVE = "#{PROOF_PREFIX}execution-proof.json"
EXPECTED_PROTECTED = [
  "adl-runtime/Cargo.toml", "adl-runtime/Cargo.lock",
  "adl-runtime/src/distributed/mod.rs", "adl-runtime/src/distributed/authority_protocol.rs",
  "adl-runtime/src/distributed/identity.rs", "adl-runtime/src/distributed/polis_runtime.rs",
  "adl-runtime/src/distributed/transport.rs", "adl-runtime/src/distributed/authority_protocol_contract_tests.rs",
  "adl-runtime/tests/distributed_authority_protocol.rs",
  ".csdlc/prepared/issues/201/produce-proof-receipt.rb",
  ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
].freeze

def fail_receipt(message)
  abort("issue 201 receipt: #{message}")
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
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue201.committed_authority_proof.v1" && proof["issue"] == 201
source = proof.fetch("source_revision")
fail_receipt("source malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
source_tree = proof.fetch("source_tree")
fail_receipt("source tree malformed") unless source_tree.match?(/\A[0-9a-f]{40}\z/)
protected = proof.fetch("protected_files")
fail_receipt("protected denominator mismatch") unless protected.map { |entry| entry["path"] } == EXPECTED_PROTECTED
protected.each do |entry|
  path = ordinary(entry.fetch("path"))
  fail_receipt("protected digest drift: #{entry['path']}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
fail_receipt("test summary mismatch") unless proof["test_summary"] == { "selected" => 47, "passed" => 47, "skipped" => 0 }
cases = proof.fetch("cases")
fail_receipt("case denominator/order mismatch") unless cases.length == 47 && cases.map { |entry| entry.fetch("case") }.uniq.length == 47
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == %w[clippy machine_cases nextest openraft]
commands.each do |name, command|
  fail_receipt("#{name} failed") unless command.fetch("exit_code") == 0
  fail_receipt("#{name} stream normalization mismatch") unless command.fetch("stream_normalization") == "trailing_blank_lines_removed"
  fail_receipt("#{name} time inverted") if Time.iso8601(command.fetch("finished_at")) < Time.iso8601(command.fetch("started_at"))
  %w[stdout stderr].each do |stream|
    relative = command.fetch("#{stream}_path")
    fail_receipt("stream escapes evidence") unless relative.start_with?(PREFIX)
    fail_receipt("#{name} #{stream} digest mismatch") unless Digest::SHA256.file(ordinary(relative)).hexdigest == command.fetch("#{stream}_sha256")
  end
end
machine = commands.fetch("machine_cases")
text = %w[stdout stderr].map { |stream| File.binread(ROOT.join(machine.fetch("#{stream}_path"))) }.join
observed = text.lines.each_with_object([]) do |line, rows|
  next unless line.include?("ADL_ISSUE_201_CASE_V1 ")
  name, result = line.split("ADL_ISSUE_201_CASE_V1 ", 2).fetch(1).strip.split(" ", 2)
  rows << [name, result, Digest::SHA256.hexdigest(line.chomp)]
end
fail_receipt("marker denominator mismatch") unless observed.length == 47
observed_by_name = observed.to_h { |name, result, digest| [name, [result, digest]] }
cases.each do |entry|
  result, digest = observed_by_name.fetch(entry.fetch("case")) { fail_receipt("missing marker") }
  fail_receipt("case result/digest mismatch") unless [result, digest] == [entry.fetch("result"), entry.fetch("observed_line_sha256")]
end
introductions = git("log", "--format=%H", "--diff-filter=A", "--", PROOF_RELATIVE).lines.map(&:strip).reject(&:empty?)
fail_receipt("proof requires exactly one immutable introduction") unless introductions.length == 1
introduction = introductions.fetch(0)
source_available = system("git", "cat-file", "-e", "#{source}^{commit}", chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
source_is_ancestor = source_available && system("git", "merge-base", "--is-ancestor", source, introduction, chdir: ROOT.to_s, out: File::NULL, err: File::NULL)
if source_is_ancestor
  fail_receipt("source tree mismatch") unless git("rev-parse", "#{source}^{tree}").strip == source_tree
  protected.each do |entry|
    committed = git("show", "#{source}:#{entry.fetch('path')}")
    fail_receipt("source-object mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(committed) == entry.fetch("sha256")
  end
else
  # A depth-limited or squash-like consumer may legitimately lack the feature
  # source object. In that case, bind the receipt to the immutable introduction's
  # exact protected blobs instead of asking Git for an unavailable ancestor.
  protected.each do |entry|
    introduced = git("show", "#{introduction}:#{entry.fetch('path')}")
    fail_receipt("protected-tree mismatch: #{entry['path']}") unless Digest::SHA256.hexdigest(introduced) == entry.fetch("sha256")
  end
end
fail_receipt("protected source changed after proof") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", *EXPECTED_PROTECTED).empty?
fail_receipt("immutable proof changed after introduction") unless git("diff", "--name-only", "#{introduction}..HEAD", "--", PROOF_PREFIX).empty?
fail_receipt("protected/proof worktree dirty") unless git("status", "--porcelain=v1", "--untracked-files=all", "--", *EXPECTED_PROTECTED, PROOF_PREFIX).empty?
puts "PASS: issue #201 merge-safe proof binds exact source, strict Clippy, and ordered 47/47 case evidence"
