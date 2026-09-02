#!/usr/bin/env ruby
require "json"
root = File.expand_path("../../../..", __dir__)
receipt = JSON.parse(File.read(File.join(root, "docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json")))
abort("invalid exact revision") unless receipt.fetch("runtime_revision").match?(/\A[0-9a-f]{40}\z/)
expected_failures = %w[identity provider transport]
abort("failure denominator mismatch") unless receipt.fetch("fail_closed_cases").sort == expected_failures.sort
observatory = receipt.fetch("observatory")
abort("Observatory evidence is not Runtime-authentic") unless observatory.fetch("runtime_emitted") == true && !observatory.fetch("artifact_sha256").to_s.empty?
soak = receipt.fetch("soak")
abort("soak is not bounded") unless soak.fetch("bounded") == true && soak.fetch("duration_seconds").to_i > 0
abort("synthesis missing") unless receipt.fetch("decision").is_a?(String) && !receipt.fetch("decision").empty?
abort("cleanup not proven") unless receipt.fetch("cleanup").values.all? { |v| v == "absent" }
puts '{"schema":"adl.v0921.drt_c.implementation.v1","outcome":"passed"}'
