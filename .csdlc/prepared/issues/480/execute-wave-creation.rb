#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = File.join(ROOT, "docs/milestones/v0.92.1")
EVIDENCE = File.join(MILESTONE, "evidence/wp-01")
OPERATIONS = File.join(EVIDENCE, "operations")
REQUESTS = File.join(EVIDENCE, "requests")
PLAN_PATH = File.join(EVIDENCE, "creation-plan.json")
PARTIAL_PATH = File.join(EVIDENCE, "partial-receipt.json")
FINAL_PATH = File.join(EVIDENCE, "final-creation-receipt.json")
WAVE_PATH = File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC_PATH = File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
REPOSITORY = "agent-logic/agent-design-language"
MILESTONE_NUMBER = 1
VERSION_LABEL = "version:v0.92.1"
WP01_ISSUE = 480
EXISTING = {
  "issue-51" => 51, "issue-84" => 84, "issue-122" => 122,
  "issue-251" => 251, "issue-261" => 261, "issue-262" => 262,
  "issue-263" => 263, "issue-264" => 264, "issue-342" => 342,
  "issue-345" => 345, "WP-01" => WP01_ISSUE
}.freeze
EXISTING_TARGETS = {
  84 => ["[v0.92.1][Observatory] Complete live Unity Observatory Runtime v3 integration", %w[area:observatory track:roadmap type:task version:v0.92.1]],
  122 => ["[v0.92.1][Observatory] Deploy public exposure with Route53 and ACM", %w[area:observatory track:roadmap type:task version:v0.92.1]],
  251 => ["[v0.92.1][Runtime] Support TLS 1.2 on public Axum HTTPS/WSS for Unity", %w[area:runtime track:roadmap type:bug version:v0.92.1]],
  345 => ["[v0.92.1][Sidecar] Harden and retain the AWS GPU Shepherd proof runner", %w[area:runtime track:roadmap type:task version:v0.92.1]]
}.freeze

def load_yaml(path)
  YAML.safe_load(File.read(path), permitted_classes: [], aliases: false)
end

def wave_rows
  load_yaml(WAVE_PATH).fetch("work_packages").flat_map do |row|
    row["packages"] || (row["creation_owner"] == "WP-01" ? [row] : [])
  end.select { |row| row["creation_owner"] == "WP-01" }.to_h { |row| [row.fetch("id"), row] }
end

def specifications
  load_yaml(SPEC_PATH).fetch("issue_specifications").to_h { |row| [row.fetch("id"), row] }
end

def denominator(specs)
  specs.fetch("WP-01").fetch("creation_denominator")
end

def planning_digest
  Digest::SHA256.hexdigest([Digest::SHA256.file(WAVE_PATH).hexdigest, Digest::SHA256.file(SPEC_PATH).hexdigest].join(":"))
end

def area_for(id)
  return "area:security" if %w[CORP-B CORP-D].include?(id)
  return "area:runtime" if id.match?(/\A(?:CORP|AWS|GCP|XCL|DRT|HOT|PROV)/)
  return "area:architecture" if id.match?(/\A(?:RUST|DEC)/)
  return "area:csdlc" if id.start_with?("V3-")
  return "area:observatory" if id.start_with?("OBS-")
  return "area:quality" if %w[INT-01 TAIL-01 TAIL-06].include?(id)
  return "area:docs" if %w[TAIL-02 TAIL-03 TAIL-07 TAIL-08].include?(id)
  return "area:review" if %w[TAIL-04 TAIL-05 TAIL-09].include?(id)
  return "area:release" if id == "TAIL-10"

  abort "unmapped area for #{id}"
end

def ensure_dirs
  [EVIDENCE, OPERATIONS, REQUESTS].each { |path| FileUtils.mkdir_p(path) }
end

def fsync_dir(path)
  File.open(path, File::RDONLY) { |dir| dir.fsync }
rescue Errno::EINVAL
  nil
end

def create_json(path, value)
  bytes = JSON.pretty_generate(value) + "\n"
  File.open(path, File::WRONLY | File::CREAT | File::EXCL, 0o600) do |file|
    file.write(bytes)
    file.flush
    file.fsync
  end
  fsync_dir(File.dirname(path))
rescue Errno::EEXIST
  existing = File.read(path)
  abort "existing receipt differs: #{path}" unless existing == bytes
end

def replace_json(path, value)
  bytes = JSON.pretty_generate(value) + "\n"
  tmp = "#{path}.next"
  File.open(tmp, File::WRONLY | File::CREAT | File::TRUNC, 0o600) do |file|
    file.write(bytes)
    file.flush
    file.fsync
  end
  File.rename(tmp, path)
  fsync_dir(File.dirname(path))
