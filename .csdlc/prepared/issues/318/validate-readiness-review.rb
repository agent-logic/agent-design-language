#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"
require "digest"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = ENV.fetch("WP29_MILESTONE_ROOT", File.join(ROOT, "docs/milestones/v0.92.1"))
EVIDENCE = ENV.fetch("WP29_EVIDENCE_ROOT", File.join(ROOT, ".csdlc/evidence/318"))

TAIL_TITLES = {
  "TAIL-01" => "Quality gate",
  "TAIL-02" => "Documentation review and external-review handoff",
  "TAIL-03" => "Publication finalization",
  "TAIL-04" => "Internal review",
  "TAIL-05" => "External / third-party review",
  "TAIL-06" => "Review findings remediation",
  "TAIL-07" => "Next-milestone planning",
  "TAIL-08" => "Next-milestone closeout plan",
  "TAIL-09" => "Next milestone review pass",
  "TAIL-10" => "Release ceremony"
}.freeze
RELEASE_TITLES = {"INT-01" => "Release-tail admission"}.merge(TAIL_TITLES).freeze
CREATION_IDS = %w[
  CORP-A CORP-B CORP-C CORP-D
  AWS-A AWS-B AWS-C AWS-D AWS-E AWS-F AWS-G
  GCP-A GCP-B GCP-C GCP-D GCP-E XCL-01 RUST-01
  V3-A V3-B V3-C V3-D V3-E V3-F
  DRT-A DRT-B DRT-C DEC-01 PROV-A PROV-B DRT-D HOT-01 OBS-A OBS-B
  INT-01 TAIL-01 TAIL-02 TAIL-03 TAIL-04 TAIL-05 TAIL-06 TAIL-07 TAIL-08 TAIL-09 TAIL-10
].freeze

def fail_with(errors)
  errors.each { |error| warn("BLOCK: #{error}") }
  exit 1
end

