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
abort("proof not bound to HEAD") unless proof["source_revision"] == git("rev-parse", "HEAD").strip
abort("main binding drift") unless proof["required_main_ancestor"] == git("rev-parse", "origin/main").strip
abort("product drift") unless git("diff", "--name-only", "origin/main...HEAD", "--", "adl-runtime", "adl/Cargo.lock").empty?
abort("historical proof disposition missing") unless proof["historical_proof_disposition"] == "superseded_nonclaim"
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
