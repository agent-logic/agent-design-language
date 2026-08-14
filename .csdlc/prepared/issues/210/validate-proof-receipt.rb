#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
PROOF = ROOT.join(".csdlc/evidence/210/v1/continuity-transfer-proof.json")
MAP = ROOT.join(".csdlc/prepared/issues/210/continuity-transfer-acceptance-map.json")

def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  abort("issue 210 receipt git failure: #{err}") unless status.success?
  out
end

proof = JSON.parse(File.binread(PROOF))
abort("issue 210 receipt schema mismatch") unless proof["schema"] == "adl.issue210.continuity_transfer_proof.v1" && proof["issue"] == 210
source = proof.fetch("source_revision")
abort("issue 210 source revision malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
abort("issue 210 proof source must equal HEAD") unless source == git("rev-parse", "HEAD").strip
abort("issue 210 acceptance map digest drift") unless proof["acceptance_map_sha256"] == Digest::SHA256.file(MAP).hexdigest
map = JSON.parse(File.binread(MAP))
expected_markers = map.fetch("case_manifest").map { |entry| entry.fetch("marker") }
abort("issue 210 marker denominator drift") unless proof["case_markers"] == expected_markers
abort("issue 210 count mismatch") unless proof["case_count"] == 45 && proof["acceptance_count"] == 8 && proof["subassertion_count"] == 84
proof.fetch("commands").each do |name, command|
  abort("issue 210 failed command #{name}") unless command["exit_code"] == 0
  %w[stdout stderr].each do |stream|
    path = ROOT.join(".csdlc/evidence/210/v1/#{name}.#{stream}.log")
    abort("issue 210 log digest mismatch #{name}/#{stream}") unless Digest::SHA256.file(path).hexdigest == command["#{stream}_sha256"]
  end
end
stdout = File.binread(ROOT.join(".csdlc/evidence/210/v1/continuity-transfer.stdout.log"))
abort("issue 210 test denominator mismatch") unless stdout.include?("test result: ok. 14 passed; 0 failed;")
expected_markers.each do |marker|
  abort("issue 210 missing proof marker #{marker}") unless stdout.include?(marker)
end
abort("issue 210 worktree must be clean for receipt validation") unless git("status", "--porcelain=v1", "--untracked-files=all").empty?
puts "PASS: issue #210 continuity-transfer receipt validates"
