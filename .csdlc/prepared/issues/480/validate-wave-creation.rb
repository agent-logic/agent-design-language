#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = File.join(ROOT, "docs/milestones/v0.92.1")
WAVE_PATH = File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC_PATH = File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
FINAL_RECEIPT = File.join(MILESTONE, "evidence/wp-01/final-creation-receipt.json")
EXPECTED_EXISTING = [51, 84, 122, 251, 261, 262, 263, 264, 342, 345].freeze
EXCLUDED = [269].freeze
REPOSITORY = "agent-logic/agent-design-language"

def fail!(messages)
  messages.each { |message| warn "BLOCK: #{message}" }
  exit 1
end

def child_rows(wave)
  wave.fetch("work_packages").flat_map do |row|
    if row["packages"]
      row.fetch("packages")
    elsif row["creation_owner"] == "WP-01"
      [row]
    else
      []
    end
  end.select { |row| row["creation_owner"] == "WP-01" }
end

def expected_area(id)
  return "area:security" if %w[CORP-B CORP-D].include?(id)
  return "area:runtime" if id.match?(/\A(?:CORP|AWS|GCP|XCL|DRT|HOT|PROV)/)
  return "area:architecture" if id.match?(/\A(?:RUST|DEC)/)
  return "area:csdlc" if id.start_with?("V3-")
  return "area:observatory" if id.start_with?("OBS-")
  return "area:quality" if %w[INT-01 TAIL-01 TAIL-06].include?(id)
  return "area:docs" if %w[TAIL-02 TAIL-03 TAIL-07 TAIL-08].include?(id)
  return "area:review" if %w[TAIL-04 TAIL-05 TAIL-09].include?(id)
  return "area:release" if id == "TAIL-10"

  nil
end

def validate_plan
  errors = []
  wave = YAML.safe_load(File.read(WAVE_PATH), permitted_classes: [], aliases: false)
  specs = YAML.safe_load(File.read(SPEC_PATH), permitted_classes: [], aliases: false).fetch("issue_specifications")
  conductor = specs.find { |row| row["id"] == "WP-01" } || {}
  wave_rows = child_rows(wave)
  wave_ids = wave_rows.map { |row| row.fetch("id") }
  denominator = conductor.fetch("creation_denominator", [])
  wave_by_id = wave_rows.to_h { |row| [row.fetch("id"), row] }
  rows = denominator.map { |id| wave_by_id[id] }.compact
  ids = rows.map { |row| row.fetch("id") }
  spec_by_id = specs.to_h { |row| [row.fetch("id"), row] }

  errors << "creation denominator must contain exactly 45 IDs" unless denominator.length == 45
  errors << "creation denominator contains duplicates" unless denominator.uniq == denominator
  errors << "wave creation set differs from WP-01 denominator" unless wave_ids.sort == denominator.sort
  errors << "ordered creation rows differ from WP-01 denominator" unless ids == denominator
  errors << "wave contains a concrete conductor issue" unless wave["conductor_issue"].nil?
  errors << "WP-01 opening identity mismatch" unless wave["conductor_id"] == "WP-01"
  errors << "excluded issue #269 entered the active denominator" if ids.include?("269")

  rows.each do |row|
    id = row.fetch("id")
    spec = spec_by_id[id]
    errors << "missing issue-level specification for #{id}" unless spec
    errors << "missing exact area mapping for #{id}" unless expected_area(id)
    errors << "non-number-free creation slot #{id}" unless row["issue"].nil?
    errors << "wrong creation owner for #{id}" unless row["creation_owner"] == "WP-01"
    next unless spec

    %w[objective primary_deliverable verification_result unit_boundary acceptance_criteria owned_paths pvf_lanes stop_conditions non_goals].each do |field|
      value = spec[field]
      errors << "#{id} missing #{field}" if value.nil? || (value.respond_to?(:empty?) && value.empty?)
    end
  end

  fail!(errors) unless errors.empty?
  {
    schema: "adl.v0921.wp01.creation-plan-validation.v1",
    result: "passed",
    creation_slots: ids.length,
    ordered_ids: ids,
    existing_issues: EXPECTED_EXISTING,
    excluded_issues: EXCLUDED,
    wave_sha256: Digest::SHA256.file(WAVE_PATH).hexdigest,
    specifications_sha256: Digest::SHA256.file(SPEC_PATH).hexdigest
  }
