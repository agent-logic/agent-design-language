#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "tmpdir"

SOURCE = "c640066f284a915b638add377cc4b0a2e221e6f9"
HISTORICAL = "b27b61597b7e6bc6563d6a7fef6f13ec9c6d3e98"
EXPECTED_RUN = "31453636709"
EXPECTED_WORKFLOW_REF = "agent-logic/agent-design-language/.github/workflows/wp14-production-acip-repair.yml@refs/heads/codex/209-acip-authority-repair"

def fail!(message)
  warn(message)
  exit 1
end

def capture!(*argv, chdir: nil, env: {})
  options = chdir ? { chdir: chdir } : {}
  stdout, stderr, status = Open3.capture3(env, *argv, **options)
  fail!("#{argv.join(' ')} failed: #{stderr.strip}") unless status.success?
  stdout
end

root = Pathname.new(capture!("git", "rev-parse", "--show-toplevel").strip).realpath
self_test = ARGV.delete("--self-test")
denominator_path = Pathname.new(ARGV.fetch(0, ".csdlc/prepared/issues/217/historical-c640-denominator.json"))
fail!("unexpected arguments") unless ARGV.length <= 1
denominator = JSON.parse(root.join(denominator_path).read)
files = denominator.fetch("files")
fail!("historical source mismatch") unless denominator["source_revision"] == SOURCE
fail!("historical denominator mismatch") unless denominator["expected_file_count"] == 10 && files.length == 10 && files.map { |entry| entry["path"] }.uniq.length == 10

files.each do |entry|
  relative = entry.fetch("path")
  current = root.join(relative)
  fail!("historical path missing: #{relative}") unless current.file?
  fail!("historical digest mismatch: #{relative}") unless Digest::SHA256.file(current).hexdigest == entry.fetch("sha256")
  historical = capture!("git", "show", "#{HISTORICAL}:#{relative}", chdir: root.to_s)
  fail!("historical byte mismatch: #{relative}") unless Digest::SHA256.hexdigest(historical) == entry.fetch("sha256")
end

if self_test
  puts JSON.generate(status: "passed", check: "historical-c640-contract", files: files.length)
  exit 0
end

Dir.mktmpdir("adl-217-c640-") do |directory|
  worktree = Pathname.new(directory).join("source")
  begin
    capture!("git", "worktree", "add", "--detach", worktree.to_s, SOURCE, chdir: root.to_s)
    files.each do |entry|
      relative = entry.fetch("path")
      destination = worktree.join(relative)
      FileUtils.mkdir_p(destination.dirname)
      FileUtils.cp(root.join(relative), destination, preserve: true)
    end
    env = {
      "GITHUB_ACTIONS" => "true",
      "GITHUB_REPOSITORY" => "agent-logic/agent-design-language",
      "GITHUB_WORKFLOW_REF" => EXPECTED_WORKFLOW_REF,
      "GITHUB_RUN_ID" => EXPECTED_RUN,
      "GITHUB_RUN_ATTEMPT" => "1"
    }
    output = capture!(
      "ruby", ".csdlc/prepared/issues/209/validate-native-receipts.rb",
      ".csdlc/evidence/209/native-platform/macos.json",
      ".csdlc/evidence/209/native-platform/linux.json",
      chdir: worktree.to_s, env: env
    )
    result = JSON.parse(output)
    fail!("detached c640 validation did not pass") unless result["status"] == "passed" && result["reviewed_head"] == SOURCE
    puts JSON.generate(status: "passed", source_revision: SOURCE, historical_revision: HISTORICAL, files: files.length)
  ensure
    files.each { |entry| FileUtils.rm_f(worktree.join(entry.fetch("path"))) }
    _stdout, _stderr, status = Open3.capture3("git", "worktree", "remove", worktree.to_s, chdir: root.to_s)
    fail!("detached worktree cleanup failed") unless status.success?
  end
end
