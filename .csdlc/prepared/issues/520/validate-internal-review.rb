#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-04"
mode = ARGV.fetch(0, "all")
required = %w[run_manifest.json repo_inventory.json issue_inventory.json acceptance_coverage.json findings.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing review artifacts: #{missing.join(', ')}") unless missing.empty?
required.each { |name| JSON.parse(File.read(File.join(root, name))) }
abort("unsupported mode: #{mode}") unless %w[all denominator findings integrity].include?(mode)
puts JSON.generate(schema: "adl.v0921.internal_review_validation.v1", mode: mode, status: "passed", artifacts: required.length)
