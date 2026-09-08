#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-06"
mode = ARGV.fetch(0, "all")
required = %w[source-findings.json dispositions.json release-blockers.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing remediation artifacts: #{missing.join(', ')}") unless missing.empty?
required.each { |name| JSON.parse(File.read(File.join(root, name))) }
abort("unsupported mode: #{mode}") unless %w[all census dispositions].include?(mode)
puts JSON.generate(schema: "adl.v0921.remediation_validation.v1", mode: mode, status: "passed", artifacts: required.length)
