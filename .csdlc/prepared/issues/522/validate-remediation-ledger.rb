#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

EXTERNAL_FINDING_IDS = %w[TPR-001 TPR-002 TPR-003 TPR-004 TPR-005].freeze
RETURNED_FINDING_IDS = %w[
  D833-REPORT-001
  D833-REPORT-002
  D833-REPORT-003
  D833-EXEC-001
  D833-EXEC-002
  D833-EXEC-003
].freeze
INTERNAL_FINDING_IDS = %w[
  D520-RET-001
  D520-V3F-001
  D520-REL-001
  D520-DOC-003
  D520-DOC-004
  D520-EVID-001
  D520-EVID-002
  D520-RUNTIME-001
  D520-RUNTIME-002
  D520-SEC-001
  D520-SEC-002
  D520-SEC-003
  D520-SEC-004
  D520-TEST-001
].freeze
EXPECTED_REMEDIATION_ISSUES = {
  "D520-RET-001" => [818, 819, 820, 821],
  "D520-V3F-001" => [817, 843],
  "D520-REL-001" => [817],
  "D520-DOC-003" => [817],
  "D520-DOC-004" => [817],
  "D520-EVID-001" => [817],
  "D520-EVID-002" => [817],
  "D520-RUNTIME-001" => [814],
  "D520-RUNTIME-002" => [814],
  "D520-SEC-001" => [815],
  "D520-SEC-002" => [815],
  "D520-SEC-003" => [816],
  "D520-SEC-004" => [814],
  "D520-TEST-001" => [816],
  "TPR-001" => [833],
  "TPR-002" => [834],
  "TPR-003" => [835],
  "TPR-004" => [836],
  "TPR-005" => [837],
  "D833-REPORT-001" => [833],
  "D833-REPORT-002" => [833],
  "D833-REPORT-003" => [833],
  "D833-EXEC-001" => [833],
  "D833-EXEC-002" => [856],
  "D833-EXEC-003" => [833]
}.freeze

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
end

def nonempty?(value)
  value.respond_to?(:empty?) && !value.empty?
end

def passed?(document)
  %w[passed pass].include?(document["outcome"] || document["status"] || document["result"])
end

def canonical_json(value)
  case value
  when Hash then "{" + value.keys.sort.map { |key| JSON.generate(key) + ":" + canonical_json(value.fetch(key)) }.join(",") + "}"
  when Array then "[" + value.map { |item| canonical_json(item) }.join(",") + "]"
  else JSON.generate(value)
  end
end

def blocking?(finding)
  %w[P0 P1].include?(finding.fetch("severity")) || finding["release_blocking"] == true || finding["status"] == "blocking"
end

def git_blob(revision, path)
  output, error, status = Open3.capture3("git", "show", "#{revision}:#{path}")
  fail!("merged source artifact unavailable: #{path}: #{error.strip}") unless status.success?
  output
end

def validate_packet!(root:, mode: "all")
fail!("unsupported mode: #{mode}") unless %w[all census dispositions].include?(mode)
required = %w[source-findings.json returned-findings.json]
required += %w[dispositions.json release-blockers.json] unless mode == "census"
required << "packet-manifest.json" if mode == "all"
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing remediation artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }
ledger_candidate = docs.fetch("source-findings.json").fetch("ledger_candidate_sha")
fail!("ledger candidate is not an immutable commit") unless ledger_candidate.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{ledger_candidate}^{commit}")

