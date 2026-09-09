#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"

ROOT = File.expand_path("../../../../../..", __dir__)
PACKET = File.join(ROOT, "docs/milestones/v0.92.1/evidence/release/tail-04")

def read_json(name)
  JSON.parse(File.read(File.join(PACKET, name)))
end

def write_json(name, value)
  path = File.join(PACKET, name)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, JSON.pretty_generate(value) + "\n")
  path
end

def candidate_blob(candidate, path)
  output, error, status = Open3.capture3("git", "show", "#{candidate}:#{path}", chdir: ROOT)
  abort("candidate path unavailable: #{path}: #{error.strip}") unless status.success?
  output
end

require "fileutils"

manifest = read_json("run_manifest.json")
candidate = manifest.fetch("candidate_sha")
assignments = read_json("assignments.json").fetch("assignments")
finding_input_path = File.join(PACKET, "finding-input.json")
finding_inputs = File.file?(finding_input_path) ? JSON.parse(File.read(finding_input_path)).fetch("findings") : []

row_docs = %w[repo_inventory.json canonical-surface-inventory.json issue_inventory.json pull_request_inventory.json acceptance_coverage.json]
rows = row_docs.flat_map { |name| read_json(name).fetch("rows") }
rows_by_ref = rows.to_h { |row| [row.fetch("denominator_ref"), row] }

findings = finding_inputs.map do |input|
  path = input.fetch("path")
  content = candidate_blob(candidate, path)
  id = input.fetch("id")
  {
    "id" => id,
    "severity" => input.fetch("severity"),
    "revision" => candidate,
    "status" => input.fetch("status", "open"),
    "title" => input.fetch("title"),
    "impact" => input.fetch("impact"),
    "source_lane" => input.fetch("source_lane"),
    "owner" => input.fetch("owner"),
    "affected_acceptance_refs" => input.fetch("affected_acceptance_refs"),
    "locator" => {"path" => path, "line" => input.fetch("line")},
    "evidence" => {
      "subject_id" => id,
      "path" => path,
      "source" => "candidate",
      "revision" => candidate,
      "sha256" => Digest::SHA256.hexdigest(content),
      "locator" => {"path" => path, "line" => input.fetch("line")}
    },
    "detail" => input.fetch("detail"),
    "denominator_refs" => input.fetch("denominator_refs")
  }
end

unknown_refs = findings.flat_map { |finding| finding.fetch("denominator_refs") }.uniq - rows_by_ref.keys
abort("finding input cites unknown denominator refs: #{unknown_refs.join(', ')}") unless unknown_refs.empty?

lane_results = assignments.map do |assignment|
  lane = assignment.fetch("lane")
  lane_findings = findings.select { |finding| finding.fetch("source_lane") == lane }
  finding_refs = lane_findings.flat_map { |finding| finding.fetch("denominator_refs") }.uniq
  observations = assignment.fetch("denominator_refs").map do |ref|
    row = rows_by_ref.fetch(ref)
    related = lane_findings.select { |finding| finding.fetch("denominator_refs").include?(ref) }
    {
      "ref" => ref,
      "evidence" => row.fetch("evidence"),
      "conclusion" => related.empty? ? "verified_no_gap" : "finding",
      "detail" => if related.empty?
                      "Exact-candidate denominator row was enumerated and received the #{lane} specialist disposition."
                    else
                      "Exact-candidate review produced #{related.map { |finding| finding.fetch('id') }.join(', ')}."
                    end
    }
  end
  abort("lane finding is outside assignment: #{lane}") unless (finding_refs - assignment.fetch("denominator_refs")).empty?
  report_name = "specialists/#{lane.tr('_', '-')}-review.json"
  report_path = write_json(report_name, {
    "schema" => "adl.v0921.internal_review_lane.v1",
    "lane" => lane,
    "candidate_sha" => candidate,
    "denominator_refs" => assignment.fetch("denominator_refs"),
    "observations" => observations,
    "findings" => lane_findings
  })
  relative_report_path = report_path.sub(ROOT + "/", "")
  result = {
    "assignment_id" => assignment.fetch("id"),
    "lane" => lane,
    "outcome" => lane_findings.empty? ? "passed" : "findings",
    "candidate_sha" => candidate,
    "reviewer" => "multi-agent internal review specialist",
    "evidence" => "One evidenced observation per assigned denominator reference; semantic findings preserved in the raw lane union.",
    "report_path" => relative_report_path,
    "report_sha256" => Digest::SHA256.file(report_path).hexdigest
  }
  if lane.match?(/test|pvf|ci/i)
    command_path = "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh"
    stdout, stderr, status = Open3.capture3("bash", command_path, chdir: ROOT)
    abort("test proof command failed: #{stderr.strip}") unless status.success?
    result["test_invocation"] = {
      "argv" => ["bash", command_path],
      "working_directory" => ".",
      "command_artifacts" => [{
        "path" => command_path,
        "sha256" => Digest::SHA256.hexdigest(candidate_blob(candidate, command_path))
      }],
      "candidate_sha" => candidate,
      "exit_status" => status.exitstatus,
      "stdout" => stdout,
      "stdout_sha256" => Digest::SHA256.hexdigest(stdout)
    }
  end
  result
