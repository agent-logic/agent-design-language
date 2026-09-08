#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"
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

def git_blob(revision, path)
  output, error, status = Open3.capture3("git", "show", "#{revision}:#{path}")
  fail!("canonical candidate artifact unavailable: #{path}: #{error.strip}") unless status.success?
  output
end

def validate_fixture!(fixture)
  authority = fixture.fetch("opening_authority")
  fail!("fixture base is not derived from WP-01") unless authority == {
    "issue" => 480, "pull_request" => 527,
    "merge_sha" => fixture.fetch("opening_merge_sha"),
    "base_sha" => fixture.fetch("base_sha")
  }
  fail!("fixture milestone snapshot is incomplete") unless fixture.dig("milestone_snapshot", "pagination_complete") == true
  live_issues = fixture.dig("milestone_snapshot", "issues")
  fail!("fixture live issue denominator is empty") unless live_issues.is_a?(Array) && live_issues.any?
  fail!("fixture canonical planned mapping differs from WP-01 receipt") unless fixture.fetch("planned_mapping") == fixture.fetch("receipt_mapping")
  fail!("fixture live query receipt is incomplete") unless fixture.dig("milestone_snapshot", "api_receipt", "final_has_next_page") == false && fixture.dig("milestone_snapshot", "api_receipt", "response_digest_valid") == true
  fail!("fixture captured snapshot differs from external query result") unless live_issues == fixture.fetch("external_query_issues")
  live_prs = live_issues.flat_map { |row| row.fetch("pull_requests") }.sort
  fail!("fixture issue denominator differs from live snapshot") unless fixture.fetch("issue_rows").sort == live_issues.map { |row| row.fetch("number") }.sort
  fail!("fixture PR denominator differs from live snapshot") unless fixture.fetch("pr_rows").sort == live_prs
  fail!("fixture must reject empty repo/acceptance denominators") unless fixture.fetch("repo_rows").any? && fixture.fetch("acceptance_rows").any?
  fail!("fixture must reject empty assignments/results") unless fixture.fetch("assignments").any? && fixture.fetch("results").any?
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  validate_fixture!(fixture)
  puts JSON.generate(status: "passed", fixture: ARGV[1])
  exit
end

root = ENV.fetch("ADL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-04")
mode = ARGV.fetch(0, "all")
fail!("unsupported mode: #{mode}") unless %w[all denominator findings integrity].include?(mode)
required = %w[run_manifest.json live-milestone-snapshot.json repo_inventory.json issue_inventory.json acceptance_coverage.json assignments.json lane-results.json findings.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }

manifest = docs.fetch("run_manifest.json")
base = manifest.fetch("base_sha")
candidate = manifest.fetch("candidate_sha")
fail!("base/candidate must be full distinct git SHAs") unless full_sha?(base) && full_sha?(candidate) && base != candidate
fail!("candidate is not the canonical #519 merge") unless manifest.fetch("candidate_source_issue") == 519 && manifest.fetch("candidate_merge_sha") == candidate
opening = manifest.fetch("opening_authority")
fail!("base authority is not canonical WP-01/#480/PR #527") unless opening.fetch("issue") == 480 && opening.fetch("pull_request") == 527
opening_merge = opening.fetch("merge_sha")
fail!("opening merge must be a full SHA") unless full_sha?(opening_merge)
system("git", "cat-file", "-e", "#{base}^{commit}") or fail!("base commit is unavailable")
system("git", "cat-file", "-e", "#{candidate}^{commit}") or fail!("candidate commit is unavailable")
system("git", "cat-file", "-e", "#{opening_merge}^{commit}") or fail!("opening merge is unavailable")
opening_parents = `git show -s --format=%P #{opening_merge}`.split
fail!("base must be the sole parent of the immutable WP-01 merge") unless opening_parents == [base] && opening.fetch("base_sha") == base
system("git", "merge-base", "--is-ancestor", base, candidate) or fail!("base is not ancestral to candidate")
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
planned_rows = issue_rows.select { |row| nonempty?(row["planned_id"]) }
fail!("issue inventory does not exactly match canonical specification") unless planned_rows.map { |row| row.fetch("planned_id") }.sort == planned_ids

