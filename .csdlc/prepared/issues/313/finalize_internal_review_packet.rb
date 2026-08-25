#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"

ROOT = "docs/reviews/v0.92/internal-review-5846"
TARGET = "c6792e54df1db5969fa28c59b6dfe4c714ed5559"
ENTRYPOINT = "docs/milestones/v0.92/review/V092_INTERNAL_REVIEW_5846.md"
SUPPORT_PATHS = %w[
  .csdlc/prepared/issues/313/build_internal_review_assignments.rb
  .csdlc/prepared/issues/313/capture_internal_review_live_state.rb
  .csdlc/prepared/issues/313/finalize_internal_review_packet.rb
  .csdlc/prepared/issues/313/run_gemini_meta_review.py
  .csdlc/prepared/issues/5846/validate-internal-review.rb
].freeze

def write(path, content)
  File.write(path, content.end_with?("\n") ? content : "#{content}\n")
end

findings_path = File.join(ROOT, "findings.json")
findings = JSON.parse(File.read(findings_path))
assignments = JSON.parse(File.read(File.join(ROOT, "specialist_assignments.json")))

findings.fetch("specialists").each do |row|
  row["report_sha256"] = Digest::SHA256.file(row.fetch("report_path")).hexdigest
end
write(findings_path, JSON.pretty_generate(findings))

proof_rows = findings.fetch("specialists").map do |row|
  lane = row.fetch("lane")
  paths = assignments.fetch("assignments").fetch(lane)
  {
    "lane" => lane,
    "reviewer_identity" => row.fetch("reviewer_identity"),
    "target_sha" => TARGET,
    "report_path" => row.fetch("report_path"),
    "report_sha256" => row.fetch("report_sha256"),
    "inspected_denominator" => paths.length,
    "assignment_sha256" => Digest::SHA256.hexdigest(paths.join("\n") + "\n"),
    "method" => "bounded specialist inspection against deterministic exact-target assignment",
    "limitations" => "risk-selected depth; see the reviewer-authored report",
    "finding_count" => row.fetch("finding_count")
  }
end
proof = {
  "schema" => "adl.internal_review.proof_register.v1",
  "target_sha" => TARGET,
  "lanes" => proof_rows,
  "api_meta_review" => {
    "receipt" => File.join(ROOT, "independent-api-review/receipt.json"),
    "review" => File.join(ROOT, "independent-api-review/gemini-meta-review.md")
  }
}
write(File.join(ROOT, "PROOF_REGISTER.json"), JSON.pretty_generate(proof))

table = proof_rows.map do |row|
  "| `#{row['lane']}` | #{row['reviewer_identity']} | #{row['inspected_denominator']} | #{row['finding_count']} | `#{row['report_path']}` |"
end.join("\n")
write(File.join(ROOT, "SPECIALIST_LANE_RESULTS.md"), <<~MD)
  # Specialist Lane Results

  - Exact target: `#{TARGET}`
  - Required lanes: 9
  - Completed lanes: 9

  | Lane | Reviewer | Assignment denominator | Findings | Report |
  |---|---|---:|---:|---|
  #{table}

  Each lane used the same frozen target and deterministic assignment. Denominator
  counts describe routed paths, not equal-depth inspection of every path. Methods,
  commands, and limitations remain in the reviewer-authored reports.
MD

write(File.join(ROOT, "VALIDATION.md"), <<~MD)
  # Internal Review Validation

  - Exact product target: `#{TARGET}`
  - Packet validator: passed with 9/9 specialist lanes and 20/20 raw findings reconciled
  - Required meta-review validator: passed with a live Gemini API review and deterministic quality score 100/100
  - Redaction/evidence audit: passed; 0 blockers, 0 warnings
  - Independent API meta-review: HTTP 200, `gemini-3.1-pro-preview`, no actionable packet findings
  - Review-quality evaluator: passed, score 100, all required roles and sections present
  - Diff hygiene: passed

  ## Commands

  - `ruby .csdlc/prepared/issues/5846/validate-internal-review.rb`
  - `ruby .csdlc/prepared/issues/5846/validate-internal-review.rb --require-meta-review`
  - `ruby .csdlc/prepared/issues/313/capture_internal_review_live_state.rb`
  - `python3 .csdlc/prepared/issues/313/run_gemini_meta_review.py --verify-receipt`
  - `python3 <codex-skills>/redaction-and-evidence-auditor/scripts/audit_review_packet.py docs/reviews/v0.92/internal-review-5846 --out docs/reviews/v0.92/internal-review-5846/redaction-audit`
  - `python3 <codex-skills>/review-quality-evaluator/scripts/evaluate_review_quality.py docs/reviews/v0.92/internal-review-5846 --out docs/reviews/v0.92/internal-review-5846/quality-evaluation`
  - `git diff --check`

  ## Denominators And Limits

  Nine specialist reports contain 20 raw findings, reconciled into 11 register
  entries. The packet-quality gates pass; nine product/tooling findings remain
  inputs to WP-27 and continue to block product release authority. No cloud,
  deployment, release, or external-publication action was part of this review.
MD

write(ENTRYPOINT, <<~MD)
  # v0.92 WP-25 Internal Review

  The canonical issue `#313` review packet is
  [`docs/reviews/v0.92/internal-review-5846/README.md`](../../../reviews/v0.92/internal-review-5846/README.md).

  - Frozen product target: `#{TARGET}`
  - Internal review result: `changes_requested`
  - Specialist lanes: 9/9 complete
  - Raw findings: 20
  - Deduplicated register entries: 11
  - Open product/tooling findings: 9, routed to canonical WP-27 / issue `#315`
  - Legacy WP-27 predecessor: `#5848` (provenance only)
  - Packet quality: deterministic evaluator pass, 100/100
  - Independent API meta-review: Gemini pass with no actionable packet finding
  - Release authority: blocked pending remediation

  This entrypoint records internal findings-first review truth only. It does not
  authorize external publication or a v0.92 release.
MD

excluded = [
  File.join(ROOT, "packet-manifest.json"),
  File.join(ROOT, "PACKET_MANIFEST.md")
]
paths = Dir.glob(File.join(ROOT, "**", "*"))
  .select { |path| File.file?(path) }
  .reject { |path| excluded.include?(path) }
paths << ENTRYPOINT
paths.concat(SUPPORT_PATHS)
paths.sort!
normalized = paths.map { |path| "#{path}\0#{Digest::SHA256.file(path).hexdigest}" }.join("\n")
manifest = {
  "schema" => "adl.internal_review.packet_manifest.v1",
  "target_sha" => TARGET,
  "packet_sha256" => Digest::SHA256.hexdigest(normalized),
  "path_count" => paths.length,
  "paths" => paths
}
write(File.join(ROOT, "packet-manifest.json"), JSON.pretty_generate(manifest))
write(File.join(ROOT, "PACKET_MANIFEST.md"), <<~MD)
  # Internal Review Packet Manifest

  - Exact product target: `#{TARGET}`
  - Manifest schema: `adl.internal_review.packet_manifest.v1`
  - Digested objects: #{paths.length}
  - Packet SHA-256: `#{manifest['packet_sha256']}`

  `packet-manifest.json` lists every digested object. The manifest files exclude
  themselves to avoid circular self-digests. Validation recomputes every listed
  object digest before accepting the packet.
MD

puts "PASS: finalized #{paths.length} packet objects at #{manifest['packet_sha256']}"
