#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
REPORT = ROOT.join(".csdlc/evidence/5819/copy-report.json")
REPOSITORIES = [
  ["cognitive-sdlc-paper", "private"],
  ["godel-hadamard-bayes-paper", "private"],
  ["general-intelligence-paper-private", "private"],
  ["universal-tool-schema", "private"],
  ["agent-design-language", "public"]
].freeze
CONTROLS = ["asksifu", "Horust"].freeze
OPERATOR = "danielbaustin"
ISSUE = 5819
LIVE_API_SURFACES = {
  "projects" => "repository",
  "discussions" => "repository",
  "wiki" => "repository",
  "security" => "repository",
  "actions" => "actions_permissions",
  "workflows" => "workflows",
  "environments" => "environments",
  "rulesets" => "rulesets",
  "releases" => "releases",
  "collaborators" => "collaborators",
  "webhooks" => "webhooks",
  "deploy_keys" => "deploy_keys",
  "pages" => "pages",
  "secrets" => "action_secret_names",
  "variables" => "action_variable_names",
  "branch_protections" => "branch_protection"
}.freeze

def gh_json(path, allow_missing: false)
  stdout, stderr, status = Open3.capture3("gh", "api", path)
  return nil if allow_missing && !status.success? && stderr.match?(/HTTP 404|Not Found/i)
  abort "gh api #{path} failed: #{stderr.strip}" unless status.success?
  JSON.parse(stdout)
end

def gh_pages(path, key: nil)
  rows = []
  page = 1
  loop do
    separator = path.include?("?") ? "&" : "?"
    value = gh_json("#{path}#{separator}per_page=100&page=#{page}")
    batch = key ? value.fetch(key, []) : value
    abort "gh api #{path} did not return a list" unless batch.is_a?(Array)
    rows.concat(batch)
    break if batch.length < 100
    page += 1
  end
  rows
end

def canonical(value)
  case value
  when Hash
    value.keys.sort.to_h { |key| [key, canonical(value.fetch(key))] }
  when Array
    value.map { |entry| canonical(entry) }
  else
    value
  end
end

def json_digest(value)
  Digest::SHA256.hexdigest(JSON.generate(canonical(value)))
end

def artifact(relative, expected_digest, label)
  path = ROOT.join(relative.to_s).cleanpath
  abort "#{label} path escapes repository" unless path.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  abort "missing #{label}: #{relative}" unless path.file? && !path.zero?
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected_digest
  JSON.parse(path.read)
end

def refs(repository)
  gh_pages("repos/#{repository}/git/matching-refs/").map do |row|
    [row.fetch("ref"), row.dig("object", "sha")]
  end.select do |ref, _sha|
    ref.start_with?("refs/heads/", "refs/tags/", "refs/notes/")
  end.sort
end

def absent_or(path)
  value = gh_json(path, allow_missing: true)
  value.nil? ? {"status" => "absent"} : yield(value)
end

