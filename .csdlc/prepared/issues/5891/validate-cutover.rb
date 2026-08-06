#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
INVENTORY_PATH = File.join(ROOT, "docs/repository-cutover/ADL_CANONICAL_REPOSITORY_CUTOVER_INVENTORY.json")
RUNBOOK_PATH = File.join(ROOT, "docs/repository-cutover/ADL_CANONICAL_REPOSITORY_CUTOVER.md")
CANONICAL = "agent-logic/agent-design-language"
LEGACY = "danielbaustin/agent-design-language"
IGNORED_PREPARATION_PREFIX = ".csdlc/preparation/"

def abort_with(message)
  abort "cutover validation failed: #{message}"
end

def capture(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  abort_with("#{argv.join(' ')}: #{stderr.strip}") unless status.success?
  stdout
end

def changed_and_untracked_paths
  changed = capture("git", "diff", "--name-only", "HEAD").lines.map(&:strip)
  untracked = capture("git", "ls-files", "--others", "--exclude-standard").lines.map(&:strip)
  (changed + untracked).reject(&:empty?).uniq.sort
end

def scope_delta(actual, allowed)
  {
    "unexpected" => (actual - allowed).sort,
    "missing" => (allowed - actual).sort
  }
end

def read_tsv(relative, expected_header)
  path = File.join(ROOT, relative)
  abort_with("missing manifest #{relative}") unless File.file?(path)
  lines = File.readlines(path, chomp: true)
  abort_with("empty manifest #{relative}") if lines.empty?
  header = lines.shift.split("\t", -1)
  abort_with("wrong header for #{relative}") unless header == expected_header
  lines.map.with_index do |line, index|
    values = line.split("\t", -1)
    abort_with("wrong column count in #{relative}:#{index + 2}") unless values.length == header.length
    header.zip(values).to_h
  end
end

def require_dispositions(relative, rows)
  rows.each_with_index do |row, index|
    abort_with("blank disposition in #{relative}:#{index + 2}") if row.fetch("disposition", "").strip.empty?
  end
end

def reject_machine_local_paths(relative)
  text = File.read(File.join(ROOT, relative))
  match = text.match(%r{/(?:Users|Volumes|private|var/folders)/})
  abort_with("machine-local absolute path retained in #{relative}: #{match[0]}") if match
end

abort_with("missing cutover inventory") unless File.file?(INVENTORY_PATH)
abort_with("missing cutover runbook") unless File.file?(RUNBOOK_PATH)

inventory = JSON.parse(File.read(INVENTORY_PATH))
abort_with("wrong inventory schema") unless inventory["schema"] == "adl.repository_cutover_inventory.v2"
abort_with("inventory must remain provisional") unless inventory["status"] == "provisional"
abort_with("missing immediate refresh gate") unless inventory["refresh_required"].to_s.include?("immediately before activation")
abort_with("wrong canonical repository") unless inventory["canonical_repository"] == CANONICAL
abort_with("wrong legacy repository") unless inventory["legacy_repository"] == LEGACY

allowed = inventory.dig("scope", "exact_allowlist")
abort_with("exact allowlist missing") unless allowed.is_a?(Array) && !allowed.empty?
abort_with("exact allowlist contains duplicates") unless allowed.uniq.length == allowed.length
abort_with("preparation state entered allowlist") if allowed.any? { |path| path.start_with?(IGNORED_PREPARATION_PREFIX) }

actual = changed_and_untracked_paths.reject { |path| path.start_with?(IGNORED_PREPARATION_PREFIX) }
delta = scope_delta(actual, allowed.sort)
unless delta.values.all?(&:empty?)
  abort_with("scope mismatch unexpected=#{delta['unexpected'].inspect} missing=#{delta['missing'].inspect}")
end

negative = scope_delta(
  allowed.sort + ["docs/unrelated-cutover-note.md", ".csdlc/evidence/9999/unrelated.json"],
  allowed.sort
)
abort_with("negative scope self-test failed") unless negative["unexpected"] == [
  ".csdlc/evidence/9999/unrelated.json",
  "docs/unrelated-cutover-note.md"
]
if ARGV.include?("--self-test-only")
  puts JSON.generate(schema: "adl.repository_cutover_scope_self_test.v1", result: "pass")
  exit 0
end

manifest_paths = inventory.fetch("disposition_manifests")
issue_rows = read_tsv(
  manifest_paths.fetch("active_issues"),
  %w[issue title authority disposition]
)
pr_rows = read_tsv(
  manifest_paths.fetch("active_pull_requests"),
  %w[pr title head base draft disposition]
)
worktree_rows = read_tsv(
  manifest_paths.fetch("worktrees"),
  %w[worktree_id head mode branch dirty disposition]
)
automation_rows = read_tsv(
  manifest_paths.fetch("automations"),
  %w[surface legacy_state canonical_state disposition]
)
integration_rows = read_tsv(
  manifest_paths.fetch("integrations"),
  %w[surface legacy_state canonical_state disposition]
)
reference_rows = read_tsv(
  manifest_paths.fetch("references"),
  %w[path classification disposition]
)

[
  [manifest_paths.fetch("active_issues"), issue_rows],
  [manifest_paths.fetch("active_pull_requests"), pr_rows],
  [manifest_paths.fetch("worktrees"), worktree_rows],
  [manifest_paths.fetch("automations"), automation_rows],
  [manifest_paths.fetch("integrations"), integration_rows],
  [manifest_paths.fetch("references"), reference_rows]
].each { |relative, rows| require_dispositions(relative, rows) }

raw_issues = JSON.parse(File.read(File.join(ROOT, ".csdlc/evidence/5891/open-issues.json")))
issue_ids = issue_rows.map { |row| Integer(row.fetch("issue")) }.sort
abort_with("active issue manifest mismatch") unless issue_ids == raw_issues.map { |row| row.fetch("number") }.sort

raw_prs = JSON.parse(File.read(File.join(ROOT, ".csdlc/evidence/5891/open-pull-requests.json")))
declared_pr_ids = pr_rows.map { |row| Integer(row.fetch("pr")) }.sort
abort_with("active PR manifest mismatch") unless declared_pr_ids == raw_prs.map { |row| row.fetch("number") }.sort
abort_with("active PR snapshot count mismatch") unless raw_prs.length == inventory.dig("snapshot_counts", "active_pull_requests")

workflow_paths = File.readlines(File.join(ROOT, ".csdlc/evidence/5891/workflows.txt"), chomp: true).reject(&:empty?).sort
workflow_rows = automation_rows.select { |row| row.fetch("surface").start_with?(".github/workflows/") }
abort_with("workflow disposition mismatch") unless workflow_rows.map { |row| row.fetch("surface") }.sort == workflow_paths

expected_counts = inventory.fetch("snapshot_counts")
abort_with("worktree count mismatch") unless worktree_rows.length == expected_counts.fetch("worktrees")
abort_with("branch-bound worktree count mismatch") unless worktree_rows.count { |row| row["mode"] == "branch" } == expected_counts.fetch("branch_bound_worktrees")
abort_with("detached worktree count mismatch") unless worktree_rows.count { |row| row["mode"] == "detached" } == expected_counts.fetch("detached_worktrees")
abort_with("dirty worktree count mismatch") unless worktree_rows.count { |row| row["dirty"] == "dirty" } == expected_counts.fetch("dirty_worktrees")
abort_with("invalid worktree mode") unless worktree_rows.all? { |row| %w[branch detached].include?(row["mode"]) }
abort_with("invalid worktree dirty state") unless worktree_rows.all? { |row| %w[clean dirty unknown].include?(row["dirty"]) }
abort_with("detached worktree has branch") unless worktree_rows.select { |row| row["mode"] == "detached" }.all? { |row| row["branch"] == "-" }

operational = inventory.fetch("operational_references")
operational_paths = operational.map { |entry| entry.fetch("path") }.sort
reference_operational = reference_rows
  .select { |row| row["classification"] == "current_operational" }
  .map { |row| row.fetch("path") }
  .sort
abort_with("operational reference manifest mismatch") unless operational_paths == reference_operational
abort_with("operational reference count must be 12") unless operational_paths.length == 12

operational_paths.each do |relative|
  path = File.join(ROOT, relative)
  abort_with("missing current operational file #{relative}") unless File.file?(path)
  abort_with("canonical repository missing from #{relative}") unless File.read(path).include?(CANONICAL)
end

allowed.each do |relative|
  abort_with("allowlisted file missing #{relative}") unless File.file?(File.join(ROOT, relative))
  reject_machine_local_paths(relative)
end

runbook = File.read(RUNBOOK_PATH)
%w[legacy-origin rollback active issue provisional].each do |term|
  abort_with("runbook missing #{term}") unless runbook.downcase.include?(term)
end
runbook_lower = runbook.downcase.gsub(/\s+/, " ")
abort_with("runbook missing Sprint 1 refresh gate") unless runbook.include?("Sprint 1") && runbook_lower.include?("immediately before activation")

puts JSON.generate(
  schema: "adl.repository_cutover_static_validation.v2",
  result: "pass",
  exact_allowlist_paths: allowed.length,
  operational_references: operational_paths.length,
  active_issue_dispositions: issue_rows.length,
  active_pull_request_dispositions: pr_rows.length,
  worktree_dispositions: worktree_rows.length,
  automation_dispositions: automation_rows.length,
  integration_dispositions: integration_rows.length,
  negative_scope_self_test: "pass"
)
