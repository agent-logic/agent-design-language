#!/usr/bin/env ruby
require "json"
require "pathname"

abort "usage: validate-publication-gate.rb --check-only" unless ARGV == ["--check-only"]
root = Pathname.new(__dir__).join("../../..").cleanpath
required = [
  "demos/v0.92/first-birthday/positive.json",
  ".csdlc/evidence/5836/publication-gate.json",
  "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md",
  "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md",
  "docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md",
  "docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md"
]
missing = required.reject { |path| root.join(path).file? }
abort "publication gate missing: #{missing.join(', ')}" unless missing.empty?
packet = JSON.parse(root.join(required.first).read)
abort "publication gate requires an accepted complete packet" unless packet["status"] == "complete" && packet.dig("decision", "accepted")
gate = JSON.parse(root.join(required[1]).read)
checks = {
  "missing_accepted_witness_receipt_proof" => gate.dig("accepted_witness_receipt_proof", "passed"),
  "stale_or_missing_exact_head_review" => gate.dig("current_exact_head_review", "passed"),
  "unsupported_claims_unresolved" => gate.dig("unsupported_claims_resolved", "passed"),
  "unresolved_negative_suite" => gate.dig("negative_suite_complete", "passed"),
  "absent_operator_authorization" => gate.dig("operator_publication_authorization", "passed")
}
blockers = checks.reject { |_name, passed| passed == true }.keys
result = {
  "schema" => "adl.first_birthday.publication_gate_result.v1",
  "decision" => blockers.empty? ? "eligible_for_operator_publication" : "do_not_publish",
  "blockers" => blockers,
  "can_publish" => false
}
puts JSON.generate(result)
exit(blockers.empty? ? 0 : 65)
