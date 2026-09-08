#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
end

def nonempty?(value)
  value.respond_to?(:empty?) && !value.empty?
end

def git_blob(revision, path)
  output, error, status = Open3.capture3("git", "show", "#{revision}:#{path}")
  fail!("merged predecessor artifact unavailable: #{path}: #{error.strip}") unless status.success?
  output
end

def evidence_resolves?(evidence, candidate, root)
  return false unless evidence.is_a?(Hash) && %w[path sha256 source].all? { |key| nonempty?(evidence[key]) }
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

def validate_fixture!(fixture)
  fail!("independence requires evidence") unless fixture.fetch("independent") == true && nonempty?(fixture.fetch("independence_evidence"))
  canonical = fixture.fetch("internal_refs")
  fail!("canonical #520 scope cannot be empty") unless nonempty?(canonical)
  fail!("fixture does not consume merged #520 artifacts") unless fixture.fetch("predecessor_merged") == true && fixture.fetch("artifacts_from_merge") == true
  fail!("fixture lacks passing #520 semantic attestation") unless fixture.fetch("internal_semantic_validation") == "passed"
  fail!("fixture lacks retained #520 invocation proof") unless fixture.fetch("invocation_receipt_valid") == true
  fail!("fixture lacks current exact-head #520 review authority") unless fixture.fetch("internal_exact_head_review") == "passed"
  fail!("packet-authored expected scope differs from #520") unless fixture.fetch("expected_scope").sort == canonical.sort
  fail!("reviewed scope differs from #520") unless fixture.fetch("reviewed_scope").sort == canonical.sort && fixture.fetch("reviewed_scope").uniq.length == canonical.length
  fail!("fixture scope rows do not cover reviewed scope") unless fixture.fetch("scope_rows").map { |row| row.fetch("ref") }.sort == fixture.fetch("reviewed_scope").sort
  fail!("raw independent output is missing or unreconciled") unless fixture.fetch("raw_output_digest_valid") == true && fixture.fetch("raw_scope") == fixture.fetch("scope_rows") && fixture.fetch("raw_findings") == fixture.fetch("findings")
  fail!("zero findings require evidence") if fixture.fetch("findings").empty? && !nonempty?(fixture.fetch("zero_findings_evidence"))
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  validate_fixture!(fixture)
  puts JSON.generate(status: "passed", fixture: ARGV[1])
  exit
end

