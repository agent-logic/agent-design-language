#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
REPOSITORY = "agent-logic/agent-design-language"
MILESTONE = 1
TOKEN_FILE = "/Users/daniel/keys/github.token"
OWNER = "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-github-issue"
EVIDENCE = File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01")
MODE = ARGV.fetch(0, "create")
abort "usage: create-sprint-umbrellas.rb [create|update]" unless %w[create update].include?(MODE)
REQUESTS = File.join(EVIDENCE, MODE == "create" ? "umbrella-requests" : "umbrella-update-requests")
RESULTS = File.join(EVIDENCE, MODE == "create" ? "umbrella-operations" : "umbrella-update-operations")
RECEIPT = File.join(EVIDENCE, MODE == "create" ? "sprint-umbrella-receipt.json" : "sprint-umbrella-update-receipt.json")

SPRINTS = [
  [1, "Independent foundations", [482, 483, 510, 513, 514, 499]],
  [2, "Parallel cloud foundations", [484, 485, 486, 487, 488, 490, 491, 492, 493, 122, 251, 84, 345]],
  [3, "Cloud convergence", [495, 489, 496, 494]],
  [4, "Corporate acceptance", [497, 498]],
  [5, "C-SDLC v3 foundation", [500, 501, 502]],
  [6, "C-SDLC v3 delivery and cutover", [503, 504, 505]],
  [7, "Distributed Runtime qualification", [506, 345, 507, 508, 509]],
  [8, "Product lanes", [51, 261, 262, 263, 264, 342, 511, 251, 122, 345, 84, 512]],
  [9, "Provider comparison and convergence", [515, 516, 517, 518, 519]],
  [10, "Review and remediation", [520, 521, 522]],
  [11, "Handoff and release", [523, 524, 525, 526]]
].freeze

def write_json(path, value)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, JSON.pretty_generate(value) + "\n", mode: "w", perm: 0o600)
end

def issue_title(number, name)
  "[v0.92.1][Sprint #{number}] #{name}"
end

def issue_body(number, name, members)
  <<~BODY
    ## Outcome

    Coordinate Sprint #{number} — #{name} as one bounded execution wave without absorbing child implementation.

    ## Initial child membership baseline

    #{members.map { |issue| "- ##{issue}" }.join("\n")}

    ## Change protocol

    - This roster is the opening baseline, not a permanent freeze.
    - The milestone operator may add, remove, split, or reroute work through a typed issue update that records the reason and rechecks dependencies.
    - Every update must preserve one bounded result per child and must not silently drop unfinished work.

    ## Completion gate

    - Every child in the current declared roster has an independently reviewed, green merge into `main` and its merge commit is ancestral to the sprint closing revision.
    - Blocked or deferred children retain an explicit operator-approved disposition; the umbrella never fabricates completion.
    - Typed finish and worktree cleanup are asynchronous and never gate another issue.

    ## Concrete result

    One reviewable Sprint #{number} result records the current roster version, dispositions, reviewed merge heads, green checks, merge commits, and ancestry.

    ## Non-goals

    - Implementing child work inside this umbrella.
    - Serializing children that have no declared dependency.
    - Treating closeout or cleanup as an execution dependency.

    <!-- csdlc-github-operation:v0921-wp01:sprint-#{number}:create -->
  BODY
end

FileUtils.mkdir_p([REQUESTS, RESULTS])
existing = if MODE == "update"
             JSON.parse(File.read(File.join(EVIDENCE, "sprint-umbrella-receipt.json")))
                 .fetch("umbrellas").to_h { |row| [row.fetch("sprint"), row.fetch("issue")] }
           else
             {}
           end
observed = SPRINTS.map do |number, name, members|
  key = "v0921-wp01:sprint-#{number}:#{MODE == "create" ? "create" : "membership-v2-update"}"
  request_path = File.join(REQUESTS, format("sprint-%02d.json", number))
  result_path = File.join(RESULTS, format("sprint-%02d.json", number))
  request = {
    "repository" => REPOSITORY,
    "action" => MODE == "create" ? "issue_create" : "issue_update",
    "operation_key" => key,
    "token_file" => nil,
    "issue" => existing[number],
    "pull_request" => nil,
    "title" => issue_title(number, name),
    "body" => issue_body(number, name, members),
    "labels" => ["area:runtime", "track:roadmap", "type:task", "version:v0.92.1"],
    "assignees" => [],
    "milestone" => MILESTONE,
    "state" => nil,
    "comment_body" => nil,
    "required_checks" => [],
    "require_review" => false,
    "linked_issue" => nil
  }
  write_json(request_path, request)
  stdout, stderr, status = Open3.capture3({ "ADL_GITHUB_TOKEN_FILE" => TOKEN_FILE }, OWNER,
                                          "run", "--request", request_path, chdir: ROOT)
  abort "Sprint #{number} creation failed: #{stderr}" unless status.success?
  result = JSON.parse(stdout)
  write_json(result_path, result)
  issue = result.fetch("issue")
  abort "Sprint #{number} identity mismatch" unless issue.fetch("title") == request.fetch("title") &&
                                                  issue.fetch("state") == "open" &&
                                                  issue.fetch("milestone") == MILESTONE &&
                                                  issue.fetch("marker_present")
  {
    "sprint" => number,
    "name" => name,
    "issue" => issue.fetch("number"),
    "title" => issue.fetch("title"),
    "members" => members,
    "operation_key" => key,
    "result_sha256" => Digest::SHA256.file(result_path).hexdigest
  }
end

write_json(RECEIPT, {
  "schema" => "adl.v0921.wp01.sprint-umbrella-receipt.v1",
  "repository" => REPOSITORY,
  "conductor_issue" => 480,
  "umbrellas" => observed
})
puts JSON.pretty_generate("result" => "passed", "umbrellas" => observed)
