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
LIVE_CENSUS_PATH = File.join(EVIDENCE, "live-census.json")
WAVE_PATH = File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC_PATH = File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
CATALOG_PATH = File.join(MILESTONE, "PLANNED_ISSUE_CATALOG_v0.92.1.md")
READINESS_PATH = File.join(MILESTONE, "WP_EXECUTION_READINESS_v0.92.1.md")
REPOSITORY = "agent-logic/agent-design-language"
MILESTONE_NUMBER = 1
VERSION_LABEL = "version:v0.92.1"
WP01_ISSUE = 480
EXPECTED_PLANNING_DIGEST = "f00977324d7bfbfcb17a04d1798d14eca9c99c6d6299a0ae21977f564b518251"
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
EXPECTED_EXISTING = [51, 84, 122, 251, 261, 262, 263, 264, 342, 345].freeze
RETAINED_TARGETS = {
  51 => ["[v0.92.1][podcast][coordination] Coordinate gated podcast publication and directory-submission children", %w[area:demo track:roadmap type:task version:v0.92.1]],
  261 => ["[v0.92.1][podcast][51.a] Finalize show identity, artwork, rights, metadata, and mailbox readiness", %w[area:demo track:roadmap type:task version:v0.92.1]],
  262 => ["[v0.92.1][podcast][51.b] Publish and validate production hosting, RSS, enclosures, and playback", %w[area:demo track:roadmap type:task version:v0.92.1]],
  263 => ["[v0.92.1][podcast][51.c] Prepare directory submission runbooks and operator preflight", %w[area:demo track:roadmap type:docs version:v0.92.1]],
  264 => ["[v0.92.1][podcast][51.d] Execute directory submissions only after explicit operator authorization", %w[area:demo track:roadmap type:task version:v0.92.1]],
  342 => ["[v0.92.1][WP-24A] Podcast Studio first ten episodes", %w[area:authoring track:roadmap type:docs version:v0.92.1]]
}.freeze
HISTORICAL_TITLE_PROVENANCE = { "INT-01" => 188 }.freeze

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
  Digest::SHA256.hexdigest([WAVE_PATH, SPEC_PATH, CATALOG_PATH, READINESS_PATH].map { |path| Digest::SHA256.file(path).hexdigest }.join(":"))
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