root = ENV.fetch("ADL_EXTERNAL_REVIEW_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-05")
required = %w[run_manifest.json reviewer-independence.json provider-request.json provider-invocation-receipt.json standard-runner-receipt.json provider-native-response.json raw-review-output.json scope.json findings.json limitations.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing external-review artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }
manifest = docs.fetch("run_manifest.json")
candidate = manifest.fetch("candidate_sha")
fail!("candidate must be a current full git SHA") unless candidate.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{candidate}^{commit}")
predecessor = manifest.fetch("internal_review_predecessor")
fail!("wrong internal-review predecessor") unless predecessor.fetch("issue") == 520 && predecessor.fetch("pull_request").is_a?(Integer)
internal_merge = predecessor.fetch("merge_sha")
fail!("#520 merge SHA is invalid") unless internal_merge.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{internal_merge}^{commit}")
issue_json, issue_error, issue_status = Open3.capture3("gh", "issue", "view", "520", "--repo", "agent-logic/agent-design-language", "--json", "state,closedByPullRequestsReferences")
fail!("cannot verify merged #520 authority: #{issue_error.strip}") unless issue_status.success?
issue_state = JSON.parse(issue_json)
fail!("#520 is not closed by declared PR") unless issue_state.fetch("state") == "CLOSED" && issue_state.fetch("closedByPullRequestsReferences").any? { |pr| pr.fetch("number") == predecessor.fetch("pull_request") }
pr_json, pr_error, pr_status = Open3.capture3("gh", "pr", "view", predecessor.fetch("pull_request").to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,mergedAt,mergeCommit,headRefOid")
fail!("cannot verify #520 PR: #{pr_error.strip}") unless pr_status.success?
pr_state = JSON.parse(pr_json)
fail!("declared #520 output is not the exact merged PR") unless pr_state.fetch("state") == "MERGED" && nonempty?(pr_state.fetch("mergedAt")) && pr_state.dig("mergeCommit", "oid") == internal_merge
reviewed_head = predecessor.fetch("reviewed_head_sha")
fail!("#520 reviewed head differs from merged PR head") unless reviewed_head == pr_state.fetch("headRefOid")
internal_manifest_path = manifest.fetch("internal_packet_manifest")
internal_manifest_blob = git_blob(internal_merge, internal_manifest_path)
fail!("#520 packet digest mismatch") unless Digest::SHA256.hexdigest(internal_manifest_blob) == manifest.fetch("internal_packet_manifest_sha256")
internal_manifest = JSON.parse(internal_manifest_blob)
fail!("candidate differs from #520 packet") unless manifest.fetch("internal_candidate_sha") == candidate && internal_manifest.fetch("candidate_sha") == candidate
internal_entries = internal_manifest.fetch("entries")
internal_entries.each do |entry|
  path = entry.fetch("path")
  fail!("#520 merged artifact digest mismatch: #{path}") unless Digest::SHA256.hexdigest(git_blob(internal_merge, path)) == entry.fetch("sha256")
end
internal_by_name = internal_entries.to_h { |entry| [File.basename(entry.fetch("path")), entry.fetch("path")] }
denominator_names = %w[repo_inventory.json issue_inventory.json acceptance_coverage.json]
fail!("#520 manifest omits canonical denominator artifacts") unless denominator_names.all? { |name| internal_by_name.key?(name) }
canonical_refs = denominator_names.flat_map do |name|
  JSON.parse(git_blob(internal_merge, internal_by_name.fetch(name))).fetch("rows").map { |row| row.fetch("denominator_ref") }
end
fail!("#520 canonical denominator is empty or duplicated") unless canonical_refs.any? && canonical_refs.uniq.length == canonical_refs.length
review_artifact_names = %w[findings.json lane-results.json proof-results.json validation-results.json redaction-report.json quality-report.json packet-manifest.json]
fail!("#520 manifest omits full review output") unless review_artifact_names.all? { |name| internal_by_name.key?(name) || name == "packet-manifest.json" }
review_artifact_refs = review_artifact_names.map { |name| "internal-artifact:#{name}" }
canonical_refs = (canonical_refs + review_artifact_refs).uniq
attestation = manifest.fetch("internal_semantic_validation")
fail!("#520 packet manifest omits semantic-validation evidence") unless internal_entries.any? { |entry| entry.fetch("path") == attestation.fetch("evidence_path") }
attestation_blob = git_blob(internal_merge, attestation.fetch("evidence_path"))
fail!("#520 semantic-validation evidence digest mismatch") unless Digest::SHA256.hexdigest(attestation_blob) == attestation.fetch("sha256")
attestation_doc = JSON.parse(attestation_blob)
fail!("#520 semantics were not validated at the reviewed candidate") unless attestation.fetch("outcome") == "passed" && attestation_doc.fetch("status") == "passed" && attestation_doc.fetch("candidate_sha") == candidate && attestation_doc.fetch("validator") == ".csdlc/prepared/issues/520/validate-internal-review.rb"
invocation = manifest.fetch("internal_validation_invocation")
fail!("#520 packet manifest omits invocation receipt") unless internal_entries.any? { |entry| entry.fetch("path") == invocation.fetch("receipt_path") }
invocation_blob = git_blob(internal_merge, invocation.fetch("receipt_path"))
fail!("#520 invocation receipt digest mismatch") unless Digest::SHA256.hexdigest(invocation_blob) == invocation.fetch("sha256")
invocation_doc = JSON.parse(invocation_blob)
validator_path = ".csdlc/prepared/issues/520/validate-internal-review.rb"
fail!("#520 invocation used wrong validator") unless invocation_doc.fetch("validator_path") == validator_path && invocation_doc.fetch("validator_sha256") == Digest::SHA256.hexdigest(git_blob(reviewed_head, validator_path))
fail!("#520 invocation argv/exit/candidate is invalid") unless invocation_doc.fetch("argv") == ["ruby", validator_path, "all"] && invocation_doc.fetch("exit_status") == 0 && invocation_doc.fetch("candidate_sha") == candidate
stdout = invocation_doc.fetch("stdout")
fail!("#520 invocation stdout digest mismatch") unless Digest::SHA256.hexdigest(stdout) == invocation_doc.fetch("stdout_sha256")
stdout_doc = JSON.parse(stdout)
fail!("#520 invocation stdout does not prove semantic pass") unless stdout_doc.fetch("status") == "passed" && stdout_doc.fetch("candidate_sha") == candidate
review_authority = manifest.fetch("internal_exact_head_review")
review_path = review_authority.fetch("receipt_path")
fail!("#520 exact-head review receipt is outside #521 packet") unless review_path.start_with?(root + "/") && File.file?(review_path)
review_blob = File.read(review_path)
fail!("#520 exact-head review receipt digest mismatch") unless Digest::SHA256.hexdigest(review_blob) == review_authority.fetch("sha256")
review_doc = JSON.parse(review_blob)
fail!("#520 lacks passing current exact-head review authority") unless review_doc.fetch("reviewed_sha") == reviewed_head && review_doc.fetch("outcome") == "passed" && review_doc.fetch("findings") == [] && nonempty?(review_doc.fetch("reviewer"))

independence = docs.fetch("reviewer-independence.json")
reviewer = independence.fetch("reviewer")
fail!("reviewer independence is not established with evidence") unless independence.fetch("independent") == true && nonempty?(reviewer) && evidence_resolves?(independence.fetch("evidence"), candidate, root)
conflicted_roles = independence.fetch("implementation_reviewers") + independence.fetch("internal_reviewers")
fail!("external reviewer participated in implementation/internal review") if conflicted_roles.include?(reviewer)
raw_path = File.join(root, "raw-review-output.json")
fail!("raw independent output digest mismatch") unless Digest::SHA256.file(raw_path).hexdigest == independence.fetch("raw_output_sha256")
fail!("raw reviewer provenance is incomplete") unless %w[provider model invocation_id observed_at].all? { |key| nonempty?(independence.fetch(key)) }
raw = docs.fetch("raw-review-output.json")
fail!("raw output identity is inconsistent") unless raw.fetch("candidate_sha") == candidate && raw.fetch("reviewer") == reviewer
request_path = File.join(root, "provider-request.json")
receipt = docs.fetch("provider-invocation-receipt.json")
fail!("provider request digest mismatch") unless Digest::SHA256.file(request_path).hexdigest == receipt.fetch("request_sha256")
fail!("provider response digest mismatch") unless Digest::SHA256.file(raw_path).hexdigest == receipt.fetch("response_sha256")
request_doc = docs.fetch("provider-request.json")
%w[provider model invocation_id].each do |key|
  fail!("provider invocation provenance mismatch") unless request_doc.fetch(key) == receipt.fetch(key) && raw.fetch(key) == receipt.fetch(key) && independence.fetch(key) == receipt.fetch(key)
end
fail!("provider invocation request is not candidate/scope bound") unless request_doc.fetch("candidate_sha") == candidate && request_doc.fetch("scope_refs").sort == canonical_refs.sort && nonempty?(request_doc.fetch("prompt"))
fail!("provider invocation receipt is incomplete") unless receipt.fetch("exit_status") == 0 && nonempty?(receipt.fetch("observed_at"))
native_path = File.join(root, "provider-native-response.json")
fail!("provider-native response digest mismatch") unless Digest::SHA256.file(native_path).hexdigest == receipt.fetch("provider_native_response_sha256")
native = docs.fetch("provider-native-response.json")
fail!("provider-native response is not invocation-bound") unless native.fetch("provider") == receipt.fetch("provider") && native.fetch("model") == receipt.fetch("model") && native.fetch("invocation_id") == receipt.fetch("invocation_id") && nonempty?(native.fetch("response_id")) && nonempty?(native.fetch("content"))
runner = docs.fetch("standard-runner-receipt.json")
fail!("standard runner did not produce the retained provider exchange") unless runner.fetch("runner") == "docs/tooling/OPUS_REVIEW_RUNBOOK.md" && runner.fetch("request_sha256") == receipt.fetch("request_sha256") && runner.fetch("response_sha256") == receipt.fetch("response_sha256") && runner.fetch("provider_native_response_sha256") == receipt.fetch("provider_native_response_sha256") && runner.fetch("exit_status") == 0

scope = docs.fetch("scope.json")
expected_scope = scope.fetch("expected_refs")
reviewed_scope = scope.fetch("reviewed_refs")
fail!("packet-authored expected scope differs from #520") unless expected_scope.sort == canonical_refs.sort
fail!("external review scope does not exactly cover #520") unless reviewed_scope.sort == canonical_refs.sort && reviewed_scope.uniq.length == canonical_refs.length
scope_rows = scope.fetch("rows")
fail!("scope rows do not exactly match reviewed scope") unless scope_rows.map { |row| row.fetch("ref") }.sort == reviewed_scope.sort
fail!("scope rows lack exact-head evidence or disposition") unless scope_rows.all? { |row| row.fetch("candidate_sha") == candidate && evidence_resolves?(row.fetch("evidence"), candidate, root) && nonempty?(row.fetch("disposition")) }
fail!("raw reviewer scope/dispositions differ from retained projection") unless raw.fetch("scope_rows").sort_by { |row| row.fetch("ref") } == scope_rows.sort_by { |row| row.fetch("ref") }

findings_doc = docs.fetch("findings.json")
findings = findings_doc.fetch("findings")
fail!("external findings register is not exact-candidate bound") unless findings_doc.fetch("candidate_sha") == candidate
fail!("external findings outcome contradicts content") unless findings_doc.fetch("outcome") == (findings.empty? ? "passed" : "findings")
fail!("raw reviewer findings differ from retained projection") unless raw.fetch("findings").sort_by { |row| row.fetch("id") } == findings.sort_by { |row| row.fetch("id") }
fail!("zero findings lack affirmative review evidence") if findings.empty? && !nonempty?(findings_doc.fetch("zero_findings_evidence"))
ids = findings.map { |finding| finding.fetch("id") }
fail!("finding IDs are not unique") unless ids.uniq.length == ids.length
fail!("finding schema is incomplete, stale, or cites unresolved evidence") unless findings.all? { |finding| %w[P0 P1 P2 P3].include?(finding.fetch("severity")) && finding.fetch("revision") == candidate && %w[status impact disposition_route].all? { |key| nonempty?(finding.fetch(key)) } && evidence_resolves?(finding.fetch("evidence"), candidate, root) }

limitations_doc = docs.fetch("limitations.json")
limitations = limitations_doc.fetch("limitations")
fail!("limitations must be an array") unless limitations.is_a?(Array)
fail!("empty limitations lack affirmative evidence") if limitations.empty? && !nonempty?(limitations_doc.fetch("no_limitations_evidence"))
fail!("limitation rows lack consequence and handling") unless limitations.all? { |row| nonempty?(row.fetch("description")) && nonempty?(row.fetch("consequence")) && nonempty?(row.fetch("handling")) }
fail!("raw reviewer limitations differ from retained projection") unless raw.fetch("limitations") == limitations
observations = raw.fetch("observations")
fail!("raw reviewer output is content-free or cites unresolved evidence") unless observations.is_a?(Array) && observations.any? && observations.all? { |row| reviewed_scope.include?(row.fetch("ref")) && evidence_resolves?(row.fetch("evidence"), candidate, root) && nonempty?(row.fetch("conclusion")) }
fail!("raw reviewer output omits reviewed scope") unless observations.map { |row| row.fetch("ref") }.sort == reviewed_scope.sort

entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| paths.include?(File.join(root, name)) }
fail!("packet manifest omits #520 exact-head review authority") unless paths.include?(review_path)
puts JSON.generate(schema: "adl.v0921.external_review_validation.v2", status: "passed", scope: reviewed_scope.length, findings: findings.length, limitations: limitations.length)