def check_planning_contract
  errors = []
  wave_path = File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.1.yaml")
  specs_path = File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
  wave = YAML.safe_load(File.read(wave_path), aliases: true)
  specs = YAML.safe_load(File.read(specs_path), aliases: true)

  nodes = []
  walk = lambda do |value|
    case value
    when Hash
      nodes << value if value["id"]
      value.each_value { |child| walk.call(child) }
    when Array
      value.each { |child| walk.call(child) }
    end
  end
  walk.call(wave)
  by_id = nodes.to_h { |row| [row.fetch("id"), row] }
  RELEASE_TITLES.each do |id, title|
    row = by_id[id]
    errors << "missing #{id} from issue wave" unless row
    errors << "#{id} title variance: #{row && row['title'].inspect}" unless row && row["title"] == title
  end
  tail_rows = nodes.select { |row| row["lane"] == "release_tail" }
  errors << "release-tail order mismatch" unless tail_rows.map { |row| row["id"] } == TAIL_TITLES.keys
  TAIL_TITLES.keys.each_with_index do |id, index|
    predecessor = index.zero? ? "INT-01" : TAIL_TITLES.keys[index - 1]
    errors << "#{id} dependency mismatch" unless by_id.fetch(id, {})["depends_on"] == [predecessor]
  end

  %w[PLANNED_ISSUE_CATALOG_v0.92.1.md WBS_v0.92.1.md].each do |name|
    text = File.read(File.join(MILESTONE, name))
    RELEASE_TITLES.each do |id, title|
      errors << "#{name} title variance for #{id}" unless text.include?("| #{id} | #{title} |")
    end
  end
  forbidden_titles = [
    "Docs and release-truth pass",
    "Internal milestone review",
    "External or third-party review",
    "Accepted-findings remediation or explicit deferral",
    "Next-milestone planning and CodeFriend Beta 1 handoff",
    "Next-milestone closeout planning",
    "Next-milestone planning review",
    "Release ceremony, final validation, notes, tag, and cleanup",
    "Final validation, notes, tag, cleanup, and release ceremony"
  ]
  Dir.glob(File.join(MILESTONE, "**/*")).select { |path| File.file?(path) }.each do |path|
    text = File.binread(path)
    forbidden_titles.each do |title|
      errors << "#{path.delete_prefix(ROOT + '/')} retains bundled or variant title #{title.inspect}" if text.include?(title)
    end
  end

  spec_rows = Array(specs.fetch("issue_specifications"))
  spec_by_id = spec_rows.to_h { |row| [row.fetch("id"), row] }
  unit_contracts = specs.fetch("unit_contracts")
  creation_ids = nodes.select { |row| row["creation_owner"] == "WP-01" }.map { |row| row.fetch("id") }
  errors << "creation-owned denominator mismatch" unless creation_ids == CREATION_IDS && creation_ids.uniq.length == CREATION_IDS.length
  errors << "unit-contract denominator mismatch" unless unit_contracts.keys == CREATION_IDS
  errors << "future issue creation already recorded" unless nodes.select { |row| row["creation_owner"] == "WP-01" }.all? { |row| row["issue"].nil? }
  errors << "milestone opening authority is already concrete" unless wave["conductor_issue"].nil? && wave["conductor_id"] == "WP-01"
  creation_ids.each do |id|
    row = spec_by_id[id]
    unless row
      errors << "#{id} missing execution specification"
      next
    end
    objective = row["objective"]
    errors << "#{id} must define one outcome-shaped objective" unless objective.is_a?(String) && objective.start_with?("Produce one ")
    deliverable = row["primary_deliverable"]
    errors << "#{id} must define exactly one primary_deliverable" unless deliverable.is_a?(String) && deliverable.start_with?("One ")
    result = row["verification_result"]
    errors << "#{id} must define exactly one independently verifiable verification_result" unless result.is_a?(String) && !result.strip.empty?
    boundary = row["unit_boundary"]
    errors << "#{id} must define an explicit non-bundled unit_boundary" unless boundary.is_a?(String) && boundary.start_with?("Issue completion is exactly ") && boundary.match?(/evidence input|proof input|inputs to|internal step|implementation part|cannot close|non-closeable|do not close|not separately|separately reviewable|independently closed|external input|follow-up|rows? within|no .* executed/)
    contract = unit_contracts[id]
    errors << "#{id} missing unique structural primary result" unless contract.is_a?(Hash) && contract["primary_result"].to_s.match?(/\A[a-z0-9_]+\z/)
    errors << "#{id} permits supporting work to close independently" unless contract.is_a?(Hash) && contract["supporting_work_closeable"] == false
    dependency_text = Array(row["dependencies"]).compact.join(" ").downcase
    errors << "#{id} incorrectly gates execution on administrative closeout" if dependency_text.match?(/(depend|require|before|gate).*(finish|cleanup|terminal)|(finish|cleanup|terminal).*(depend|require|before|gate)/)
  end
  primary_results = unit_contracts.values.map { |contract| contract["primary_result"] if contract.is_a?(Hash) }.compact
  errors << "unit contracts reuse a primary result" unless primary_results.uniq.length == CREATION_IDS.length

  [errors, creation_ids]
end

