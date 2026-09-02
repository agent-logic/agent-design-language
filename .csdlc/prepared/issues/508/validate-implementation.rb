#!/usr/bin/env ruby
require "json"
require "shellwords"
root = File.expand_path("../../../..", __dir__)
receipt = JSON.parse(File.read(File.join(root, "docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json")))
abort("invalid exact revision") unless receipt.fetch("runtime_revision").match?(/\A[0-9a-f]{40}\z/)
runtime_subject = `git -C #{root.shellescape} log -n 1 --format=%H -- adl-runtime/Cargo.toml adl-runtime/src/qualification/mod.rs adl-runtime/tests/distributed_failure/drt_c_qualification.rs`.strip
abort("runtime revision does not bind latest Runtime qualification source commit") unless receipt.fetch("runtime_revision") == runtime_subject
expected_failures = %w[identity provider transport]
abort("failure denominator mismatch") unless receipt.fetch("fail_closed_cases").sort == expected_failures.sort
observatory = receipt.fetch("observatory")
abort("Observatory evidence is not Runtime-authentic") unless observatory.fetch("runtime_emitted") == true && !observatory.fetch("artifact_sha256").to_s.empty?
soak = receipt.fetch("soak")
required_windows = %w[local-production-window hybrid-production-window]
abort("soak windows mismatch") unless soak.fetch("bounded") == true && soak.fetch("required_windows").sort == required_windows.sort
attempts = soak.fetch("attempts")
abort("soak attempt denominator mismatch") unless attempts.map { |attempt| attempt.fetch("id") }.sort == required_windows.sort
duration_sum = 0
attempts.each do |attempt|
  duration = attempt.fetch("duration_seconds").to_i
  duration_sum += duration
  abort("soak attempt has invalid clock bounds") unless duration > 0 && attempt.fetch("ended_at_unix_seconds").to_i == attempt.fetch("started_at_unix_seconds").to_i + duration
  abort("soak attempt is not bound to runtime revision") unless attempt.fetch("source_revision") == receipt.fetch("runtime_revision")
  %w[command_digest model_digest receipt_digest].each do |field|
    abort("soak #{field} missing") unless attempt.fetch(field).match?(/\A[0-9a-f]{64}\z/)
  end
  abort("soak replay/cleanup proof missing") unless attempt.fetch("independent_replay") == true && attempt.fetch("cleanup_readback") == "absent"
end
abort("soak total denominator incomplete") unless soak.fetch("total_duration_seconds").to_i == duration_sum && duration_sum >= 1800
abort("synthesis missing") unless receipt.fetch("decision").is_a?(String) && !receipt.fetch("decision").empty?
abort("cleanup not proven") unless receipt.fetch("cleanup").values.all? { |v| v == "absent" }
puts '{"schema":"adl.v0921.drt_c.implementation.v1","outcome":"passed"}'
