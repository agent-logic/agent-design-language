#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-05"
required = %w[run_manifest.json reviewer-independence.json findings.json limitations.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing external-review artifacts: #{missing.join(', ')}") unless missing.empty?
required.each { |name| JSON.parse(File.read(File.join(root, name))) }
puts JSON.generate(schema: "adl.v0921.external_review_validation.v1", status: "passed", artifacts: required.length)
