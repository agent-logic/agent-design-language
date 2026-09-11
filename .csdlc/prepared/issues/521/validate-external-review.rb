#!/usr/bin/env ruby

require "digest"
require "json"

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  fail!("invalid JSON at #{path}: #{error.message}")
end

def nonempty_string?(value)
  value.is_a?(String) && !value.strip.empty?
end

def validate_packet!(root:)
  root = File.expand_path(root)
  readme_path = File.join(root, "README.md")
  findings_path = File.join(root, "findings.json")
  manifest_path = File.join(root, "packet-manifest.json")
  [readme_path, findings_path, manifest_path].each do |path|
    fail!("missing retained review artifact: #{path}") unless File.file?(path)
  end

  findings_doc = read_json(findings_path)
  fail!("wrong findings schema") unless findings_doc["schema"] == "adl.v0921.third_party_review_findings.v1"
  fail!("wrong review issue") unless findings_doc["review_issue"] == 521
  fail!("failed review must not claim an exact candidate") unless findings_doc["candidate_sha"].nil?
  fail!("review outcome must remain failed and non-proving") unless findings_doc["outcome"] == "failed" && findings_doc["non_proving"] == true && findings_doc["verdict"] == "changes_required"

  reviewer = findings_doc.fetch("reviewer")
  fail!("reviewer independence is not recorded truthfully") unless reviewer["independent"] == true && reviewer["identity_verified"] == false && nonempty_string?(reviewer["recorded_identity"]) && reviewer.fetch("basis").is_a?(Array) && reviewer.fetch("basis").all? { |value| nonempty_string?(value) }

  source = findings_doc.fetch("source")
  fail!("source identity is incomplete") unless source["name"] == "ADL_v0.92.1_Third_Party_Review.pdf" && source["sha256"].match?(/\A[0-9a-f]{64}\z/) && source["pages"] == 5 && source["repository_copy"] == false

  findings = findings_doc.fetch("findings")
  expected_ids = (1..5).map { |number| format("TPR-%03d", number) }
  fail!("review must retain exactly the five source findings") unless findings.map { |row| row["id"] } == expected_ids
  fail!("review severity count changed") unless findings.count { |row| row["severity"] == "P1" } == 3 && findings.count { |row| row["severity"] == "P2" } == 2
  required_fields = %w[id severity status title evidence impact remediation remediation_issue]
  fail!("a finding is incomplete") unless findings.all? do |row|
    required_fields.all? { |field| row.key?(field) } &&
      %w[title evidence impact remediation].all? { |field| nonempty_string?(row[field]) } &&
      %w[blocking open].include?(row["status"]) &&
      row["remediation_issue"].is_a?(Integer)
  end
  expected_routes = {
    "TPR-001" => 833,
    "TPR-002" => 834,
    "TPR-003" => 835,
    "TPR-004" => 836,
    "TPR-005" => 837
  }
  actual_routes = findings.to_h { |row| [row.fetch("id"), row.fetch("remediation_issue")] }
  fail!("finding remediation routing changed") unless actual_routes == expected_routes

  limitations = findings_doc.fetch("limitations")
  fail!("review limitations are missing") unless limitations.is_a?(Array) && limitations.length == 5 && limitations.all? { |value| nonempty_string?(value) }

  manifest = read_json(manifest_path)
  fail!("wrong packet manifest schema") unless manifest["schema"] == "adl.v0921.third_party_review_packet_manifest.v1"
  fail!("manifest identity differs from findings") unless manifest["review_issue"] == 521 && manifest["candidate_sha"].nil? && manifest["outcome"] == "failed" && manifest["non_proving"] == true
  entries = manifest.fetch("entries")
  packet_relative_root = "docs/milestones/v0.92.1/evidence/release/tail-05"
  expected_paths = %w[README.md findings.json].map { |name| File.join(packet_relative_root, name) }.sort
  actual_paths = entries.map { |entry| entry.fetch("path") }.sort
  fail!("manifest must bind exactly README.md and findings.json") unless actual_paths == expected_paths
  entries.each do |entry|
    path = File.join(root, File.basename(entry.fetch("path")))
    fail!("manifested artifact is missing: #{entry.fetch('path')}") unless File.file?(path)
    fail!("artifact digest mismatch: #{entry.fetch('path')}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
  end

  {
    schema: "adl.v0921.external_review_validation.v3",
    status: "passed",
    outcome: "failed",
    non_proving: true,
    findings: findings.length,
    p1: 3,
    p2: 2
  }
end

if __FILE__ == $PROGRAM_NAME
  root = ENV.fetch("ADL_EXTERNAL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-05")
  puts JSON.generate(validate_packet!(root: root))
end