def run_read_json(argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  abort "read command failed (#{status.exitstatus}): #{argv.join(' ')}\n#{stderr}" unless status.success?
  JSON.parse(stdout)
end

def normalize_live_issue(packet)
  milestone = packet["milestone"]
  milestone_number = milestone.is_a?(Hash) ? milestone["number"] : milestone
  milestone_title = milestone.is_a?(Hash) ? milestone["title"] : (milestone_number == 1 ? "v0.92.1" : nil)
  {
    "number" => packet.fetch("number"), "title" => packet.fetch("title"),
    "body" => packet["body"] || "", "state" => packet.fetch("state").downcase,
    "labels" => packet.fetch("labels", []).map { |label| label.is_a?(Hash) ? label.fetch("name") : label }.sort,
    "milestone" => milestone_number, "milestone_title" => milestone_title
  }
end

def live_issue(issue)
  normalize_live_issue(run_read_json(["gh", "api", "repos/#{REPOSITORY}/issues/#{issue}"]))
end

def assert_execution_authority!
  abort "planning authority digest changed" unless planning_digest == EXPECTED_PLANNING_DIGEST
  head = `git rev-parse HEAD`.strip
  abort "execution revision is not the operator-approved exact HEAD" unless ENV.fetch("WP01_APPROVED_REVISION") == head
  tracked_clean = system("git", "diff", "--quiet", "HEAD", "--", WAVE_PATH, SPEC_PATH, CATALOG_PATH, READINESS_PATH,
                         File.join(__dir__, "execute-wave-creation.rb"), File.join(__dir__, "validate-wave-creation.rb"),
                         PLAN_PATH, out: File::NULL, err: File::NULL)
  abort "reviewed execution surfaces have uncommitted drift" unless tracked_clean
  wp01 = live_issue(WP01_ISSUE)
  prerequisite = live_issue(432)
  abort "WP-01 live authority mismatch" unless wp01["title"] == "[v0.92.1][WP-01] Open the v0.92.1 milestone and create the execution wave" && wp01["state"] == "open" && wp01["milestone"] == MILESTONE_NUMBER && wp01["milestone_title"] == "v0.92.1"
  abort "#432 prerequisite is not terminal" unless prerequisite["state"] == "closed"
end

def live_milestone_census
  pages = run_read_json(["gh", "api", "--paginate", "--slurp", "repos/#{REPOSITORY}/issues?state=all&per_page=100"])
  pages.flatten.reject { |row| row.key?("pull_request") }.map { |row| normalize_live_issue(row) }
end

def github_binary
  ENV.fetch("CSDLC_GITHUB_ISSUE_BIN")
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

  memo = Dir.glob(File.join(OPERATIONS, "*-observed.json")).sort.each_with_object({}) do |path, rows|
    row = JSON.parse(File.read(path))
    next unless row["kind"] == "child_create"
    abort "stale planning digest in #{path}" unless row["planning_digest"] == planning_digest
    intent_path = path.sub(/-observed\.json\z/, "-intent.json")
    abort "missing intent for #{path}" unless File.file?(intent_path)
    intent = JSON.parse(File.read(intent_path))
    abort "intent/observed fingerprint mismatch for #{path}" unless intent["request_fingerprint"] == row["request_fingerprint"]
    abort "intent/observed operation mismatch for #{path}" unless intent["operation_key"] == row["operation_key"]
    request_path = File.join(REQUESTS, File.basename(path).sub(/-observed\.json\z/, "-request.json"))
    abort "missing retained request for #{path}" unless File.file?(request_path)
    request = JSON.parse(File.read(request_path))
    abort "retained request fingerprint mismatch for #{path}" unless canonical_request_fingerprint(request) == row["request_fingerprint"]
    id = row.fetch("planned_id")
    wave = wave_rows.fetch(id)
    spec = specifications.fetch(id)
    deps = dependency_numbers(wave, rows)
    expected_key = "v0921-wp01:#{planning_digest}:#{id}:create"
    expected_title = "[v0.92.1][#{id}] #{wave.fetch('title')}"
    expected_request = {
      repository: REPOSITORY, action: "issue_create", operation_key: expected_key, token_file: nil,
      issue: nil, pull_request: nil, title: expected_title,
      body: body_for(id, expected_title, spec, deps, Array(wave["predecessor_issues"])),
      labels: [area_for(id), "track:roadmap", "type:task", VERSION_LABEL].sort, assignees: [],
      milestone: MILESTONE_NUMBER, state: nil, comment_body: nil, required_checks: [], require_review: false, linked_issue: nil
    }
    abort "observed operation no longer matches current plan" unless row["operation_key"] == expected_key && request["operation_key"] == expected_key
    abort "retained request differs from current canonical request" unless canonical_request_fingerprint(expected_request) == row["request_fingerprint"]
    abort "retained dependency map differs from current plan" unless row["dependencies"] == deps
    live = live_issue(row.fetch("issue"))
    valid = live["title"] == row["title"] && live["labels"] == row["labels"] && live["milestone"] == row["milestone"] &&
            live["state"] == "open" && row["state"] == "open" && Digest::SHA256.hexdigest(live["body"]) == row["body_sha256"]
    abort "retained child live drift for #{id}" unless valid
    rows[id] = row
  end
  abort "retained child issue numbers are not unique" unless memo.values.map { |row| row["issue"] }.uniq.length == memo.length
  memo
end

def canonical_request_fingerprint(request)
  canonicalize = lambda do |value|
    case value
    when Hash then value.to_h { |key, child| [key.to_s, canonicalize.call(child)] }.sort.to_h
    when Array then value.map { |child| canonicalize.call(child) }
    else value
    end
  end
  portable = request.reject { |key, _| key.to_s == "token_file" }
  Digest::SHA256.hexdigest(JSON.generate(canonicalize.call(portable)))
end

def append_marker(body, operation_key)
  marker = "<!-- csdlc-github-operation:#{operation_key} -->"
  return body if body.include?(marker)
  body.end_with?("\n") ? "#{body}#{marker}\n" : "#{body}\n\n#{marker}\n"
end

def issue_matches_request?(issue, request)
  issue["title"] == request.fetch("title") && issue["labels"] == request.fetch("labels").sort &&
    issue["milestone"] == request.fetch("milestone") && issue["milestone_title"] == "v0.92.1" && issue["state"] == "open" &&
    issue["body"] == append_marker(request.fetch("body"), request.fetch("operation_key"))
end

def assert_no_conflicts!(plan)
  census = live_milestone_census
  observed = observed_children
  plan.fetch(:children).each do |entry|
    id = entry.fetch(:planned_id)
    marker = "<!-- csdlc-github-operation:#{entry.fetch(:operation_key)} -->"
    planned_identity = "- Planned ID: `#{id}`"
    candidates = census.select { |issue| issue["title"] == entry.fetch(:title) || issue["title"].include?("[#{id}]") || issue["body"].include?(marker) || issue["body"].include?(planned_identity) || issue["body"].include?(entry.fetch(:operation_key)) }
    candidates.reject! do |issue|
      issue["number"] == HISTORICAL_TITLE_PROVENANCE[id] && issue["state"] == "closed" &&
        issue["title"] == "[v0.92.1][INT-01] Run integrated independent review and remediation" &&
        !issue["body"].include?(entry.fetch(:operation_key)) && !issue["body"].include?(planned_identity)
    end
    allowed = observed[id]&.fetch("issue", nil)
    candidates.reject! { |issue| issue["number"] == allowed }
    # An intent-only retry may safely adopt the one remotely marked issue.
    intent = Dir.glob(File.join(OPERATIONS, "*-#{id.downcase}-intent.json")).first
    marked = candidates.select { |issue| issue["body"].include?(marker) }
    abort "ambiguous live operation marker for #{id}" if marked.length > 1
    request_path = intent && File.join(REQUESTS, File.basename(intent).sub(/-intent\.json\z/, "-request.json"))
    request = JSON.parse(File.read(request_path)) if request_path && File.file?(request_path)
    adoptable = intent && request && marked.length == 1 && issue_matches_request?(marked.first, request)
    candidates.reject! { |issue| adoptable && issue["number"] == marked.first["number"] }
    abort "live conflict for #{id}: #{candidates.map { |row| "##{row['number']} #{row['title']}" }.join(', ')}" unless candidates.empty?
  end
  plan.fetch(:existing_routing).each do |entry|
    marker = "<!-- csdlc-github-operation:#{entry.fetch(:operation_key)} -->"
    conflicts = census.select do |issue|
      issue["number"] != entry.fetch(:issue) && (issue["title"] == entry.fetch(:title) || issue["body"].include?(marker))
    end
    abort "existing-route live conflict for ##{entry.fetch(:issue)}" unless conflicts.empty?
  end
  retained = census.map do |issue|
    { number: issue["number"], title: issue["title"], state: issue["state"], labels: issue["labels"],
      milestone: issue["milestone"], milestone_title: issue["milestone_title"],
      body_sha256: Digest::SHA256.hexdigest(issue["body"]) }
  end
  replace_json(LIVE_CENSUS_PATH, { schema: "adl.v0921.wp01.live-census.v1", planning_digest: planning_digest,
                                   issues: retained })
  census
end

def update_partial(ids)
  observed = observed_children
  ordered = ids.map { |id| observed[id] }.compact
  next_absent = ids.find { |id| !observed.key?(id) }
  journal = (Dir.glob(File.join(OPERATIONS, "*.json")) + Dir.glob(File.join(REQUESTS, "*.json"))).sort.map do |path|
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
  assert_execution_authority!
  plan = build_plan
  assert_no_conflicts!(plan)
  abort "unsupported existing issue #{issue}" unless EXPECTED_EXISTING.include?(issue)
  live_before = live_issue(issue)
  row = plan.fetch(:existing_routing).find { |entry| entry[:issue] == issue }
  if row.nil?
    target = RETAINED_TARGETS.fetch(issue)
    abort "retained existing identity mismatch ##{issue}" unless live_before["title"] == target[0] && live_before["labels"] == target[1].sort && live_before["milestone"] == 1 && live_before["milestone_title"] == "v0.92.1" && live_before["state"] == "open"
    sequence = 800 + EXPECTED_EXISTING.index(issue) + 1
    intent_path, observed_path, = operation_paths(sequence, "existing-#{issue}")
    create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "existing_verify", issue: issue,
                               planning_digest: planning_digest, live_before_sha256: Digest::SHA256.hexdigest(JSON.generate(live_before)) })
    create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_verify", issue: issue,
                                 planning_digest: planning_digest, live_issue: live_before })
    puts JSON.pretty_generate(live_before)
    return
  end
  sequence = 900 + plan.fetch(:existing_routing).index(row) + 1
  intent_path, observed_path, request_path = operation_paths(sequence, "existing-#{issue}")
  if File.file?(observed_path)
    retained = JSON.parse(File.read(observed_path))
    abort "stale existing observed receipt" unless retained["planning_digest"] == planning_digest && retained.fetch("live_issue") == live_before
    puts JSON.pretty_generate(retained)
    return
  end
  if File.file?(intent_path)
    retained_intent = JSON.parse(File.read(intent_path))
    if retained_intent["kind"] == "existing_verify"
      abort "stale existing no-op intent" unless retained_intent["planning_digest"] == planning_digest && retained_intent["issue"] == issue && retained_intent["live_before_sha256"] == Digest::SHA256.hexdigest(JSON.generate(live_before))
      create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_verify", issue: issue,
                                   planning_digest: planning_digest, live_issue: live_before })
      puts JSON.pretty_generate(live_before)
      return
    end
    abort "stale existing route intent" unless retained_intent["planning_digest"] == planning_digest && retained_intent["issue"] == issue &&
      retained_intent["operation_key"] == row[:operation_key] && retained_intent["request_fingerprint"] == canonical_request_fingerprint(retained_intent.fetch("request")) &&
      retained_intent.dig("request", "title") == row[:title] && retained_intent.dig("request", "labels") == row[:labels] &&
      retained_intent.dig("request", "milestone") == 1 && retained_intent.dig("request", "operation_key") == row[:operation_key]
  end
  if !File.file?(intent_path) && live_before["title"] == row[:title] && live_before["labels"] == row[:labels] && live_before["milestone"] == row[:milestone]
    create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "existing_verify", issue: issue,
                               planning_digest: planning_digest, live_before_sha256: Digest::SHA256.hexdigest(JSON.generate(live_before)) })
    create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_verify", issue: issue,
                                 planning_digest: planning_digest, live_issue: live_before })
    puts JSON.pretty_generate(live_before)
    return
  end
  marker = "<!-- csdlc-github-operation:#{row[:operation_key]} -->"
  original_body = live_before["body"].end_with?("#{marker}\n") ? live_before["body"].delete_suffix("#{marker}\n") : live_before["body"]
  request = {
    repository: REPOSITORY, action: "issue_update", operation_key: row[:operation_key], token_file: nil,
    issue: issue, pull_request: nil, title: row[:title], body: original_body, labels: row[:labels], assignees: [],
    milestone: row[:milestone], state: nil, comment_body: nil, required_checks: [], require_review: false, linked_issue: nil
  }
  if File.file?(intent_path)
    retained_intent = JSON.parse(File.read(intent_path))
    request = retained_intent.fetch("request").transform_keys(&:to_sym)
  end
  fingerprint = canonical_request_fingerprint(request)
  unless File.file?(intent_path)
    create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "existing_route", issue: issue,
                               operation_key: row[:operation_key], request_fingerprint: fingerprint, planning_digest: planning_digest,
                               request: request, preimage: live_before })
  end
  retained_intent = JSON.parse(File.read(intent_path))
  postimage = live_before.merge("title" => row[:title], "labels" => row[:labels], "milestone" => 1,
                                "milestone_title" => "v0.92.1", "body" => append_marker(request.fetch(:body), row[:operation_key]))
  if live_before == postimage
    create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_route", issue: issue,
                                 operation_key: row[:operation_key], request_fingerprint: fingerprint,
                                 planning_digest: planning_digest, live_issue: live_before, adopted: true })
    puts JSON.pretty_generate(live_before)
    return
  end
  abort "existing route live state is neither retained preimage nor exact postimage ##{issue}" unless live_before == retained_intent.fetch("preimage")
  replace_json(request_path, request)
  assert_no_conflicts!(plan)
  abort "existing route changed after retained preimage ##{issue}" unless live_issue(issue) == retained_intent.fetch("preimage")
  result = run_json([github_binary, "run", "--request", request_path])
  packet = result.fetch("issue")
  expected_body = append_marker(request.fetch(:body), row[:operation_key])
  abort "existing issue readback mismatch ##{issue}" unless packet["number"] == issue && packet["title"] == row[:title] && packet["labels"].sort == row[:labels] && packet["milestone"] == MILESTONE_NUMBER && packet["body"] == expected_body
  fresh = live_issue(issue)
  abort "existing issue independent readback mismatch ##{issue}" unless fresh["title"] == row[:title] && fresh["labels"] == row[:labels] && fresh["milestone"] == 1 && fresh["milestone_title"] == "v0.92.1" && fresh["body"] == expected_body
  create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "existing_route", issue: issue,
                               operation_key: row[:operation_key], request_fingerprint: fingerprint,
                               planning_digest: planning_digest, live_issue: fresh, live_response: result, state: packet["state"] })
  puts JSON.pretty_generate(result)
