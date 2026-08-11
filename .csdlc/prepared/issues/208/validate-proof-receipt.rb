#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PROOF_RELATIVE = ".csdlc/evidence/208/v1/execution-proof.json"
MAP_RELATIVE = ".csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json"
MAP_SHA256 = "9a6d7834557f626487aae3115464ee60f19b06609b7ea9e6a24399a60eec8745"

def fail_receipt(message)
  abort("issue 208 receipt: #{message}")
end
def git!(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  fail_receipt("git #{args.join(' ')} failed: #{err.strip}") unless status.success?
  out
end
def ordinary(relative)
  fail_receipt("unsafe path: #{relative}") if Pathname.new(relative).absolute? || Pathname.new(relative).cleanpath.to_s != relative
  path = ROOT.join(relative)
  fail_receipt("missing or symlink file: #{relative}") unless path.file? && !path.symlink?
  path
end

proof = JSON.parse(File.binread(ordinary(PROOF_RELATIVE)))
fail_receipt("schema/issue mismatch") unless proof["schema"] == "adl.issue208.guardian_kernel_continuity_proof.v1" && proof["issue"] == 208
map = JSON.parse(File.binread(ordinary(MAP_RELATIVE)))
fail_receipt("map digest drift") unless Digest::SHA256.file(ROOT.join(MAP_RELATIVE)).hexdigest == MAP_SHA256
expected_cases = map.fetch("cases")
expected_boundaries = map.fetch("boundaries").flat_map { |row| row.fetch("subassertions") }
expected_lifecycle = map.fetch("lifecycle_subassertions")
fail_receipt("map contract mismatch") unless proof.fetch("map") == {
  "path" => MAP_RELATIVE, "sha256" => MAP_SHA256, "case_count" => 56,
  "boundary_row_count" => 8, "subassertion_count" => 64, "lifecycle_subassertion_count" => 12
}
fail_receipt("case order/parity mismatch") unless proof.fetch("cases") == expected_cases
fail_receipt("boundary parity mismatch") unless proof.fetch("boundary_subassertions") == expected_boundaries
fail_receipt("lifecycle parity mismatch") unless proof.fetch("lifecycle_subassertions") == expected_lifecycle
source = proof.fetch("source_revision")
base = proof.fetch("execution_base_revision")
fail_receipt("revision malformed") unless [source, base].all? { |revision| revision.match?(/\A[0-9a-f]{40}\z/) }
system("git", "merge-base", "--is-ancestor", base, source, chdir: ROOT.to_s, out: File::NULL, err: File::NULL) || fail_receipt("base/source nonancestry")
fail_receipt("source tree mismatch") unless git!("rev-parse", "#{source}^{tree}").strip == proof.fetch("source_tree")
proof.fetch("protected_files").each do |entry|
  path = ordinary(entry.fetch("path"))
  fail_receipt("protected digest drift: #{entry['path']}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
  committed = git!("show", "#{source}:#{entry.fetch('path')}")
  fail_receipt("source object drift: #{entry['path']}") unless Digest::SHA256.hexdigest(committed) == entry.fetch("sha256")
end
expected_commands = %w[diff_hygiene kernel_clippy kernel_markers kernel_nextest runtime_clippy runtime_markers runtime_nextest]
commands = proof.fetch("commands")
fail_receipt("command denominator mismatch") unless commands.keys.sort == expected_commands
commands.each do |name, command|
  fail_receipt("#{name} failed") unless command.fetch("exit_code") == 0
  fail_receipt("#{name} time inverted") if Time.iso8601(command.fetch("finished_at")) < Time.iso8601(command.fetch("started_at"))
  %w[stdout stderr].each do |stream|
    path = ordinary(command.fetch("#{stream}_path"))
    fail_receipt("#{name} #{stream} digest drift") unless Digest::SHA256.file(path).hexdigest == command.fetch("#{stream}_sha256")
  end
end
markers_text = %w[runtime_markers kernel_markers].flat_map { |name| %w[stdout stderr].map { |stream| File.binread(ROOT.join(commands[name]["#{stream}_path"])) } }.join
observed = markers_text.scan(/(?:proved:case|accepted|denied):[a-z0-9_:-]+/)
expected = expected_cases.map { |row| row.fetch("marker") } + expected_boundaries.map { |row| row.fetch("marker") } + expected_lifecycle.map { |row| row.fetch("marker") }
fail_receipt("observed marker mismatch") unless observed.sort == expected.sort && observed.uniq.length == expected.length
puts "PASS: issue #208 exact source binds 56 cases, 64 boundary assertions, and 12 lifecycle assertions"
