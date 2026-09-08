#!/usr/bin/env ruby
require "digest"
require "json"

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
end

def nonempty?(value)
  value.respond_to?(:empty?) && !value.empty?
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  fail!("independence requires evidence") unless fixture.fetch("independent") == true && nonempty?(fixture.fetch("independence_evidence"))
  fail!("scope cannot be empty") unless nonempty?(fixture.fetch("expected_scope")) && fixture.fetch("expected_scope").sort == fixture.fetch("reviewed_scope").sort
  fail!("zero findings require evidence") if fixture.fetch("findings").empty? && !nonempty?(fixture.fetch("zero_findings_evidence"))
  puts JSON.generate(status: "passed", fixture: ARGV[1])
  exit
end

root = ENV.fetch("ADL_EXTERNAL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-05")
required = %w[run_manifest.json reviewer-independence.json scope.json findings.json limitations.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing external-review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }
manifest = docs.fetch("run_manifest.json")
candidate = manifest.fetch("candidate_sha")
fail!("candidate must be a current full git SHA") unless candidate.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{candidate}^{commit}")
internal_manifest_path = manifest.fetch("internal_packet_manifest")
fail!("#520 packet manifest is unavailable") unless File.file?(internal_manifest_path)
fail!("#520 packet digest mismatch") unless Digest::SHA256.file(internal_manifest_path).hexdigest == manifest.fetch("internal_packet_manifest_sha256")
internal_manifest = read_json(internal_manifest_path)
fail!("candidate differs from #520 packet") unless manifest.fetch("internal_candidate_sha") == candidate && internal_manifest.fetch("candidate_sha") == candidate

independence = docs.fetch("reviewer-independence.json")
reviewer = independence.fetch("reviewer")
fail!("reviewer independence is not established with evidence") unless independence.fetch("independent") == true && nonempty?(reviewer) && nonempty?(independence.fetch("evidence"))
conflicted_roles = independence.fetch("implementation_reviewers") + independence.fetch("internal_reviewers")
fail!("external reviewer participated in implementation/internal review") if conflicted_roles.include?(reviewer)

scope = docs.fetch("scope.json")
expected_scope = scope.fetch("expected_refs")
reviewed_scope = scope.fetch("reviewed_refs")
fail!("external review scope is empty or incomplete") unless nonempty?(expected_scope) && expected_scope.sort == reviewed_scope.sort && reviewed_scope.uniq.length == reviewed_scope.length
fail!("scope rows lack evidence or disposition") unless scope.fetch("rows").all? { |row| expected_scope.include?(row.fetch("ref")) && nonempty?(row.fetch("evidence")) && nonempty?(row.fetch("disposition")) }

findings_doc = docs.fetch("findings.json")
findings = findings_doc.fetch("findings")
fail!("zero findings lack affirmative review evidence") if findings.empty? && !nonempty?(findings_doc.fetch("zero_findings_evidence"))
ids = findings.map { |finding| finding.fetch("id") }
fail!("finding IDs are not unique") unless ids.uniq.length == ids.length
fail!("finding schema is incomplete or stale") unless findings.all? { |finding| finding.fetch("revision") == candidate && %w[severity status evidence impact disposition_route].all? { |key| nonempty?(finding.fetch(key)) } }

limitations_doc = docs.fetch("limitations.json")
limitations = limitations_doc.fetch("limitations")
fail!("limitations must be an array") unless limitations.is_a?(Array)
fail!("empty limitations lack affirmative evidence") if limitations.empty? && !nonempty?(limitations_doc.fetch("no_limitations_evidence"))
fail!("limitation rows lack consequence and handling") unless limitations.all? { |row| nonempty?(row.fetch("description")) && nonempty?(row.fetch("consequence")) && nonempty?(row.fetch("handling")) }

entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| paths.include?(File.join(root, name)) }
puts JSON.generate(schema: "adl.v0921.external_review_validation.v2", status: "passed", scope: reviewed_scope.length, findings: findings.length, limitations: limitations.length)