source_doc = docs.fetch("source-findings.json")
reports = source_doc.fetch("reports")
fail!("#520/#521 source reports must appear once in deterministic order") unless reports.map { |row| row.fetch("issue") } == [520, 521]
reports.each do |report|
  fail!("source report is not #520 or #521") unless [520, 521].include?(report.fetch("issue"))
  pr_number = report.fetch("pull_request")
  merge_sha = report.fetch("merge_sha")
  fail!("source merge SHA is invalid") unless merge_sha.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{merge_sha}^{commit}")
  system("git", "merge-base", "--is-ancestor", merge_sha, ledger_candidate) or fail!("source review merge is not ancestral to immutable ledger candidate")
  issue_out, issue_err, issue_status = Open3.capture3("gh", "issue", "view", report.fetch("issue").to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,closedByPullRequestsReferences")
  fail!("cannot verify source issue: #{issue_err.strip}") unless issue_status.success?
  issue_doc = JSON.parse(issue_out)
  fail!("source issue is not closed by declared PR") unless issue_doc.fetch("state") == "CLOSED" && issue_doc.fetch("closedByPullRequestsReferences").any? { |pr| pr.fetch("number") == pr_number }
  pr_out, pr_err, pr_status = Open3.capture3("gh", "pr", "view", pr_number.to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,mergeCommit")
  fail!("cannot verify source PR: #{pr_err.strip}") unless pr_status.success?
  pr_doc = JSON.parse(pr_out)
  fail!("source report is not from exact merged predecessor") unless pr_doc.fetch("state") == "MERGED" && pr_doc.dig("mergeCommit", "oid") == merge_sha
  path = report.fetch("path")
  report_blob = git_blob(merge_sha, path)
  fail!("source report digest mismatch: #{path}") unless Digest::SHA256.hexdigest(report_blob) == report.fetch("sha256")
  source_manifest_blob = git_blob(merge_sha, report.fetch("packet_manifest_path"))
  fail!("source packet manifest digest mismatch") unless Digest::SHA256.hexdigest(source_manifest_blob) == report.fetch("packet_manifest_sha256")
  source_manifest = JSON.parse(source_manifest_blob)
  fail!("source report is not a manifested merged output") unless source_manifest.fetch("entries").any? { |entry| entry.fetch("path") == path && entry.fetch("sha256") == report.fetch("sha256") }
  report_doc = JSON.parse(report_blob)
  outcome = report_doc.fetch("outcome")
  fail!("source review outcome is invalid") unless %w[passed findings failed].include?(outcome)
  parsed_findings = report_doc.fetch("findings")
  fail!("source review outcome contradicts findings") if (outcome == "passed") != parsed_findings.empty?
  parsed_ids = parsed_findings.map { |finding| finding.fetch("id") }
  fail!("source report finding-ID projection is false") unless report.fetch("finding_ids").sort == parsed_ids.sort
  parsed_digests = parsed_findings.to_h { |finding| [finding.fetch("id"), Digest::SHA256.hexdigest(canonical_json(finding))] }
  fail!("source report finding-content digests are false") unless report.fetch("finding_digests") == parsed_digests
  case report.fetch("binding")
  when "exact_revision"
    revision = report.fetch("reviewed_revision")
    fail!("source report lacks exact reviewed revision") unless revision.match?(/\A[0-9a-f]{40}\z/)
    fail!("source reviewed revision is not an immutable commit") unless system("git", "cat-file", "-e", "#{revision}^{commit}")
    fail!("source report is not exact-head bound") unless report_doc.fetch("candidate_sha") == revision
    fail!("source finding is stale") unless parsed_findings.all? { |finding| finding.fetch("revision") == revision }
  when "missing_revision"
    fail!("only the retained failed #521 review may lack revision binding") unless report.fetch("issue") == 521 && report["reviewed_revision"].nil? && report_doc["candidate_sha"].nil? && report_doc["non_proving"] == true && outcome == "failed"
  else
    fail!("source report has unsupported revision binding")
  end
  fail!("source finding severity or evidence is invalid") unless parsed_findings.all? { |finding| %w[P0 P1 P2 P3].include?(finding.fetch("severity")) && nonempty?(finding.fetch("status")) && nonempty?(finding.fetch("evidence")) }
end
source_reports = reports.to_h { |report| [report.fetch("issue"), JSON.parse(git_blob(report.fetch("merge_sha"), report.fetch("path")))] }
source_bindings = reports.to_h { |report| [report.fetch("issue").to_s, report["reviewed_revision"]] }
declared_bindings = source_doc.fetch("source_bindings")
fail!("source binding declaration must contain exactly #520 and #521") unless declared_bindings.keys.sort == %w[520 521]
fail!("source reports do not match their ledger-declared bindings") unless declared_bindings == source_bindings
internal_ids = source_reports.fetch(520).fetch("findings").map { |finding| finding.fetch("id") }
external_ids = source_reports.fetch(521).fetch("findings").map { |finding| finding.fetch("id") }
fail!("internal-review finding denominator is not the 14 accepted D520 findings") unless internal_ids.sort == INTERNAL_FINDING_IDS.sort
fail!("third-party finding denominator is not exactly TPR-001 through TPR-005") unless external_ids.sort == EXTERNAL_FINDING_IDS
source = reports.flat_map { |report| source_reports.fetch(report.fetch("issue")).fetch("findings") }
source_ids = source.map { |finding| finding.fetch("id") }
fail!("source finding IDs are duplicated") unless source_ids.uniq.length == source_ids.length
fail!("combined review finding denominator is not exactly 19") unless source_ids.length == 19
fail!("declared source finding content differs from parsed reviews") unless source_doc.fetch("findings") == source

returned_doc = docs.fetch("returned-findings.json")
review_issue = returned_doc.fetch("review_issue")
review_pr = returned_doc.fetch("review_pull_request")
review_head = returned_doc.fetch("review_head_sha")
review_merge = returned_doc.fetch("review_merge_sha")
fail!("returned findings must come from #833 / PR #853") unless review_issue == 833 && review_pr == 853
fail!("returned-review identity is invalid") unless [review_head, review_merge].all? { |sha| sha.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{sha}^{commit}") }
system("git", "merge-base", "--is-ancestor", review_merge, ledger_candidate) or fail!("returned-review merge is not ancestral to immutable ledger candidate")
review_pr_out, review_pr_err, review_pr_status = Open3.capture3("gh", "pr", "view", review_pr.to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,headRefOid,mergeCommit")
fail!("cannot verify returned-review PR: #{review_pr_err.strip}") unless review_pr_status.success?
review_pr_doc = JSON.parse(review_pr_out)
fail!("returned-review PR identity is false") unless review_pr_doc.fetch("state") == "MERGED" && review_pr_doc.fetch("headRefOid") == review_head && review_pr_doc.dig("mergeCommit", "oid") == review_merge
returned_artifacts = returned_doc.fetch("artifacts")
fail!("returned-review artifact set is empty") if returned_artifacts.empty?
returned_blobs = returned_artifacts.to_h do |artifact|
  path = artifact.fetch("path")
  blob = git_blob(review_head, path)
  fail!("returned-review artifact digest mismatch: #{path}") unless Digest::SHA256.hexdigest(blob) == artifact.fetch("sha256")
  [path, blob]
end
returned = returned_doc.fetch("findings")
returned_ids = returned.map { |finding| finding.fetch("id") }
fail!("returned-review finding denominator is not exactly the six #833 findings") unless returned_ids.sort == RETURNED_FINDING_IDS.sort && returned_ids.uniq.length == returned_ids.length
returned.each do |finding|
  fail!("returned finding severity is invalid") unless %w[P0 P1 P2 P3].include?(finding.fetch("severity"))
  fail!("returned finding metadata is incomplete") unless %w[title evidence source_artifact source_heading].all? { |key| nonempty?(finding.fetch(key)) }
  source_blob = returned_blobs.fetch(finding.fetch("source_artifact"))
  fail!("returned finding is not provenance-bound to its source artifact") unless source_blob.include?(finding.fetch("source_heading")) && source_blob.include?(finding.fetch("title"))
end
source += returned
source_ids += returned_ids

return {schema: "adl.v0921.remediation_validation.v3", mode: mode, status: "passed", source_findings: source.length, original_source_findings: 19, returned_findings: returned.length, dispositions: 0} if mode == "census"

dispositions = docs.fetch("dispositions.json").fetch("dispositions")
disposed_ids = dispositions.flat_map { |row| row.fetch("source_finding_ids") }
fail!("finding census does not disposition every source finding exactly once") unless disposed_ids.sort == source_ids.sort && disposed_ids.uniq.length == disposed_ids.length
fail!("nonempty finding census has no dispositions") if source.any? && dispositions.empty?
dispositions.each do |row|
  case row.fetch("kind")
  when "fixed"
    remediations = row.fetch("remediations")
    fail!("fixed disposition has no remediation") if remediations.empty?
    expected_issues = row.fetch("source_finding_ids").flat_map { |id| EXPECTED_REMEDIATION_ISSUES.fetch(id) }.uniq.sort
    actual_issues = remediations.map { |remediation| remediation.fetch("issue") }.uniq.sort
    fail!("finding disposition does not match its remediation issue graph") unless actual_issues == expected_issues
    remediations.each do |remediation|
    remediation_issue = remediation.fetch("issue")
    remediation_pr = remediation.fetch("pull_request")
    head_sha = remediation.fetch("head_sha")
    merge_sha = remediation.fetch("merge_sha")
    fail!("remediation head/merge identity is invalid") unless head_sha.match?(/\A[0-9a-f]{40}\z/) && merge_sha.match?(/\A[0-9a-f]{40}\z/)
    issue_out, issue_err, issue_status = Open3.capture3("gh", "issue", "view", remediation_issue.to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,closedByPullRequestsReferences")
    fail!("cannot verify remediation issue: #{issue_err.strip}") unless issue_status.success?
    issue_doc = JSON.parse(issue_out)
    native_review_close = remediation["closure_kind"] == "native_review_close" && remediation_issue == 833 && remediation["closure_dependency_issue"] == 522
    issue_matches = issue_doc.fetch("state") == "CLOSED" && (native_review_close || issue_doc.fetch("closedByPullRequestsReferences").any? { |pr| pr.fetch("number") == remediation_pr })
    fail!("remediation issue closure does not match declared authority") unless issue_matches
    pr_out, pr_err, pr_status = Open3.capture3("gh", "pr", "view", remediation_pr.to_s, "--repo", "agent-logic/agent-design-language", "--json", "state,headRefOid,mergeCommit")
    fail!("cannot verify remediation PR: #{pr_err.strip}") unless pr_status.success?
    pr_doc = JSON.parse(pr_out)
    fail!("remediation PR/head/merge identity is false") unless pr_doc.fetch("state") == "MERGED" && pr_doc.fetch("headRefOid") == head_sha && pr_doc.dig("mergeCommit", "oid") == merge_sha
    system("git", "merge-base", "--is-ancestor", merge_sha, ledger_candidate) or fail!("remediation merge is not ancestral to immutable ledger candidate")
    artifacts = remediation.fetch("artifacts")
    fail!("remediation has no immutable head artifacts") if artifacts.empty?
    artifacts.each do |artifact|
      blob = git_blob(head_sha, artifact.fetch("path"))
      fail!("remediation head artifact digest mismatch") unless Digest::SHA256.hexdigest(blob) == artifact.fetch("sha256")
    end
    review = remediation.fetch("review")
    review_basis = review.fetch("basis", "exact_implementation_head")
    reviewed_sha = review.fetch("reviewed_sha")
    case review_basis
    when "exact_implementation_head"
      fail!("exact-head review does not match remediation head") unless reviewed_sha == head_sha && review.fetch("observed_pr_head_sha") == head_sha
    when "candidate_current"
      fail!("candidate-current review is not immutable") unless reviewed_sha.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{reviewed_sha}^{commit}")
      system("git", "merge-base", "--is-ancestor", merge_sha, reviewed_sha) or fail!("candidate-current review predates remediation merge")
      system("git", "merge-base", "--is-ancestor", reviewed_sha, ledger_candidate) or fail!("candidate-current review is not ancestral to ledger candidate")
    else
      fail!("unsupported remediation review basis")
    end
    fail!("fix lacks passing review identity") unless review.fetch("outcome") == "passed" && review.fetch("findings") == [] && review.fetch("blockers") == [] && nonempty?(review.fetch("reviewer")) && nonempty?(review.fetch("observed_at"))
    review_path = review.fetch("report_path")
    fail!("exact-head review report is outside remediation packet") unless review_path.start_with?(root + "/") && File.file?(review_path)
    fail!("exact-head review digest mismatch") unless Digest::SHA256.file(review_path).hexdigest == review.fetch("sha256")
    review_doc = read_json(review_path)
    fail!("review report does not prove canonical passing reviewed-candidate result") unless review_doc.fetch("outcome") == "passed" && review_doc.fetch("findings") == [] && review_doc.fetch("blockers") == [] && (review_doc["candidate_sha"] || review_doc["head_sha"]) == reviewed_sha
    fail!("review report does not prove these findings resolved") unless (row.fetch("source_finding_ids") - review_doc.fetch("resolved_finding_ids")).empty?
    validations = remediation.fetch("validation")
    fail!("fixed disposition lacks passing validation evidence") unless validations.any?
    validations.each do |validation|
      evidence_path = validation.fetch("evidence")
      fail!("validation receipt is outside remediation packet") unless evidence_path.start_with?(root + "/") && File.file?(evidence_path)
      evidence_blob = File.binread(evidence_path)
      fail!("validation evidence digest mismatch") unless Digest::SHA256.hexdigest(evidence_blob) == validation.fetch("sha256")
      validation_doc = JSON.parse(evidence_blob)
      evidence_sha = validation_doc["candidate_sha"] || validation_doc["head_sha"] || validation_doc["revision"]
      validation_basis = validation.fetch("basis", "exact_implementation_head")
      case validation_basis
      when "exact_implementation_head"
        fail!("validation evidence is not bound to remediation head") unless evidence_sha == head_sha
      when "candidate_current"
        fail!("candidate-current validation is not immutable") unless evidence_sha&.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{evidence_sha}^{commit}")
        system("git", "merge-base", "--is-ancestor", merge_sha, evidence_sha) or fail!("candidate-current validation predates remediation merge")
        system("git", "merge-base", "--is-ancestor", evidence_sha, ledger_candidate) or fail!("candidate-current validation is not ancestral to ledger candidate")
      else
        fail!("unsupported validation basis")
      end
      observations = validation_doc.fetch("observations")
      commands = validation_doc.fetch("commands")
      commands_bound = commands.is_a?(Array) && commands.any? && commands.all? do |command|
        nonempty?(command.fetch("argv")) && command.fetch("exit_code") == 0 && command.fetch("denominator") > 0
      end
      behavior_bound = observations.is_a?(Array) && observations.any? && observations.all? do |observation|
        artifact_revision = observation.fetch("artifact_revision", head_sha)
        blob = git_blob(artifact_revision, observation.fetch("artifact_path"))
        Digest::SHA256.hexdigest(blob) == observation.fetch("artifact_sha256") && %w[verified passed].include?(observation.fetch("result")) && nonempty?(observation.fetch("behavior"))
      end
      fail!("validation evidence does not prove behavior at its declared immutable revision") unless validation.fetch("outcome") == "passed" && validation_doc.fetch("outcome") == "passed" && validation_doc.fetch("failures", []) == [] && commands_bound && behavior_bound
    end
    end
  when "deferred"
    fail!("deferral metadata is incomplete") unless %w[owner rationale target_milestone release_consequence].all? { |key| nonempty?(row.fetch(key)) }
    row.fetch("source_finding_ids").each do |id|
      finding = source.find { |candidate_finding| candidate_finding.fetch("id") == id }
      fail!("source-defined release blocker cannot be deferred") if blocking?(finding)
    end
  else
    fail!("unsupported disposition kind")
  end
end

blockers = docs.fetch("release-blockers.json").fetch("unresolved")
fail!("release-blocking findings remain") unless blockers == []
return {schema: "adl.v0921.remediation_validation.v3", mode: mode, status: "passed", source_findings: source.length, original_source_findings: 19, returned_findings: returned.length, dispositions: dispositions.length} if mode == "dispositions"

entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| paths.include?(File.join(root, name)) }
fixed_remediations = dispositions.select { |row| row.fetch("kind") == "fixed" }.flat_map { |row| row.fetch("remediations") }
fixed_review_paths = fixed_remediations.map { |remediation| remediation.fetch("review").fetch("report_path") }
fail!("packet manifest omits fixed-disposition review reports") unless fixed_review_paths.all? { |path| paths.include?(path) }
fixed_validation_paths = fixed_remediations.flat_map { |remediation| remediation.fetch("validation").map { |validation| validation.fetch("evidence") } }
fail!("packet manifest omits fixed-disposition validation receipts") unless fixed_validation_paths.all? { |path| paths.include?(path) }
  {schema: "adl.v0921.remediation_validation.v3", mode: mode, status: "passed", source_findings: source.length, original_source_findings: 19, returned_findings: returned.length, dispositions: dispositions.length}
end

if __FILE__ == $PROGRAM_NAME
  root = ENV.fetch("ADL_REMEDIATION_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-06")
  puts JSON.generate(validate_packet!(root: root, mode: ARGV.fetch(0, "all")))
end