def api_surface_snapshot(repository, branch)
  repo = gh_json("repos/#{repository}")
  {
    "repository" => repo.slice(
      "id", "full_name", "visibility", "default_branch", "archived", "fork",
      "has_issues", "has_projects", "has_wiki", "has_discussions",
      "security_and_analysis"
    ),
    "actions_permissions" => gh_json("repos/#{repository}/actions/permissions").slice(
      "enabled", "allowed_actions", "selected_actions_url"
    ),
    "workflows" => gh_pages("repos/#{repository}/actions/workflows", key: "workflows")
      .map { |row| row.slice("id", "name", "path", "state") }
      .sort_by { |row| row.fetch("id") },
    "environments" => gh_pages("repos/#{repository}/environments", key: "environments")
      .map { |row| row.slice("id", "name", "protection_rules", "deployment_branch_policy") }
      .sort_by { |row| row.fetch("id") },
    "rulesets" => gh_pages("repos/#{repository}/rulesets")
      .map { |row| row.slice("id", "name", "target", "enforcement", "source_type") }
      .sort_by { |row| row.fetch("id") },
    "releases" => gh_pages("repos/#{repository}/releases")
      .map { |row| row.slice("id", "tag_name", "target_commitish", "draft", "prerelease") }
      .sort_by { |row| row.fetch("id") },
    "collaborators" => gh_pages("repos/#{repository}/collaborators?affiliation=all")
      .map { |row| row.slice("id", "login", "role_name", "permissions") }
      .sort_by { |row| row.fetch("id") },
    "webhooks" => gh_pages("repos/#{repository}/hooks")
      .map do |row|
        {
          "id" => row["id"],
          "type" => row["type"],
          "active" => row["active"],
          "events" => Array(row["events"]).sort,
          "content_type" => row.dig("config", "content_type"),
          "insecure_ssl" => row.dig("config", "insecure_ssl")
        }
      end.sort_by { |row| row.fetch("id") },
    "deploy_keys" => gh_pages("repos/#{repository}/keys")
      .map { |row| row.slice("id", "title", "read_only") }
      .sort_by { |row| row.fetch("id") },
    "pages" => absent_or("repos/#{repository}/pages") do |row|
      row.slice("status", "cname", "custom_404", "html_url", "build_type", "source", "https_enforced")
    end,
    "action_secret_names" => gh_pages("repos/#{repository}/actions/secrets", key: "secrets")
      .map { |row| row.fetch("name") }.sort,
    "action_variable_names" => gh_pages("repos/#{repository}/actions/variables", key: "variables")
      .map { |row| row.fetch("name") }.sort,
    "branch_protection" => absent_or("repos/#{repository}/branches/#{branch}/protection") do |row|
      row.slice(
        "required_status_checks", "enforce_admins", "required_pull_request_reviews",
        "restrictions", "required_linear_history", "allow_force_pushes", "allow_deletions",
        "block_creations", "required_conversation_resolution", "lock_branch", "allow_fork_syncing"
      )
    end
  }
end

def confirmation(comment_id, required_lines, label)
  abort "#{label} comment id missing" unless comment_id.to_i.positive?
  comment = gh_json("repos/danielbaustin/agent-design-language/issues/comments/#{comment_id}")
  abort "#{label} comment is not on issue ##{ISSUE}" unless comment["issue_url"].to_s.end_with?("/issues/#{ISSUE}")
  abort "#{label} comment author mismatch" unless comment.dig("user", "login") == OPERATOR
  body = comment["body"].to_s
  required_lines.each do |line|
    abort "#{label} comment lacks #{line.inspect}" unless body.lines.map(&:strip).include?(line)
  end
end

abort "missing copy report" unless REPORT.file? && !REPORT.zero?
report = JSON.parse(REPORT.read)
rows = report.fetch("repositories")

org = report.fetch("organization_readiness")
confirmation(
  org["confirmation_comment_id"],
  [
    "WP-02-ORG-READINESS: CONFIRMED",
    "OWNERS: CONFIRMED",
    "BILLING: CONFIRMED",
    "RECOVERY: CONFIRMED",
    "ACTIONS-POLICY: CONFIRMED",
    "PACKAGES: CONFIRMED",
    "GITHUB-APPS: CONFIRMED"
  ],
  "organization readiness"
)

