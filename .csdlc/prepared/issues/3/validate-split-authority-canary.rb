#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "time"

ROOT = File.expand_path("../../../..", __dir__)
CANARY = JSON.parse(File.read(File.join(ROOT, ".csdlc/prepared/issues/3/split-authority-canary.json")))

def require_truth(condition, message)
  raise message unless condition
end

issue = CANARY.fetch("issue")
pr = CANARY.fetch("pull_request")
timeline = CANARY.fetch("timeline")
reconciliation_pr = CANARY.fetch("post_closure_reconciliation_pull_request")

require_truth(CANARY.fetch("schema") == "csdlc.split_authority_canary.v1", "canary schema mismatch")
require_truth(issue.fetch("repository") == "danielbaustin/agent-design-language", "issue repository mismatch")
require_truth(issue.fetch("number") == 5901 && issue.fetch("state") == "closed", "legacy issue is not closed")
require_truth(pr.fetch("repository") == "agent-logic/agent-design-language", "PR repository mismatch")
require_truth(pr.fetch("number") == 4 && pr.fetch("state") == "merged", "canonical PR is not merged")
require_truth(pr.fetch("base") == "main", "canonical PR base mismatch")
require_truth(pr.fetch("closing_reference") == "Closes danielbaustin/agent-design-language#5901", "qualified closing reference mismatch")
require_truth(pr.fetch("required_checks") == "success", "canonical PR checks did not pass")
require_truth(pr.fetch("head_sha").match?(/\A[0-9a-f]{40}\z/), "head SHA is invalid")
require_truth(pr.fetch("merge_sha").match?(/\A[0-9a-f]{40}\z/), "merge SHA is invalid")

require_truth(timeline.fetch("cross_reference_repository") == pr.fetch("repository"), "timeline repository mismatch")
require_truth(timeline.fetch("cross_reference_pull_request") == pr.fetch("number"), "timeline PR mismatch")
require_truth(timeline.fetch("cross_reference_closing_reference") == pr.fetch("closing_reference"), "timeline closing reference mismatch")
require_truth(timeline.fetch("closed_event_at") == issue.fetch("closed_event_at"), "closed event mismatch")
require_truth(timeline.fetch("closed_event_closer_type") == "PullRequest", "closed event closer is not a PR")
require_truth(timeline.fetch("closed_event_closer_repository") == pr.fetch("repository"), "closed event closer repository mismatch")
require_truth(timeline.fetch("closed_event_closer_pull_request") == pr.fetch("number"), "closed event closer PR mismatch")

cross_reference_at = Time.iso8601(timeline.fetch("cross_reference_event_at"))
merged_at = Time.iso8601(pr.fetch("merged_at"))
issue_closed_at = Time.iso8601(issue.fetch("closed_at"))
closed_event_at = Time.iso8601(timeline.fetch("closed_event_at"))
require_truth(cross_reference_at < merged_at, "qualified cross-reference was not observed before merge")
require_truth(merged_at <= issue_closed_at, "legacy issue closed before canonical PR merged")
require_truth(issue_closed_at <= closed_event_at, "GitHub closed event precedes issue closed_at")
require_truth(closed_event_at - merged_at <= 5, "legacy issue closure is not temporally bound to canonical PR merge")

require_truth(reconciliation_pr.fetch("repository") == pr.fetch("repository"), "reconciliation PR repository mismatch")
require_truth(reconciliation_pr.fetch("number") == 5 && reconciliation_pr.fetch("state") == "merged", "reconciliation PR is not merged")
require_truth(reconciliation_pr.fetch("closing_reference") == pr.fetch("closing_reference"), "reconciliation PR closing reference mismatch")
require_truth(Time.iso8601(reconciliation_pr.fetch("merged_at")) > closed_event_at, "reconciliation PR must remain explicitly non-causal")
require_truth(reconciliation_pr.fetch("role").include?("not the causal"), "reconciliation PR role overclaims closure")

ancestor = system("git", "-C", ROOT, "merge-base", "--is-ancestor", pr.fetch("merge_sha"), "HEAD")
require_truth(ancestor, "canonical canary merge is not ancestral to the issue #3 candidate")

puts "split_authority_canary: PASS"
