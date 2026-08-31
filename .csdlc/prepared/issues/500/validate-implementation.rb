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
expected_requirements = {
  161 => %w[
    issue-161-ac-1-public-command-output-contracts
    issue-161-ac-2-v2-invariant-owner-proof-map
    issue-161-ac-3-unweakened-review-github-topology-state-cleanup
    issue-161-ac-4-explicit-reviewed-v2-drift
    issue-161-ac-5-importer-retention-window
    issue-161-ac-6-in-process-filter-template-boundary
    issue-161-ac-7-reviewer-independence-check
    issue-161-ac-8-closing-vs-partof-publication
    issue-161-ac-9-authoritative-field-owner-matrix
    issue-161-ac-10-capability-matrix-derived-help-auth-tests
    issue-161-ac-11-state-size-warning-block-audit
    issue-161-ac-12-measured-largest-v2-bundle
    issue-161-ac-13-architecture-review-on-impractical-state-size
    issue-161-ac-14-v3-16-canary-sizing
    issue-161-ac-15-frozen-jq-subset
    issue-161-ac-16-official-cli-source-baseline
  ],
  162 => %w[
    issue-162-ac-1-one-binary-one-library-four-layers
    issue-162-ac-2-parse-without-repo-credentials-network-child-task
    issue-162-ac-3-fake-adapter-determinism
    issue-162-ac-4-github-operation-capability-classification
    issue-162-ac-5-end-to-end-recovery-journey
    issue-162-ac-6-measurement-threshold-stop-go
    issue-162-ac-7-decision-11-not-satisfied-by-recommendation
  ],
  163 => %w[
    issue-163-ac-1-platform-commit-primitive-durability
    issue-163-ac-2-windows-proven-or-fail-closed-read-only
    issue-163-ac-3-operator-decision-cites-v3-02-evidence
    issue-163-ac-4-v3-08-blocked-until-terminal
  ]
}
entries.each do |entry|
  issue = Integer(entry.fetch("issue"))
  requirements = entry.fetch("requirements")
  ids = requirements.map { |requirement| requirement.fetch("id") }
  abort("requirement denominator mismatch for ##{issue}") unless ids.sort == expected_requirements.fetch(issue).sort
  abort("duplicate requirement disposition for ##{issue}") unless ids.uniq.length == ids.length
  requirements.each do |requirement|
    abort("source acceptance missing for ##{issue}/#{requirement.fetch('id')}") unless requirement.fetch("source_acceptance").match?(/\AAC-\d+\z/)
    abort("requirement disposition missing for ##{issue}/#{requirement.fetch('id')}") unless requirement.fetch("disposition").strip == "retained"
    maps_to = requirement.fetch("maps_to")
    abort("requirement mapping missing for ##{issue}/#{requirement.fetch('id')}") unless maps_to.is_a?(Array) && maps_to.any? { |target| !target.strip.empty? }
  end
end

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
abort("routine sprint readiness budget must be three minutes or less") unless (1..3).cover?(default_path.fetch("three_issue_ready_minutes_max"))
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
diff_base = ENV.fetch("ADL_PR_BASE", "origin/main")
diff_head = ENV.fetch("ADL_PR_HEAD", "HEAD")
ok = system("git", "-C", root, "diff", "--check", "#{diff_base}...#{diff_head}")
abort("diff hygiene failed for #{diff_base}...#{diff_head}") unless ok

puts '{"schema":"adl.v0921.v3a_implementation.v1","outcome":"passed","issue":500}'
