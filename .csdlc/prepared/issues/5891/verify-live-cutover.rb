#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
LEGACY_REPO = "danielbaustin/agent-design-language"
CANONICAL_REPO = "agent-logic/agent-design-language"
LEGACY_URL = "https://github.com/#{LEGACY_REPO}.git"
CANONICAL_URL = "https://github.com/#{CANONICAL_REPO}.git"
PHASE = ENV.fetch("ADL_CUTOVER_PHASE", "pre")

def capture(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  abort "command failed: #{argv.join(' ')}: #{stderr.strip}" unless status.success?
  stdout.strip
end

def repo(name)
  JSON.parse(capture("gh", "repo", "view", name, "--json", "nameWithOwner,visibility,isFork,defaultBranchRef,url"))
end

legacy_sha = capture("git", "ls-remote", LEGACY_URL, "refs/heads/main").split.first
canonical_sha = capture("git", "ls-remote", CANONICAL_URL, "refs/heads/main").split.first
abort "missing main ref" if legacy_sha.to_s.empty? || canonical_sha.to_s.empty?
abort "pre-cutover main parity failed" if PHASE == "pre" && legacy_sha != canonical_sha

[repo(LEGACY_REPO), repo(CANONICAL_REPO)].each do |metadata|
  abort "repository must remain public" unless metadata["visibility"] == "PUBLIC"
  abort "repository must remain independent" if metadata["isFork"]
  abort "default branch must remain main" unless metadata.dig("defaultBranchRef", "name") == "main"
end

open_prs = JSON.parse(capture("gh", "pr", "list", "--repo", LEGACY_REPO, "--state", "open", "--limit", "200", "--json", "number"))
abort "legacy repository has undisposed open pull requests" unless open_prs.empty?

negative_repos = %w[agent-logic/asksifu agent-logic/Horust]
negative_repos.each do |name|
  _out, _err, status = Open3.capture3("gh", "repo", "view", name, chdir: ROOT)
  abort "excluded repository unexpectedly exists: #{name}" if status.success?
end

if PHASE == "post"
  origin = capture("git", "remote", "get-url", "origin")
  legacy_origin = capture("git", "remote", "get-url", "legacy-origin")
  abort "origin is not canonical" unless origin == CANONICAL_URL
  abort "legacy-origin is not preserved" unless legacy_origin == LEGACY_URL

  actions = JSON.parse(capture("gh", "api", "repos/#{CANONICAL_REPO}/actions/permissions"))
  abort "destination Actions are not enabled" unless actions["enabled"]
  capture("git", "push", "--dry-run", "origin", "HEAD:refs/heads/codex/5891-push-authority-check")
end

puts JSON.generate(
  schema: "adl.repository_cutover_live_validation.v1",
  phase: PHASE,
  result: "pass",
  legacy_main: legacy_sha,
  canonical_main: canonical_sha,
  open_legacy_pull_requests: open_prs.length,
  excluded_repositories_absent: negative_repos
)
