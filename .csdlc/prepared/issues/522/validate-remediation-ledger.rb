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

def validate_fixture!(fixture)
  parsed = fixture.fetch("parsed_source_findings")
  declared = fixture.fetch("source_findings")
  fail!("declared source content differs from parsed reviews") unless declared == parsed
  ids = parsed.map { |row| row.fetch("id") }
  fail!("source finding IDs are duplicated") unless ids.uniq.length == ids.length
  fail!("source reviews are not merged exact-head outputs") unless fixture.fetch("source_reviews").all? { |row| row.fetch("merged") == true && row.fetch("reviewed_revision") == row.fetch("candidate_sha") && %w[passed findings].include?(row.fetch("outcome")) }
  fail!("empty source census requires zero-finding proof") if parsed.empty? && !nonempty?(fixture.fetch("zero_findings_proof"))
  dispositions = fixture.fetch("dispositions")
  disposed = dispositions.flat_map { |row| row.fetch("source_finding_ids") }
  fail!("dispositions do not exactly cover parsed findings") unless disposed.sort == ids.sort && disposed.uniq.length == disposed.length
  dispositions.select { |row| row.fetch("kind") == "deferred" }.each do |row|
    fail!("source-defined blocker cannot be deferred") if row.fetch("source_finding_ids").any? { |id| blocking?(parsed.find { |finding| finding.fetch("id") == id }) }
  end
  fixture.fetch("evidence").each do |row|
    fail!("evidence is not passing or exact-head bound") unless row.fetch("digest_valid") == true && row.fetch("outcome") == "passed" && row.fetch("evidence_sha") == row.fetch("head_sha")
  end
  fail!("release blockers remain") unless fixture.fetch("unresolved_blockers") == []
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  validate_fixture!(fixture)
  puts JSON.generate(status: "passed", fixture: ARGV[1])
  exit
end

root = ENV.fetch("ADL_REMEDIATION_PACKET_ROOT", "docs/milestones/v0.92.1/evidence/release/tail-06")
mode = ARGV.fetch(0, "all")
fail!("unsupported mode: #{mode}") unless %w[all census dispositions].include?(mode)
required = %w[source-findings.json dispositions.json release-blockers.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
fail!("missing remediation artifacts: #{missing.join(', ')}") unless missing.empty?
docs = required.to_h { |name| [name, read_json(File.join(root, name))] }

source_doc = docs.fetch("source-findings.json")
reports = source_doc.fetch("reports")
fail!("#520/#521 source report denominator is incomplete") unless reports.map { |row| row.fetch("issue") }.sort == [520, 521]
reports.each do |report|
  fail!("source report is not #520 or #521") unless [520, 521].include?(report.fetch("issue"))
  pr_number = report.fetch("pull_request")
  merge_sha = report.fetch("merge_sha")
  fail!("source merge SHA is invalid") unless merge_sha.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{merge_sha}^{commit}")
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
  fail!("source report lacks exact reviewed revision") unless report.fetch("reviewed_revision").match?(/\A[0-9a-f]{40}\z/)
  report_doc = JSON.parse(report_blob)
  fail!("source report is not exact-head bound") unless report_doc.fetch("candidate_sha") == report.fetch("reviewed_revision")
  outcome = report_doc.fetch("outcome")
  fail!("source review outcome is invalid") unless %w[passed findings].include?(outcome)
  parsed_findings = report_doc.fetch("findings")
  fail!("source review outcome contradicts findings") unless (outcome == "passed") == parsed_findings.empty?
  parsed_ids = parsed_findings.map { |finding| finding.fetch("id") }
  fail!("source report finding-ID projection is false") unless report.fetch("finding_ids").sort == parsed_ids.sort
  parsed_digests = parsed_findings.to_h { |finding| [finding.fetch("id"), Digest::SHA256.hexdigest(canonical_json(finding))] }
  fail!("source report finding-content digests are false") unless report.fetch("finding_digests") == parsed_digests
  fail!("source finding is stale") unless parsed_findings.all? { |finding| finding.fetch("revision") == report.fetch("reviewed_revision") }
end
source = reports.flat_map { |report| JSON.parse(git_blob(report.fetch("merge_sha"), report.fetch("path"))).fetch("findings") }
fail!("empty source finding census lacks affirmative zero-finding proof") if source.empty? && !nonempty?(source_doc.fetch("zero_findings_proof"))
source_ids = source.map { |finding| finding.fetch("id") }
fail!("source finding IDs are duplicated") unless source_ids.uniq.length == source_ids.length
fail!("declared source finding content differs from parsed reviews") unless source_doc.fetch("findings") == source

dispositions = docs.fetch("dispositions.json").fetch("dispositions")
disposed_ids = dispositions.flat_map { |row| row.fetch("source_finding_ids") }
fail!("finding census does not disposition every source finding exactly once") unless disposed_ids.sort == source_ids.sort && disposed_ids.uniq.length == disposed_ids.length
fail!("nonempty finding census has no dispositions") if source.any? && dispositions.empty?
dispositions.each do |row|
  case row.fetch("kind")
  when "fixed"
    review = row.fetch("review")
    fail!("fix lacks exact current review identity") unless review.fetch("head_sha").match?(/\A[0-9a-f]{40}\z/) && review.fetch("head_sha") == review.fetch("reviewed_sha") && review.fetch("head_sha") == review.fetch("observed_pr_head_sha") && nonempty?(review.fetch("observed_at"))
    system("git", "cat-file", "-e", "#{review.fetch('head_sha')}^{commit}") or fail!("reviewed fix commit is unavailable")
    review_path = review.fetch("report_path")
    fail!("exact-head review report is missing") unless File.file?(review_path)
    fail!("exact-head review digest mismatch") unless Digest::SHA256.file(review_path).hexdigest == review.fetch("sha256")
    review_doc = read_json(review_path)
    fail!("review report does not prove a passing exact-head result") unless passed?(review_doc) && (review_doc["candidate_sha"] || review_doc["head_sha"]) == review.fetch("head_sha")
    fail!("review report does not prove these findings resolved") unless review_doc.fetch("resolved_finding_ids").sort == row.fetch("source_finding_ids").sort
    validations = row.fetch("validation")
    fail!("fixed disposition lacks passing validation evidence") unless validations.any?
    validations.each do |validation|
      evidence_path = validation.fetch("evidence")
      fail!("validation evidence is missing") unless File.file?(evidence_path)
      fail!("validation evidence digest mismatch") unless Digest::SHA256.file(evidence_path).hexdigest == validation.fetch("sha256")
      validation_doc = read_json(evidence_path)
      evidence_sha = validation_doc["candidate_sha"] || validation_doc["head_sha"] || validation_doc["revision"]
      fail!("validation evidence does not prove pass at fixed head") unless validation.fetch("outcome") == "passed" && passed?(validation_doc) && evidence_sha == review.fetch("head_sha")
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
entries = docs.fetch("packet-manifest.json").fetch("entries")
entries.each do |entry|
  path = entry.fetch("path")
  fail!("manifested artifact is missing: #{path}") unless File.file?(path)
  fail!("artifact digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == entry.fetch("sha256")
end
paths = entries.map { |entry| entry.fetch("path") }
fail!("packet manifest omits required artifacts") unless (required - ["packet-manifest.json"]).all? { |name| paths.include?(File.join(root, name)) }
puts JSON.generate(schema: "adl.v0921.remediation_validation.v2", mode: mode, status: "passed", source_findings: source.length, dispositions: dispositions.length)
