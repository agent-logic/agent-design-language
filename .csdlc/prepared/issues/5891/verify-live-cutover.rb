#!/usr/bin/env ruby
# frozen_string_literal: true

require "base64"
require "digest"
require "json"
require "open3"
require "tmpdir"

ROOT = File.expand_path("../../../..", __dir__)
INVENTORY_PATH = File.join(ROOT, "docs/repository-cutover/ADL_CANONICAL_REPOSITORY_CUTOVER_INVENTORY.json")
LEGACY_REPO = "danielbaustin/agent-design-language"
CANONICAL_REPO = "agent-logic/agent-design-language"
LEGACY_URL = "https://github.com/#{LEGACY_REPO}.git"
CANONICAL_URL = "https://github.com/#{CANONICAL_REPO}.git"
PHASE = ENV.fetch("ADL_CUTOVER_PHASE", "pre")

def abort_with(message)
  abort "cutover live validation failed: #{message}"
end

def capture(*argv, chdir: ROOT)
  stdout, stderr, status = Open3.capture3(*argv, chdir: chdir)
  abort_with("#{argv.join(' ')}: #{stderr.strip}") unless status.success?
  stdout.strip
end

def repo(name)
  JSON.parse(capture("gh", "repo", "view", name, "--json", "nameWithOwner,visibility,isFork,defaultBranchRef,url"))
end

def remote_refs(url)
  capture("git", "ls-remote", "--heads", "--tags", url).lines.map(&:strip).reject(&:empty?).sort
end

def manifest_refs(relative)
  path = File.join(ROOT, relative)
  abort_with("missing #{PHASE}-cutover ref manifest: #{relative}") unless File.file?(path)
  File.readlines(path, chomp: true).reject(&:empty?).sort
end

def read_tsv(relative)
  lines = File.readlines(File.join(ROOT, relative), chomp: true)
  header = lines.shift.split("\t", -1)
  lines.map { |line| header.zip(line.split("\t", -1)).to_h }
end

def repository_file(repo_name, path, ref = "main")
  payload = JSON.parse(capture("gh", "api", "repos/#{repo_name}/contents/#{path}?ref=#{ref}"))
  Base64.decode64(payload.fetch("content"))
end

def name_inventory(repo_name, kind)
  payload = JSON.parse(capture("gh", "api", "repos/#{repo_name}/actions/#{kind}?per_page=100"))
  payload.fetch(kind).map { |row| row.fetch("name") }.sort
end

def expected_names(relative)
  JSON.parse(File.read(File.join(ROOT, relative))).sort
end

def workflow_state(repo_name, workflow)
  JSON.parse(capture("gh", "workflow", "view", workflow, "--repo", repo_name, "--json", "state")).fetch("state")
end

def integration_state(rows, surface)
  row = rows.find { |candidate| candidate.fetch("surface") == surface }
  abort_with("integration disposition missing: #{surface}") unless row
  row.fetch("canonical_state")
end

def live_worktrees
  capture("git", "worktree", "list", "--porcelain").split("\n\n").map do |record|
    fields = record.lines.map(&:strip).reject(&:empty?)
    path = fields.find { |line| line.start_with?("worktree ") }.to_s.delete_prefix("worktree ")
    head = fields.find { |line| line.start_with?("HEAD ") }.to_s.delete_prefix("HEAD ")
    branch_ref = fields.find { |line| line.start_with?("branch ") }
    mode = branch_ref ? "branch" : "detached"
    branch = branch_ref ? branch_ref.delete_prefix("branch refs/heads/") : "-"
    status = capture("git", "status", "--porcelain", chdir: path)
    {
      "worktree_id" => Digest::SHA256.hexdigest(path)[0, 16],
      "head" => head,
      "mode" => mode,
      "branch" => branch,
      "dirty" => status.empty? ? "clean" : "dirty",
      "disposition" => "preserve_head_branch_mode_and_dirty_state"
    }
  end.sort_by { |row| row.fetch("worktree_id") }
end

