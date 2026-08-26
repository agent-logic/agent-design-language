#!/usr/bin/env ruby
# frozen_string_literal: true

ROOT = File.expand_path("../../../..", __dir__)
MANIFEST = File.join(ROOT, "adl/Cargo.toml")
FILTERS = %w[
  execute_retry_policy
  retry_policy_delay_is_deterministic_and_bounded_by_jitter
  execute_timeout_policy
  timeout_event_and_artifact_ids_remain_unique_across_repeated_emissions
]

FILTERS.each do |filter|
  cmd = ["cargo", "test", "--manifest-path", MANIFEST, filter, "--", "--test-threads=1"]
  puts cmd.join(" ")
  abort "RUST-01 retry/timeout/cancellation proof failed for #{filter}" unless system(*cmd, chdir: ROOT)
end

puts "RUST-01 retry/timeout/cancellation proof passed"
