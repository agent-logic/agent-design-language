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
CATALOG_PATH = File.join(MILESTONE, "PLANNED_ISSUE_CATALOG_v0.92.1.md")
READINESS_PATH = File.join(MILESTONE, "WP_EXECUTION_READINESS_v0.92.1.md")
FINAL_RECEIPT = File.join(MILESTONE, "evidence/wp-01/final-creation-receipt.json")
EXPECTED_EXISTING = [51, 84, 122, 251, 261, 262, 263, 264, 342, 345].freeze
EXCLUDED = [269].freeze
REPOSITORY = "agent-logic/agent-design-language"
EXPECTED_PLANNING_DIGEST = "f00977324d7bfbfcb17a04d1798d14eca9c99c6d6299a0ae21977f564b518251"
EXISTING_TARGETS = {
  84 => ["[v0.92.1][Observatory] Complete live Unity Observatory Runtime v3 integration", %w[area:observatory track:roadmap type:task version:v0.92.1]],
  122 => ["[v0.92.1][Observatory] Deploy public exposure with Route53 and ACM", %w[area:observatory track:roadmap type:task version:v0.92.1]],
  251 => ["[v0.92.1][Runtime] Support TLS 1.2 on public Axum HTTPS/WSS for Unity", %w[area:runtime track:roadmap type:bug version:v0.92.1]],
  345 => ["[v0.92.1][Sidecar] Harden and retain the AWS GPU Shepherd proof runner", %w[area:runtime track:roadmap type:task version:v0.92.1]]
}.freeze

def planning_digest
  Digest::SHA256.hexdigest([WAVE_PATH, SPEC_PATH, CATALOG_PATH, READINESS_PATH].map { |path| Digest::SHA256.file(path).hexdigest }.join(":"))
end

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

def expected_body(id, title, spec, dependencies, predecessors)
  deps = dependencies.map { |name, issue| "- #{name}: ##{issue}" }
  retained = predecessors.map { |issue| "##{issue}" }
  body = <<~BODY
    ## Outcome

    #{spec.fetch("objective")}

    ## Primary deliverable

    #{spec.fetch("primary_deliverable")}

    ## Verification result

    #{spec.fetch("verification_result")}

    ## Unit boundary

    #{spec.fetch("unit_boundary")}

    ## Dependencies

    #{deps.empty? ? "- None." : deps.join("\n")}

    ## Retained predecessor scope

    #{retained.empty? ? "- None; this is new v0.92.1 work." : retained.map { |entry| "- #{entry}" }.join("\n")}

    ## Acceptance criteria

    #{spec.fetch("acceptance_criteria").map { |entry| "- [ ] #{entry}" }.join("\n")}

    ## Owned paths

    #{spec.fetch("owned_paths").map { |entry| "- `#{entry}`" }.join("\n")}

    ## PVF lanes

    #{spec.fetch("pvf_lanes").map { |entry| "- `#{entry}`" }.join("\n")}

    ## Stop conditions

    #{spec.fetch("stop_conditions").map { |entry| "- #{entry}" }.join("\n")}

    ## Non-goals

    #{spec.fetch("non_goals").map { |entry| "- #{entry}" }.join("\n")}

    ## Canonical planning identity

    - Planned ID: `#{id}`
    - Canonical title: `#{title}`
    - Planning digest: `#{planning_digest}`
    - Execution specification: `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml##{id}`
  BODY
  "#{body}<!-- csdlc-github-operation:v0921-wp01:#{planning_digest}:#{id}:create -->\n"
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
    planning_digest: planning_digest,
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
  errors << "current planning authority digest mismatch" unless planning_digest == EXPECTED_PLANNING_DIGEST
  errors << "final receipt planning digest mismatch" unless receipt["planning_digest"] == planning_digest
  wave = YAML.safe_load(File.read(WAVE_PATH), permitted_classes: [], aliases: false)
  expected_rows = child_rows(wave).to_h { |row| [row.fetch("id"), row] }
  specs = YAML.safe_load(File.read(SPEC_PATH), permitted_classes: [], aliases: false).fetch("issue_specifications").to_h { |row| [row.fetch("id"), row] }
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
      "milestone" => live["milestone"]&.fetch("number", nil), "milestone_title" => live["milestone"]&.fetch("title", nil)
    }
    errors << "existing live drift for ##{row['issue']}" unless Digest::SHA256.hexdigest(JSON.generate(normalized)) == row["live_sha256"]
    if (target = EXISTING_TARGETS[row.fetch("issue")])
      errors << "existing exact routing mismatch for ##{row['issue']}" unless normalized["title"] == target[0] && normalized["labels"] == target[1].sort && normalized["milestone"] == 1
    end
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
    expected_title = "[v0.92.1][#{row.fetch('planned_id')}] #{expected_rows.fetch(row.fetch('planned_id')).fetch('title')}"
    errors << "live title mismatch for #{row['planned_id']}" unless live.fetch("title") == expected_title && row.fetch("title") == expected_title
    errors << "live labels mismatch for #{row['planned_id']}" unless labels == row.fetch("labels").sort
    errors << "live milestone mismatch for #{row['planned_id']}" unless live.dig("milestone", "number") == 1
    errors << "live milestone title mismatch for #{row['planned_id']}" unless live.dig("milestone", "title") == "v0.92.1"
    errors << "live state mismatch for #{row['planned_id']}" unless live.fetch("state").downcase == "open"
    exact_body = expected_body(row.fetch("planned_id"), expected_title, specs.fetch(row.fetch("planned_id")),
                               row.fetch("dependencies"), expected_rows.fetch(row.fetch("planned_id")).fetch("predecessor_issues", []))
    errors << "live body mismatch for #{row['planned_id']}" unless live.fetch("body") == exact_body
  end
  census_stdout, census_stderr, census_status = Open3.capture3("gh", "api", "--paginate", "--slurp",
                                                                "repos/#{REPOSITORY}/issues?state=all&per_page=100", chdir: ROOT)
  if census_status.success?
    census = JSON.parse(census_stdout).flatten.reject { |issue| issue.key?("pull_request") }
    rows.each do |row|
      id = row.fetch("planned_id")
      marker = "<!-- csdlc-github-operation:v0921-wp01:#{planning_digest}:#{id}:create -->"
      matches = census.select { |issue| issue["title"] == row["title"] || issue["title"].include?("[#{id}]") || issue.fetch("body", "").include?(marker) }
      errors << "live census ambiguity for #{id}" unless matches.map { |issue| issue["number"] } == [row["issue"]]
    end
  else
    errors << "live census failed: #{census_stderr.strip}"
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
