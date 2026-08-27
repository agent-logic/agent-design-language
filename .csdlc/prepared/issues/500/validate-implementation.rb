#!/usr/bin/env ruby
# frozen_string_literal: true
require "json"
require "open3"
root = File.expand_path("../../../..", __dir__)
required = ["csdlc-v3/Cargo.toml", "csdlc-v3/src/lib.rs"]
missing = required.reject { |path| File.file?(File.join(root, path)) }
abort("missing V3-A implementation targets: #{missing.join(', ')}") unless missing.empty?

contract = File.read(File.join(root, "docs/csdlc-v3/CONTRACT.md"))
abort("missing v2 sole-authority boundary") unless contract.match?(/v2 remains (the )?sole operational authority/i)
abort("missing rollback decision") unless contract.downcase.include?("rollback")
abort("v3 claims operational authority during V3-A") if contract.match?(/v3 (is|becomes|has) (the )?(sole )?operational authority/i)

coverage = JSON.parse(File.read(File.join(root, "docs/csdlc-v3/predecessor-coverage.json")))
entries = coverage.fetch("entries")
issues = entries.map { |entry| Integer(entry.fetch("issue")) }
abort("predecessor denominator must be exactly 161, 162, 163") unless issues.sort == [161, 162, 163] && issues.uniq.length == 3
abort("predecessor disposition missing") unless entries.all? { |entry| !entry.fetch("disposition").strip.empty? }

matrix = JSON.parse(File.read(File.join(root, "docs/csdlc-v3/proportional-lifecycle.json")))
expected_surfaces = %w[sip stp spp vpp srp sor design_review readiness bind implementation_review publication finish cleanup sprint_umbrella_review generation_digest_cas]
rows = matrix.fetch("surfaces")
ids = rows.map { |row| row.fetch("id") }
abort("lifecycle denominator mismatch or duplicate classification") unless ids.sort == expected_surfaces.sort && ids.uniq.length == expected_surfaces.length
allowed = %w[retained collapsed derived removed]
abort("invalid lifecycle disposition") unless rows.all? { |row| allowed.include?(row.fetch("disposition")) }
abort("retained gate lacks named hazard") unless rows.all? { |row| row.fetch("disposition") != "retained" || !row.fetch("hazard").strip.empty? }

default_path = matrix.fetch("default_path")
abort("default path must have one design gate") unless default_path.fetch("design_gates") == 1
abort("default path must use focused validation") unless default_path.fetch("validation") == "focused"
abort("default path must have one implementation review") unless default_path.fetch("implementation_reviews") == 1
abort("default path must have one truthful closeout") unless default_path.fetch("closeouts") == 1
abort("routine sprint readiness budget is not minutes-scale") unless (1..10).cover?(default_path.fetch("three_issue_ready_minutes_max"))
abort("duplicate authority is permitted") unless matrix.fetch("duplicate_authority") == "forbidden"
abort("umbrella re-review of child proof is permitted") unless matrix.fetch("umbrella_repeats_child_proof") == false

test_argv = ["cargo", "test", "--manifest-path", File.join(root, "csdlc-v3/Cargo.toml"), "--", "--list"]
test_list, test_error, test_status = Open3.capture3(*test_argv)
abort("unable to enumerate V3-A contract tests: #{test_error}") unless test_status.success?
%w[contract_schema predecessor_coverage architecture_boundary proportional_lifecycle].each do |name|
  abort("missing proving Rust test: #{name}") unless test_list.include?(name)
end
ok = system("cargo", "test", "--manifest-path", File.join(root, "csdlc-v3/Cargo.toml"))
abort("focused V3-A contract tests failed") unless ok
ok = system("git", "-C", root, "diff", "--check")
abort("diff hygiene failed") unless ok

puts '{"schema":"adl.v0921.v3a_implementation.v1","outcome":"passed","issue":500}'
