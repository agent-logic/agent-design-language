#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 155
ID = "corp-03"
EVIDENCE_DIR = File.join(ROOT, "docs/milestones/v0.92.1/evidence/corporate/corp-03")
OPERATIONS_DIR = File.join(ROOT, "docs/operations/corporate/corp-03")
MANIFEST_PATH = File.join(ROOT, ".csdlc/evidence/155/evidence-manifest.json")
REQUIRED_ARTIFACTS = ["provenance-matrix.json", "disposition-register.json", "source-verification-receipt.json", "review-boundary.md"].freeze
REQUIRED_RECEIPTS = ["manifest-scan", "source-verification", "exclusion-scan"].freeze
FORBIDDEN_KEYS = /(?:secret|password|token|credential|private_key|signature|address|privileged_advice)/i

def fail!(message)
  abort("FAIL: #{ID} #{message}")
end
def assert(condition, message)
  fail!(message) unless condition
end
def load_json(path, label)
  JSON.parse(File.read(path))
rescue Errno::ENOENT
  fail!("missing #{label}: #{path}")
rescue JSON::ParserError => error
  fail!("invalid #{label}: #{error.message}")
end
def array(value, key)
  result = value[key]
  assert(result.is_a?(Array), "#{key} must be an array")
  result
end
def hash(value, key)
  result = value[key]
  assert(result.is_a?(Hash), "#{key} must be an object")
  result
end
def present(value, key, label)
  field = value[key]
  assert(!(field.nil? || (field.respond_to?(:empty?) && field.empty?)), "#{label} missing #{key}")
end
def artifact(name)
  base = name.end_with?(".md") ? OPERATIONS_DIR : EVIDENCE_DIR
  File.join(base, name)
end
def require_runbook_sections(path, sections)
  body = File.read(path)
  sections.each { |section| assert(body.match?(/^##\s+#{Regexp.escape(section.tr("\\", " "))}\b/i), "runbook missing #{section.tr("\\", " ")}") }
rescue Errno::ENOENT
  fail!("missing runbook: #{path}")
end
def scan_forbidden(value, location = "$", findings = [])
  case value
  when Hash
    value.each do |key, child|
      findings << "#{location}.#{key}" if key.match?(FORBIDDEN_KEYS) || key == "passed"
      scan_forbidden(child, "#{location}.#{key}", findings)
    end
  when Array
    value.each_with_index { |child, index| scan_forbidden(child, "#{location}[#{index}]", findings) }
  end
  findings
end

manifest = load_json(MANIFEST_PATH, "evidence manifest")
assert(manifest["schema"] == "adl.corporate.#{ID}.evidence.v1", "wrong evidence-manifest schema")
assert(manifest["issue"] == ISSUE, "wrong issue in evidence manifest")
revision = manifest["source_revision"]
assert(revision.is_a?(String) && revision.match?(/\A[0-9a-f]{40}\z/), "invalid source revision")
_out, _err, revision_status = Open3.capture3("git", "cat-file", "-e", "#{revision}^{commit}", chdir: ROOT)
assert(revision_status.success?, "source revision is not a repository commit")
expected_revision = ENV["ADL_EXPECTED_REVISION"]
assert(expected_revision.nil? || expected_revision == revision, "source revision does not match ADL_EXPECTED_REVISION")

artifacts = hash(manifest, "artifacts")
expected_paths = REQUIRED_ARTIFACTS.to_h { |name| [artifact(name).delete_prefix(ROOT + "/"), name] }
assert(artifacts.keys.sort == expected_paths.keys.sort, "artifact denominator mismatch")
artifacts.each do |relative, expected_digest|
  path = File.join(ROOT, relative)
  assert(File.file?(path), "missing artifact #{relative}")
  assert(expected_digest.match?(/\A[0-9a-f]{64}\z/), "invalid digest for #{relative}")
  assert(Digest::SHA256.file(path).hexdigest == expected_digest, "digest mismatch for #{relative}")
end

receipts = array(manifest, "producer_receipts")
assert(receipts.map { |receipt| receipt["name"] }.sort == REQUIRED_RECEIPTS.sort, "producer-receipt denominator mismatch")
receipts.each do |receipt|
  %w[name producer observed_at revision command exit_code artifact_digests].each { |field| present(receipt, field, "producer receipt") }
  assert(receipt["revision"] == revision, "producer receipt revision mismatch")
  assert(receipt["exit_code"] == 0, "producer receipt command failed")
  assert(receipt["command"].is_a?(Array) && !receipt["command"].empty?, "producer command is not structured")
  Time.iso8601(receipt["observed_at"])
  receipt_digests = hash(receipt, "artifact_digests")
  assert(!receipt_digests.empty?, "producer receipt has no artifact digests")
  receipt_digests.each { |path, digest| assert(artifacts[path] == digest, "producer digest is not manifest-bound: #{path}") }
rescue ArgumentError
  fail!("producer receipt observed_at is not ISO-8601")
end
assert(scan_forbidden(manifest).empty?, "manifest contains forbidden/self-attested fields: #{scan_forbidden(manifest).join(", ")}")

matrix = load_json(artifact("provenance-matrix.json"), "provenance matrix")
dispositions = load_json(artifact("disposition-register.json"), "disposition register")
source_receipt = load_json(artifact("source-verification-receipt.json"), "source verification")
assert(matrix["schema"] == "adl.corporate.provenance-matrix.v1", "wrong provenance schema")
rows = array(matrix, "assets")
assert(!rows.empty?, "provenance denominator is empty")
rows.each do |row|
  %w[asset_id asset_class provenance_source source_digest use_rights_disposition evidence_refs reviewer_role].each { |field| present(row, field, "provenance row") }
  assert(row["source_digest"].match?(/\A[0-9a-f]{64}\z/), "invalid source digest")
  assert(%w[verified excluded blocked counsel_review].include?(row["use_rights_disposition"]), "invalid use-rights disposition")
end
routed = array(dispositions, "rows").to_h { |row| [row.fetch("asset_id"), row] }
rows.reject { |row| row["use_rights_disposition"] == "verified" }.each do |row|
  assert(routed.key?(row["asset_id"]), "restricted asset lacks disposition route")
  assert(routed[row["asset_id"]]["release_gate"] == "blocked", "restricted asset does not block release")
end
array(dispositions, "trademarks").each { |row| assert(row["legal_judgment"] != true || row["counsel_route"].to_s != "", "trademark legal judgment lacks counsel route") }
assert(source_receipt["authoritative_sources_verified"] == rows.length, "source-verification denominator mismatch")
require_runbook_sections(artifact("review-boundary.md"), %w[Factual\ Review Counsel\ Boundary Exclusions Failure Escalation])

REQUIRED_ARTIFACTS.grep(/\.json\z/).each do |name|
  payload = load_json(artifact(name), name)
  findings = scan_forbidden(payload)
  assert(findings.empty?, "#{name} contains forbidden/self-attested fields: #{findings.join(", ")}")
end

puts "PASS: #{ID} exact artifacts, producer receipts, revision, digests, and issue-specific invariants"
