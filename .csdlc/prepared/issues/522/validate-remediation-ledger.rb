#!/usr/bin/env ruby
require "digest"
require "json"

def fail!(message)
  abort(message)
end

def read_json(path)
  JSON.parse(File.read(path))
end

def nonempty?(value)
  value.respond_to?(:empty?) && !value.empty?
end

if ARGV.first == "fixture"
  fixture = read_json(ARGV.fetch(1))
  source = fixture.fetch("source_findings")
  fail!("empty source census requires zero-finding proof") if source.empty? && !nonempty?(fixture.fetch("zero_findings_proof"))
  fail!("nonempty findings require dispositions") if source.any? && fixture.fetch("dispositions").empty?
  fail!("release blockers remain") unless fixture.fetch("unresolved_blockers") == []
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
  path = report.fetch("path")
  fail!("source report is missing: #{path}") unless File.file?(path)
  fail!("source report digest mismatch: #{path}") unless Digest::SHA256.file(path).hexdigest == report.fetch("sha256")
  fail!("source report lacks exact reviewed revision") unless report.fetch("reviewed_revision").match?(/\A[0-9a-f]{40}\z/)
end
source = source_doc.fetch("findings")
fail!("empty source finding census lacks affirmative zero-finding proof") if source.empty? && !nonempty?(source_doc.fetch("zero_findings_proof"))
source_ids = source.map { |finding| finding.fetch("id") }
fail!("source finding IDs are duplicated") unless source_ids.uniq.length == source_ids.length

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
    validations = row.fetch("validation")
    fail!("fixed disposition lacks passing validation evidence") unless validations.any? && validations.all? { |validation| validation.fetch("outcome") == "passed" && File.file?(validation.fetch("evidence")) }
  when "deferred"
    fail!("deferral metadata is incomplete") unless %w[owner rationale target_milestone release_consequence].all? { |key| nonempty?(row.fetch(key)) }
    fail!("release-blocking finding cannot be deferred") unless row.fetch("release_consequence") == "non_blocking"
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
