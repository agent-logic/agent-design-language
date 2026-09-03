#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = File.expand_path("../../../..", __dir__)
BASE = ENV.fetch("ADL_498_BASE", "origin/main")
REPO = "agent-logic/agent-design-language"
PREREQS = {
  482 => { planned_id: "CORP-A", merged_pr: 545 },
  483 => { planned_id: "CORP-B", merged_pr: 562 },
  497 => { planned_id: "CORP-C", merged_pr: 613, sidecar_issue: 624, sidecar_blocking_corp_d: false }
}.freeze
REQUEST_DIR = Pathname.new(__dir__)
BIN_DIR = Pathname.new(ENV.fetch("ADL_CSDLC_V2_BIN_DIR", File.join(ROOT, ".adl/bin/csdlc-v2"))).expand_path
ISSUE_READER = BIN_DIR.join("csdlc-github-issue").to_s
PR_READER = BIN_DIR.join("csdlc-github-pr").to_s

unless File.executable?(ISSUE_READER) && File.executable?(PR_READER)
  warn JSON.pretty_generate({
    schema: "adl.issue498.prerequisite_census.v1",
    issue: 498,
    status: "fail",
    failures: [
      "typed C-SDLC v2 GitHub read-owner binaries are not executable; set ADL_CSDLC_V2_BIN_DIR or install the repo-local owner binaries"
    ]
  })
  exit 1
end

def run(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  [stdout, stderr, status.exitstatus]
end

def read_issue(issue)
  request = REQUEST_DIR.join("github-issue-#{issue}-read.json")
  run(ISSUE_READER, "run", "--request", request.to_s)
end

def read_pr(issue, pull_request)
  request = REQUEST_DIR.join("github-pr-#{pull_request}-state.json")
  run(PR_READER, "run", "--request", request.to_s)
end

results = []
failures = []
sidecars = []

PREREQS.each do |issue, config|
  label = config.fetch(:planned_id)
  stdout, stderr, status = read_issue(issue)
  if status != 0
    failures << "#{label} ##{issue} issue readback failed: #{stderr.strip}"
    next
  end

  issue_json = JSON.parse(stdout).fetch("issue")
  if issue_json.fetch("state").to_s.casecmp("closed") != 0
    failures << "#{label} ##{issue} is not closed"
  end

  expected_pr = config.fetch(:merged_pr)
  prs_stdout, prs_stderr, prs_status = read_pr(issue, expected_pr)
  if prs_status != 0
    failures << "#{label} ##{issue} merged PR readback failed: #{prs_stderr.strip}"
    next
  end
  parsed_pr = JSON.parse(prs_stdout)
  pr_state = parsed_pr["pr_state"] || parsed_pr
  unless pr_state.fetch("pull_request") == expected_pr && pr_state.fetch("merged")
    failures << "#{label} ##{issue} expected merged PR ##{expected_pr}"
    next
  end

  sha = pr_state.fetch("merge_commit_sha").to_s
  failures << "#{label} ##{issue} merged PR ##{expected_pr} has no merge commit" unless sha.match?(/\A[0-9a-f]{40}\z/)
  _, anc_stderr, anc_status = run("git", "merge-base", "--is-ancestor", sha, BASE)
  failures << "#{label} ##{issue} merge #{sha} is not ancestral to #{BASE}: #{anc_stderr.strip}" unless anc_status == 0

  entry = {
    issue: issue,
    planned_id: label,
    state: issue_json.fetch("state"),
    merged_pr: expected_pr,
    merge_commit: sha,
    ancestral_to: BASE,
    ancestral: anc_status == 0
  }
  if config.key?(:sidecar_issue)
    sidecar_issue = config.fetch(:sidecar_issue)
    sidecar_stdout, sidecar_stderr, sidecar_status = read_issue(sidecar_issue)
    if sidecar_status != 0
      failures << "#{label} sidecar ##{sidecar_issue} issue readback failed: #{sidecar_stderr.strip}"
    else
      sidecar_json = JSON.parse(sidecar_stdout).fetch("issue")
      sidecars << {
        issue: sidecar_issue,
        source_planned_id: label,
        state: sidecar_json.fetch("state"),
        blocking_corp_d: config.fetch(:sidecar_blocking_corp_d)
      }
      entry[:sidecar_issue] = sidecar_issue
      entry[:sidecar_blocking_corp_d] = config.fetch(:sidecar_blocking_corp_d)
    end
  end
  results << entry
end

status = failures.empty? ? "pass" : "fail"
payload = {
  schema: "adl.issue498.prerequisite_census.v1",
  issue: 498,
  status: status,
  base: BASE,
  transport: "typed_csdlc_v2_github_read_owners",
  binary_resolution: "worktree .adl/bin/csdlc-v2 default, or explicit ADL_CSDLC_V2_BIN_DIR override",
  prerequisites: results,
  nonblocking_sidecars: sidecars,
  failures: failures,
  fail_closed_note: "CORP-D acceptance is prohibited until every prerequisite is closed, merged, and ancestral.",
  validated_by: ".csdlc/prepared/issues/498/check-prerequisites.rb"
}

stream = status == "pass" ? STDOUT : STDERR
stream.puts(JSON.pretty_generate(payload))
exit(status == "pass" ? 0 : 1)
