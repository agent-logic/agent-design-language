#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
REPOSITORY = "agent-logic/agent-design-language"
MILESTONE = 1
OWNER = ENV.fetch("CSDLC_GITHUB_ISSUE_BIN")
EVIDENCE = File.join(ROOT, "docs/milestones/v0.92.1/evidence/wp-01")
MODE = ARGV.fetch(0, "create")
abort "usage: create-sprint-umbrellas.rb [create|update]" unless %w[create update].include?(MODE)
MEMBERSHIP_VERSION = MODE == "create" ? 1 : Integer(ENV.fetch("SPRINT_MEMBERSHIP_VERSION"), 10)
CHANGE_REASON = MODE == "create" ? "Initial milestone-opening roster." : ENV.fetch("SPRINT_MEMBERSHIP_REASON")
abort "update membership version must be at least 2" if MODE == "update" && MEMBERSHIP_VERSION < 2
abort "membership change reason is required" if CHANGE_REASON.strip.empty?
REQUESTS = File.join(EVIDENCE, MODE == "create" ? "umbrella-requests" : "umbrella-update-v#{MEMBERSHIP_VERSION}-requests")
RESULTS = File.join(EVIDENCE, MODE == "create" ? "umbrella-operations" : "umbrella-update-v#{MEMBERSHIP_VERSION}-operations")
RECEIPT = File.join(EVIDENCE, MODE == "create" ? "sprint-umbrella-receipt.json" : "sprint-umbrella-membership-v#{MEMBERSHIP_VERSION}-receipt.json")
if MODE == "update"
  prior_versions = Dir.glob(File.join(EVIDENCE, "sprint-umbrella-membership-v*-receipt.json"))
                      .map { |path| File.basename(path)[/membership-v(\d+)-receipt/, 1]&.to_i }.compact
  prior_versions << 2 if File.exist?(File.join(EVIDENCE, "sprint-umbrella-update-receipt.json"))
  abort "membership version must advance exactly once" unless MEMBERSHIP_VERSION == prior_versions.max.to_i + 1
end

SPRINTS = [
  [1, "Independent foundations", [482, 483, 510, 513, 514, 499]],
  [2, "Parallel cloud foundations", [484, 485, 486, 487, 488, 490, 491, 492, 493, 122, 251]],
  [3, "Cloud convergence", [495, 489, 496, 494]],
  [4, "Corporate acceptance", [497, 498]],
  [5, "C-SDLC v3 foundation", [500, 501, 502]],
  [6, "C-SDLC v3 delivery and cutover", [503, 504, 505]],
  [7, "Distributed Runtime qualification", [506, 345, 507, 508, 509]],
  [8, "Product lanes", [51, 261, 262, 263, 264, 342, 511, 84, 512]],
  [9, "Provider comparison and convergence", [515, 516, 517, 518, 519]],
  [10, "Review and remediation", [520, 521, 522]],
  [11, "Handoff and release", [523, 524, 525, 526]]
].freeze

all_members = SPRINTS.flat_map { |_number, _name, members| members }
abort "duplicate issue ownership across Sprint umbrellas" unless all_members.uniq.length == all_members.length

def write_json(path, value)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, JSON.pretty_generate(value) + "\n", mode: "w", perm: 0o600)
end

def issue_title(number, name)
  "[v0.92.1][Sprint #{number}] #{name}"
end

def issue_body(number, name, members, membership_version, change_reason)
  <<~BODY
    ## Outcome

    Coordinate Sprint #{number} — #{name} as one bounded execution wave without absorbing child implementation.

    ## Initial child membership baseline

    #{members.map { |issue| "- ##{issue}" }.join("\n")}

    - Membership version: `#{membership_version}`
    - Change reason: #{change_reason}

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
  key = "v0921-wp01:sprint-#{number}:#{MODE == "create" ? "create" : "membership-v#{MEMBERSHIP_VERSION}-update"}"
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
    "body" => issue_body(number, name, members, MEMBERSHIP_VERSION, CHANGE_REASON),
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
  stdout, stderr, status = Open3.capture3(OWNER, "run", "--request", request_path, chdir: ROOT)
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
    "membership_version" => MEMBERSHIP_VERSION,
    "change_reason" => CHANGE_REASON,
    "operation_key" => key,
    "result_sha256" => Digest::SHA256.file(result_path).hexdigest
  }
end

write_json(RECEIPT, {
  "schema" => MODE == "create" ? "adl.v0921.wp01.sprint-umbrella-receipt.v1" : "adl.v0921.wp01.sprint-umbrella-membership-update.v1",
  "repository" => REPOSITORY,
  "conductor_issue" => 480,
  "membership_version" => MEMBERSHIP_VERSION,
  "change_reason" => CHANGE_REASON,
  "umbrellas" => observed
})
puts JSON.pretty_generate("result" => "passed", "umbrellas" => observed)