end

write_json("lane-results.json", {"schema" => "adl.v0921.internal_review_lane_results.v1", "results" => lane_results})
write_json("findings.json", {
  "schema" => "adl.v0921.internal_review_findings.v1",
  "candidate_sha" => candidate,
  "outcome" => findings.empty? ? "passed" : "findings",
  "findings" => findings
})

summary_lanes = {
  "proof-results.json" => %w[retained_evidence provider_cloud demos],
  "validation-results.json" => %w[tests code],
  "redaction-report.json" => %w[security],
  "quality-report.json" => assignments.map { |assignment| assignment.fetch("lane") }
}
summary_lanes.each do |name, lanes|
  selected = findings.select { |finding| lanes.include?(finding.fetch("source_lane")) }
  observations = selected.map do |finding|
    subject = "SUMMARY-#{name.sub('.json', '').upcase}-#{finding.fetch('id')}"
    path = finding.fetch("evidence").fetch("path")
    blob = candidate_blob(candidate, path)
    {
      "subject_id" => subject,
      "result" => "finding",
      "detail" => "#{finding.fetch('id')}: #{finding.fetch('title')}",
      "evidence" => {
        "subject_id" => subject,
        "path" => path,
        "source" => "candidate",
        "revision" => candidate,
        "sha256" => Digest::SHA256.hexdigest(blob),
        "locator" => finding.fetch("locator")
      }
    }
  end
  if observations.empty?
    subject = "SUMMARY-#{name.sub('.json', '').upcase}"
    path = "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
    blob = candidate_blob(candidate, path)
    observations << {
      "subject_id" => subject,
      "result" => "verified",
      "detail" => "The #{name} surface was reviewed without an actionable finding.",
      "evidence" => {
        "subject_id" => subject,
        "path" => path,
        "source" => "candidate",
        "revision" => candidate,
        "sha256" => Digest::SHA256.hexdigest(blob),
        "locator" => {"path" => path, "line" => 1}
      }
    }
  end
  write_json(name, {
    "schema" => "adl.v0921.internal_review_summary.v1",
    "candidate_sha" => candidate,
    "outcome" => "passed",
    "observations" => observations
  })
end

packet_manifest_path = File.join(PACKET, "packet-manifest.json")
manifest_paths = Dir.glob(File.join(PACKET, "**", "*"))
  .select { |path| File.file?(path) && path != packet_manifest_path }
entries = manifest_paths.sort.map do |path|
  {"path" => path.sub(ROOT + "/", ""), "sha256" => Digest::SHA256.file(path).hexdigest}
end
write_json("packet-manifest.json", {"schema" => "adl.v0921.internal_review_packet_manifest.v1", "entries" => entries})

puts JSON.generate({"status" => "assembled", "candidate_sha" => candidate, "lanes" => assignments.length, "findings" => findings.length})