def rollback_drill
  git_common_dir = File.expand_path(capture("git", "rev-parse", "--git-common-dir"), ROOT)
  Dir.mktmpdir("adl-cutover-rollback-drill-", git_common_dir) do |dir|
    capture("git", "init", "--quiet", chdir: dir)
    capture("git", "remote", "add", "origin", CANONICAL_URL, chdir: dir)
    capture("git", "remote", "add", "legacy-origin", LEGACY_URL, chdir: dir)
    capture("git", "remote", "rename", "origin", "canonical", chdir: dir)
    capture("git", "remote", "rename", "legacy-origin", "origin", chdir: dir)
    abort_with("rollback drill did not restore legacy origin") unless capture("git", "remote", "get-url", "origin", chdir: dir) == LEGACY_URL
    abort_with("rollback drill lost canonical remote") unless capture("git", "remote", "get-url", "canonical", chdir: dir) == CANONICAL_URL
    capture("git", "remote", "rename", "origin", "legacy-origin", chdir: dir)
    capture("git", "remote", "rename", "canonical", "origin", chdir: dir)
    abort_with("rollback drill did not restore canonical origin") unless capture("git", "remote", "get-url", "origin", chdir: dir) == CANONICAL_URL
  end
  "pass"
end

abort_with("phase must be pre or post") unless %w[pre post].include?(PHASE)
inventory = JSON.parse(File.read(INVENTORY_PATH))
abort_with("wrong inventory schema") unless inventory["schema"] == "adl.repository_cutover_inventory.v3"

legacy_refs = remote_refs(LEGACY_URL)
canonical_refs = remote_refs(CANONICAL_URL)
manifest_prefix = PHASE == "pre" ? "pre" : "post"
expected_legacy_refs = manifest_refs(inventory.dig("refs", "#{manifest_prefix}_legacy_manifest"))
expected_canonical_refs = manifest_refs(inventory.dig("refs", "#{manifest_prefix}_canonical_manifest"))
abort_with("legacy full-ref manifest drift") unless legacy_refs == expected_legacy_refs
abort_with("canonical full-ref manifest drift") unless canonical_refs == expected_canonical_refs
abort_with("pre-cutover full-ref parity failed") if PHASE == "pre" && legacy_refs != canonical_refs

[repo(LEGACY_REPO), repo(CANONICAL_REPO)].each do |metadata|
  abort_with("repository must remain public") unless metadata["visibility"] == "PUBLIC"
  abort_with("repository must remain independent") if metadata["isFork"]
  abort_with("default branch must remain main") unless metadata.dig("defaultBranchRef", "name") == "main"
end

open_prs = JSON.parse(capture("gh", "pr", "list", "--repo", LEGACY_REPO, "--state", "open", "--limit", "200", "--json", "number"))
abort_with("legacy repository has undisposed open pull requests") unless open_prs.empty?

negative_repos = %w[agent-logic/asksifu agent-logic/Horust]
negative_repos.each do |name|
  _out, _err, status = Open3.capture3("gh", "repo", "view", name, chdir: ROOT)
  abort_with("excluded repository unexpectedly exists: #{name}") if status.success?
end

