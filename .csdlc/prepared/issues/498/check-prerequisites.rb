#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
BASE = ENV.fetch("ADL_498_BASE", "origin/main")
REPO = "agent-logic/agent-design-language"
PREREQS = {
  482 => "CORP-A",
  483 => "CORP-B",
  497 => "CORP-C"
}.freeze

def run(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  [stdout, stderr, status.exitstatus]
end

results = []
failures = []

PREREQS.each do |issue, label|
  stdout, stderr, status = run("gh", "issue", "view", issue.to_s, "--repo", REPO, "--json", "number,state,closed,closedAt")
  if status != 0
    failures << "#{label} ##{issue} issue readback failed: #{stderr.strip}"
    next
  end

  issue_json = JSON.parse(stdout)
  if issue_json.fetch("state") != "CLOSED"
    failures << "#{label} ##{issue} is not closed"
  end

  prs_stdout, prs_stderr, prs_status = run("gh", "pr", "list", "--repo", REPO, "--state", "merged", "--search", "##{issue}", "--json", "number,mergeCommit,title,url")
  if prs_status != 0
    failures << "#{label} ##{issue} merged PR readback failed: #{prs_stderr.strip}"
    next
  end
  prs = JSON.parse(prs_stdout)
  merged = prs.find { |pr| pr.dig("mergeCommit", "oid").to_s.match?(/\A[0-9a-f]{40}\z/) }
  unless merged
    failures << "#{label} ##{issue} has no retained merged PR readback"
    next
  end

  sha = merged.fetch("mergeCommit").fetch("oid")
  _, anc_stderr, anc_status = run("git", "merge-base", "--is-ancestor", sha, BASE)
  failures << "#{label} ##{issue} merge #{sha} is not ancestral to #{BASE}: #{anc_stderr.strip}" unless anc_status == 0

  results << {
    issue: issue,
    planned_id: label,
    state: issue_json.fetch("state"),
    merged_pr: merged.fetch("number"),
    merge_commit: sha,
    ancestral_to: BASE,
    ancestral: anc_status == 0
  }
end

status = failures.empty? ? "pass" : "fail"
payload = {
  schema: "adl.issue498.prerequisite_census.v1",
  status: status,
  base: BASE,
  prerequisites: results,
  failures: failures,
  fail_closed_note: "CORP-D acceptance is prohibited until every prerequisite is closed, merged, and ancestral."
}

stream = status == "pass" ? STDOUT : STDERR
stream.puts(JSON.pretty_generate(payload))
exit(status == "pass" ? 0 : 1)
