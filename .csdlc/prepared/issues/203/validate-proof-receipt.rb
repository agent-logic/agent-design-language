#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"
require "json"
require "open3"
require "pathname"
ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
REL = ".csdlc/evidence/203/v3/integration-closeout-proof.json"
def git(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  abort("issue 203 receipt git failure: #{err}") unless status.success?
  out
end
proof = JSON.parse(File.binread(ROOT.join(REL)))
abort("issue/schema mismatch") unless proof["issue"] == 203 && proof["schema"] == "adl.issue203.integration_closeout_proof.v3"
source = proof["source_revision"]
abort("proof source malformed") unless source&.match?(/\A[0-9a-f]{40}\z/)
abort("proof source not ancestral") unless system("git", "merge-base", "--is-ancestor", source, "HEAD", chdir: ROOT.to_s)
abort("main binding drift") unless proof["required_main_ancestor"] == git("rev-parse", "origin/main").strip
abort("product drift") unless git("diff", "--name-only", "origin/main...HEAD", "--", "adl-runtime", "adl/Cargo.lock").empty?
abort("historical proof disposition missing") unless proof["historical_proof_disposition"] == "superseded_nonclaim"
expected_merges = {"258"=>"193f77d24a693f955a2fcf3bdfc759ad1db8aff4","259"=>"119bab39d4eb98cd4013c95633ff070908e4c59c","260"=>"0b5aefd6e75e56ccac59e761a7037902f581c76d"}
common = Pathname.new(git("rev-parse", "--git-common-dir").strip).expand_path(ROOT)
finish = common.parent.join(".adl/bin/csdlc-v2/csdlc-finish").to_s
expected_merges.each do |issue, expected_merge|
  raw, err, status = Open3.capture3(finish, "--root", ROOT.to_s, "--validate-cached-issue", issue, chdir: ROOT.to_s)
  abort("terminal cache validation failed ##{issue}: #{err}") unless status.success?
  live = JSON.parse(raw)
  bound = proof.fetch("terminal_children").fetch(issue)
  abort("terminal cache is not canonical ##{issue}") unless live["canonical_match"] == true && bound["canonical_match"] == true
  abort("terminal cache binding drift ##{issue}") unless live.fetch("terminal").fetch("digest") == bound["terminal_digest"] && bound["merge_sha"] == expected_merge
end
protected = [".csdlc/prepared/issues/203/produce-proof-receipt.rb", ".csdlc/prepared/issues/203/validate-proof-receipt.rb"]
abort("proof helpers changed after proof source") unless git("diff", "--name-only", "#{source}..HEAD", "--", *protected).empty?
expected = {"identity-boundary"=>4,"caller-guard"=>5,"strict-clippy"=>nil}
abort("command denominator mismatch") unless proof.fetch("commands").keys.sort == expected.keys.sort
proof.fetch("commands").each do |name, command|
  abort("command failed #{name}") unless command["exit_code"] == 0
  %w[stdout stderr].each do |stream|
    path = ROOT.join(".csdlc/evidence/203/v3/#{name}.#{stream}.log")
    abort("log digest mismatch #{name}/#{stream}") unless Digest::SHA256.file(path).hexdigest == command["#{stream}_sha256"]
  end
end
expected.each do |name, count|
  next unless count
  out = File.binread(ROOT.join(".csdlc/evidence/203/v3/#{name}.stdout.log"))
  abort("test denominator mismatch #{name}") unless out.include?("test result: ok. #{count} passed; 0 failed;")
end
abort("worktree must be exactly clean") unless git("status", "--porcelain=v1", "--untracked-files=all").empty?
puts "PASS: issue #203 current integration-closeout receipt validates"