end

def verify_existing_receipts!
  EXPECTED_EXISTING.map do |issue|
    target = EXISTING_TARGETS.key?(issue)
    sequence = target ? 900 + EXISTING_TARGETS.keys.index(issue) + 1 : 800 + EXPECTED_EXISTING.index(issue) + 1
    intent_path, observed_path, request_path = operation_paths(sequence, "existing-#{issue}")
    abort "existing issue verification is incomplete" unless File.file?(intent_path) && File.file?(observed_path)
    abort "ambiguous existing receipt set ##{issue}" unless Dir.glob(File.join(OPERATIONS, "*-existing-#{issue}-intent.json")) == [intent_path] && Dir.glob(File.join(OPERATIONS, "*-existing-#{issue}-observed.json")) == [observed_path]
    intent = JSON.parse(File.read(intent_path))
    receipt = JSON.parse(File.read(observed_path))
    abort "existing issue receipt mismatch ##{issue}" unless receipt["issue"] == issue && receipt["planning_digest"] == planning_digest
    abort "existing issue receipt kind mismatch ##{issue}" unless intent["kind"] == receipt["kind"]
    live = live_issue(issue)
    abort "existing issue drift ##{issue}" unless receipt.fetch("live_issue") == live
    if target && intent["kind"] == "existing_route"
      expected = EXISTING_TARGETS.fetch(issue)
      abort "existing route request is absent ##{issue}" unless File.file?(request_path)
      request = JSON.parse(File.read(request_path))
      abort "existing route fingerprint mismatch ##{issue}" unless canonical_request_fingerprint(request) == intent["request_fingerprint"] && intent["request_fingerprint"] == receipt["request_fingerprint"]
      abort "existing route operation mismatch ##{issue}" unless request["operation_key"] == intent["operation_key"] && intent["operation_key"] == receipt["operation_key"]
      abort "existing exact routing drift ##{issue}" unless live["title"] == expected[0] && live["labels"] == expected[1].sort && live["milestone"] == 1 && live["milestone_title"] == "v0.92.1"
    elsif target
      expected = EXISTING_TARGETS.fetch(issue)
      abort "existing no-op verification drift ##{issue}" unless live["title"] == expected[0] && live["labels"] == expected[1].sort && live["milestone"] == 1 && live["milestone_title"] == "v0.92.1"
    else
      expected = RETAINED_TARGETS.fetch(issue)
      abort "retained existing identity drift ##{issue}" unless live["title"] == expected[0] && live["labels"] == expected[1].sort && live["milestone"] == 1 && live["milestone_title"] == "v0.92.1" && live["state"] == "open"
      abort "existing verify preimage mismatch ##{issue}" unless intent["live_before_sha256"] == Digest::SHA256.hexdigest(JSON.generate(live))
    end
    { "issue" => issue, "live_sha256" => Digest::SHA256.hexdigest(JSON.generate(live)), "live_issue" => live }
  end
