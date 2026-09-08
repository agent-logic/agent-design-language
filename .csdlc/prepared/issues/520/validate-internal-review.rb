#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-04"
mode = ARGV.fetch(0, "all")
required = %w[run_manifest.json repo_inventory.json issue_inventory.json acceptance_coverage.json findings.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing review artifacts: #{missing.join(', ')}") unless missing.empty?
abort("unsupported mode: #{mode}") unless %w[all denominator findings integrity].include?(mode)

docs = required.to_h { |name| [name, JSON.parse(File.read(File.join(root, name)))] }
manifest = docs.fetch("run_manifest.json")
base = manifest.fetch("base_sha")
candidate = manifest.fetch("candidate_sha")
abort("base/candidate must be full distinct git SHAs") unless [base, candidate].all? { |sha| sha.match?(/\A[0-9a-f]{40}\z/) } && base != candidate
changed = `git diff --name-only #{base}...#{candidate}`.lines.map(&:strip).reject(&:empty?).sort
abort("candidate range is unavailable") unless $?.success?

repo_rows = docs.fetch("repo_inventory.json").fetch("rows")
reviewed_paths = repo_rows.map { |row| row.fetch("path") }.sort
abort("repo inventory does not exactly match changed-file denominator") unless reviewed_paths == changed
abort("repo inventory has unreviewed rows") unless repo_rows.all? { |row| row.fetch("classification") && row.fetch("disposition") && row.fetch("review_lane") }

issue_rows = docs.fetch("issue_inventory.json").fetch("rows")
abort("issue inventory is empty") if issue_rows.empty?
abort("issue inventory has duplicate or undispositioned rows") unless issue_rows.map { |row| row.fetch("issue") }.uniq.length == issue_rows.length && issue_rows.all? { |row| row.fetch("state") && row.fetch("disposition") }

acceptance_rows = docs.fetch("acceptance_coverage.json").fetch("rows")
abort("acceptance inventory is empty or has missing implementation/proof dispositions") if acceptance_rows.empty? || acceptance_rows.any? { |row| !row.fetch("acceptance_id") || !row.fetch("implementation_disposition") || !row.fetch("proof_disposition") }

findings = docs.fetch("findings.json").fetch("findings")
ids = findings.map { |finding| finding.fetch("id") }
abort("finding IDs are not unique") unless ids.uniq.length == ids.length
abort("finding schema is incomplete") unless findings.all? { |finding| %w[severity status title impact evidence revision source_lane owner].all? { |key| finding.key?(key) && !finding[key].to_s.empty? } }

entries = docs.fetch("packet-manifest.json").fetch("entries")
manifest_paths = entries.map { |entry| entry.fetch("path") }
abort("packet manifest omits required artifacts") unless required.all? { |name| manifest_paths.include?(File.join(root, name)) }

puts JSON.generate(schema: "adl.v0921.internal_review_validation.v1", mode: mode, status: "passed", changed_paths: changed.length, issues: issue_rows.length, acceptance_surfaces: acceptance_rows.length, findings: findings.length)
