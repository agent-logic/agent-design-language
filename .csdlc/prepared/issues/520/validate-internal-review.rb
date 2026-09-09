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

def evidence_resolves?(evidence, candidate, root, subject_id:)
  return false unless evidence.is_a?(Hash) && %w[path sha256 source].all? { |key| nonempty?(evidence[key]) }
  return false unless evidence["subject_id"] == subject_id
  locator = evidence["locator"]
  return false unless locator.is_a?(Hash) && ((nonempty?(locator["path"]) && locator["line"].is_a?(Integer) && locator["line"].positive?) || nonempty?(locator["command"]))
  path = evidence.fetch("path")
  content = case evidence.fetch("source")
            when "candidate"
              return false unless evidence["revision"] == candidate
              output, status = Open3.capture2("git", "show", "#{candidate}:#{path}")
              return false unless status.success?
              output
            when "packet"
              return false unless path.start_with?(root + "/") && File.file?(path)
              File.binread(path)
            else
              return false
            end
  Digest::SHA256.hexdigest(content) == evidence.fetch("sha256")
end

def validate_packet!(root:, mode: "all")
fail!("unsupported mode: #{mode}") unless %w[all denominator findings integrity].include?(mode)
required = %w[run_manifest.json live-milestone-snapshot.json repo_inventory.json canonical-surface-inventory.json issue_inventory.json pull_request_inventory.json acceptance_coverage.json assignments.json lane-results.json findings.json proof-results.json validation-results.json redaction-report.json quality-report.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }

manifest = docs.fetch("run_manifest.json")
base = manifest.fetch("base_sha")
candidate = manifest.fetch("candidate_sha")
fail!("base/candidate must be full distinct git SHAs") unless full_sha?(base) && full_sha?(candidate) && base != candidate
fail!("candidate source is not the frozen post-remediation origin/main revision") unless manifest.fetch("candidate_source") == "origin_main_after_review_gates" && manifest.fetch("candidate_merge_sha") == candidate
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
gates = manifest.fetch("review_gate_observations")
expected_gates = [[718, 809], [758, 805]]
actual_gates = gates.map { |gate| [gate.fetch("issue"), gate.fetch("pull_request")] }.sort
fail!("review gate observations do not exactly cover #718 and #758") unless actual_gates == expected_gates
gates.each do |gate|
  merge_sha = gate.fetch("merge_sha")
  fail!("review gate merge SHA is invalid") unless full_sha?(merge_sha)
  fail!("review gate is not terminal") unless gate.fetch("issue_state") == "CLOSED" && gate.fetch("pr_state") == "MERGED" && nonempty?(gate.fetch("merged_at")) && nonempty?(gate.fetch("issue_closed_at"))
  system("git", "cat-file", "-e", "#{merge_sha}^{commit}") or fail!("review gate merge commit is unavailable")
  system("git", "merge-base", "--is-ancestor", merge_sha, candidate) or fail!("review gate merge is not ancestral to candidate")
  pr_out, pr_err, pr_status = Open3.capture3("gh", "pr", "view", gate.fetch("pull_request").to_s, "--repo", "agent-logic/agent-design-language", "--json", "number,state,headRefOid,mergeCommit,mergedAt")
  fail!("live review-gate PR query failed: #{pr_err.strip}") unless pr_status.success?
  live_pr = JSON.parse(pr_out)
  issue_out, issue_err, issue_status = Open3.capture3("gh", "issue", "view", gate.fetch("issue").to_s, "--repo", "agent-logic/agent-design-language", "--json", "number,state,closedAt,closedByPullRequestsReferences")
  fail!("live review-gate issue query failed: #{issue_err.strip}") unless issue_status.success?
  live_issue = JSON.parse(issue_out)
  live_closing_prs = live_issue.fetch("closedByPullRequestsReferences").map { |row| row.fetch("number") }
  fail!("retained review-gate observation differs from live GitHub state") unless
    live_pr.fetch("number") == gate.fetch("pull_request") && live_pr.fetch("state") == "MERGED" &&
    live_pr.fetch("headRefOid") == gate.fetch("head_sha") && live_pr.dig("mergeCommit", "oid") == merge_sha &&
    live_pr.fetch("mergedAt") == gate.fetch("merged_at") && live_issue.fetch("number") == gate.fetch("issue") &&
    live_issue.fetch("state") == "CLOSED" && live_issue.fetch("closedAt") == gate.fetch("issue_closed_at") &&
    live_closing_prs.include?(gate.fetch("pull_request"))
end

