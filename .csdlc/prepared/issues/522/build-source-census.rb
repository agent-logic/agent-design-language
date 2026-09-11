#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

ROOT = "docs/milestones/v0.92.1/evidence/release/tail-06"

def run!(*argv)
  out, err, status = Open3.capture3(*argv)
  abort(err) unless status.success?
  out
end

def blob(revision, path)
  run!("git", "show", "#{revision}:#{path}")
end

def canonical_json(value)
  case value
  when Hash then "{" + value.keys.sort.map { |key| JSON.generate(key) + ":" + canonical_json(value.fetch(key)) }.join(",") + "}"
  when Array then "[" + value.map { |item| canonical_json(item) }.join(",") + "]"
  else JSON.generate(value)
  end
end

def source_report(issue:, pull_request:, merge_sha:, path:, manifest_path:, binding:, reviewed_revision:)
  report_blob = blob(merge_sha, path)
  report = JSON.parse(report_blob)
  findings = report.fetch("findings")
  {
    "issue" => issue,
    "pull_request" => pull_request,
    "merge_sha" => merge_sha,
    "path" => path,
    "sha256" => Digest::SHA256.hexdigest(report_blob),
    "packet_manifest_path" => manifest_path,
    "packet_manifest_sha256" => Digest::SHA256.hexdigest(blob(merge_sha, manifest_path)),
    "binding" => binding,
    "reviewed_revision" => reviewed_revision,
    "finding_ids" => findings.map { |finding| finding.fetch("id") },
    "finding_digests" => findings.to_h { |finding| [finding.fetch("id"), Digest::SHA256.hexdigest(canonical_json(finding))] }
  }
end

ledger_candidate = run!("git", "rev-parse", "HEAD").strip
reports = [
  source_report(
    issue: 520,
    pull_request: 831,
    merge_sha: "e058412611eff0975148ba9ed8e40bbd20ecb985",
    path: "docs/milestones/v0.92.1/evidence/release/tail-04/findings.json",
    manifest_path: "docs/milestones/v0.92.1/evidence/release/tail-04/packet-manifest.json",
    binding: "exact_revision",
    reviewed_revision: JSON.parse(blob("e058412611eff0975148ba9ed8e40bbd20ecb985", "docs/milestones/v0.92.1/evidence/release/tail-04/findings.json")).fetch("candidate_sha")
  ),
  source_report(
    issue: 521,
    pull_request: 850,
    merge_sha: "9c7e57d412d61898bd44ab00d53e31afbb779e5c",
    path: "docs/milestones/v0.92.1/evidence/release/tail-05/findings.json",
    manifest_path: "docs/milestones/v0.92.1/evidence/release/tail-05/packet-manifest.json",
    binding: "missing_revision",
    reviewed_revision: nil
  )
]
source_findings = reports.flat_map { |report| JSON.parse(blob(report.fetch("merge_sha"), report.fetch("path"))).fetch("findings") }
source = {
  "schema" => "adl.v0921.source_finding_census.v2",
  "ledger_candidate_sha" => ledger_candidate,
  "source_bindings" => reports.to_h { |report| [report.fetch("issue").to_s, report["reviewed_revision"]] },
  "reports" => reports,
  "findings" => source_findings
}

review_head = "2dfd01343816beb5cac6df2f644141be973f6fc2"
review_merge = "99700441767c43aa69e379b7b59f0f2a2c82d879"
report_path = "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_REPORT.md"
addendum_path = "docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXECUTABLE_REVIEW_ADDENDUM.md"
returned = {
  "schema" => "adl.v0921.returned_finding_census.v1",
  "review_issue" => 833,
  "review_pull_request" => 853,
  "review_head_sha" => review_head,
  "review_merge_sha" => review_merge,
  "artifacts" => [report_path, addendum_path].map { |path| {"path" => path, "sha256" => Digest::SHA256.hexdigest(blob(review_head, path))} },
  "findings" => [
    ["D833-REPORT-001", "P1", "Mandated coverage denominator is substantially unexercised by this lane", "The chat-only review lane could not discharge the mandated review denominator.", report_path, "### P1 — Mandated coverage denominator is substantially unexercised by this lane"],
    ["D833-REPORT-002", "P2", "Predecessor reconciliation is bound to a superseded source candidate", "The #834 reconciliation required candidate-current revalidation.", report_path, "### P2 — Predecessor reconciliation is bound to a superseded source candidate"],
    ["D833-REPORT-003", "P3", "Merged removal-approval packets retain pre-merge `pending_operator_review` fields", "Historical proposal fields required an explicit terminal projection.", report_path, "### P3 — Merged removal-approval packets retain pre-merge `pending_operator_review` fields"],
    ["D833-EXEC-001", "P1", "Corporate/Runtime retained proof is stale", "The #818 retained proof required candidate-current supersession evidence.", addendum_path, "### P1 — Corporate/Runtime retained proof is stale"],
    ["D833-EXEC-002", "P1", "Release projection is not candidate-current or ready", "Version reconciliation and native-v3 preflight remain owned by #856.", addendum_path, "### P1 — Release projection is not candidate-current or ready"],
    ["D833-EXEC-003", "P2", "#834 is historical ancestral reconciliation", "The historical reconciliation required explicit candidate revalidation and scope qualification.", addendum_path, "### P2 — #834 is historical ancestral reconciliation"]
  ].map do |id, severity, title, evidence, source_artifact, source_heading|
    {"id" => id, "severity" => severity, "title" => title, "evidence" => evidence, "source_artifact" => source_artifact, "source_heading" => source_heading}
  end
}

# The identifiers are a stable local projection, but the denominator comes from
# the immutable reviewed artifacts.  Refuse to publish the projection if either
# artifact gains, loses, or renames a finding heading.
parsed_headings = [report_path, addendum_path].flat_map do |path|
  blob(review_head, path).lines.grep(/^### P[0-3] — /).map(&:strip)
end
declared_headings = returned.fetch("findings").map { |finding| finding.fetch("source_heading") }
abort("returned-finding projection is not exhaustive") unless parsed_headings == declared_headings

File.write(File.join(ROOT, "source-findings.json"), JSON.pretty_generate(source) + "\n")
File.write(File.join(ROOT, "returned-findings.json"), JSON.pretty_generate(returned) + "\n")
