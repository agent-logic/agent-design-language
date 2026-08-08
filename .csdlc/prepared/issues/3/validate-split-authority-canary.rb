#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

ROOT = File.expand_path("../../../..", __dir__)
CANARY = JSON.parse(File.read(File.join(ROOT, ".csdlc/prepared/issues/3/split-authority-canary.json")))
BASELINE = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues/5901/index.json")))

def require_truth(condition, message)
  raise message unless condition
end

issue = CANARY.fetch("issue")
pr = CANARY.fetch("pull_request")
terminal = CANARY.fetch("terminal")

require_truth(CANARY.fetch("schema") == "csdlc.split_authority_canary.v1", "canary schema mismatch")
require_truth(issue.fetch("repository") == "danielbaustin/agent-design-language", "issue repository mismatch")
require_truth(issue.fetch("number") == 5901 && issue.fetch("state") == "closed", "legacy issue is not closed")
require_truth(pr.fetch("repository") == "agent-logic/agent-design-language", "PR repository mismatch")
require_truth(pr.fetch("number") == 5 && pr.fetch("state") == "merged", "canonical PR is not merged")
require_truth(pr.fetch("base") == "main", "canonical PR base mismatch")
require_truth(pr.fetch("closing_reference") == "Closes danielbaustin/agent-design-language#5901", "qualified closing reference mismatch")
require_truth(pr.fetch("required_checks") == "success", "canonical PR checks did not pass")
require_truth(pr.fetch("head_sha").match?(/\A[0-9a-f]{40}\z/), "head SHA is invalid")
require_truth(pr.fetch("merge_sha").match?(/\A[0-9a-f]{40}\z/), "merge SHA is invalid")

publication = BASELINE.fetch("publication")
require_truth(BASELINE.fetch("repository") == issue.fetch("repository"), "baseline issue authority mismatch")
require_truth(BASELINE.fetch("generation") == terminal.fetch("canonical_generation"), "terminal generation mismatch")
require_truth(BASELINE.fetch("digest") == terminal.fetch("canonical_digest"), "terminal digest mismatch")
require_truth(publication.fetch("repository") == pr.fetch("repository"), "baseline publication repository mismatch")
require_truth(publication.fetch("pull_request") == pr.fetch("number"), "baseline PR number mismatch")
require_truth(publication.fetch("base") == pr.fetch("base"), "baseline PR base mismatch")
require_truth(publication.fetch("head") == pr.fetch("head"), "baseline PR head mismatch")
require_truth(terminal.fetch("disposition") == "merged", "terminal disposition mismatch")
require_truth(terminal.fetch("issue_state") == "closed_by_merged_pr", "terminal issue state mismatch")

ancestor = system("git", "-C", ROOT, "merge-base", "--is-ancestor", pr.fetch("merge_sha"), "HEAD")
require_truth(ancestor, "canonical canary merge is not ancestral to the issue #3 candidate")

puts "split_authority_canary: PASS"
