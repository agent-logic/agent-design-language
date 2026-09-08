#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "shellwords"
require "yaml"

root = File.expand_path("../../../..", __dir__)
path = File.join(root, ".csdlc/evidence/516/semantic-criterion-evidence.json")
candidate = `git -C #{root.shellescape} rev-parse origin/main`.strip
abort "unable to resolve origin/main" unless candidate.match?(/\A[0-9a-f]{40}\z/)

document = JSON.parse(File.read(path))
document["candidate"] = candidate
entries = document.fetch("entries").to_h { |entry| [entry.fetch("criterion_id"), entry] }

updates = {
  "AWS-F-ac-4" => {
    "classification" => "proven",
    "implementation_evidence" => ["infra/aws/runtime/private-node/main.tf", "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"],
    "validation_evidence" => [".csdlc/issues/489/cards/sor.md", ".csdlc/issues/728/cards/sor.md", ".csdlc/evidence/728/no-auth-refusal.log"],
    "review_evidence" => [".csdlc/issues/489/cards/srp.md", ".csdlc/issues/728/cards/srp.md"],
    "docs_evidence" => [".csdlc/issues/728/cards/sor.md", "docs/operations/cloud/aws/runtime-platform/README.md"],
    "closeout_evidence" => ["github:issue-489:closed", "github:issue-728:closed"],
    "rationale" => "Issue #728 added the reviewed disposable AWS-F deployment runner, authorization-bound selectors, reverse teardown, Terraform-state emptiness checks, and AWS-side absence readbacks. The retained proof is intentionally no-mutation until a separately authorized live run."
  },
  "GCP-D-ac-4" => {
    "classification" => "proven",
    "implementation_evidence" => ["infra/gcp/platform/main.tf", ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh"],
    "validation_evidence" => [".csdlc/issues/493/cards/sor.md", ".csdlc/issues/731/cards/sor.md", ".csdlc/evidence/731/live-disposable-workload/status.json"],
    "review_evidence" => [".csdlc/issues/493/cards/srp.md", ".csdlc/issues/731/cards/srp.md"],
    "docs_evidence" => [".csdlc/issues/731/cards/sor.md"],
    "closeout_evidence" => ["github:issue-493:closed", "github:issue-731:closed"],
    "rationale" => "Issue #731 created one authorized private e2-micro workload without an external IP, destroyed it and its auto-delete disk, and retained independent zero-residue readbacks for instances, run-labelled instances, disks, and addresses."
  },
  "V3-F-ac-2" => {
    "classification" => "proven",
    "implementation_evidence" => ["csdlc-v3/Cargo.toml", "csdlc-v3/src/authority.rs", "csdlc-v3/src/commands/remote/mod.rs"],
    "validation_evidence" => [".csdlc/issues/505/cards/sor.md", ".csdlc/issues/725/cards/sor.md"],
    "review_evidence" => [".csdlc/issues/505/cards/srp.md", ".csdlc/issues/725/cards/srp.md"],
    "docs_evidence" => [".csdlc/issues/725/cards/sor.md", "docs/csdlc-v3/CONTRACT.md"],
    "closeout_evidence" => ["github:issue-505:closed", "github:issue-725:closed"],
    "rationale" => "Issue #725 and merged PR #732 activated authenticated native v3 authority, removed the csdlc-v2 build dependency, retained exact reviewed/merge authority checks, and added fail-closed missing-object coverage."
  }
}

updates.each do |criterion_id, fields|
  entry = entries.fetch(criterion_id)
  fields.each { |key, value| entry[key] = value }
end

# The current candidate inserted the explicit issue-84 proof criterion into
# OBS-B. Preserve the two existing evidence-backed rows at their new ordinals
# and classify the new criterion as a gap until issue 84 supplies the required
# reviewed merged authority.
spec_text = `git -C #{root.shellescape} show #{candidate}:docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml`
obs_b_criteria = YAML.safe_load(spec_text).fetch("issue_specifications").find { |spec| spec.fetch("id") == "OBS-B" }.fetch("acceptance_criteria")
old_runtime_projection = entries.fetch("OBS-B-ac-2").dup
old_accessibility = entries.fetch("OBS-B-ac-3").dup
entries["OBS-B-ac-2"] = {
  "criterion_id" => "OBS-B-ac-2",
  "criterion_digest" => Digest::SHA256.hexdigest(obs_b_criteria.fetch(1)),
  "classification" => "proof_gap",
  "semantic_mapping" => [],
  "implementation_evidence" => [],
  "validation_evidence" => [],
  "review_evidence" => [],
  "docs_evidence" => [],
  "closeout_evidence" => ["github:issue-84:open"],
  "rationale" => "The candidate requires reviewed merged issue 84 authority, which is not present."
}
entries["OBS-B-ac-3"] = old_runtime_projection.merge(
  "criterion_id" => "OBS-B-ac-3",
  "criterion_digest" => Digest::SHA256.hexdigest(obs_b_criteria.fetch(2)),
  "rationale" => "Independent semantic audit classified OBS-B-ac-3 as proven: Runtime projections are source-grounded"
)
entries["OBS-B-ac-4"] = old_accessibility.merge(
  "criterion_id" => "OBS-B-ac-4",
  "criterion_digest" => Digest::SHA256.hexdigest(obs_b_criteria.fetch(3)),
  "rationale" => "Independent semantic audit classified OBS-B-ac-4 as proven: Accessibility and recovery cases pass"
)

# Closed issue markers are closeout evidence, not authority to amend planned
# acceptance. Keep unmapped amendments visible as proof gaps.
entries.each_value do |entry|
  next unless entry["classification"] == "accepted_with_explicit_amendment"
  next unless entry.fetch("semantic_mapping", []).empty?

  entry["classification"] = "proof_gap"
  entry["rationale"] = "No explicit amendment authority maps this planned criterion to replacement acceptance; retain it as a proof gap."
end

document["entries"] = entries.values
File.write(path, JSON.pretty_generate(document) + "\n")
puts JSON.generate({ schema: "csdlc.semantic_evidence_refresh.v1", candidate: candidate, updated: updates.keys })