def check_review_packet
  errors = []
  required = %w[issue-universe.json findings.json readiness-review.json planning-source-addendum.json]
  required.each do |name|
    path = File.join(EVIDENCE, name)
    errors << "missing retained review artifact #{name}" unless File.file?(path)
  end
  return errors unless errors.empty?

  universe = JSON.parse(File.read(File.join(EVIDENCE, "issue-universe.json")))
  rows = Array(universe["issues"])
  issues = rows.map { |row| Integer(row.fetch("issue")) }
  errors << "canonical issue denominator mismatch" unless issues == (307..319).to_a
  rows.each do |row|
    issue = row.fetch("issue")
    errors << "issue #{issue} invalid state" unless %w[OPEN CLOSED].include?(row["state"])
    errors << "issue #{issue} missing role" if row["role"].to_s.strip.empty?
    next unless row["closing_pr"]

    errors << "issue #{issue} invalid head" unless row["head"].to_s.match?(/\A[0-9a-f]{40}\z/)
    errors << "issue #{issue} invalid merge" unless row["merge"].to_s.match?(/\A[0-9a-f]{40}\z/)
    if row["merge"].to_s.match?(/\A[0-9a-f]{40}\z/)
      system("git", "-C", ROOT, "cat-file", "-e", "#{row['merge']}^{commit}", out: File::NULL, err: File::NULL) || errors << "issue #{issue} merge commit unavailable"
      system("git", "-C", ROOT, "merge-base", "--is-ancestor", row["merge"], "HEAD", out: File::NULL, err: File::NULL) || errors << "issue #{issue} merge is not ancestral"
    end
  end

  unless ENV["WP29_SKIP_LIVE_GITHUB"] == "1"
    rows.each do |row|
      issue = row.fetch("issue")
      out, err, status = Open3.capture3("gh", "issue", "view", issue.to_s, "--repo", universe.fetch("repository"), "--json", "state,closedByPullRequestsReferences")
      unless status.success?
        errors << "issue #{issue} live GitHub observation failed: #{err.strip}"
        next
      end
      live = JSON.parse(out)
      errors << "issue #{issue} live state mismatch" unless live["state"] == row["state"]
      closing = Array(live["closedByPullRequestsReferences"]).map { |pr| pr["number"] }
      expected_pr = row["closing_pr"]
      errors << "issue #{issue} live closing PR mismatch" unless expected_pr ? closing.include?(expected_pr) : closing.empty?
      next unless expected_pr

      pr_out, pr_err, pr_status = Open3.capture3("gh", "pr", "view", expected_pr.to_s, "--repo", universe.fetch("repository"), "--json", "state,headRefOid,mergeCommit")
      unless pr_status.success?
        errors << "issue #{issue} live PR observation failed: #{pr_err.strip}"
        next
      end
      pr = JSON.parse(pr_out)
      errors << "issue #{issue} closing PR is not merged" unless pr["state"] == "MERGED"
      errors << "issue #{issue} live head mismatch" unless pr["headRefOid"] == row["head"]
      errors << "issue #{issue} live merge mismatch" unless pr.dig("mergeCommit", "oid") == row["merge"]
    end
  end

  readiness = JSON.parse(File.read(File.join(EVIDENCE, "readiness-review.json")))
  errors << "v0.92.1 issue creation claim changed" unless readiness.dig("v0_92_1", "issues_created") == false
  errors << "v0.92.2 issue creation claim changed" unless readiness.dig("v0_92_2", "issues_created") == false
  errors << "v0.93 activation claim changed" unless readiness.dig("v0_93", "status") == "inactive" && readiness.dig("v0_93", "selected") == false && readiness.dig("v0_93", "activated") == false
  errors << "single-unit contract denominator changed" unless readiness.dig("v0_92_1", "single_unit_contract") == "explicit_for_all_45_creation_owned_issues"

  source_addendum = JSON.parse(File.read(File.join(EVIDENCE, "planning-source-addendum.json")))
  source_rows = Array(source_addendum["sources"])
  expected_source_ids = %w[TBD-AWS-MOVE-IN TBD-GCP-MOVE-IN TBD-RUST-SIMPLIFICATION TBD-WP21-REDUCTION]
  errors << "planning source addendum denominator mismatch" unless source_rows.map { |row| row["source_id"] } == expected_source_ids
  source_rows.each do |row|
    errors << "planning source digest invalid: #{row['source_id']}" unless row["source_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
    promoted = row["promoted_contract"].to_s
    errors << "planning source promoted contract missing: #{row['source_id']}" unless File.file?(File.join(ROOT, promoted))
    errors << "planning source lacks planned IDs: #{row['source_id']}" if Array(row["planned_ids"]).empty?
    errors << "planning source lacks disposition: #{row['source_id']}" if row["disposition"].to_s.strip.empty?
  end

  findings = JSON.parse(File.read(File.join(EVIDENCE, "findings.json")))
  Array(findings["findings"]).each do |finding|
    %w[id severity evidence owner route disposition revision].each do |field|
      errors << "finding #{finding['id'] || '?'} missing #{field}" if finding[field].to_s.strip.empty?
    end
  end
  errors
end

mode = ARGV.fetch(0, "all")
planning_errors, creation_ids = check_planning_contract
errors = case mode
         when "planning" then planning_errors
         when "all" then planning_errors + check_review_packet
         else ["unknown mode #{mode.inspect}"]
         end

fail_with(errors) unless errors.empty?
puts JSON.generate(schema: "adl.v092.wp29.readiness-review.v1", status: "pass", creation_owned_issues: creation_ids.length, tail_titles: TAIL_TITLES.length)