rollback_result = "not_run"
observed_integrations = {}
if PHASE == "post"
  origin = capture("git", "remote", "get-url", "origin")
  legacy_origin = capture("git", "remote", "get-url", "legacy-origin")
  abort_with("origin is not canonical") unless origin == CANONICAL_URL
  abort_with("legacy-origin is not preserved") unless legacy_origin == LEGACY_URL

  actions = JSON.parse(capture("gh", "api", "repos/#{CANONICAL_REPO}/actions/permissions"))
  abort_with("destination Actions are not enabled") unless actions["enabled"]

  expected_variables = expected_names(".csdlc/evidence/5891/canonical-variable-names.json")
  expected_secrets = expected_names(".csdlc/evidence/5891/canonical-secret-names.json")
  live_variables = name_inventory(CANONICAL_REPO, "variables")
  live_secrets = name_inventory(CANONICAL_REPO, "secrets")
  abort_with("canonical variable-name drift") unless live_variables == expected_variables
  abort_with("canonical secret-name drift") unless live_secrets == expected_secrets

  environments = JSON.parse(capture("gh", "api", "repos/#{CANONICAL_REPO}/environments?per_page=100"))
  environment_names = environments.fetch("environments").map { |row| row.fetch("name") }
  abort_with("destination adl-spot-ci environment missing") unless environment_names.include?("adl-spot-ci")

  integration_rows = read_tsv(inventory.dig("disposition_manifests", "integrations"))
  observed_integrations = {
    "repository-secrets" => "#{live_secrets.length}_names",
    "repository-variables" => "#{live_variables.length}_names",
    "environment:adl-spot-ci" => "present"
  }

  aws_workflows = %w[aws-codefriend-build.yaml aws-spot-remote-validation.yaml]
  observed_integrations["AWS-OIDC-and-CodeBuild"] = if aws_workflows.all? { |workflow| workflow_state(CANONICAL_REPO, workflow) == "disabled_manually" }
                                                        "dependent_workflows_disabled"
                                                      else
                                                        "dependent_workflows_not_disabled"
                                                      end

  codecov_url = "https://codecov.io/gh/#{CANONICAL_REPO}/graph/badge.svg?branch=main"
  codecov_body, _codecov_err, codecov_status = Open3.capture3("curl", "--fail", "--silent", "--show-error", "--location", codecov_url, chdir: ROOT)
  codecov_current = codecov_status.success? && !codecov_body.downcase.match?(/(?:unknown|error|not found)/)
  observed_integrations["Codecov"] = codecov_current ? "canonical_badge_current" : "canonical_badge_unproven"

  packages = JSON.parse(capture("gh", "api", "orgs/agent-logic/packages?package_type=container&per_page=100"))
  runners = JSON.parse(capture("gh", "api", "orgs/agent-logic/actions/runners?per_page=100"))
  installations = JSON.parse(capture("gh", "api", "orgs/agent-logic/installations?per_page=100"))
  hooks = JSON.parse(capture("gh", "api", "repos/#{CANONICAL_REPO}/hooks?per_page=100"))
  observed_integrations["packages"] = "#{packages.length}_packages"
  observed_integrations["organization-runners"] = "#{runners.fetch('total_count')}_runners"
  observed_integrations["GitHub-Apps-and-webhooks"] = "#{installations.length}_apps_#{hooks.length}_webhooks"

  observed_integrations.each do |surface, observed|
    expected = integration_state(integration_rows, surface)
    abort_with("integration drift for #{surface}: expected #{expected}, observed #{observed}") unless observed == expected
  end

  expected_worktrees = read_tsv(inventory.dig("disposition_manifests", "worktrees")).sort_by { |row| row.fetch("worktree_id") }
  abort_with("registered worktree continuity drift") unless live_worktrees == expected_worktrees

  canonical_badges = [
    "https://github.com/#{CANONICAL_REPO}/actions/workflows/ci.yaml",
    "https://codecov.io/gh/#{CANONICAL_REPO}/graph/badge.svg"
  ]
  %w[README.md adl/README.md].each do |path|
    text = repository_file(CANONICAL_REPO, path)
    canonical_badges.each { |badge| abort_with("canonical #{path} badge missing: #{badge}") unless text.include?(badge) }
    abort_with("legacy Actions badge remains in canonical #{path}") if text.include?("https://github.com/#{LEGACY_REPO}/actions/workflows/ci.yaml")
    abort_with("legacy Codecov badge remains in canonical #{path}") if text.include?("https://codecov.io/gh/#{LEGACY_REPO}")
  end

  common_main = inventory.dig("refs", "common_main_sha")
  comparison = JSON.parse(capture("gh", "api", "repos/#{LEGACY_REPO}/compare/#{common_main}...main"))
  legacy_changed_files = comparison.fetch("files").map { |row| row.fetch("filename") }.sort
  abort_with("legacy mutation exceeded README notice: #{legacy_changed_files.inspect}") unless legacy_changed_files == ["README.md"]
  legacy_notice = "> **Canonical development has moved:** New code, branches, and pull requests belong in " \
                  "[agent-logic/agent-design-language](https://github.com/agent-logic/agent-design-language)."
  baseline_readme = repository_file(LEGACY_REPO, "README.md", common_main)
  expected_legacy_readme = "#{legacy_notice}\n\n#{baseline_readme}"
  abort_with("legacy README differs from the exact notice-only projection") unless repository_file(LEGACY_REPO, "README.md") == expected_legacy_readme

  capture("git", "push", "--dry-run", "origin", "HEAD:refs/heads/codex/5891-push-authority-check")
  rollback_result = rollback_drill
end

puts JSON.generate(
  schema: "adl.repository_cutover_live_validation.v2",
  phase: PHASE,
  result: "pass",
  legacy_ref_count: legacy_refs.length,
  canonical_ref_count: canonical_refs.length,
  legacy_ref_digest: Digest::SHA256.hexdigest(legacy_refs.join("\n")),
  canonical_ref_digest: Digest::SHA256.hexdigest(canonical_refs.join("\n")),
  open_legacy_pull_requests: open_prs.length,
  excluded_repositories_absent: negative_repos,
  rollback_drill: rollback_result,
  observed_integrations: observed_integrations
)