end

def run_json(argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  abort "command failed (#{status.exitstatus}): #{argv.join(' ')}\n#{stderr}\n#{stdout}" unless status.success?
  JSON.parse(stdout)
end

def github_binary
  ENV.fetch("CSDLC_GITHUB_ISSUE_BIN")
end

def token_file
  ENV.fetch("ADL_GITHUB_TOKEN_FILE", File.join(Dir.home, "keys/github.token"))
end

def operation_paths(sequence, identity)
  prefix = format("%03d-%s", sequence, identity.downcase)
  [File.join(OPERATIONS, "#{prefix}-intent.json"), File.join(OPERATIONS, "#{prefix}-observed.json"), File.join(REQUESTS, "#{prefix}-request.json")]
end

def body_for(id, title, spec, dependency_map, predecessors)
  deps = dependency_map.map { |name, issue| "- #{name}: ##{issue}" }
  retained = predecessors.map { |issue| "##{issue}" }
  <<~BODY
    ## Outcome

    #{spec.fetch("objective")}

    ## Primary deliverable

    #{spec.fetch("primary_deliverable")}

    ## Dependencies

    #{deps.empty? ? "- #480 (WP-01 opening authority)" : deps.join("\n")}

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
end

def dependency_numbers(row, observed)
  Array(row["depends_on"]).to_h do |dependency|
    issue = EXISTING[dependency] || observed.dig(dependency, "issue")
    abort "unresolved dependency #{dependency}" unless issue
    [dependency, issue]
  end
end

def observed_children
  return {} unless Dir.exist?(OPERATIONS)

  Dir.glob(File.join(OPERATIONS, "*-observed.json")).sort.each_with_object({}) do |path, memo|
    row = JSON.parse(File.read(path))
    next unless row["kind"] == "child_create"
    memo[row.fetch("planned_id")] = row
  end
end

def update_partial(ids)
  observed = observed_children
  ordered = ids.map { |id| observed[id] }.compact
  next_absent = ids.find { |id| !observed.key?(id) }
  journal = Dir.glob(File.join(OPERATIONS, "*.json")).sort.map do |path|
    [File.basename(path), Digest::SHA256.file(path).hexdigest]
  end
  replace_json(PARTIAL_PATH, {
    schema: "adl.v0921.wp01.partial-receipt.v1",
    planning_digest: planning_digest,
    verified_prefix: ordered.map { |row| { planned_id: row["planned_id"], issue: row["issue"] } },
    next_absent_id: next_absent,
    journal_root_sha256: Digest::SHA256.hexdigest(JSON.generate(journal)),
    complete: next_absent.nil?
  })
end

def build_plan
  ensure_dirs
  specs = specifications
  rows = wave_rows
  ids = denominator(specs)
  plan = {
    schema: "adl.v0921.wp01.creation-plan.v1",
    repository: REPOSITORY,
    conductor_issue: WP01_ISSUE,
    milestone: MILESTONE_NUMBER,
    planning_digest: planning_digest,
    children: ids.each_with_index.map do |id, index|
      row = rows.fetch(id)
      {
        sequence: index + 1,
        planned_id: id,
        title: "[v0.92.1][#{id}] #{row.fetch('title')}",
        labels: [area_for(id), "track:roadmap", "type:task", VERSION_LABEL].sort,
        depends_on: Array(row["depends_on"]),
        predecessor_issues: Array(row["predecessor_issues"]),
        operation_key: "v0921-wp01:#{planning_digest}:#{id}:create",
        specification_sha256: Digest::SHA256.hexdigest(JSON.generate(specs.fetch(id)))
      }
    end,
    existing_routing: EXISTING_TARGETS.map do |issue, (title, labels)|
      { issue: issue, title: title, labels: labels.sort, milestone: MILESTONE_NUMBER,
        operation_key: "v0921-wp01:#{planning_digest}:existing-#{issue}:route" }
    end
  }
  replace_json(PLAN_PATH, plan)
  update_partial(ids)
  plan
end

def reconcile_existing(issue)
  plan = build_plan
  row = plan.fetch(:existing_routing).find { |entry| entry[:issue] == issue } or abort "unsupported existing issue #{issue}"
  sequence = 900 + plan.fetch(:existing_routing).index(row) + 1
  intent_path, observed_path, request_path = operation_paths(sequence, "existing-#{issue}")
  request = {
    repository: REPOSITORY, action: "issue_update", operation_key: row[:operation_key], token_file: token_file,
    issue: issue, pull_request: nil, title: row[:title], body: nil, labels: row[:labels], assignees: [],
    milestone: row[:milestone], state: nil, comment_body: nil, required_checks: [], require_review: false, linked_issue: nil
  }
  fingerprint = Digest::SHA256.hexdigest(JSON.generate(request))
  create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "existing_route", issue: issue,
                             operation_key: row[:operation_key], request_fingerprint: fingerprint, planning_digest: planning_digest })
  replace_json(request_path, request)
  result = run_json([github_binary, "run", "--request", request_path])
  packet = result.fetch("issue")
  abort "existing issue readback mismatch ##{issue}" unless packet["number"] == issue && packet["title"] == row[:title] && packet["labels"].sort == row[:labels] && packet["milestone"] == MILESTONE_NUMBER
  create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_route", issue: issue,
                               operation_key: row[:operation_key], request_fingerprint: fingerprint,
                               live_response_sha256: Digest::SHA256.hexdigest(JSON.generate(result)), state: packet["state"] })
  puts JSON.pretty_generate(result)
