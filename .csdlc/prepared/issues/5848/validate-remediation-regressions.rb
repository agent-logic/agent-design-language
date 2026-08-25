#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

ROOT = "docs/reviews/v0.92/remediation-5848"
SOURCES = [
  "docs/reviews/v0.92/internal-review-5846/findings.json",
  "docs/reviews/v0.92/external-review-5847/findings-index.json"
].freeze
VALIDATORS = {
  "quality_matrix" => %w[ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb matrix],
  "quality_negative" => %w[ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb negative],
  "docs_release_truth" => %w[ruby .csdlc/prepared/issues/5843/validate-doc-release-truth.rb],
  "internal_review" => %w[ruby .csdlc/prepared/issues/5846/validate-internal-review.rb],
  "external_review" => %w[ruby .csdlc/prepared/issues/5847/validate-external-review.rb report],
  "release_evidence" => %w[ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb manifest],
  "production_birthday_focused" => %w[cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test production_birthday -- --test-threads=1]
}.freeze

def read_json!(path, label)
  abort "missing #{label}: #{path}" unless File.file?(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  abort "invalid #{label}: #{error.message}"
end

def source_findings(path)
  json = read_json!(path, "finding source")
  json.fetch("findings")
end

manifest = read_json!(ARGV.fetch(0, File.join(ROOT, "disposition-register.json")), "remediation disposition register")
abort "target revision policy must be current_git_head" unless manifest["target_revision_policy"] == "current_git_head"
SOURCES.each { |path| abort "canonical finding source missing: #{path}" unless File.file?(path) }
expected = SOURCES.flat_map { |path| source_findings(path).map { |row| row.fetch("id") } }.uniq.sort
rows = manifest["findings"]
abort "disposition register empty" unless rows.is_a?(Array) && !rows.empty?
abort "canonical finding universe mismatch" unless rows.map { |row| row["id"] }.sort == expected
abort "duplicate disposition rows" unless rows.map { |row| row["id"] }.uniq.length == rows.length

rows.each do |row|
  required = %w[id source severity evidence owner decision disposition residual_risk release_impact]
  abort "#{row['id']} disposition schema incomplete" unless required.all? { |field| !row[field].to_s.strip.empty? }
  case row["disposition"]
  when "resolved_existing_authority"
    %w[authority evidence_ref evidence_sha256].each do |field|
      abort "#{row['id']} #{field} missing" if row[field].to_s.strip.empty?
    end
    abort "#{row['id']} existing-authority evidence missing" unless File.file?(row["evidence_ref"])
    abort "#{row['id']} existing-authority digest mismatch" unless Digest::SHA256.file(row["evidence_ref"]).hexdigest == row["evidence_sha256"]
  when "resolved_current_pr"
    abort "#{row['id']} must be owned by #315" unless row["remediation_issue"] == 315
    %w[validation_ref validation_sha256].each do |field|
      abort "#{row['id']} #{field} missing" if row[field].to_s.strip.empty?
    end
    abort "#{row['id']} validation evidence missing" unless File.file?(row["validation_ref"])
    abort "#{row['id']} validation digest mismatch" unless Digest::SHA256.file(row["validation_ref"]).hexdigest == row["validation_sha256"]
  when "accepted_risk"
    abort "#{row['id']} accepted risk lacks operator authority" unless row["authority"].to_s.start_with?("operator:")
    abort "#{row['id']} accepted risk lacks expiry" if row["expires_at"].to_s.empty?
  when "follow_on"
    abort "#{row['id']} follow-on issue missing" unless row["follow_on_issue"].is_a?(Integer)
  else
    abort "#{row['id']} actionable finding remains open"
  end
end

regressions = manifest["regressions"]
abort "regression validator set empty" unless regressions.is_a?(Array) && !regressions.empty?
regressions.each do |entry|
  argv = VALIDATORS[entry["validator_id"]]
  abort "validator not allowlisted" unless argv
  stdout, stderr, status = Open3.capture3(*argv)
  abort "regression failed for #{entry['id']}: #{stdout}\n#{stderr}" unless status.success?
  case entry["evidence_mode"]
  when "status_only"
    abort "status-only regression must omit observed_sha256" if entry.key?("observed_sha256")
  else
    abort "regression output digest mismatch" unless Digest::SHA256.hexdigest(stdout + stderr) == entry["observed_sha256"]
  end
end

puts "PASS: complete finding universe, authoritative dispositions, current remediation evidence, and regressions"