changed = `git diff --name-only #{base}...#{candidate}`.lines.map(&:strip).reject(&:empty?).sort
fail!("candidate range is unavailable") unless $?.success?
repo_rows = docs.fetch("repo_inventory.json").fetch("rows")
fail!("repo inventory does not exactly match changed-file denominator") unless repo_rows.map { |row| row.fetch("path") }.sort == changed
fail!("repo inventory contains unreviewed or non-resolving rows") unless repo_rows.all? { |row| nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("classification")) && nonempty?(row.fetch("disposition")) && nonempty?(row.fetch("review_lane")) && evidence_resolves?(row.fetch("evidence"), candidate, root, subject_id: row.fetch("denominator_ref")) }

canonical_rows = docs.fetch("canonical-surface-inventory.json").fetch("rows")
canonical_kinds = %w[documentation demo provider_cloud retained_evidence]
fail!("canonical release review surfaces are incomplete") unless canonical_rows.map { |row| row.fetch("kind") }.uniq.sort == canonical_kinds.sort
fail!("canonical surfaces are not immutable-candidate bound") unless canonical_rows.all? { |row| nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("path")) && evidence_resolves?(row.fetch("evidence"), candidate, root, subject_id: row.fetch("denominator_ref")) }

spec_path = "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
spec_blob = git_blob(candidate, spec_path)
fail!("immutable execution-spec digest mismatch") unless Digest::SHA256.hexdigest(spec_blob) == manifest.fetch("execution_spec_sha256")
specs = YAML.safe_load(spec_blob).fetch("issue_specifications")
planned_ids = specs.map { |row| row.fetch("id") }.sort
issue_rows = docs.fetch("issue_inventory.json").fetch("rows")
planned_rows = issue_rows.select { |row| nonempty?(row["planned_id"]) }
fail!("issue inventory does not exactly match canonical specification") unless planned_rows.map { |row| row.fetch("planned_id") }.sort == planned_ids
fail!("planned IDs are duplicated") unless planned_rows.map { |row| row.fetch("planned_id") }.uniq.length == planned_rows.length

snapshot = docs.fetch("live-milestone-snapshot.json")
fail!("milestone snapshot source is wrong") unless snapshot.fetch("repository") == "agent-logic/agent-design-language" && snapshot.fetch("milestone") == "v0.92.1"
api_receipt = snapshot.fetch("api_receipt")
fail!("milestone API receipt is incomplete") unless api_receipt.fetch("transport") == "github_graphql" && api_receipt.fetch("page_size") == 100 && api_receipt.fetch("page_count").positive? && api_receipt.fetch("issue_page_count").positive? && api_receipt.fetch("closing_reference_page_count") >= snapshot.fetch("issues").length && api_receipt.fetch("pull_request_page_count").positive? && api_receipt.fetch("final_has_next_page") == false && nonempty?(api_receipt.fetch("retrieved_at"))
query = api_receipt.fetch("query")
fail!("API receipt query digest mismatch") unless Digest::SHA256.hexdigest(query) == api_receipt.fetch("query_sha256")
fail!("API receipt query does not prove complete cursor pagination and PR projection") unless %w[issues pullRequests milestone pageInfo hasNextPage endCursor closedByPullRequestsReferences].all? { |token| query.include?(token) }
response_path = api_receipt.fetch("response_path")
fail!("milestone API response is missing") unless File.file?(response_path)
fail!("milestone API response digest mismatch") unless Digest::SHA256.file(response_path).hexdigest == api_receipt.fetch("response_sha256")
fail!("milestone snapshot is capped or incomplete") unless snapshot.fetch("pagination_complete") == true && snapshot.fetch("next_cursor").nil? && snapshot.fetch("query_limit").nil?
live_issues = snapshot.fetch("issues")
fail!("live milestone snapshot is empty") unless live_issues.any?
live_numbers = live_issues.map { |row| row.fetch("number") }
fail!("live milestone snapshot duplicates issues") unless live_numbers.uniq.length == live_numbers.length
live_prs = snapshot.fetch("pull_requests")
live_pr_numbers = live_prs.map { |row| row.fetch("number") }
fail!("live milestone snapshot duplicates pull requests") unless live_pr_numbers.uniq.length == live_pr_numbers.length
captured_response = read_json(response_path)
fail!("snapshot differs from immutable API response") unless captured_response.fetch("issues") == live_issues && captured_response.fetch("pull_requests") == live_prs

