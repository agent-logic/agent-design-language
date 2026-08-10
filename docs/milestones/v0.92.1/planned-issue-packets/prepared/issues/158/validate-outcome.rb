#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "time"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 158
ID = "corp-06"
EVIDENCE_DIR = File.join(ROOT, "docs/milestones/v0.92.1/evidence/corporate/corp-06")
OPERATIONS_DIR = File.join(ROOT, "docs/operations/corporate/corp-06")
MANIFEST_PATH = File.join(ROOT, ".csdlc/evidence/158/evidence-manifest.json")
REQUIRED_ARTIFACTS = ["infrastructure-migration-manifest.json", "cutover-rollback-receipts.json", "temporary-resource-cleanup.json", "aws-migration-runbook.md"].freeze
REQUIRED_RECEIPTS = ["account-identity", "service-canary", "rollback-rehearsal", "cleanup-readback"].freeze
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

infra = load_json(artifact("infrastructure-migration-manifest.json"), "infrastructure manifest")
cutover = load_json(artifact("cutover-rollback-receipts.json"), "cutover receipts")
cleanup = load_json(artifact("temporary-resource-cleanup.json"), "cleanup receipts")
assert(infra["profile"] == "agent-logic-admin" && infra["business_account_verified"] == true, "business AWS identity gate failed")
tls = hash(infra, "public_tls")
assert(%w[acm public_ca].include?(tls["issuer_class"]), "public TLS issuer is not trusted")
assert(tls["self_signed"] == false, "self-signed production certificate detected")
required_services = %w[dns tls email storage cdn workload monitoring backup budget]
services = array(infra, "services")
assert((required_services - services.map { |row| row["kind"] }).empty?, "AWS service denominator incomplete")
phases = array(cutover, "phases")
assert(!phases.empty? && phases.all? { |row| row["canary"] == "verified" && row["rollback"] == "verified" }, "cutover or rollback phase is unverified")
resources = array(cleanup, "resources")
assert(resources.all? { |row| row["tagged"] == true && row["provider_readback"] == "absent" }, "temporary resource remains or lacks cleanup proof")
assert(cleanup["remaining_count"] == 0, "temporary resource count is nonzero")
require_runbook_sections(artifact("aws-migration-runbook.md"), %w[Account\ Gate Baseline Phases Canary Rollback Cleanup Escalation])

REQUIRED_ARTIFACTS.grep(/\.json\z/).each do |name|
  payload = load_json(artifact(name), name)
  findings = scan_forbidden(payload)
  assert(findings.empty?, "#{name} contains forbidden/self-attested fields: #{findings.join(", ")}")
end

puts "PASS: #{ID} exact artifacts, producer receipts, revision, digests, and issue-specific invariants"
