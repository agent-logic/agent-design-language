#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "yaml"
require "digest"

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
  creation_ids = nodes.select { |row| row["creation_owner"] == "WP-01" }.map { |row| row.fetch("id") }
  errors << "creation-owned denominator mismatch" unless creation_ids.length == 31 && creation_ids.uniq.length == 31
  creation_ids.each do |id|
    row = spec_by_id[id]
    unless row
      errors << "#{id} missing execution specification"
      next
    end
    errors << "#{id} must define exactly one nonempty objective" unless row["objective"].is_a?(String) && !row["objective"].strip.empty?
    deliverable = row["primary_deliverable"]
    errors << "#{id} must define exactly one primary_deliverable" unless deliverable.is_a?(String) && !deliverable.strip.empty?
    result = row["verification_result"]
    errors << "#{id} must define exactly one independently verifiable verification_result" unless result.is_a?(String) && !result.strip.empty?
    dependency_text = Array(row["dependencies"]).compact.join(" ").downcase
    errors << "#{id} incorrectly gates execution on administrative closeout" if dependency_text.match?(/(depend|require|before|gate).*(finish|cleanup|terminal)|(finish|cleanup|terminal).*(depend|require|before|gate)/)
  end

  [errors, creation_ids]
end

def check_review_packet
  errors = []
  required = %w[issue-universe.json findings.json readiness-review.json]
  required.each do |name|
    path = File.join(EVIDENCE, name)
    errors << "missing retained review artifact #{name}" unless File.file?(path)
  end
  return errors unless errors.empty?

  universe = JSON.parse(File.read(File.join(EVIDENCE, "issue-universe.json")))
  issues = Array(universe["issues"]).map { |row| Integer(row.fetch("issue")) }
  errors << "canonical issue denominator mismatch" unless issues == (307..319).to_a

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