query_argv = ["gh", "issue", "list", "--repo", "agent-logic/agent-design-language", "--milestone", "v0.92.1", "--state", "all", "--limit", "10000", "--json", "number,title,state,closedByPullRequestsReferences"]
live_out, live_err, live_status = Open3.capture3(*query_argv)
fail!("live milestone query failed: #{live_err.strip}") unless live_status.success?
live_now = JSON.parse(live_out).map do |row|
  {"number" => row.fetch("number"), "title" => row.fetch("title"), "state" => row.fetch("state"), "pull_requests" => row.fetch("closedByPullRequestsReferences").map { |pr| pr.fetch("number") }.sort}
end.sort_by { |row| row.fetch("number") }
fail!("captured milestone snapshot is stale, truncated, or invented") unless live_issues.sort_by { |row| row.fetch("number") } == live_now

pr_query_argv = ["gh", "pr", "list", "--repo", "agent-logic/agent-design-language", "--state", "all", "--limit", "10000", "--json", "number,title,state,mergedAt,url,milestone"]
pr_out, pr_err, pr_status = Open3.capture3(*pr_query_argv)
fail!("live milestone PR query failed: #{pr_err.strip}") unless pr_status.success?
live_prs_now = JSON.parse(pr_out)
  .select { |row| row.dig("milestone", "title") == "v0.92.1" }
  .map { |row| row.reject { |key, _value| key == "milestone" } }
  .sort_by { |row| row.fetch("number") }
fail!("captured milestone PR snapshot is stale, truncated, or invented") unless live_prs.sort_by { |row| row.fetch("number") } == live_prs_now

creation_receipt_path = "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"
creation_receipt = JSON.parse(git_blob(candidate, creation_receipt_path))
fail!("WP-01 creation receipt is not verified") unless creation_receipt.fetch("live_verified") == true && creation_receipt.fetch("child_count") == creation_receipt.fetch("children").length
canonical_mapping = {"WP-01" => 480}.merge(creation_receipt.fetch("children").to_h { |row| [row.fetch("planned_id"), row.fetch("issue")] })
planned_mapping = planned_rows.to_h { |row| [row.fetch("planned_id"), row.fetch("issue")] }
fail!("planned live issue mappings are duplicated") unless planned_mapping.values.uniq.length == planned_mapping.values.length
fail!("planned-ID to live-issue mapping differs from immutable WP-01 receipt") unless planned_mapping == canonical_mapping
fail!("issue inventory does not cover every live milestone issue") unless issue_rows.map { |row| row.fetch("issue") }.sort == live_numbers.sort
live_by_number = live_issues.to_h { |row| [row.fetch("number"), row] }
fail!("issue inventory differs from live title/state/PR authority") unless issue_rows.all? do |row|
  live = live_by_number.fetch(row.fetch("issue"))
  row.fetch("title") == live.fetch("title") && row.fetch("state") == live.fetch("state") && row.fetch("pull_requests").sort == live.fetch("pull_requests").sort
end
fail!("issue inventory has duplicate, stale, undispositioned, or non-resolving rows") unless issue_rows.map { |row| row.fetch("issue") }.uniq.length == issue_rows.length && issue_rows.all? { |row| row.fetch("issue").is_a?(Integer) && nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("state")) && nonempty?(row.fetch("retrieved_at")) && nonempty?(row.fetch("disposition")) && evidence_resolves?(row.fetch("evidence"), candidate, root, subject_id: row.fetch("denominator_ref")) && row.fetch("pull_requests").is_a?(Array) }

pull_request_rows = docs.fetch("pull_request_inventory.json").fetch("rows")
fail!("pull-request inventory does not exactly cover the live milestone") unless pull_request_rows.map { |row| row.fetch("pull_request") }.sort == live_pr_numbers.sort
live_prs_by_number = live_prs.to_h { |row| [row.fetch("number"), row] }
fail!("pull-request inventory differs from live title/state/merge authority") unless pull_request_rows.all? do |row|
  live = live_prs_by_number.fetch(row.fetch("pull_request"))
  row.fetch("title") == live.fetch("title") && row.fetch("state") == live.fetch("state") && row.fetch("merged_at") == live.fetch("mergedAt") && row.fetch("url") == live.fetch("url")
end
fail!("pull-request inventory has duplicate, stale, undispositioned, or non-resolving rows") unless pull_request_rows.map { |row| row.fetch("pull_request") }.uniq.length == pull_request_rows.length && pull_request_rows.all? { |row| row.fetch("pull_request").is_a?(Integer) && nonempty?(row.fetch("denominator_ref")) && nonempty?(row.fetch("state")) && nonempty?(row.fetch("retrieved_at")) && nonempty?(row.fetch("disposition")) && evidence_resolves?(row.fetch("evidence"), candidate, root, subject_id: row.fetch("denominator_ref")) }
fail!("manifest issue/PR counts differ from complete inventories") unless manifest.fetch("milestone_issue_count") == issue_rows.length && manifest.fetch("milestone_pull_request_count") == pull_request_rows.length

