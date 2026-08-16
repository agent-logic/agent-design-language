#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
OUT = ROOT.join(".csdlc/evidence/210/v1")
PROOF = OUT.join("continuity-transfer-proof.json")
MAP = ROOT.join(".csdlc/prepared/issues/210/continuity-transfer-acceptance-map.json")
TEST = ROOT.join("adl-runtime/tests/distributed_continuity_transfer.rs")

COMMANDS = {
  "continuity-transfer" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_continuity_transfer -- --test-threads=1 --nocapture],
  "continuity-transfer-clippy" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_continuity_transfer -- -D warnings],
  "continuity-transfer-diff-hygiene" => %w[ruby .csdlc/prepared/issues/210/verify-diff-hygiene.rb]
}.freeze

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  abort("issue 210 proof git failure: #{err}") unless status.success?
  out
end

def run_command(name, argv)
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT.to_s)
  finished = Time.now.utc.iso8601(6)
  File.binwrite(OUT.join("#{name}.stdout.log"), stdout)
  File.binwrite(OUT.join("#{name}.stderr.log"), stderr)
  abort("issue 210 proof lane failed: #{name}\n#{stderr}") unless status.success?
  {
    "argv" => argv,
    "exit_code" => status.exitstatus,
    "started_at" => started,
    "finished_at" => finished,
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr)
  }
end

dirty = git("status", "--porcelain=v1", "--untracked-files=all").lines.map(&:strip)
dirty.reject! { |line| line.end_with?(".csdlc/evidence/210/") || line.include?(" .csdlc/evidence/210/") }
abort("issue 210 proof requires a clean worktree") unless dirty.empty?
source = git("rev-parse", "HEAD").strip
base = git("merge-base", "origin/main", "HEAD").strip
map = JSON.parse(File.binread(MAP))
abort("issue 210 map schema/count mismatch") unless map["issue"] == 210 && map["case_count"] == 45 && map["acceptance_count"] == 8 && map["subassertion_count"] == 84
expected_markers = map.fetch("case_manifest").map { |entry| entry.fetch("marker") }
expected_subassertions = map.fetch("acceptances").flat_map { |acceptance|
  acceptance.fetch("subassertions").map { |subassertion| subassertion.fetch("marker") }
}
abort("issue 210 subassertion marker denominator mismatch") unless expected_subassertions.uniq.length == 84
source_markers = File.binread(TEST)
  .scan(/marker\("(CASE-\d{3}:[a-z0-9_]+)"\)/)
  .flatten
  .map { |marker| "pass:#{marker}" }
abort("issue 210 source marker set mismatch") unless source_markers.sort == expected_markers.sort
abort("issue 210 duplicate source markers") unless source_markers.uniq.length == source_markers.length

FileUtils.mkdir_p(OUT, mode: 0o700)
commands = COMMANDS.transform_values.with_index { |argv, index| run_command(COMMANDS.keys[index], argv) }
test_stdout = File.binread(OUT.join("continuity-transfer.stdout.log"))
abort("issue 210 test denominator mismatch") unless test_stdout.include?("test result: ok. 15 passed; 0 failed;")
expected_markers.each do |marker|
  abort("issue 210 missing runtime marker #{marker}") unless test_stdout.include?(marker)
end
expected_subassertions.each do |marker|
  abort("issue 210 missing subassertion marker #{marker}") unless test_stdout.include?(marker)
end

proof = {
  "schema" => "adl.issue210.continuity_transfer_proof.v1",
  "issue" => 210,
  "source_revision" => source,
  "execution_base_revision" => base,
  "acceptance_map_sha256" => Digest::SHA256.file(MAP).hexdigest,
  "case_count" => expected_markers.length,
  "acceptance_count" => map.fetch("acceptance_count"),
  "subassertion_count" => map.fetch("subassertion_count"),
  "case_markers" => expected_markers,
  "subassertion_markers" => expected_subassertions,
  "commands" => commands,
  "nonclaims" => [
    "No #204 migration decision, activation, ownership, serving, or cloud authority.",
    "No Observatory runtime scope."
  ]
}
File.binwrite(PROOF, JSON.generate(proof) + "\n")
puts "PASS: issue #210 continuity-transfer proof receipt written for #{source}"