end

def create_child(id)
  assert_execution_authority!
  plan = build_plan
  assert_no_conflicts!(plan)
  entry = plan.fetch(:children).find { |row| row[:planned_id] == id } or abort "unknown planned ID #{id}"
  observed = observed_children
  verify_existing_receipts!
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
    repository: REPOSITORY, action: "issue_create", operation_key: entry.fetch(:operation_key), token_file: nil,
    issue: nil, pull_request: nil, title: title, body: body, labels: entry.fetch(:labels), assignees: [],
    milestone: MILESTONE_NUMBER, state: nil, comment_body: nil, required_checks: [], require_review: false, linked_issue: nil
  }
  fingerprint = canonical_request_fingerprint(request)
  intent_path, observed_path, request_path = operation_paths(entry.fetch(:sequence), id)
  create_json(intent_path, { schema: "adl.v0921.wp01.operation-intent.v1", kind: "child_create", planned_id: id,
                             operation_key: entry.fetch(:operation_key), request_fingerprint: fingerprint,
                             planning_digest: planning_digest, dependencies: deps })
  replace_json(request_path, request)
  adopted = live_milestone_census.select { |issue| issue["body"].include?("<!-- csdlc-github-operation:#{entry.fetch(:operation_key)} -->") }
  abort "ambiguous intent-only recovery for #{id}" if adopted.length > 1
  if adopted.length == 1
    packet = adopted.first
    abort "intent-only live issue does not match request for #{id}" unless issue_matches_request?(packet, request.transform_keys(&:to_s))
    create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "child_create", planned_id: id,
                                 issue: packet.fetch("number"), title: packet.fetch("title"), labels: packet.fetch("labels"),
                                 milestone: packet.fetch("milestone"), state: packet.fetch("state"), dependencies: deps,
                                 body_sha256: Digest::SHA256.hexdigest(packet.fetch("body")), operation_key: entry.fetch(:operation_key),
                                 request_fingerprint: fingerprint, planning_digest: planning_digest,
                                 specification_sha256: entry.fetch(:specification_sha256), live_response: packet })
    update_partial(plan.fetch(:children).map { |child| child.fetch(:planned_id) })
    puts JSON.pretty_generate(packet)
    return
  end
  assert_no_conflicts!(plan)
  result = run_json([github_binary, "run", "--request", request_path])
  packet = result.fetch("issue")
  expected_body = append_marker(body, entry.fetch(:operation_key))
  abort "child readback mismatch #{id}" unless packet["title"] == title && packet["labels"].sort == entry.fetch(:labels) && packet["milestone"] == MILESTONE_NUMBER && packet["body"] == expected_body && packet["state"].to_s.downcase == "open"
  create_json(observed_path, { schema: "adl.v0921.wp01.operation-observed.v1", kind: "child_create", planned_id: id,
                               issue: packet.fetch("number"), title: packet.fetch("title"), labels: packet.fetch("labels").sort,
                               milestone: packet.fetch("milestone"), state: "open", dependencies: deps,
                               body_sha256: Digest::SHA256.hexdigest(packet.fetch("body")), operation_key: entry.fetch(:operation_key),
                               request_fingerprint: fingerprint, planning_digest: planning_digest,
                               specification_sha256: entry.fetch(:specification_sha256), live_response: result })
  update_partial(plan.fetch(:children).map { |child| child.fetch(:planned_id) })
  puts JSON.pretty_generate(result)
