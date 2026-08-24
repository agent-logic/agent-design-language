#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "digest"
require "fileutils"
require "open3"
require "yaml"

ROOT_FILES = %w[
  README.md CHANGELOG.md AGENTS.md REVIEW.md docs/README.md
  docs/planning/ADL_FEATURE_LIST.md csdlc-v2/AGENTS.md
].freeze

BASE_COMMIT = "035b249096c6a27a6e40af9435d6df8e35090000"
ISSUE_LIFECYCLE_PATHS = %w[
  .csdlc/issues/312/audit.jsonl
  .csdlc/issues/312/index.json
  .csdlc/issues/312/cards/sip.md
  .csdlc/issues/312/cards/sip.values.json
  .csdlc/issues/312/cards/sor.md
  .csdlc/issues/312/cards/sor.values.json
  .csdlc/issues/312/cards/spp.md
  .csdlc/issues/312/cards/spp.values.json
  .csdlc/issues/312/cards/srp.md
  .csdlc/issues/312/cards/srp.values.json
  .csdlc/issues/312/cards/stp.md
  .csdlc/issues/312/cards/stp.values.json
  .csdlc/issues/312/cards/vpp.md
  .csdlc/issues/312/cards/vpp.values.json
  .csdlc/prepared/issues/312/design.md
  .csdlc/prepared/issues/312/diagram.mmd
  .csdlc/evidence/312/diff-hygiene.log
  .csdlc/evidence/312/docs-negative-suite.log
  .csdlc/evidence/312/docs-release-truth.log
  .csdlc/evidence/312/docs-structure-links-handoff.log
].freeze

def git_paths(*pathspecs)
  out, err, status = Open3.capture3("git", "ls-files", "--", *pathspecs)
  abort err unless status.success?
  out.lines.map(&:strip).reject(&:empty?)
end

def command_paths(*argv)
  out, err, status = Open3.capture3(*argv)
  abort err unless status.success?
  out.lines.map(&:strip).reject(&:empty?)
end

def declared_deliverables
  text = File.read(".csdlc/issues/312/cards/stp.md")
  match = text.match(/## Deliverables\n\n(.*?)\n\n## Acceptance/m)
  abort "STP deliverables section missing" unless match
  body = match[1]
  body.lines.each_with_object([]) do |line, paths|
    paths << line.delete_prefix("- ").strip if line.start_with?("- ")
  end
end

canonical = (ROOT_FILES + git_paths("docs/milestones/v0.92") +
  git_paths("csdlc-v2/operator/skills/*/SKILL.md") + %w[
    docs/milestones/v0.92/CANONICAL_DOC_INVENTORY_v0.92.md
    docs/milestones/v0.92/review/README.md
    docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md
  ]).uniq.sort
abort "canonical denominator is empty" if canonical.empty?
abort "tracked .adl dependency" if canonical.any? { |path| path.start_with?(".adl/") }

milestone_paths = git_paths("docs/milestones/v0.92") + canonical.grep(%r{\Adocs/milestones/v0\.92/})
milestone_paths << ENV["ADL_DOC_EXTRA_SCAN"] if ENV["ADL_DOC_EXTRA_SCAN"]
abort "tracked milestone .adl dependency" if milestone_paths.uniq.any? do |path|
  File.file?(path) && File.binread(path).include?(".adl/")
end

case ARGV.fetch(0, "packet")
when "generate"
  path = "docs/reviews/v0.92/docs-release-truth-312/inventory.json"
  FileUtils.mkdir_p(File.dirname(path))
  rows = canonical.map do |document|
    abort "canonical document missing: #{document}" unless File.file?(document)
    {
      "path" => document,
      "owner" => "WP-23/#312",
      "status" => "external_review_input",
      "evidence_source" => "candidate file SHA-256",
      "evidence_sha256" => Digest::SHA256.file(document).hexdigest,
      "required_action" => "independent findings-first review"
    }
  end
  File.write(path, JSON.pretty_generate({
    "schema" => "adl.v0.92.canonical_doc_inventory.v1",
    "issue" => 312,
    "rows" => rows
  }) + "\n")
when "packet"
  inventory = ENV.fetch("ADL_DOC_INVENTORY", "docs/reviews/v0.92/docs-release-truth-312/inventory.json")
  abort "missing inventory" unless File.file?(inventory)
  rows = JSON.parse(File.read(inventory)).fetch("rows")
  abort "canonical denominator mismatch" unless rows.map { |row| row.fetch("path") }.sort == canonical
  rows.each do |row|
    abort "inventory digest mismatch: #{row.fetch('path')}" unless
      Digest::SHA256.file(row.fetch("path")).hexdigest == row.fetch("evidence_sha256")
  end
when "structure-handoff"
  canonical.each do |path|
    abort "canonical document missing: #{path}" unless File.file?(path)
    case File.extname(path)
    when ".json" then JSON.parse(File.read(path))
    when ".yaml", ".yml" then YAML.safe_load(File.read(path), aliases: true)
    when ".md"
      File.read(path).scan(/\[[^\]]+\]\(([^)]+)\)/).flatten.each do |target|
        next if target.start_with?("http://", "https://", "mailto:", "#")
        relative = target.split("#", 2).first
        next if relative.empty?
        abort "broken link #{target} in #{path}" unless File.exist?(File.expand_path(relative, File.dirname(path)))
      end
    end
  end
  handoff = ENV.fetch("ADL_DOC_HANDOFF", "docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md")
  abort "missing external-review handoff" unless File.file?(handoff)
  handoff_text = File.read(handoff)
  abort "machine-local handoff path" if handoff_text.match?(%r{/(Users|home)/[^/\s]+/})
  %w[Send\ Gate Target\ Revision Reviewer\ Authority Findings\ Format Non-claims].each do |heading|
    abort "handoff section missing: #{heading}" unless handoff_text.include?(heading.tr("\\", ""))
  end
  changed = command_paths("git", "diff", "--name-only", BASE_COMMIT, "--") +
    command_paths("git", "ls-files", "--others", "--exclude-standard")
  changed.reject! do |path|
    path == ".csdlc/locks/312.lock" ||
      path.start_with?(".csdlc/evidence/.csdlc-finalize-312-")
  end
  allowed = (declared_deliverables + ISSUE_LIFECYCLE_PATHS).uniq
  unexpected = changed.uniq - allowed
  abort "out-of-scope changed paths: #{unexpected.join(',')}" unless unexpected.empty?
else
  abort "unknown mode"
end

puts JSON.generate(schema: "adl.v0.92.doc_release_truth.v1", status: "passed", canonical_paths: canonical.length)