end

def create_child(id)
  plan = build_plan
  entry = plan.fetch(:children).find { |row| row[:planned_id] == id } or abort "unknown planned ID #{id}"
  observed = observed_children
  if observed.key?(id)
    puts JSON.pretty_generate(observed.fetch(id))
    return
  end
  expected_next = plan.fetch(:children).find { |row| !observed.key?(row[:planned_id]) }.fetch(:planned_id)
  abort "out-of-order create: expected #{expected_next}, received #{id}" unless expected_next == id
  row = wave_rows.fetch(id)
  deps = dependency_numbers(row, observed)
  title = entry.fetch(:title)
  body = body_for(id, title, specifications.fetch(id), deps, entry.fetch(:predecessor_issues))
  request = {
    repository: REPOSITORY, action: "issue_create", operation_key: entry.fetch(:operation_key), token_file: token_file,
    issue: nil, pull_request: nil, title: title, body: body, labels: entry.fetch(:labels), assignees: [],
    milestone: MILESTONE_NUMBER, state: nil, comment_body: nil, required_checks: [], require_review: false, linked_issue: nil
  }
  fingerprint = Digest::SHA256.hexdigest(JSON.generate(request))
  intent_path, observed_path, request_path = operation_paths(entry.fetch(:sequence), id)
  create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "child_create", planned_id: id,
                             operation_key: entry.fetch(:operation_key), request_fingerprint: fingerprint,
                             planning_digest: planning_digest, dependencies: deps })
  replace_json(request_path, request)
  result = run_json([github_binary, "run", "--request", request_path])
  packet = result.fetch("issue")
  expected_body = body + "\n<!-- csdlc-github-operation:#{entry.fetch(:operation_key)} -->\n"
  abort "child readback mismatch #{id}" unless packet["title"] == title && packet["labels"].sort == entry.fetch(:labels) && packet["milestone"] == MILESTONE_NUMBER && packet["body"] == expected_body
  create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "child_create", planned_id: id,
                               issue: packet.fetch("number"), title: packet.fetch("title"), labels: packet.fetch("labels").sort,
                               milestone: packet.fetch("milestone"), state: packet.fetch("state"), dependencies: deps,
                               body_sha256: Digest::SHA256.hexdigest(packet.fetch("body")), operation_key: entry.fetch(:operation_key),
                               request_fingerprint: fingerprint, live_response_sha256: Digest::SHA256.hexdigest(JSON.generate(result)) })
  update_partial(plan.fetch(:children).map { |child| child.fetch(:planned_id) })
  puts JSON.pretty_generate(result)
end

def finalize
  plan = build_plan
  ids = plan.fetch(:children).map { |row| row.fetch(:planned_id) }
  observed = observed_children
  abort "cannot finalize: #{45 - observed.length} children absent" unless observed.keys.sort == ids.sort
  children = ids.map { |id| observed.fetch(id) }
  create_json(FINAL_PATH, {
    schema: "adl.v0921.wp01.final-creation-receipt.v1", repository: REPOSITORY, conductor_issue: WP01_ISSUE,
    planning_digest: planning_digest, children: children, child_count: children.length,
    issue_numbers_sha256: Digest::SHA256.hexdigest(JSON.generate(children.map { |row| row.fetch("issue") }))
  })
  puts File.read(FINAL_PATH)
end

command = ARGV.shift || "plan"
case command
when "plan" then puts JSON.pretty_generate(build_plan)
when "reconcile-existing" then reconcile_existing(Integer(ARGV.fetch(0)))
when "create" then create_child(ARGV.fetch(0))
when "finalize" then finalize
else abort "usage: execute-wave-creation.rb plan|reconcile-existing ISSUE|create ID|finalize"
end
