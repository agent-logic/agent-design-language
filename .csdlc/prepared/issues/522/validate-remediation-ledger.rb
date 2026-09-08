#!/usr/bin/env ruby
require "json"

root = "docs/milestones/v0.92.1/evidence/release/tail-06"
mode = ARGV.fetch(0, "all")
required = %w[source-findings.json dispositions.json release-blockers.json packet-manifest.json]
missing = required.reject { |name| File.file?(File.join(root, name)) }
abort("missing remediation artifacts: #{missing.join(', ')}") unless missing.empty?
abort("unsupported mode: #{mode}") unless %w[all census dispositions].include?(mode)
docs = required.to_h { |name| [name, JSON.parse(File.read(File.join(root, name)))] }
source = docs.fetch("source-findings.json").fetch("findings")
dispositions = docs.fetch("dispositions.json").fetch("dispositions")
source_ids = source.map { |finding| finding.fetch("id") }
disposed_ids = dispositions.flat_map { |row| row.fetch("source_finding_ids") }
abort("source finding IDs are duplicated") unless source_ids.uniq.length == source_ids.length
abort("finding census does not disposition every source finding exactly once") unless disposed_ids.sort == source_ids.sort && disposed_ids.uniq.length == disposed_ids.length
abort("disposition metadata is incomplete") unless dispositions.all? do |row|
  case row.fetch("kind")
  when "fixed"
    %w[pr exact_head_review validation].all? { |key| row.key?(key) && !row[key].to_s.empty? }
  when "deferred"
    %w[owner rationale target_milestone release_consequence].all? { |key| row.key?(key) && !row[key].to_s.empty? }
  else
    false
  end
end
blockers = docs.fetch("release-blockers.json").fetch("unresolved")
abort("release-blocking findings remain") unless blockers == []
paths = docs.fetch("packet-manifest.json").fetch("entries").map { |entry| entry.fetch("path") }
abort("packet manifest omits required artifacts") unless required.all? { |name| paths.include?(File.join(root, name)) }
puts JSON.generate(schema: "adl.v0921.remediation_validation.v1", mode: mode, status: "passed", source_findings: source.length, dispositions: dispositions.length)
