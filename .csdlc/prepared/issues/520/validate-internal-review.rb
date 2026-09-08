#!/usr/bin/env ruby
require "digest"
require "json"
require "yaml"

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
end

def full_sha?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{40}\z/)
end

def nonempty?(value)
  value.respond_to?(:empty?) && !value.empty?
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  fail!("fixture must reject empty denominators") unless fixture.fetch("repo_rows").any? && fixture.fetch("issue_rows").any? && fixture.fetch("acceptance_rows").any?
  fail!("fixture must reject empty assignments/results") unless fixture.fetch("assignments").any? && fixture.fetch("results").any?
  puts JSON.generate(status: "passed", fixture: ARGV[1])
  exit
end

root = ENV.fetch("ADL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-04")
mode = ARGV.fetch(0, "all")
fail!("unsupported mode: #{mode}") unless %w[all denominator findings integrity].include?(mode)
required = %w[run_manifest.json repo_inventory.json issue_inventory.json acceptance_coverage.json assignments.json lane-results.json findings.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }

manifest = docs.fetch("run_manifest.json")
base = manifest.fetch("base_sha")
candidate = manifest.fetch("candidate_sha")
fail!("base/candidate must be full distinct git SHAs") unless full_sha?(base) && full_sha?(candidate) && base != candidate
fail!("candidate is not the canonical #519 merge") unless manifest.fetch("candidate_source_issue") == 519 && manifest.fetch("candidate_merge_sha") == candidate
fail!("base source is not immutable") unless nonempty?(manifest.fetch("base_ref")) && manifest.fetch("base_ref_sha") == base
system("git", "cat-file", "-e", "#{base}^{commit}") or fail!("base commit is unavailable")
system("git", "cat-file", "-e", "#{candidate}^{commit}") or fail!("candidate commit is unavailable")
system("git", "merge-base", "--is-ancestor", base, candidate) or fail!("base is not ancestral to candidate")
resolved_base = `git rev-parse #{manifest.fetch('base_ref')}^{commit}`.strip
fail!("base ref does not resolve to retained base SHA") unless $?.success? && resolved_base == base
live_519 = manifest.fetch("tail_03_observation")
fail!("#519 live observation does not prove the candidate") unless live_519.fetch("issue") == 519 && live_519.fetch("state") == "CLOSED" && live_519.fetch("merge_sha") == candidate && nonempty?(live_519.fetch("retrieved_at"))

changed = `git diff --name-only #{base}...#{candidate}`.lines.map(&:strip).reject(&:empty?).sort
fail!("candidate range is unavailable") unless $?.success?
repo_rows = docs.fetch("repo_inventory.json").fetch("rows")
fail!("repo inventory does not exactly match changed-file denominator") unless repo_rows.map { |row| row.fetch("path") }.sort == changed
fail!("repo inventory contains unreviewed rows") unless repo_rows.all? { |row| nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("classification")) && nonempty?(row.fetch("disposition")) && nonempty?(row.fetch("review_lane")) && nonempty?(row.fetch("evidence")) }

specs = YAML.safe_load(File.read("docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")).fetch("issue_specifications")
planned_ids = specs.map { |row| row.fetch("id") }.sort
issue_rows = docs.fetch("issue_inventory.json").fetch("rows")
fail!("issue inventory does not exactly match canonical specification") unless issue_rows.map { |row| row.fetch("planned_id") }.sort == planned_ids
fail!("issue inventory has duplicate, stale, or undispositioned rows") unless issue_rows.map { |row| row.fetch("issue") }.uniq.length == issue_rows.length && issue_rows.all? { |row| row.fetch("issue").is_a?(Integer) && nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("state")) && nonempty?(row.fetch("retrieved_at")) && nonempty?(row.fetch("disposition")) && nonempty?(row.fetch("evidence")) }

expected_acceptance = specs.flat_map { |row| row.fetch("acceptance_criteria").each_index.map { |index| "#{row.fetch('id')}:AC-#{index + 1}" } }.sort
acceptance_rows = docs.fetch("acceptance_coverage.json").fetch("rows")
actual_acceptance = acceptance_rows.map { |row| "#{row.fetch('planned_id')}:#{row.fetch('acceptance_id')}" }.sort
fail!("acceptance inventory does not exactly match canonical specification") unless actual_acceptance == expected_acceptance
fail!("acceptance inventory has missing implementation/proof dispositions") unless acceptance_rows.all? { |row| nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("implementation_disposition")) && nonempty?(row.fetch("proof_disposition")) && nonempty?(row.fetch("evidence")) }

all_refs = (repo_rows + issue_rows + acceptance_rows).map { |row| row.fetch("denominator_ref") }
fail!("denominator references are not unique") unless all_refs.uniq.length == all_refs.length
assignments = docs.fetch("assignments.json").fetch("assignments")
results = docs.fetch("lane-results.json").fetch("results")
fail!("assignments/results cannot be empty") if assignments.empty? || results.empty?
assigned_refs = assignments.flat_map { |row| row.fetch("denominator_refs") }
fail!("assignments do not cover each denominator row exactly once") unless assigned_refs.sort == all_refs.sort && assigned_refs.uniq.length == assigned_refs.length
assignment_ids = assignments.map { |row| row.fetch("id") }
fail!("assignment IDs are not unique") unless assignment_ids.uniq.length == assignment_ids.length
fail!("lane results do not match assignments exactly") unless results.map { |row| row.fetch("assignment_id") }.sort == assignment_ids.sort
fail!("lane result is empty, stale, or evidence-free") unless results.all? { |row| %w[passed findings].include?(row.fetch("outcome")) && row.fetch("candidate_sha") == candidate && nonempty?(row.fetch("reviewer")) && nonempty?(row.fetch("evidence")) }
fail!("test lane reports zero executed tests") if results.any? { |row| row.fetch("lane").match?(/test|pvf|ci/i) && row.fetch("tests_run", 0).to_i <= 0 }

findings = docs.fetch("findings.json").fetch("findings")
ids = findings.map { |finding| finding.fetch("id") }
fail!("finding IDs are not unique") unless ids.uniq.length == ids.length
fail!("finding schema is incomplete or stale") unless findings.all? { |finding| finding.fetch("revision") == candidate && %w[severity status title impact evidence source_lane owner].all? { |key| nonempty?(finding.fetch(key)) } }

entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
manifest_paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| manifest_paths.include?(File.join(root, name)) }

puts JSON.generate(schema: "adl.v0921.internal_review_validation.v2", mode: mode, status: "passed", changed_paths: changed.length, issues: issue_rows.length, acceptance_surfaces: acceptance_rows.length, assignments: assignments.length, findings: findings.length)