end

def validate_live(plan)
  fail!(["final creation receipt is absent; no live completion claim is allowed"]) unless File.file?(FINAL_RECEIPT)
  receipt = JSON.parse(File.read(FINAL_RECEIPT))
  errors = []
  rows = receipt.fetch("children", [])
  ids = rows.map { |row| row["planned_id"] }
  errors << "final receipt denominator mismatch" unless ids == plan.fetch(:ordered_ids)
  errors << "final receipt issue numbers are not unique" unless rows.map { |row| row["issue"] }.uniq.length == 45
  errors << "final receipt contains a non-open child" unless rows.all? { |row| row["state"] == "open" }
  errors << "final receipt routing mismatch" unless rows.all? do |row|
    labels = row.fetch("labels", []).sort
    labels == [expected_area(row.fetch("planned_id")), "track:roadmap", "type:task", "version:v0.92.1"].sort &&
      row["milestone"] == 1
  end
  errors << "final receipt lacks independent-live flag" unless receipt["live_verified"] == true
  errors << "existing issue verification denominator mismatch" unless receipt["existing_issues_verified"] == EXPECTED_EXISTING
  existing_rows = receipt.fetch("existing_issues", [])
  errors << "existing live row denominator mismatch" unless existing_rows.map { |row| row["issue"] } == EXPECTED_EXISTING
  existing_rows.each do |row|
    stdout, stderr, status = Open3.capture3("gh", "issue", "view", row.fetch("issue").to_s, "--repo", REPOSITORY,
                                            "--json", "number,title,body,state,labels,milestone", chdir: ROOT)
    unless status.success?
      errors << "existing live read failed for ##{row['issue']}: #{stderr.strip}"
      next
    end
    live = JSON.parse(stdout)
    normalized = {
      "number" => live.fetch("number"), "title" => live.fetch("title"), "body" => live.fetch("body", ""),
      "state" => live.fetch("state").downcase,
      "labels" => live.fetch("labels", []).map { |label| label.fetch("name") }.sort,
      "milestone" => live["milestone"]&.fetch("number", nil)
    }
    errors << "existing live drift for ##{row['issue']}" unless Digest::SHA256.hexdigest(JSON.generate(normalized)) == row["live_sha256"]
  end
  rows.each do |row|
    stdout, stderr, status = Open3.capture3("gh", "issue", "view", row.fetch("issue").to_s, "--repo", REPOSITORY,
                                            "--json", "number,title,body,state,labels,milestone", chdir: ROOT)
    unless status.success?
      errors << "live read failed for #{row['planned_id']}: #{stderr.strip}"
      next
    end
    live = JSON.parse(stdout)
    labels = live.fetch("labels", []).map { |label| label.fetch("name") }.sort
    expected_title = "[v0.92.1][#{row.fetch('planned_id')}] "
    errors << "live title mismatch for #{row['planned_id']}" unless live.fetch("title").start_with?(expected_title) && live.fetch("title") == row.fetch("title")
    errors << "live labels mismatch for #{row['planned_id']}" unless labels == row.fetch("labels").sort
    errors << "live milestone mismatch for #{row['planned_id']}" unless live.dig("milestone", "number") == 1
    errors << "live state mismatch for #{row['planned_id']}" unless live.fetch("state").downcase == "open"
    errors << "live body mismatch for #{row['planned_id']}" unless Digest::SHA256.hexdigest(live.fetch("body")) == row.fetch("body_sha256")
  end
  fail!(errors) unless errors.empty?
  plan.merge(live_result: "passed", final_receipt_sha256: Digest::SHA256.file(FINAL_RECEIPT).hexdigest)
end

mode = ARGV.fetch(0, "plan")
plan = validate_plan
result = case mode
         when "plan" then plan
         when "live", "all" then validate_live(plan)
         else fail!(["unknown validation mode #{mode.inspect}"])
         end
puts JSON.pretty_generate(result)
