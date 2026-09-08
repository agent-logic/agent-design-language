#!/usr/bin/env ruby
# frozen_string_literal: true
require "yaml"
require "json"
require "digest"
ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = File.join(ROOT, "docs/milestones/v0.92.2")
MANIFEST = File.join(MILESTONE, "SOURCE_DENOMINATOR_v0.92.2.json")
TAIL = (1..10).map { |n| format("TAIL-%02d", n) }.freeze
REQUIRED_IDS = %w[WP-01 CF-SHELL CF-ADAPTER CF-EVIDENCE CF-COG CF-GOV CF-REVIEW CF-MEMORY CF-UX CF-PROOF CF-INTEGRATE PLAT-PROVIDER PLAT-MLX PLAT-UTS PLAT-RUST OPS-AWS PUB-MEDIUM PUB-CSDLC PLAT-MEMORY SPEC-RETEST].freeze
DISPOSITIONS = %w[delivered residual deferred excluded].freeze

def package_digest(paths)
  Digest::SHA256.hexdigest(paths.sort.map { |path| "#{path}\0#{Digest::SHA256.file(File.join(ROOT, path)).hexdigest}\n" }.join)
end

def revision_package_digest(revision, paths)
  rows = paths.sort.map do |path|
    bytes = IO.popen(["git", "-C", ROOT, "show", "#{revision}:#{path}"], err: File::NULL, &:read)
    return nil unless $CHILD_STATUS.success?
    "#{path}\0#{Digest::SHA256.hexdigest(bytes)}\n"
  end
  Digest::SHA256.hexdigest(rows.join)
end

