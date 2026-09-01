#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../..").expand_path

PREREQUISITE_MERGES = {
  "CORP-A #482 / PR #545" => "e2c1d1649b0c930a5a1254575a07ef2a4496d48d",
  "CORP-B #483 / PR #562" => "4a0b49c0071bacdaab19d6d9eb8c44380beb51be",
  "AWS-G #496 / PR #599" => "83077ca029d52c9d613ed5a373da30f1dd42d9b3",
  "GCP-D #493 / PR #587" => "c0bf217934508d6dbc70d78633e6a95d5ddd9d06"
}.freeze

ISSUE_PATHS = [
  ".csdlc/issues/497",
  ".csdlc/prepared/issues/497",
  ".csdlc/evidence/497"
].freeze

SECRET_MARKERS = /(aws_secret_access_key|aws_access_key_id|secret_access_key|private_key|BEGIN [A-Z ]*PRIVATE KEY|ghp_|github_pat_|xox[baprs]-|AKIA[0-9A-Z]{16})/i

def run_git(*args)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *args)
  [stdout, stderr, status]
end

failures = []

PREREQUISITE_MERGES.each do |label, sha|
  _stdout, stderr, status = run_git("merge-base", "--is-ancestor", sha, "origin/main")
  failures << "#{label} merge #{sha} is not ancestral to origin/main: #{stderr.strip}" unless status.success?
end

index_path = ROOT.join(".csdlc/issues/497/index.json")
if index_path.file?
  index = JSON.parse(index_path.read)
  failures << "issue #497 is not initialized" unless index["phase"] == "initialized" || index["phase"] == "ready"
  failures << "issue #497 repository mismatch" unless index["repository"] == "agent-logic/agent-design-language"
else
  failures << "missing issue #497 index"
end

ISSUE_PATHS.each do |relative|
  path = ROOT.join(relative)
  next unless path.exist?

  path.find do |entry|
    next unless entry.file?
    next if entry.extname == ".rb"

    content = entry.read
    failures << "private or credential marker found in #{entry.relative_path_from(ROOT)}" if content.match?(SECRET_MARKERS)
  end
end

if failures.empty?
  puts JSON.pretty_generate({
    schema: "adl.corp_c_readiness.v1",
    status: "pass",
    issue: 497,
    prerequisite_merges: PREREQUISITE_MERGES,
    checked_paths: ISSUE_PATHS
  })
  exit 0
end

warn JSON.pretty_generate({
  schema: "adl.corp_c_readiness.v1",
  status: "fail",
  issue: 497,
  failures: failures
})
exit 1
