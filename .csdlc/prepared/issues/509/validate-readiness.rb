#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

root = File.expand_path("../../../..", __dir__)
design = File.read(File.join(root, ".csdlc/prepared/issues/509/design.md"))

def abort_with(message)
  warn(message)
  exit 1
end

def run_git(root, *args)
  stdout, stderr, status = Open3.capture3("git", "-C", root, *args)
  abort_with("git #{args.join(' ')} failed: #{stderr.strip}") unless status.success?
  stdout.strip
end

%w[#508 #494 #495].each do |id|
  abort_with("missing dependency #{id}") unless design.include?(id)
end

[
  "cs-poc-cha8mmii0xk0iaw5vpf8mxf",
  "GOOGLE_APPLICATION_CREDENTIALS",
  "operator-approved",
  "fixed cost ceiling",
  "cleanup-zero",
  "credential material remains outside the repository"
].each do |required|
  abort_with("missing readiness design text: #{required}") unless design.include?(required)
end

head = run_git(root, "rev-parse", "HEAD")

git_common_dir = run_git(root, "rev-parse", "--git-common-dir")
git_common_dir = File.expand_path(git_common_dir, root)
terminal_root = File.join(git_common_dir, "csdlc-v2", "derived-terminal")
[494, 495, 508].each do |issue|
  path = File.join(terminal_root, "#{issue}.json")
  abort_with("missing terminal cache for ##{issue}: #{path}") unless File.file?(path)
  packet = JSON.parse(File.read(path))
  abort_with("terminal cache issue mismatch for #{path}") unless packet["issue"] == issue
  abort_with("terminal cache not merged for ##{issue}") unless packet["disposition"] == "merged"
  abort_with("terminal cache did not close issue ##{issue}") unless packet["issue_state"] == "closed_by_merged_pr"
  merge_sha = packet["merge_sha"].to_s
  abort_with("terminal cache ##{issue} missing merge_sha") unless merge_sha.match?(/\A[0-9a-f]{40}\z/)
  _out, _err, status = Open3.capture3("git", "-C", root, "merge-base", "--is-ancestor", merge_sha, head)
  abort_with("terminal merge #{merge_sha} for ##{issue} is not ancestral to #{head}") unless status.success?
end

puts JSON.pretty_generate(
  schema: "adl.v0921.drt_d.readiness.v2",
  outcome: "passed",
  head: head,
  dependencies: {
    issue_494: "terminal_and_ancestral",
    issue_495: "terminal_and_ancestral",
    issue_508: "terminal_and_ancestral"
  },
  paid_run_gate: "explicit_authorization_required_before_live_launch"
)
