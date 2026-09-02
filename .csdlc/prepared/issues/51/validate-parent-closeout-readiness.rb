#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

def fail!(message)
  warn(message)
  exit 1
end

root = Dir.pwd
packet = File.join(root, "docs/milestones/v0.92.1/review/podcast_parent_51")
readme = File.read(File.join(packet, "README.md"))
design = File.read(File.join(root, ".csdlc/prepared/issues/51/design.md"))
state = JSON.parse(File.read(File.join(root, ".csdlc/evidence/51/parent-closeout-readiness.json")))
issue264 = JSON.parse(File.read(File.join(root, ".csdlc/issues/264/index.json")))
ledger = JSON.parse(File.read(File.join(root, "docs/milestones/v0.92.1/review/podcast_submission_264/submission-ledger.json")))

fail!("wrong schema") unless state["schema"] == "agent_logic.podcast.parent_51_readiness.v1"
fail!("wrong issue") unless state["issue"] == 51
fail!("wrong show") unless state["show"] == "The Cognitive Stack"
fail!("operator acceptance must be required") unless state["operator_acceptance_required_for_parent_closeout"] == true
fail!("provider submission must not be performed") unless state["provider_submission_performed"] == false
fail!("public launch must not be claimed") unless state["public_launch_claimed"] == false
fail!("destination links must not be activated by #51") unless state["destination_links_activated_by_51"] == false

fail!("#264 must be published in the stacked preparation base") unless issue264["issue"] == 264 && issue264["phase"] == "published"
fail!("#264 PR mismatch") unless issue264.dig("publication", "pull_request") == 649

children = state.fetch("child_state")
%w[261 262 263].each do |child|
  fail!("child #{child} is not marked retained/closed") unless children[child] == "closed_on_github_retained_evidence_present"
end
fail!("#264 state must remain not-yet-merged") unless children["264"] == "published_green_mergeable_not_yet_merged"

entries = ledger.fetch("entries")
fail!("expected four provider ledger entries") unless entries.length == 4
entries.each do |entry|
  fail!("provider unexpectedly authorized/submitted") unless entry["status"] == "not_authorized"
  fail!("provider submitted timestamp must be null") unless entry["submitted_at_utc"].nil?
  fail!("provider canonical URL/ID must be null") unless entry["canonical_url_or_id"].nil?
  fail!("provider evidence must not retain secrets") unless entry.dig("evidence", "secret_material_retained") == false
end

required_phrases = [
  "#264 PR #649 is merged",
  "operator explicitly accepts",
  "does not close #51",
  "No directory submission",
  "No provider account access",
  "No public launch announcement"
]
required_phrases.each do |phrase|
  fail!("missing phrase #{phrase.inspect}") unless readme.include?(phrase) || design.include?(phrase)
end

puts JSON.dump(
  schema: "agent_logic.podcast.parent_51_readiness_validation.v1",
  status: "passed",
  issue: 51,
  show: "The Cognitive Stack",
  prepared_from_pr: 649,
  child_state: children,
  operator_acceptance_required_for_parent_closeout: true,
  provider_submission_performed: false
)
