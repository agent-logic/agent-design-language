#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "shellwords"

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
  }
}

updates.each do |criterion_id, fields|
  entry = entries.fetch(criterion_id)
  fields.each { |key, value| entry[key] = value }
end

File.write(path, JSON.pretty_generate(document) + "\n")
puts JSON.generate({ schema: "csdlc.semantic_evidence_refresh.v1", candidate: candidate, updated: updates.keys })