end

def finalize
  assert_execution_authority!
  plan = build_plan
  ids = plan.fetch(:children).map { |row| row.fetch(:planned_id) }
  observed = observed_children
  abort "cannot finalize: #{45 - observed.length} children absent" unless observed.keys.sort == ids.sort
  children = ids.map { |id| observed.fetch(id) }
  assert_no_conflicts!(plan)
  live_children = children.map do |row|
    live = live_issue(row.fetch("issue"))
    expected = plan.fetch(:children).find { |entry| entry.fetch(:planned_id) == row.fetch("planned_id") }
    spec = specifications.fetch(row.fetch("planned_id"))
    exact_body = append_marker(body_for(row.fetch("planned_id"), expected.fetch(:title), spec,
                                        row.fetch("dependencies"), expected.fetch(:predecessor_issues)), expected.fetch(:operation_key))
    valid = live["title"] == expected.fetch(:title) && live["labels"] == expected.fetch(:labels) &&
            live["milestone"] == MILESTONE_NUMBER && live["milestone_title"] == "v0.92.1" && live["state"] == "open" && live.fetch("body") == exact_body &&
            row["specification_sha256"] == expected.fetch(:specification_sha256)
    abort "final live readback mismatch #{row.fetch('planned_id')} ##{row.fetch('issue')}" unless valid
    row.merge("live_readback_sha256" => Digest::SHA256.hexdigest(JSON.generate(live)))
  end
  existing_live = verify_existing_receipts!
  create_json(FINAL_PATH, {
    schema: "adl.v0921.wp01.final-creation-receipt.v1", repository: REPOSITORY, conductor_issue: WP01_ISSUE,
    planning_digest: planning_digest, children: live_children, child_count: live_children.length,
    live_verified: true, existing_issues_verified: EXPECTED_EXISTING, existing_issues: existing_live,
    journal_root_sha256: JSON.parse(File.read(PARTIAL_PATH)).fetch("journal_root_sha256"),
    issue_numbers_sha256: Digest::SHA256.hexdigest(JSON.generate(live_children.map { |row| row.fetch("issue") }))
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
