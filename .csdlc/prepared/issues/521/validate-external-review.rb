#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-05"
required = %w[run_manifest.json reviewer-independence.json findings.json limitations.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing external-review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, JSON.parse(File.read(File.join(root, name)))] }
manifest = docs.fetch("run_manifest.json")
candidate = manifest.fetch("candidate_sha")
abort("candidate must be a full git SHA") unless candidate.match?(/\A[0-9a-f]{40}\z/)
independence = docs.fetch("reviewer-independence.json")
abort("reviewer independence is not established") unless independence.fetch("independent") == true && !independence.fetch("reviewer").to_s.empty? && independence.fetch("conflicts").is_a?(Array)
findings = docs.fetch("findings.json").fetch("findings")
ids = findings.map { |finding| finding.fetch("id") }
abort("finding IDs are not unique") unless ids.uniq.length == ids.length
abort("finding schema is incomplete or stale") unless findings.all? { |finding| finding.fetch("revision") == candidate && %w[severity status evidence impact disposition_route].all? { |key| finding.key?(key) && !finding[key].to_s.empty? } }
limitations = docs.fetch("limitations.json").fetch("limitations")
abort("limitations must be an array") unless limitations.is_a?(Array)
paths = docs.fetch("packet-manifest.json").fetch("entries").map { |entry| entry.fetch("path") }
abort("packet manifest omits required artifacts") unless required.all? { |name| paths.include?(File.join(root, name)) }
puts JSON.generate(schema: "adl.v0921.external_review_validation.v1", status: "passed", findings: findings.length, limitations: limitations.length)