def errors_for(wave, spec, text, manifest, actual_paths, source_paths)
  rows = wave.fetch("work_packages")
  ids = rows.map { |row| row.fetch("id") }
  errors = []
  errors << "duplicate planned ids" unless ids.uniq == ids
  errors << "planned denominator mismatch" unless ids.sort == (REQUIRED_IDS + TAIL).sort
  errors << "issue numbers allocated before milestone opening" unless rows.all? { |row| row["issue"].nil? }
  rows.each { |row| Array(row["depends_on"]).each { |dep| errors << "unknown dependency #{row['id']} -> #{dep}" unless ids.include?(dep) } }
  specs = spec.fetch("specifications")
  errors << "wave/spec denominator mismatch" unless specs.map { |row| row.fetch("id") } == ids
  errors << "wave tail mismatch" unless wave.fetch("canonical_release_tail") == TAIL
  errors << "spec tail mismatch" unless spec.fetch("release_tail").fetch("order") == TAIL
  errors << "machine-local absolute source path" if text.match?(%r{(?:/Users/|/Volumes/|file://)})
  errors << "Google Drive execution dependency" if text.match?(%r{https?://(?:docs|drive)\.google\.com})
  bad_adl = text.scan(%r{(?:`|\s)(\.adl/[^`\s,;)]+)}).flatten.reject { |path| path.start_with?(".adl/docs/TBD/") }
  errors << "non-provenance .adl dependency: #{bad_adl.uniq.join(', ')}" unless bad_adl.empty?
  errors << "wrong predecessor" unless manifest.dig("predecessor", "issue") == 522 && manifest.dig("predecessor", "reviewed_merge") == true && manifest.dig("predecessor", "merge_sha").to_s.match?(/\A[0-9a-f]{40}\z/)
  errors << "#522 review authority missing" unless manifest.dig("predecessor", "review_path").to_s == ".csdlc/issues/522/cards/srp.md" && manifest.dig("predecessor", "review_sha256").to_s.match?(/\A[0-9a-f]{64}\z/)
  errors << "#522 audit authority missing" unless manifest.dig("source_audit", "path").to_s.start_with?("docs/milestones/v0.92.1/evidence/release/tail-06/") && manifest.dig("source_audit", "sha256").to_s.match?(/\A[0-9a-f]{64}\z/)
  errors << "immutable predecessor-item denominator missing" unless manifest["predecessor_items"].is_a?(Array) && !manifest["predecessor_items"].empty?
  predecessor_paths = manifest.fetch("predecessor_paths", [])
  errors << "invalid predecessor path denominator" if predecessor_paths.empty? || predecessor_paths != predecessor_paths.sort.uniq
  errors << "canonical path denominator mismatch" unless manifest["canonical_paths"] == actual_paths
  source_rows = manifest.fetch("sources", [])
  errors << "source denominator mismatch" unless source_rows.map { |row| row["path"] }.sort == source_paths.sort
  errors << "source/audit item denominator mismatch" unless source_rows.map { |row| row["audit_id"] }.sort == manifest.fetch("predecessor_items", []).sort
  errors << "invalid source disposition" unless source_rows.all? { |row| DISPOSITIONS.include?(row["disposition"]) && !row["owner"].to_s.empty? }
  before = manifest["predecessor_package_sha256"].to_s
  after = manifest["package_sha256"].to_s
  errors << "invalid package digests" unless [before, after].all? { |value| value.match?(/\A[0-9a-f]{64}\z/) }
  errors << "stale unchanged successor package" if before == after
  errors << "current package digest mismatch" unless after == package_digest(actual_paths)
  errors
end

actual_paths = Dir.glob(File.join(MILESTONE, "**/*")).select { |path| File.file?(path) && path != MANIFEST }.map { |path| path.delete_prefix("#{ROOT}/") }.sort
actual_paths << "docs/planning/ADL_FEATURE_LIST.md"
actual_paths.sort!
reconciliation = File.read(File.join(MILESTONE, "TBD_SCHEDULING_RECONCILIATION_v0.92.2.md"))
source_paths = reconciliation.scan(/`(\.adl\/docs\/TBD\/[^`]+)`/).flatten.flat_map { |entry| entry.split(/`;\s*`/) }.sort

if ARGV == ["--negative"]
  rows = (REQUIRED_IDS + TAIL).map { |id| {"id" => id, "issue" => nil, "depends_on" => []} }
  wave = {"work_packages" => rows, "canonical_release_tail" => TAIL}
  spec = {"specifications" => rows.map { |row| {"id" => row["id"]} }, "release_tail" => {"order" => TAIL}}
  audit_ids = source_paths.each_index.map { |index| "F-#{index + 1}" }
  base = {"predecessor" => {"issue" => 522, "reviewed_merge" => true, "merge_sha" => "a" * 40, "review_path" => ".csdlc/issues/522/cards/srp.md", "review_sha256" => "f" * 64},
          "source_audit" => {"path" => "docs/milestones/v0.92.1/evidence/release/tail-06/dispositions.json", "sha256" => "e" * 64}, "predecessor_items" => audit_ids,
          "predecessor_paths" => actual_paths, "canonical_paths" => actual_paths,
          "sources" => source_paths.zip(audit_ids).map { |path, audit_id| {"path" => path, "audit_id" => audit_id, "disposition" => "residual", "owner" => "WP-01"} },
          "predecessor_package_sha256" => "b" * 64, "package_sha256" => package_digest(actual_paths)}
  mutations = [[wave.merge("work_packages" => rows + [rows.first]), spec, "", base],
               [wave, spec.merge("specifications" => []), "", base], [wave, spec, "/Users/example/TBD.md", base],
               [wave, spec, " `.adl/private/cache` ", base], [wave, spec, "", base.merge("canonical_paths" => actual_paths.drop(1))],
               [wave, spec, "", base.merge("sources" => [])], [wave, spec, "", base.merge("predecessor_items" => [])], [wave, spec, "", base.merge("predecessor_paths" => [])],
               [wave, spec, "", base.merge("predecessor_package_sha256" => base["package_sha256"])]]
  abort "negative mutation escaped" unless mutations.all? { |w, s, t, m| !errors_for(w, s, t, m, actual_paths, source_paths).empty? }
  puts "issue 523 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

abort "missing exact source denominator" unless File.file?(MANIFEST)
wave = YAML.safe_load(File.read(File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.2.yaml")), aliases: false)
spec = YAML.safe_load(File.read(File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml")), aliases: false)
manifest = JSON.parse(File.read(MANIFEST))
text = actual_paths.map { |path| File.binread(File.join(ROOT, path)) }.join("\n")
errors = errors_for(wave, spec, text, manifest, actual_paths, source_paths)
merge_sha = manifest.dig("predecessor", "merge_sha")
errors << "#522 merge is not ancestral" unless merge_sha && system("git", "-C", ROOT, "merge-base", "--is-ancestor", merge_sha, "HEAD", out: File::NULL, err: File::NULL)
if merge_sha
  review_path = manifest.dig("predecessor", "review_path")
  review_bytes = IO.popen(["git", "-C", ROOT, "show", "#{merge_sha}:#{review_path}"], err: File::NULL, &:read)
  errors << "#522 review artifact digest mismatch" unless $CHILD_STATUS.success? && Digest::SHA256.hexdigest(review_bytes) == manifest.dig("predecessor", "review_sha256") && review_bytes.include?("Result: pass")
  audit_path = manifest.dig("source_audit", "path")
  audit_bytes = IO.popen(["git", "-C", ROOT, "show", "#{merge_sha}:#{audit_path}"], err: File::NULL, &:read)
  if !$CHILD_STATUS.success? || Digest::SHA256.hexdigest(audit_bytes) != manifest.dig("source_audit", "sha256")
    errors << "#522 source audit digest mismatch"
  else
    begin
      audit = JSON.parse(audit_bytes)
      rows = audit["dispositions"] || audit["findings"] || audit["items"] || []
      item_ids = rows.map { |row| row["id"] || row["finding_id"] }.compact.sort
      errors << "#522 predecessor-item denominator mismatch" unless manifest["predecessor_items"] == item_ids
    rescue JSON::ParserError
      errors << "#522 source audit is not JSON"
    end
  end
  predecessor_paths = IO.popen(["git", "-C", ROOT, "ls-tree", "-r", "--name-only", merge_sha, "--", "docs/milestones/v0.92.2", "docs/planning/ADL_FEATURE_LIST.md"], err: File::NULL, &:read).lines.map(&:strip).reject(&:empty?).sort
  errors << "predecessor path denominator mismatch" unless manifest["predecessor_paths"] == predecessor_paths
  errors << "predecessor package digest mismatch" unless revision_package_digest(merge_sha, predecessor_paths) == manifest["predecessor_package_sha256"]
  changed = system("git", "-C", ROOT, "diff", "--quiet", merge_sha, "HEAD", "--", "docs/milestones/v0.92.2", "docs/planning/ADL_FEATURE_LIST.md")
  errors << "stale unchanged successor package" if changed
end
abort errors.join("\n") unless errors.empty?
puts "issue 523 exact successor denominator passed (#{actual_paths.length} paths; #{source_paths.length} sources)"