snapshot = docs.fetch("live-milestone-snapshot.json")
fail!("milestone snapshot source is wrong") unless snapshot.fetch("repository") == "agent-logic/agent-design-language" && snapshot.fetch("milestone") == "v0.92.1"
api_receipt = snapshot.fetch("api_receipt")
fail!("milestone API receipt is incomplete") unless api_receipt.fetch("transport") == "github_graphql" && api_receipt.fetch("page_size") == 100 && api_receipt.fetch("page_count").positive? && api_receipt.fetch("final_has_next_page") == false && nonempty?(api_receipt.fetch("retrieved_at"))
query = api_receipt.fetch("query")
fail!("API receipt query digest mismatch") unless Digest::SHA256.hexdigest(query) == api_receipt.fetch("query_sha256")
fail!("API receipt query does not prove cursor pagination and PR projection") unless %w[issues pageInfo hasNextPage endCursor closedByPullRequestsReferences].all? { |token| query.include?(token) }
response_path = api_receipt.fetch("response_path")
fail!("milestone API response is missing") unless File.file?(response_path)
fail!("milestone API response digest mismatch") unless Digest::SHA256.file(response_path).hexdigest == api_receipt.fetch("response_sha256")
fail!("milestone snapshot is capped or incomplete") unless snapshot.fetch("pagination_complete") == true && snapshot.fetch("next_cursor").nil? && snapshot.fetch("query_limit").nil?
live_issues = snapshot.fetch("issues")
fail!("live milestone snapshot is empty") unless live_issues.any?
live_numbers = live_issues.map { |row| row.fetch("number") }
fail!("live milestone snapshot duplicates issues") unless live_numbers.uniq.length == live_numbers.length
live_prs = live_issues.flat_map { |row| row.fetch("pull_requests") }
captured_rows = read_json(response_path).fetch("issues")
fail!("snapshot differs from immutable API response") unless captured_rows == live_issues

query_argv = ["gh", "issue", "list", "--repo", "agent-logic/agent-design-language", "--milestone", "v0.92.1", "--state", "all", "--limit", "10000", "--json", "number,title,state,closedByPullRequestsReferences"]
live_out, live_err, live_status = Open3.capture3(*query_argv)
fail!("live milestone query failed: #{live_err.strip}") unless live_status.success?
live_now = JSON.parse(live_out).map do |row|
  {"number" => row.fetch("number"), "title" => row.fetch("title"), "state" => row.fetch("state"), "pull_requests" => row.fetch("closedByPullRequestsReferences").map { |pr| pr.fetch("number") }.sort}
end.sort_by { |row| row.fetch("number") }
fail!("captured milestone snapshot is stale, truncated, or invented") unless live_issues.sort_by { |row| row.fetch("number") } == live_now

creation_receipt_path = "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"
creation_receipt = JSON.parse(git_blob(candidate, creation_receipt_path))
fail!("WP-01 creation receipt is not verified") unless creation_receipt.fetch("live_verified") == true && creation_receipt.fetch("child_count") == creation_receipt.fetch("children").length
canonical_mapping = {"WP-01" => 480}.merge(creation_receipt.fetch("children").to_h { |row| [row.fetch("planned_id"), row.fetch("issue")] })
planned_mapping = planned_rows.to_h { |row| [row.fetch("planned_id"), row.fetch("issue")] }
fail!("planned-ID to live-issue mapping differs from immutable WP-01 receipt") unless planned_mapping == canonical_mapping
fail!("issue inventory does not cover every live milestone issue") unless issue_rows.map { |row| row.fetch("issue") }.sort == live_numbers.sort
fail!("issue inventory PR census differs from live milestone snapshot") unless issue_rows.flat_map { |row| row.fetch("pull_requests") }.sort == live_prs.sort
fail!("issue inventory has duplicate, stale, or undispositioned rows") unless issue_rows.map { |row| row.fetch("issue") }.uniq.length == issue_rows.length && issue_rows.all? { |row| row.fetch("issue").is_a?(Integer) && nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("state")) && nonempty?(row.fetch("retrieved_at")) && nonempty?(row.fetch("disposition")) && nonempty?(row.fetch("evidence")) && row.fetch("pull_requests").is_a?(Array) }

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

findings_doc = docs.fetch("findings.json")
findings = findings_doc.fetch("findings")
fail!("findings register is not exact-candidate bound") unless findings_doc.fetch("candidate_sha") == candidate
fail!("findings outcome contradicts content") unless findings_doc.fetch("outcome") == (findings.empty? ? "passed" : "findings")
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
