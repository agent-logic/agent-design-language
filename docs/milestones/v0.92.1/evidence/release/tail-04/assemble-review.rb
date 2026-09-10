#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
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

def redact_machine_local_paths(value)
  prefixes = [
    ["", "Volumes", "FastWork"].join(File::SEPARATOR) + File::SEPARATOR,
    ["", "Users"].join(File::SEPARATOR) + File::SEPARATOR,
    ["", "private", "tmp"].join(File::SEPARATOR),
    ["", "var", "folders"].join(File::SEPARATOR)
  ]
  case value
  when Hash
    value.transform_values { |item| redact_machine_local_paths(item) }
  when Array
    value.map { |item| redact_machine_local_paths(item) }
  when String
    prefixes.reduce(value) do |text, prefix|
      text.gsub(%r{#{Regexp.escape(prefix)}[^\s\"')]+}, "<machine-local-path>")
    end
  else
    value
  end
end

def candidate_blob(candidate, path)
  output, error, status = Open3.capture3("git", "show", "#{candidate}:#{path}", chdir: ROOT)
  abort("candidate path unavailable: #{path}: #{error.strip}") unless status.success?
  output
end

manifest = read_json("run_manifest.json")
candidate = manifest.fetch("candidate_sha")
assignments = read_json("assignments.json").fetch("assignments")
finding_input_path = File.join(PACKET, "finding-input.json")
finding_inputs = File.file?(finding_input_path) ? JSON.parse(File.read(finding_input_path)).fetch("findings") : []

row_docs = %w[repo_inventory.json canonical-surface-inventory.json issue_inventory.json pull_request_inventory.json acceptance_coverage.json]
row_documents = row_docs.to_h { |name| [name, read_json(name)] }
row_documents.each_value do |document|
  document.fetch("rows").each do |row|
    evidence = row.fetch("evidence")
    next unless evidence.fetch("source") == "candidate" && evidence.fetch("sha256") == Digest::SHA256.hexdigest("")

    path = evidence.fetch("path")
    abort("empty candidate evidence digest contradicts candidate blob: #{path}") unless candidate_blob(candidate, path).empty?
    evidence["locator"] = {"command" => "test ! -s #{path}"}
  end
end
row_documents.each { |name, document| write_json(name, document) }
rows = row_documents.values.flat_map { |document| document.fetch("rows") }
rows_by_ref = rows.to_h { |row| [row.fetch("denominator_ref"), row] }

findings = finding_inputs.map do |input|
  path = input.fetch("path")
  content = candidate_blob(candidate, path)
  id = input.fetch("id")
  locator = if input.key?("command")
              {"command" => input.fetch("command")}
            else
              {"path" => path, "line" => input.fetch("line")}
            end
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
    "locator" => locator,
    "evidence" => {
      "subject_id" => id,
      "path" => path,
      "source" => "candidate",
      "revision" => candidate,
      "sha256" => Digest::SHA256.hexdigest(content),
      "locator" => locator
    },
    "detail" => input.fetch("detail"),
    "denominator_refs" => input.fetch("denominator_refs")
  }
end

unknown_refs = findings.flat_map { |finding| finding.fetch("denominator_refs") }.uniq - rows_by_ref.keys
abort("finding input cites unknown denominator refs: #{unknown_refs.join(', ')}") unless unknown_refs.empty?

specialist_inputs = assignments.to_h do |assignment|
  lane = assignment.fetch("lane")
  input_path = File.join(PACKET, "specialist-input", "#{lane.tr('_', '-')}.json")
  abort("missing independent specialist input: #{input_path.sub(ROOT + '/', '')}") unless File.file?(input_path)
  input = JSON.parse(File.read(input_path))
  redacted_input = redact_machine_local_paths(input)
  write_json("specialist-input/#{lane.tr('_', '-')}.json", redacted_input) unless redacted_input == input
  input = redacted_input
  abort("specialist input schema mismatch: #{lane}") unless input.fetch("schema") == "adl.v0921.internal_review_specialist_input.v1"
  abort("specialist input is not exact-candidate complete: #{lane}") unless
    input.fetch("lane") == lane && input.fetch("candidate_sha") == candidate && input.fetch("status") == "completed"
  reviewer = input.fetch("reviewer")
  abort("specialist reviewer identity is absent or non-independent: #{lane}") unless reviewer.start_with?("subagent:") && reviewer.length > 12
  abort("specialist review method is absent: #{lane}") if input.fetch("review_method").strip.length < 20
  assigned_refs = assignment.fetch("denominator_refs")
  abort("specialist input does not cover its exact assignment: #{lane}") unless input.fetch("denominator_refs").sort == assigned_refs.sort
  observations = input.fetch("observations")
  abort("specialist input observation coverage mismatch: #{lane}") unless
    observations.map { |row| row.fetch("ref") }.sort == assigned_refs.sort && observations.map { |row| row.fetch("ref") }.uniq.length == observations.length
  abort("specialist input contains non-terminal or content-free observations: #{lane}") unless observations.all? do |row|
    %w[verified_no_gap finding].include?(row.fetch("conclusion")) &&
      row.fetch("detail").strip.length >= 20 &&
      row.fetch("detail") != "Exact-candidate denominator row was enumerated and received the #{lane} specialist disposition."
  end
  [lane, {"path" => input_path, "document" => input}]
end

acceptance_by_ref = read_json("acceptance_coverage.json").fetch("rows").to_h { |row| [row.fetch("denominator_ref"), row] }
completed_assignments = assignments.map do |assignment|
  input = specialist_inputs.fetch(assignment.fetch("lane")).fetch("document")
  assignment.merge("status" => "completed", "reviewer" => input.fetch("reviewer"), "completed_at" => input.fetch("completed_at"))
end
write_json("assignments.json", {"schema" => "adl.v0921.internal_review_assignments.v2", "assignments" => completed_assignments})
assignments = completed_assignments

lane_results = assignments.map do |assignment|
  lane = assignment.fetch("lane")
  input = specialist_inputs.fetch(lane).fetch("document")
  lane_findings = findings.select { |finding| finding.fetch("source_lane") == lane }
  finding_refs = lane_findings.flat_map { |finding| finding.fetch("denominator_refs") }.uniq
  expected_finding_ids = lane_findings.map { |finding| finding.fetch("id") }.sort
  abort("specialist finding IDs differ from canonical lane findings: #{lane}") unless input.fetch("finding_ids").sort == expected_finding_ids
  observations = input.fetch("observations").map do |specialist_observation|
    ref = specialist_observation.fetch("ref")
    row = rows_by_ref.fetch(ref)
    acceptance_ref = ref.start_with?("ACCEPT-") ? ref.delete_prefix("ACCEPT-") : nil
    related = lane_findings.select do |finding|
      finding.fetch("denominator_refs").include?(ref) ||
        acceptance_ref && finding.fetch("affected_acceptance_refs").include?(acceptance_ref)
    end
    expected_conclusion = related.empty? ? "verified_no_gap" : "finding"
    abort("specialist observation contradicts canonical findings: #{lane}: #{ref}") unless specialist_observation.fetch("conclusion") == expected_conclusion
    if acceptance_by_ref.key?(ref)
      implementation = specialist_observation.fetch("implementation_disposition")
      proof = specialist_observation.fetch("proof_disposition")
      terminal_implementation = %w[implemented partial missing not_applicable]
      terminal_proof = %w[proved partial missing not_applicable]
      abort("acceptance disposition is not terminal: #{ref}") unless terminal_implementation.include?(implementation) && terminal_proof.include?(proof)
      acceptance_by_ref.fetch(ref)["implementation_disposition"] = implementation
      acceptance_by_ref.fetch(ref)["proof_disposition"] = proof
      acceptance_by_ref.fetch(ref)["specialist_detail"] = specialist_observation.fetch("detail")
      acceptance_by_ref.fetch(ref)["reviewer"] = input.fetch("reviewer")
    end
    {
      "ref" => ref,
      "evidence" => row.fetch("evidence"),
      "conclusion" => specialist_observation.fetch("conclusion"),
      "detail" => specialist_observation.fetch("detail"),
      "review_basis" => specialist_observation.fetch("review_basis")
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
    "findings" => lane_findings,
    "reviewer" => input.fetch("reviewer"),
    "review_method" => input.fetch("review_method"),
    "completed_at" => input.fetch("completed_at")
  })
  relative_report_path = report_path.sub(ROOT + "/", "")
  result = {
    "assignment_id" => assignment.fetch("id"),
    "lane" => lane,
    "outcome" => lane_findings.empty? ? "passed" : "findings",
    "candidate_sha" => candidate,
    "reviewer" => input.fetch("reviewer"),
    "evidence" => "Independent specialist input consumed from #{specialist_inputs.fetch(lane).fetch('path').sub(ROOT + '/', '')}; no observation was synthesized by the assembler.",
    "report_path" => relative_report_path,
    "report_sha256" => Digest::SHA256.file(report_path).hexdigest
  }
  if lane == "tests"
    invocations = input.fetch("test_invocations")
    abort("test specialist supplied fewer than three distinct proving invocations") unless invocations.length >= 3 && invocations.map { |row| row.fetch("id") }.uniq.length == invocations.length
    result["test_invocations"] = invocations
    result["execution_scope"] = input.fetch("execution_scope")
  end
  result
end

write_json("acceptance_coverage.json", {"schema" => "adl.v0921.internal_review_acceptance.v2", "rows" => acceptance_by_ref.values.sort_by { |row| row.fetch("denominator_ref") }})

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
  selected_results = lane_results.select { |result| lanes.include?(result.fetch("lane")) }
  observations = selected_results.map do |result|
    subject = "SUMMARY-#{name.sub('.json', '').upcase}-#{result.fetch('lane').upcase}"
    report_path = result.fetch("report_path")
    {
      "subject_id" => subject,
      "result" => result.fetch("outcome") == "findings" ? "finding" : "verified",
      "detail" => "#{result.fetch('lane')} specialist completed its exact assignment with outcome #{result.fetch('outcome')}.",
      "evidence" => {
        "subject_id" => subject,
        "path" => report_path,
        "source" => "packet",
        "sha256" => Digest::SHA256.file(report_path).hexdigest,
        "locator" => {"path" => report_path, "line" => 1}
      }
    }
  end
  abort("summary #{name} has no completed specialist lane") if observations.empty?
  write_json(name, {
    "schema" => "adl.v0921.internal_review_summary.v1",
    "candidate_sha" => candidate,
    "outcome" => selected.empty? ? "passed" : "findings",
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