expected_acceptance = specs.flat_map { |row| row.fetch("acceptance_criteria").each_index.map { |index| "#{row.fetch('id')}:AC-#{index + 1}" } }.sort
canonical_acceptance = specs.flat_map do |spec|
  spec.fetch("acceptance_criteria").each_with_index.map do |criterion, index|
    ["#{spec.fetch('id')}:AC-#{index + 1}", criterion, Digest::SHA256.hexdigest(canonical_json = JSON.generate(criterion))]
  end
end.to_h { |ref, criterion, digest| [ref, {"criterion" => criterion, "sha256" => digest}] }
acceptance_rows = docs.fetch("acceptance_coverage.json").fetch("rows")
actual_acceptance = acceptance_rows.map { |row| "#{row.fetch('planned_id')}:#{row.fetch('acceptance_id')}" }.sort
fail!("acceptance inventory does not exactly match canonical specification") unless actual_acceptance == expected_acceptance
fail!("acceptance rows rewrite canonical criterion content") unless acceptance_rows.all? do |row|
  ref = "#{row.fetch('planned_id')}:#{row.fetch('acceptance_id')}"
  row.fetch("criterion") == canonical_acceptance.fetch(ref).fetch("criterion") && row.fetch("criterion_sha256") == canonical_acceptance.fetch(ref).fetch("sha256")
end
fail!("acceptance inventory has missing or non-resolving implementation/proof dispositions") unless acceptance_rows.all? { |row| nonempty?(row.fetch("denominator_ref")) && row.fetch("evidence").fetch("criterion_id") == "#{row.fetch('planned_id')}:#{row.fetch('acceptance_id')}" && nonempty?(row.fetch("implementation_disposition")) && nonempty?(row.fetch("proof_disposition")) && evidence_resolves?(row.fetch("evidence"), candidate, root, subject_id: row.fetch("denominator_ref")) }

all_refs = (repo_rows + canonical_rows + issue_rows + pull_request_rows + acceptance_rows).map { |row| row.fetch("denominator_ref") }
fail!("denominator references are not unique") unless all_refs.uniq.length == all_refs.length
assignments = docs.fetch("assignments.json").fetch("assignments")
results = docs.fetch("lane-results.json").fetch("results")
fail!("assignments/results cannot be empty") if assignments.empty? || results.empty?
assigned_refs = assignments.flat_map { |row| row.fetch("denominator_refs") }
fail!("assignments do not cover each denominator row exactly once") unless assigned_refs.sort == all_refs.sort && assigned_refs.uniq.length == assigned_refs.length
assignment_ids = assignments.map { |row| row.fetch("id") }
fail!("assignment IDs are not unique") unless assignment_ids.uniq.length == assignment_ids.length
mandatory_lanes = %w[code tests documentation security architecture dependency provider_cloud demos retained_evidence]
fail!("mandatory specialist lane set is incomplete") unless (mandatory_lanes - assignments.map { |row| row.fetch("lane") }).empty?
fail!("lane results do not match assignments exactly") unless results.map { |row| row.fetch("assignment_id") }.sort == assignment_ids.sort
raw_findings = []
results.each do |row|
  fail!("lane result is empty, stale, or evidence-free") unless %w[passed findings].include?(row.fetch("outcome")) && row.fetch("candidate_sha") == candidate && nonempty?(row.fetch("reviewer")) && nonempty?(row.fetch("evidence"))
  report_path = row.fetch("report_path")
  fail!("lane report is missing") unless File.file?(report_path)
  fail!("lane report digest mismatch") unless Digest::SHA256.file(report_path).hexdigest == row.fetch("report_sha256")
  report = read_json(report_path)
  assignment = assignments.find { |candidate_assignment| candidate_assignment.fetch("id") == row.fetch("assignment_id") }
  fail!("lane report is not bound to its complete assignment") unless report.fetch("candidate_sha") == candidate && report.fetch("denominator_refs").sort == assignment.fetch("denominator_refs").sort
  observations = report.fetch("observations")
  fail!("lane report is content-free or cites unresolved evidence") unless observations.is_a?(Array) && observations.any? && observations.all? { |observation| nonempty?(observation.fetch("ref")) && evidence_resolves?(observation.fetch("evidence"), candidate, root, subject_id: observation.fetch("ref")) && %w[finding verified_no_gap].include?(observation.fetch("conclusion")) && nonempty?(observation.fetch("detail")) }
  fail!("lane report omits assigned review rows") unless observations.map { |observation| observation.fetch("ref") }.sort == assignment.fetch("denominator_refs").sort
  report_findings = report.fetch("findings")
  fail!("lane report outcome contradicts findings") unless row.fetch("outcome") == (report_findings.empty? ? "passed" : "findings")
  raw_findings.concat(report_findings)