REPOSITORIES.each_with_index do |(name, visibility), index|
  row = rows.fetch(index)
  source_name = "danielbaustin/#{name}"
  destination_name = "agent-logic/#{name}"
  source = gh_json("repos/#{source_name}")
  destination = gh_json("repos/#{destination_name}")
  source_before = artifact(row["source_before_path"], row["source_before_sha256"], "#{name} source-before")
  destination_after = artifact(row["destination_after_path"], row["destination_after_sha256"], "#{name} destination-after")

  abort "source identity mismatch for #{name}" unless source["full_name"] == source_name
  abort "destination identity mismatch for #{name}" unless destination["full_name"] == destination_name
  abort "source repository id drift for #{name}" unless source["id"].to_s == source_before["repository_id"].to_s
  abort "destination repository id drift for #{name}" unless destination["id"].to_s == destination_after["repository_id"].to_s
  abort "destination visibility mismatch for #{name}" unless destination["visibility"] == visibility
  abort "source visibility drift for #{name}" unless source["visibility"] == source_before["visibility"]
  abort "source default branch drift for #{name}" unless source["default_branch"] == source_before["default_branch"]
  abort "destination default branch mismatch for #{name}" unless destination["default_branch"] == source["default_branch"]

  source_head = gh_json("repos/#{source_name}/commits/#{source.fetch('default_branch')}").fetch("sha")
  destination_head = gh_json("repos/#{destination_name}/commits/#{destination.fetch('default_branch')}").fetch("sha")
  abort "source HEAD drift for #{name}" unless source_head == source_before["exact_head"]
  abort "destination HEAD mismatch for #{name}" unless destination_head == source_head
  source_refs = refs(source_name)
  destination_refs = refs(destination_name)
  abort "live source ref drift for #{name}" unless json_digest(source_refs) == source_before["refs_sha256"]
  abort "live destination ref drift for #{name}" unless json_digest(destination_refs) == destination_after["refs_sha256"]
  abort "Git ref mismatch for #{name}" unless source_refs == destination_refs

  source_api = api_surface_snapshot(source_name, source.fetch("default_branch"))
  destination_api = api_surface_snapshot(destination_name, destination.fetch("default_branch"))
  source_api_digest = json_digest(source_api)
  destination_api_digest = json_digest(destination_api)
  abort "live source API surface drift for #{name}" unless source_api_digest == source_before["api_surface_sha256"]
  abort "live destination API surface drift for #{name}" unless destination_api_digest == destination_after["api_surface_sha256"]

  packet = artifact(
    row["platform_disposition_packet_path"],
    row["platform_disposition_packet_sha256"],
    "#{name} platform disposition packet"
  )
  LIVE_API_SURFACES.each do |surface, key|
    proof = packet.fetch("surfaces").fetch(surface).fetch("proof")
    abort "#{name} #{surface} is not live-API proven" unless proof["kind"] == "live_api"
    abort "#{name} #{surface} source API digest mismatch" unless proof["source_sha256"] == json_digest(source_api.fetch(key))
    abort "#{name} #{surface} destination API digest mismatch" unless proof["destination_sha256"] == json_digest(destination_api.fetch(key))
  end

  actions = gh_json("repos/#{destination_name}/actions/permissions")
  abort "destination Actions state mismatch for #{name}" unless actions["enabled"] == row["expected_actions_enabled"]

  confirmation(
    row["operator_confirmation_comment_id"],
    [
      "WP-02-REPOSITORY: #{name}",
      "ACTIONS-DISABLED: #{row.fetch('actions_disabled_receipt_sha256')}",
      "ACTIONS-BEFORE-FIRST-PUSH: #{row.fetch('first_push_receipt_sha256')}",
      "LFS-PARITY: #{row.fetch('lfs').fetch('receipt_sha256')}",
      "PLATFORM-DISPOSITIONS: #{row.fetch('platform_disposition_packet_sha256')}",
      "SOURCE-IMMUTABILITY: #{row.fetch('source_after_sha256')}"
    ],
    "#{name} operator confirmation"
  )
end

handoff = gh_json("repos/danielbaustin/agent-design-language/issues/5888")
abort "#5888 website handoff is not open" unless handoff["state"] == "open"
handoff_body = handoff["body"].to_s.downcase
abort "#5888 is not bound to WP-02" unless handoff_body.include?("#5819")
abort "#5888 lacks the ADL destination gate" unless handoff_body.include?("agent-logic/agent-design-language")

CONTROLS.each do |name|
  source_name = "danielbaustin/#{name}"
  destination_name = "agent-logic/#{name}"
  expected = report.fetch("negative_controls").fetch(name)
  before = artifact(expected["source_before_path"], expected["source_before_sha256"], "#{name} control-before")
  source = gh_json("repos/#{source_name}")
  abort "control identity mismatch for #{name}" unless source["id"].to_s == before["repository_id"].to_s
  head = gh_json("repos/#{source_name}/commits/#{source.fetch('default_branch')}").fetch("sha")
  abort "control HEAD drift for #{name}" unless head == before["exact_head"]
  abort "control ref drift for #{name}" unless json_digest(refs(source_name)) == before["refs_sha256"]
  abort "control API surface drift for #{name}" unless json_digest(api_surface_snapshot(source_name, source.fetch("default_branch"))) == before["api_surface_sha256"]
  abort "control destination exists for #{name}" unless gh_json("repos/#{destination_name}", allow_missing: true).nil?
end

puts "WP-02 live verification valid: organization confirmation, five destination copies, API-visible settings, exact refs, and two untouched controls"
