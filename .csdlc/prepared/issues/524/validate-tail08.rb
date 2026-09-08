#!/usr/bin/env ruby
# frozen_string_literal: true
require "yaml"
require "json"
require "digest"
ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = File.join(ROOT, "docs/milestones/v0.92.2")
AUTHORITY = File.join(ROOT, ".csdlc/evidence/524/predecessor.json")
TAIL = (1..10).map { |n| format("TAIL-%02d", n) }.freeze

def numbered_ids(text)
  text.lines.filter_map { |line| match = line.match(/\A\s*(\d+)\.\s+(TAIL-\d{2})\s+—/); [match[1].to_i, match[2]] if match }.sort.map(&:last)
end

def checklist_ids(text)
  text.lines.filter_map { |line| match = line.match(/\A- \[ \] (TAIL-\d{2})\b/); match[1] if match }
end

def errors_for(wave, spec, plan_ids, checklist, authority)
  rows = wave.fetch("work_packages")
  tail_rows = rows.select { |row| row.fetch("id").start_with?("TAIL-") }
  map = tail_rows.to_h { |row| [row.fetch("id"), Array(row["depends_on"])] }
  expected_map = TAIL.each_with_index.to_h { |id, index| [id, [index.zero? ? "CF-INTEGRATE" : TAIL[index - 1]]] }
  errors = []
  errors << "release-tail denominator mismatch" unless tail_rows.map { |row| row.fetch("id") } == TAIL
  errors << "release-tail dependency map mismatch" unless map == expected_map
  errors << "execution-spec denominator mismatch" unless spec.fetch("specifications").map { |row| row.fetch("id") } == rows.map { |row| row.fetch("id") }
  errors << "execution-spec tail mismatch" unless spec.dig("release_tail", "order") == TAIL
  errors << "release-plan tail mismatch" unless plan_ids == TAIL
  errors << "checklist tail mismatch" unless checklist == TAIL
  errors << "wrong predecessor authority" unless authority["issue"] == 523 && authority["reviewed"] == true && authority["merged"] == true
  errors << "invalid predecessor merge" unless authority["merge_sha"].to_s.match?(/\A[0-9a-f]{40}\z/)
  errors << "planning digest missing" unless authority["planning_package_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
  errors << "review evidence identity missing" unless authority["review_path"].to_s.start_with?(".csdlc/issues/523/") && authority["review_sha256"].to_s.match?(/\A[0-9a-f]{64}\z/)
  errors
end

if ARGV == ["--negative"]
  rows = [
    {"id" => "CF-INTEGRATE", "depends_on" => []},
    *TAIL.each_with_index.map { |id, index| {"id" => id, "depends_on" => [index.zero? ? "CF-INTEGRATE" : TAIL[index - 1]]} }
  ]
  wave = {"work_packages" => rows}
  spec = {"specifications" => rows.map { |row| {"id" => row["id"]} }, "release_tail" => {"order" => TAIL}}
  authority = {"issue" => 523, "reviewed" => true, "merged" => true, "merge_sha" => "a" * 40, "planning_package_sha256" => "b" * 64,
               "review_path" => ".csdlc/issues/523/cards/srp.md", "review_sha256" => "c" * 64}
  mutations = [[wave.merge("work_packages" => rows.map(&:dup).tap { |r| r.last["depends_on"] = ["TAIL-01"] }), spec, TAIL, TAIL, authority],
               [wave, spec.merge("release_tail" => {"order" => TAIL.reverse}), TAIL, TAIL, authority],
               [wave, spec, TAIL.drop(1), TAIL, authority], [wave, spec, TAIL, TAIL.reverse, authority],
               [wave, spec, TAIL, TAIL, authority.merge("merged" => false)], [wave, spec, TAIL, TAIL, authority.merge("merge_sha" => "main")]]
  abort "negative mutation escaped" unless mutations.all? { |args| !errors_for(*args).empty? }
  puts "issue 524 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

abort "missing #523 reviewed-merge authority" unless File.file?(AUTHORITY)
wave = YAML.safe_load(File.read(File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.2.yaml")), aliases: false)
spec = YAML.safe_load(File.read(File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml")), aliases: false)
authority = JSON.parse(File.read(AUTHORITY))
plan_ids = numbered_ids(File.read(File.join(MILESTONE, "RELEASE_PLAN_v0.92.2.md")))
checklist = checklist_ids(File.read(File.join(MILESTONE, "MILESTONE_CHECKLIST_v0.92.2.md")))
errors = errors_for(wave, spec, plan_ids, checklist, authority)
merge_sha = authority["merge_sha"]
errors << "#523 merge is not ancestral" unless merge_sha && system("git", "-C", ROOT, "merge-base", "--is-ancestor", merge_sha, "HEAD", out: File::NULL, err: File::NULL)
review_path = authority["review_path"]
review_bytes = IO.popen(["git", "-C", ROOT, "show", "#{merge_sha}:#{review_path}"], err: File::NULL, &:read)
errors << "#523 review evidence digest mismatch" unless $CHILD_STATUS.success? && Digest::SHA256.hexdigest(review_bytes) == authority["review_sha256"] && review_bytes.include?("Result: pass")
manifest_path = "docs/milestones/v0.92.2/SOURCE_DENOMINATOR_v0.92.2.json"
manifest_bytes = IO.popen(["git", "-C", ROOT, "show", "#{merge_sha}:#{manifest_path}"], err: File::NULL, &:read)
if $CHILD_STATUS.success?
  manifest = JSON.parse(manifest_bytes)
  errors << "#523 package digest mismatch" unless manifest["package_sha256"] == authority["planning_package_sha256"]
  rows = manifest.fetch("canonical_paths", []).sort.map do |path|
    bytes = IO.popen(["git", "-C", ROOT, "show", "#{merge_sha}:#{path}"], err: File::NULL, &:read)
    errors << "#523 package path missing: #{path}" unless $CHILD_STATUS.success?
    "#{path}\0#{Digest::SHA256.hexdigest(bytes)}\n"
  end
  errors << "#523 package recomputation mismatch" unless Digest::SHA256.hexdigest(rows.join) == authority["planning_package_sha256"]
else
  errors << "#523 source denominator missing at merge"
end
abort errors.join("\n") unless errors.empty?
puts "issue 524 exact release-tail map passed (#{TAIL.length} units)"