end
results.select { |row| row.fetch("lane").match?(/test|pvf|ci/i) }.each do |row|
  invocation = row.fetch("test_invocation")
  stdout = invocation.fetch("stdout")
  argv = invocation.fetch("argv")
  artifacts = invocation.fetch("command_artifacts")
  artifacts_valid = artifacts.any? && artifacts.all? { |artifact| Digest::SHA256.hexdigest(git_blob(candidate, artifact.fetch("path"))) == artifact.fetch("sha256") && Digest::SHA256.file(artifact.fetch("path")).hexdigest == artifact.fetch("sha256") }
  fresh_stdout, fresh_stderr, fresh_status = Open3.capture3(*argv, chdir: invocation.fetch("working_directory"))
  fail!("test lane lacks replayed immutable successful invocation proof: #{fresh_stderr.strip}") unless nonempty?(argv) && invocation.fetch("candidate_sha") == candidate && invocation.fetch("exit_status") == 0 && Digest::SHA256.hexdigest(stdout) == invocation.fetch("stdout_sha256") && nonempty?(stdout) && artifacts_valid && fresh_status.exitstatus == invocation.fetch("exit_status") && fresh_stdout == stdout
end

findings_doc = docs.fetch("findings.json")
findings = findings_doc.fetch("findings")
fail!("findings register is not exact-candidate bound") unless findings_doc.fetch("candidate_sha") == candidate
fail!("findings outcome contradicts content") unless findings_doc.fetch("outcome") == (findings.empty? ? "passed" : "findings")
ids = findings.map { |finding| finding.fetch("id") }
fail!("finding IDs are not unique") unless ids.uniq.length == ids.length
fail!("synthesized findings differ from raw lane union") unless findings.sort_by { |row| row.fetch("id") } == raw_findings.sort_by { |row| row.fetch("id") }
fail!("finding schema is incomplete, stale, or cites unresolved evidence") unless findings.all? do |finding|
  locator = finding.fetch("locator")
  concrete_locator = (nonempty?(locator["path"]) && locator["line"].is_a?(Integer) && locator["line"].positive?) || nonempty?(locator["command"])
  %w[P0 P1 P2 P3].include?(finding.fetch("severity")) && finding.fetch("revision") == candidate && %w[status title impact source_lane owner].all? { |key| nonempty?(finding.fetch(key)) } && nonempty?(finding.fetch("affected_acceptance_refs")) && finding.fetch("affected_acceptance_refs").all? { |ref| expected_acceptance.include?(ref) } && concrete_locator && evidence_resolves?(finding.fetch("evidence"), candidate, root, subject_id: finding.fetch("id"))
end

%w[proof-results.json validation-results.json redaction-report.json quality-report.json].each do |name|
  artifact = docs.fetch(name)
  observations = artifact.fetch("observations")
  fail!("#{name} is not a contentful passing exact-candidate artifact") unless artifact.fetch("candidate_sha") == candidate && artifact.fetch("outcome") == "passed" && observations.is_a?(Array) && observations.any? && observations.all? { |observation| %w[verified finding].include?(observation.fetch("result")) && nonempty?(observation.fetch("detail")) && evidence_resolves?(observation.fetch("evidence"), candidate, root, subject_id: observation.fetch("subject_id")) }
end

entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
manifest_paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| manifest_paths.include?(File.join(root, name)) }
fail!("packet manifest omits lane reports") unless results.all? { |row| manifest_paths.include?(row.fetch("report_path")) }
fail!("packet manifest omits raw milestone API response") unless manifest_paths.include?(response_path)

  {schema: "adl.v0921.internal_review_validation.v2", mode: mode, status: "passed", candidate_sha: candidate, changed_paths: changed.length, issues: issue_rows.length, acceptance_surfaces: acceptance_rows.length, assignments: assignments.length, findings: findings.length}
end

if __FILE__ == $PROGRAM_NAME
  root = ENV.fetch("ADL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-04")
  puts JSON.generate(validate_packet!(root: root, mode: ARGV.fetch(0, "all")))
end
