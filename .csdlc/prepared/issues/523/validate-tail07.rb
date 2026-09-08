#!/usr/bin/env ruby
# frozen_string_literal: true
require "yaml"
ROOT = File.expand_path("../../../..", __dir__)
MILESTONE = File.join(ROOT, "docs/milestones/v0.92.2")
TAIL = (1..10).map { |n| format("TAIL-%02d", n) }.freeze
REQUIRED_IDS = %w[WP-01 CF-SHELL CF-ADAPTER CF-EVIDENCE CF-COG CF-GOV CF-REVIEW CF-MEMORY CF-UX CF-PROOF CF-INTEGRATE PLAT-PROVIDER PLAT-MLX PLAT-UTS PLAT-RUST OPS-AWS PUB-MEDIUM PUB-CSDLC PLAT-MEMORY SPEC-RETEST].freeze
REQUIRED_DEFERRALS = %w[jira linear slack broad_workspace autonomous_mutation public_customer_scale security_tournaments ate oci_model_packaging optional_openrewrite runtime_v4].freeze

def semantic_errors(wave, spec, text)
  rows = wave.fetch("work_packages")
  ids = rows.map { |row| row.fetch("id") }
  errors = []
  errors << "duplicate planned ids" unless ids.uniq == ids
  errors << "required planned ids missing" unless (REQUIRED_IDS + TAIL - ids).empty?
  errors << "issue numbers allocated before milestone opening" unless rows.all? { |row| row["issue"].nil? }
  rows.each { |row| Array(row["depends_on"]).each { |dep| errors << "unknown dependency #{row['id']} -> #{dep}" unless ids.include?(dep) } }
  errors << "wave/spec denominator mismatch" unless spec.fetch("specifications").map { |row| row.fetch("id") } == ids
  errors << "wave tail mismatch" unless wave.fetch("canonical_release_tail") == TAIL
  errors << "spec tail mismatch" unless spec.fetch("release_tail").fetch("order") == TAIL
  errors << "required deferred route missing" unless (REQUIRED_DEFERRALS - wave.fetch("deferred_tracks")).empty?
  errors << "machine-local .adl source dependency" if text.include?(".adl/")
  errors << "Google Drive execution dependency" if text.match?(%r{https?://(?:docs|drive)\.google\.com})
  errors
end

if ARGV == ["--negative"]
  rows = (REQUIRED_IDS + TAIL).map { |id| {"id" => id, "issue" => nil, "depends_on" => []} }
  base = {"work_packages" => rows, "canonical_release_tail" => TAIL, "deferred_tracks" => REQUIRED_DEFERRALS}
  spec = {"specifications" => rows.map { |row| {"id" => row["id"]} }, "release_tail" => {"order" => TAIL}}
  mutations = [
    [base.merge("work_packages" => rows + [rows.first]), spec, ""],
    [base.merge("work_packages" => rows.map(&:dup).tap { |r| r.last["depends_on"] = ["MISSING"] }), spec, ""],
    [base.merge("canonical_release_tail" => TAIL.reverse), spec, ""],
    [base.merge("deferred_tracks" => REQUIRED_DEFERRALS - ["runtime_v4"]), spec, ""],
    [base, spec.merge("specifications" => []), ""],
    [base, spec, ".adl/docs/TBD/local.md"]
  ]
  abort "negative mutation escaped" unless mutations.all? { |args| !semantic_errors(*args).empty? }
  puts "issue 523 negative contract passed (#{mutations.length} mutations)"
  exit 0
end

required = %w[README.md WBS_v0.92.2.md WP_ISSUE_WAVE_v0.92.2.yaml WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml PLANNED_ISSUE_CATALOG_v0.92.2.md TBD_SCHEDULING_RECONCILIATION_v0.92.2.md]
abort "missing planning artifact" unless required.all? { |path| File.file?(File.join(MILESTONE, path)) && !File.zero?(File.join(MILESTONE, path)) }
wave = YAML.safe_load(File.read(File.join(MILESTONE, "WP_ISSUE_WAVE_v0.92.2.yaml")), aliases: false)
spec = YAML.safe_load(File.read(File.join(MILESTONE, "WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml")), aliases: false)
text = Dir.glob(File.join(MILESTONE, "**/*")).select { |path| File.file?(path) }.map { |path| File.binread(path) }.join("\n")
errors = semantic_errors(wave, spec, text)
abort errors.join("\n") unless errors.empty?
feature_text = File.read(File.join(ROOT, "docs/planning/ADL_FEATURE_LIST.md"))
abort "feature routing omits CodeFriend Beta 1/v0.95" unless feature_text.include?("CodeFriend Beta 1") && feature_text.include?("v0.92.2") && feature_text.include?("v0.95")
abort "diff hygiene failed" unless system("git", "diff", "--check")
puts "issue 523 planning semantics passed (#{wave.fetch('work_packages').length} planned units)"
