#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, "docs/milestones/v0.91.8/evidence/wp13-external-bands")
MANIFEST = File.join(EVIDENCE, "external-band-deletion-manifest.json")
COORDINATION = File.join(EVIDENCE, "wp13-deletion-coordination.json")
ACCOUNTING = File.join(EVIDENCE, "deletion-accounting.json")

def fail!(message)
  warn("#5347 validation failed: #{message}")
  exit(1)
end

def rel(path)
  path.sub(ROOT + "/", "")
end

def load_json(path)
  fail!("missing #{rel(path)}") unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  fail!("invalid JSON #{rel(path)}: #{error.message}")
end

def git!(*argv)
  out, err, status = Open3.capture3("git", "-C", ROOT, *argv)
  fail!("git #{argv.join(' ')} failed: #{err.lines.first}") unless status.success?
  out
end

def validate_relative_path!(path)
  fail!("absolute path #{path}") if path.start_with?("/")
  fail!("escaping path #{path}") if path.split("/").include?("..")
  fail!("non-canonical path #{path}") unless path == File.expand_path(path, "/").sub(%r{\A/}, "")
  fail!("build/cache path #{path}") if (path.split("/") & %w[target build dist node_modules .git]).any?
end

def baseline_revision
  manifest.fetch("baseline_revision")
end

def tracked_blob_lines(path, object)
  actual_object = git!("rev-parse", "#{baseline_revision}:#{path}").strip
  fail!("baseline object mismatch for #{path}") unless actual_object == object
  git!("show", "#{baseline_revision}:#{path}").lines.count
end

def manifest
  @manifest ||= load_json(MANIFEST)
end

def coordination
  @coordination ||= load_json(COORDINATION)
end

def accounting
  @accounting ||= load_json(ACCOUNTING)
end

def deleted_rows
  rows = manifest.fetch("deleted_files")
  fail!("deleted_files must be sorted") unless rows.map { |row| row.fetch("path") } == rows.map { |row| row.fetch("path") }.sort
  rows
end

def retained_paths
  manifest.fetch("retained_current_binaries")
end

def validate_manifest!
  fail!("manifest schema mismatch") unless manifest["schema"] == "adl.wp13.external_band_deletion_manifest.v1"
  fail!("issue mismatch") unless manifest["issue"] == 5347
  fail!("repository mismatch") unless manifest["repository"] == "danielbaustin/agent-design-language"
  fail!("baseline revision malformed") unless baseline_revision.match?(/\A[0-9a-f]{40}\z/)
  _out, _err, status = Open3.capture3("git", "-C", ROOT, "merge-base", "--is-ancestor", baseline_revision, "HEAD")
  fail!("baseline revision is not ancestral to HEAD") unless status.success?
  fail!("merge order must keep #5347 before #5346") unless manifest["merge_order"] == [5347, 5346]
  deleted_rows.each do |row|
    path = row.fetch("path")
    validate_relative_path!(path)
    fail!("deleted row must be regular file") unless row["file_kind"] == "regular_file"
    fail!("deleted row must be non-generated") unless row["generated"] == false
    fail!("unexpected disposition for #{path}") unless row["disposition"] == "delete_external"
    fail!("unexpected owner for #{path}") unless row["replacement_owner"].to_s.match?(/Runtime v3|ADL v2|retained evidence|historical evidence|C-SDLC v2/)
    fail!("missing replacement proof for #{path}") if row["replacement_proof"].to_s.empty?
    expected = tracked_blob_lines(path, row.fetch("baseline_object"))
    fail!("line count mismatch for #{path}") unless row["measured_lines"] == expected
  end
  retained_paths.each do |path|
    validate_relative_path!(path)
    fail!("retained current binary missing: #{path}") unless File.file?(File.join(ROOT, path))
  end
end

def validate_deletions!
  deleted_rows.each do |row|
    path = row.fetch("path")
    fail!("deleted file still exists: #{path}") if File.exist?(File.join(ROOT, path))
    staged = git!("status", "--short", "--", path).strip
    historical = git!("diff", "--name-status", "#{baseline_revision}..HEAD", "--", path).strip
    deleted_in_history = historical.start_with?("D\t")
    deleted_in_worktree = staged.start_with?("D ") || staged.start_with?(" D")
    fail!("#{path} is not deleted relative to baseline") unless deleted_in_history || deleted_in_worktree
  end
  retained_paths.each do |path|
    fail!("retained current binary changed unexpectedly: #{path}") unless git!("diff", "--", path).strip.empty?
  end
end

def validate_coordination!
  fail!("coordination schema mismatch") unless coordination["schema"] == "adl.wp13.deletion_coordination.v1"
  fail!("safe merge order drift") unless coordination["safe_serialized_merge_order"] == [5347, 5346]
  reserved = coordination.fetch("reserved_for_5346")
  deleted_rows.each do |row|
    path = row.fetch("path")
    reserved.each do |entry|
      prefix = entry.fetch("path_prefix")
      fail!("#5347 path overlaps #5346 reserved prefix #{prefix}: #{path}") if path == prefix || path.start_with?("#{prefix}/")
    end
  end
end

def validate_accounting!
  fail!("accounting schema mismatch") unless accounting["schema"] == "adl.wp13.external_band_deletion_accounting.v1"
  removed = deleted_rows.sum { |row| row.fetch("measured_lines") }
  fail!("removed line accounting mismatch") unless accounting["removed_lines"] == removed
  cargo_removed = accounting.fetch("cargo_toml_removed_lines")
  fail!("Cargo removal must be positive") unless cargo_removed.positive?
  fail!("net line accounting mismatch") unless accounting["net_removed_lines"] == removed + cargo_removed
  fail!("deleted file count mismatch") unless accounting["deleted_file_count"] == deleted_rows.length
  fail!("WP-16 must not be a dependency") if accounting.fetch("execution_dependencies").include?(5351)
  fail!("#5346 must be coordination, not prerequisite") if accounting.fetch("execution_dependencies").include?(5346)
end

case ARGV.fetch(0, nil)
when "execution"
  validate_manifest!
  validate_deletions!
  validate_coordination!
  validate_accounting!
when "validate-contracts", "manifest-disjointness", "owner-and-consumer-proof", "deletion-budgets-and-evidence", "post-deletion-exact"
  validate_manifest!
  validate_deletions!
  validate_coordination!
  validate_accounting!
else
  fail!("unknown lane #{ARGV.fetch(0, '<missing>')}; expected execution")
end

puts(JSON.generate({
  schema: "adl.wp13.external_band_validation.v1",
  issue: 5347,
  lane: ARGV.fetch(0),
  status: "pass",
  deleted_files: deleted_rows.length,
  removed_lines: accounting["net_removed_lines"]
}))
