#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "time"

REPOSITORY = "agent-logic/agent-design-language"
TARGET = "c6792e54df1db5969fa28c59b6dfe4c714ed5559"
OUTPUT = "docs/reviews/v0.92/internal-review-5846/LIVE_STATE.json"
verify_only = ARGV == ["--verify"]
abort "unexpected arguments" unless ARGV.empty? || verify_only

def run!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort "command failed: #{argv.join(' ')}: #{stderr.strip}" unless status.success?
  stdout
end

def issue!(number, fields)
  JSON.parse(run!("gh", "issue", "view", number.to_s, "--repo", REPOSITORY, "--json", fields.join(",")))
end

remote = run!("git", "remote", "get-url", "origin").strip
abort "wrong repository" unless remote.match?(%r{(?:github\.com[:/])agent-logic/agent-design-language(?:\.git)?\z})
abort "target missing" unless system("git", "cat-file", "-e", "#{TARGET}^{commit}", out: File::NULL, err: File::NULL)

issue_313 = issue!(313, %w[number state title body])
issue_307 = issue!(307, %w[number state title body])
issue_342 = issue!(342, %w[number state title body milestone])
issue_315 = issue!(315, %w[number state title body])

abort "#313 state mismatch" unless issue_313["state"] == "OPEN"
abort "#313 dependency/deferment mismatch" unless issue_313.fetch("body").include?("depends_on`: canonical predecessor #312") &&
  issue_313.fetch("body").include?("WP-24 #10") &&
  issue_313.fetch("body").include?("WP-24A #342 is assigned to v0.92.1")
sequence = "#312 -> #313 -> #314 -> #315"
abort "#307 graph mismatch" unless issue_307["state"] == "OPEN" && issue_307.fetch("body").include?(sequence)
abort "#342 deferment mismatch" unless issue_342["state"] == "OPEN" && issue_342.dig("milestone", "title") == "v0.92.1"
abort "#315 WP-27 authority mismatch" unless issue_315["state"] == "OPEN" && issue_315.fetch("title").include?("[WP-27]") && issue_315.fetch("body").include?("replaces")

common = run!("git", "rev-parse", "--git-common-dir").strip
terminal = {}
[312, 10].each do |number|
  path = File.join(common, "csdlc-v2", "derived-terminal", "#{number}.json")
  abort "missing derived terminal #{number}" unless File.file?(path)
  receipt = JSON.parse(File.read(path))
  abort "terminal issue mismatch" unless receipt["issue"] == number && receipt["repository"] == REPOSITORY
  abort "terminal disposition mismatch" unless receipt["disposition"] == "merged" && receipt["issue_state"] == "closed_by_merged_pr"
  merge_sha = receipt.fetch("merge_sha")
  abort "terminal merge is not ancestral" unless system("git", "merge-base", "--is-ancestor", merge_sha, TARGET, out: File::NULL, err: File::NULL)
  terminal[number.to_s] = {
    "disposition" => receipt["disposition"],
    "issue_state" => receipt["issue_state"],
    "merge_sha" => merge_sha,
    "terminal_digest" => receipt.fetch("digest"),
    "source" => receipt.fetch("source")
  }
end

worktrees = run!("git", "worktree", "list", "--porcelain")
abort "#312 worktree still registered" if worktrees.match?(%r{(?:branch refs/heads/codex/312-|worktree .*/adl-issue-312-)})
abort "#10 worktree still registered" if worktrees.match?(%r{(?:branch refs/heads/codex/10-|worktree .*/adl-issue-10-)})

payload = {
  "schema" => "adl.internal_review.live_state.v1",
  "captured_at" => Time.now.utc.iso8601,
  "repository" => REPOSITORY,
  "target_sha" => TARGET,
  "issues" => {
    "307" => {"state" => issue_307["state"], "body_sha256" => Digest::SHA256.hexdigest(issue_307.fetch("body")), "graph_check" => sequence},
    "313" => {"state" => issue_313["state"], "body_sha256" => Digest::SHA256.hexdigest(issue_313.fetch("body")), "dependencies" => [312, 10], "deferred_non_blocking" => [342]},
    "342" => {"state" => issue_342["state"], "body_sha256" => Digest::SHA256.hexdigest(issue_342.fetch("body")), "milestone" => issue_342.dig("milestone", "title")},
    "315" => {"state" => issue_315["state"], "body_sha256" => Digest::SHA256.hexdigest(issue_315.fetch("body")), "work_package" => "WP-27", "legacy_predecessor" => 5848}
  },
  "terminal_dependencies" => terminal,
  "dependency_worktrees_absent" => [312, 10]
}
if verify_only
  abort "missing retained live state" unless File.file?(OUTPUT)
  retained = JSON.parse(File.read(OUTPUT))
  payload["captured_at"] = retained["captured_at"]
  abort "retained live state is stale" unless retained == payload
  puts "PASS: verified retained live #307/#313/#315/#342 and terminal #312/#10 state"
else
  File.write(OUTPUT, JSON.pretty_generate(payload) + "\n")
  puts "PASS: captured live #307/#313/#315/#342 and terminal #312/#10 review state"
end
