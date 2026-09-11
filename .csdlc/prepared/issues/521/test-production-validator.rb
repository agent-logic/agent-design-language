#!/usr/bin/env ruby

require "fileutils"
require "json"
require "tmpdir"
require_relative "validate-external-review"

ROOT = "docs/milestones/v0.92.1/evidence/release/tail-05"

def reject_mutation!(name)
  Dir.mktmpdir("issue-521-validator-", File.expand_path("../../../../.adl", __dir__)) do |directory|
    fixture_root = File.join(directory, ROOT)
    FileUtils.mkdir_p(fixture_root)
    FileUtils.cp_r(Dir.glob(File.join(ROOT, "*")), fixture_root)
    yield fixture_root
    rejected = false
    begin
      validate_packet!(root: fixture_root)
    rescue SystemExit, KeyError, TypeError
      rejected = true
    end
    abort("negative mutation passed: #{name}") unless rejected
    puts JSON.generate(status: "passed", production_negative: name)
  end
end

result = validate_packet!(root: ROOT)
abort("valid retained review packet failed") unless result[:status] == "passed"

reject_mutation!("invented_candidate") do |root|
  path = File.join(root, "findings.json")
  document = JSON.parse(File.read(path))
  document["candidate_sha"] = "0" * 40
  File.write(path, JSON.pretty_generate(document) + "\n")
end

reject_mutation!("dropped_finding") do |root|
  path = File.join(root, "findings.json")
  document = JSON.parse(File.read(path))
  document.fetch("findings").pop
  File.write(path, JSON.pretty_generate(document) + "\n")
end

reject_mutation!("changed_remediation_route") do |root|
  path = File.join(root, "findings.json")
  document = JSON.parse(File.read(path))
  document.fetch("findings").first["remediation_issue"] = 522
  File.write(path, JSON.pretty_generate(document) + "\n")
end

reject_mutation!("false_identity_verification") do |root|
  path = File.join(root, "findings.json")
  document = JSON.parse(File.read(path))
  document.fetch("reviewer")["identity_verified"] = true
  File.write(path, JSON.pretty_generate(document) + "\n")
end

reject_mutation!("digest_mismatch") do |root|
  File.write(File.join(root, "README.md"), "tampered\n")
end

reject_mutation!("changed_source_digest_with_recomputed_manifest") do |root|
  findings_path = File.join(root, "findings.json")
  manifest_path = File.join(root, "packet-manifest.json")
  document = JSON.parse(File.read(findings_path))
  document.fetch("source")["sha256"] = "0" * 64
  File.write(findings_path, JSON.pretty_generate(document) + "\n")
  manifest = JSON.parse(File.read(manifest_path))
  manifest.fetch("entries").find { |entry| entry.fetch("path").end_with?("findings.json") }["sha256"] = Digest::SHA256.file(findings_path).hexdigest
  File.write(manifest_path, JSON.pretty_generate(manifest) + "\n")
end

reject_mutation!("changed_finding_with_recomputed_manifest") do |root|
  findings_path = File.join(root, "findings.json")
  manifest_path = File.join(root, "packet-manifest.json")
  document = JSON.parse(File.read(findings_path))
  document.fetch("findings").first["title"] = "Substantive finding silently replaced"
  File.write(findings_path, JSON.pretty_generate(document) + "\n")
  manifest = JSON.parse(File.read(manifest_path))
  manifest.fetch("entries").find { |entry| entry.fetch("path").end_with?("findings.json") }["sha256"] = Digest::SHA256.file(findings_path).hexdigest
  File.write(manifest_path, JSON.pretty_generate(manifest) + "\n")
end

reject_mutation!("overstated_reviewer_independence") do |root|
  findings_path = File.join(root, "findings.json")
  document = JSON.parse(File.read(findings_path))
  document.fetch("reviewer").fetch("independence")["status"] = "independent"
  File.write(findings_path, JSON.pretty_generate(document) + "\n")
end

puts JSON.generate(result)
