#!/usr/bin/env ruby
require "digest"
require "fileutils"
require "json"
require "tmpdir"
require "yaml"
require_relative "validate-internal-review"

def write_json(path, value)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, JSON.pretty_generate(value) + "\n")
end

def sh!(*argv)
  system(*argv) or abort("command failed: #{argv.join(' ')}")
end

Dir.mktmpdir("issue-520-production-", File.expand_path("../../../../.adl", __dir__)) do |repo|
  Dir.chdir(repo) do
    sh!("git", "init", "-q")
    sh!("git", "config", "user.email", "fixture@example.invalid")
    sh!("git", "config", "user.name", "fixture")
    File.write("base.txt", "base\n")
    sh!("git", "add", ".")
    sh!("git", "commit", "-qm", "base")
    base = `git rev-parse HEAD`.strip
    File.write("opening.txt", "opening\n")
    sh!("git", "add", ".")
    sh!("git", "commit", "-qm", "opening")
    opening = `git rev-parse HEAD`.strip

    File.write("test.rb", "puts 'test surface 1 passed'\n")
    File.write("test2.rb", "puts 'test surface 2 passed'\n")
    File.write("test3.rb", "puts 'test surface 3 passed'\n")
    specs = {"issue_specifications" => [{"id" => "WP-01", "acceptance_criteria" => ["observable result"]}]}
    spec_path = "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
    FileUtils.mkdir_p(File.dirname(spec_path)); File.write(spec_path, specs.to_yaml)
    receipt_path = "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"
    write_json(receipt_path, {"live_verified" => true, "child_count" => 0, "children" => []})
    %w[documentation demo provider_cloud retained_evidence].each { |kind| File.write("#{kind}.txt", "#{kind}\n") }
    sh!("git", "add", "."); sh!("git", "commit", "-qm", "candidate")
    candidate = `git rev-parse HEAD`.strip
    changed = `git diff --name-only #{base}...#{candidate}`.lines.map(&:strip).sort
    blob_evidence = lambda do |path, subject_id|
      content = `git show #{candidate}:#{path}`
      {"path" => path, "sha256" => Digest::SHA256.hexdigest(content), "source" => "candidate", "revision" => candidate, "subject_id" => subject_id, "locator" => {"path" => path, "line" => 1}}
    end

    root = File.join(repo, "packet")
    FileUtils.mkdir_p(root)
    issue = {"number" => 480, "title" => "WP-01", "state" => "CLOSED", "pull_requests" => [527]}
    response_path = File.join(root, "github-response.json")
    write_json(response_path, {"issues" => [issue]})
    query = "issues pageInfo hasNextPage endCursor closedByPullRequestsReferences"
    manifest = {
      "base_sha" => base, "candidate_sha" => candidate, "candidate_source" => "origin_main_after_review_gates",
      "candidate_merge_sha" => candidate,
      "opening_authority" => {"issue" => 480, "pull_request" => 527, "merge_sha" => opening, "base_sha" => base},
      "review_gate_observations" => [
        {"issue" => 718, "pull_request" => 809, "issue_state" => "CLOSED", "pr_state" => "MERGED", "head_sha" => candidate, "merge_sha" => candidate, "merged_at" => "2026-09-09T00:00:00Z", "issue_closed_at" => "2026-09-09T00:00:01Z"},
        {"issue" => 758, "pull_request" => 805, "issue_state" => "CLOSED", "pr_state" => "MERGED", "head_sha" => candidate, "merge_sha" => candidate, "merged_at" => "2026-09-09T00:00:02Z", "issue_closed_at" => "2026-09-09T00:00:03Z"}
      ],
      "execution_spec_sha256" => Digest::SHA256.hexdigest(File.read(spec_path)),
      "milestone_issue_count" => 1,
      "milestone_pull_request_count" => 1
    }
    write_json(File.join(root, "run_manifest.json"), manifest)
    milestone_pr = {"number" => 527, "title" => "WP-01 PR", "state" => "MERGED", "mergedAt" => "2026-09-01T00:00:00Z", "url" => "https://example.invalid/pr/527"}
    query = "issues pullRequests milestone pageInfo hasNextPage endCursor closedByPullRequestsReferences"
    write_json(response_path, {"issues" => [issue], "pull_requests" => [milestone_pr]})
    snapshot = {"repository" => "agent-logic/agent-design-language", "milestone" => "v0.92.1",
      "api_receipt" => {"transport" => "github_graphql", "page_size" => 100, "page_count" => 3, "issue_page_count" => 1, "closing_reference_page_count" => 1, "pull_request_page_count" => 1, "final_has_next_page" => false, "retrieved_at" => "now", "query" => query, "query_sha256" => Digest::SHA256.hexdigest(query), "response_path" => response_path, "response_sha256" => Digest::SHA256.file(response_path).hexdigest},
      "pagination_complete" => true, "next_cursor" => nil, "query_limit" => nil, "issues" => [issue], "pull_requests" => [milestone_pr]}
    write_json(File.join(root, "live-milestone-snapshot.json"), snapshot)
    repo_rows = changed.map.with_index { |path, i| ref="repo:#{i}"; {"path" => path, "denominator_ref" => ref, "classification" => "code", "disposition" => "review", "review_lane" => "code", "evidence" => blob_evidence.call(path, ref)} }
    write_json(File.join(root, "repo_inventory.json"), {"rows" => repo_rows})
    canonical = %w[documentation demo provider_cloud retained_evidence].map { |kind| ref="canonical:#{kind}"; {"kind" => kind, "path" => "#{kind}.txt", "denominator_ref" => ref, "evidence" => blob_evidence.call("#{kind}.txt", ref)} }
    write_json(File.join(root, "canonical-surface-inventory.json"), {"rows" => canonical})
    issue_row = issue.merge("planned_id" => "WP-01", "issue" => 480, "denominator_ref" => "issue:480", "retrieved_at" => "now", "disposition" => "review", "evidence" => blob_evidence.call(spec_path, "issue:480"))
    write_json(File.join(root, "issue_inventory.json"), {"rows" => [issue_row]})
    pr_row = {"pull_request" => 527, "title" => milestone_pr.fetch("title"), "state" => milestone_pr.fetch("state"), "merged_at" => milestone_pr.fetch("mergedAt"), "url" => milestone_pr.fetch("url"), "denominator_ref" => "pr:527", "retrieved_at" => "now", "disposition" => "review", "evidence" => blob_evidence.call(spec_path, "pr:527")}
    write_json(File.join(root, "pull_request_inventory.json"), {"rows" => [pr_row]})
    criterion = "observable result"
    ac_evidence=blob_evidence.call(spec_path,"ac:1").merge("criterion_id"=>"WP-01:AC-1")
    ac = {"planned_id" => "WP-01", "acceptance_id" => "AC-1", "criterion" => criterion, "criterion_sha256" => Digest::SHA256.hexdigest(JSON.generate(criterion)), "denominator_ref" => "ac:1", "implementation_disposition" => "implemented", "proof_disposition" => "proved", "specialist_detail" => "Implementation and proof are both present in the candidate fixture.", "reviewer" => "subagent:fixture", "evidence" => ac_evidence}
    write_json(File.join(root, "acceptance_coverage.json"), {"rows" => [ac]})
    refs = (repo_rows + canonical + [issue_row, pr_row, ac]).map { |row| row.fetch("denominator_ref") }
    lanes = %w[code tests documentation security architecture dependency provider_cloud demos retained_evidence]
    assignments = lanes.map.with_index { |lane, i| {"id" => "lane-#{i}", "lane" => lane, "denominator_refs" => [], "status" => "completed", "reviewer" => "subagent:fixture-#{lane}", "completed_at" => "2026-09-09T00:00:00Z"} }
    refs.each_with_index { |ref, i| assignments[i % assignments.length]["denominator_refs"] << ref }
    results = assignments.map do |assignment|
      report_path = File.join(root, "#{assignment.fetch('id')}.json")
      observations = assignment.fetch("denominator_refs").map { |ref| {"ref" => ref, "evidence" => blob_evidence.call(spec_path,ref), "conclusion" => "verified_no_gap", "detail" => "Inspected exact candidate surface and found no issue.", "review_basis" => {"kind" => "candidate_path", "subject" => spec_path}} }
      write_json(report_path, {"candidate_sha" => candidate, "denominator_refs" => assignment.fetch("denominator_refs"), "observations" => observations, "findings" => []})
      result = {"assignment_id" => assignment.fetch("id"), "lane" => assignment.fetch("lane"), "outcome" => "passed", "candidate_sha" => candidate, "reviewer" => "subagent:fixture-#{assignment.fetch('lane')}", "evidence" => "retained independent specialist fixture", "report_path" => report_path, "report_sha256" => Digest::SHA256.file(report_path).hexdigest}
      if assignment.fetch("lane") == "tests"
        result["execution_scope"] = "Three distinct deterministic candidate test surfaces were replayed; static review covers the remainder."
        result["test_invocations"] = %w[test.rb test2.rb test3.rb].each_with_index.map do |test_path, index|
          captured_output = "test surface #{index + 1} passed\n"
          {"id"=>"fixture-test-#{index + 1}","argv"=>["ruby",test_path],"working_directory"=>".","command_artifacts"=>[{"path"=>test_path,"sha256"=>Digest::SHA256.file(test_path).hexdigest}],"candidate_sha"=>candidate,"exit_status"=>0,"captured_output"=>captured_output,"captured_output_sha256"=>Digest::SHA256.hexdigest(captured_output),"success_markers"=>["test surface #{index + 1} passed"]}
        end
      end
      result
    end
    write_json(File.join(root, "assignments.json"), {"assignments" => assignments})
    write_json(File.join(root, "lane-results.json"), {"results" => results})
    write_json(File.join(root, "findings.json"), {"candidate_sha" => candidate, "outcome" => "passed", "findings" => []})
    %w[proof-results.json validation-results.json redaction-report.json quality-report.json].each { |name| subject="artifact:#{name}"; write_json(File.join(root, name), {"candidate_sha" => candidate, "outcome" => "passed", "observations" => [{"subject_id"=>subject,"result"=>"verified","detail"=>"resolved exact candidate evidence","evidence"=>blob_evidence.call(spec_path,subject)}]}) }
    required = %w[run_manifest.json live-milestone-snapshot.json repo_inventory.json canonical-surface-inventory.json issue_inventory.json pull_request_inventory.json acceptance_coverage.json assignments.json lane-results.json findings.json proof-results.json validation-results.json redaction-report.json quality-report.json]
    paths = required.map { |name| File.join(root, name) } + results.map { |row| row.fetch("report_path") } + [response_path]
    write_json(File.join(root, "packet-manifest.json"), {"entries" => paths.map { |path| {"path" => path, "sha256" => Digest::SHA256.file(path).hexdigest} }})

    bin = File.join(repo, "bin"); FileUtils.mkdir_p(bin)
    File.write(File.join(bin, "gh"), <<~SH)
      #!/bin/sh
      if [ "$1" = "pr" ] && [ "$2" = "view" ]; then
        merged_at="2026-09-09T00:00:00Z"
        [ "$3" = "805" ] && merged_at="2026-09-09T00:00:02Z"
        printf '{"number":%s,"state":"MERGED","headRefOid":"%s","mergeCommit":{"oid":"%s"},"mergedAt":"%s"}\n' "$3" "#{candidate}" "#{candidate}" "$merged_at"
      elif [ "$1" = "issue" ] && [ "$2" = "view" ]; then
        closing=809
        closed_at="2026-09-09T00:00:01Z"
        if [ "$3" = "758" ]; then closing=805; closed_at="2026-09-09T00:00:03Z"; fi
        printf '{"number":%s,"state":"CLOSED","closedAt":"%s","closedByPullRequestsReferences":[{"number":%s}]}\n' "$3" "$closed_at" "$closing"
      elif [ "$1" = "pr" ] && [ "$2" = "list" ]; then
        printf '%s\n' '[{"number":527,"title":"WP-01 PR","state":"MERGED","mergedAt":"2026-09-01T00:00:00Z","url":"https://example.invalid/pr/527","milestone":{"title":"v0.92.1"}}]'
      else
        printf '%s\n' '[{"number":480,"title":"WP-01","state":"CLOSED","closedByPullRequestsReferences":[{"number":527}]}]'
      fi
    SH
    FileUtils.chmod(0o755, File.join(bin, "gh")); ENV["PATH"] = "#{bin}:#{ENV.fetch('PATH')}"

    result = validate_packet!(root: root)
    abort("production fixture failed") unless result.fetch(:status) == "passed"

    reject_mutation = lambda do |name, paths, &mutation|
      originals = paths.to_h { |path| [path, File.binread(path)] }
      mutation.call
      packet_manifest_path = File.join(root, "packet-manifest.json")
      packet_manifest = JSON.parse(File.read(packet_manifest_path))
      packet_manifest.fetch("entries").each { |entry| entry["sha256"] = Digest::SHA256.file(entry.fetch("path")).hexdigest }
      write_json(packet_manifest_path, packet_manifest)
      rejected = false
      begin
        validate_packet!(root: root)
      rescue SystemExit, KeyError, TypeError
        rejected = true
      ensure
        originals.each { |path, content| File.binwrite(path, content) }
      end
      abort("#{name} production mutation unexpectedly passed") unless rejected
      puts JSON.generate(status: "passed", production_negative: name)
    end

    issue_path = File.join(root, "issue_inventory.json")
    reject_mutation.call("mutated_test_command", ["test.rb", File.join(root, "packet-manifest.json")]) do
      File.write("test.rb", "puts 'fabricated pass'\n")
    end
    reject_mutation.call("non_resolving_evidence", [issue_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(issue_path)); doc.fetch("rows").first["evidence"] = "looks convincing"; write_json(issue_path, doc)
    end
    reject_mutation.call("out_of_bounds_evidence_locator", [issue_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(issue_path)); doc.fetch("rows").first.fetch("evidence").fetch("locator")["line"] = 99_999; write_json(issue_path, doc)
    end
    reject_mutation.call("duplicate_planned_mapping", [issue_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(issue_path)); doc.fetch("rows") << doc.fetch("rows").first.dup; write_json(issue_path, doc)
    end
    snapshot_path = File.join(root, "live-milestone-snapshot.json")
    reject_mutation.call("omitted_live_pr", [snapshot_path, response_path, File.join(root, "packet-manifest.json")]) do
      snap = JSON.parse(File.read(snapshot_path)); snap.fetch("issues").first["pull_requests"] = []; write_json(snapshot_path, snap)
      write_json(response_path, {"issues" => snap.fetch("issues")}); snap.fetch("api_receipt")["response_sha256"] = Digest::SHA256.file(response_path).hexdigest; write_json(snapshot_path, snap)
    end
    pr_inventory_path = File.join(root, "pull_request_inventory.json")
    manifest_path = File.join(root, "run_manifest.json")
    assignments_path = File.join(root, "assignments.json")
    reject_mutation.call("jointly_omitted_milestone_pr", [snapshot_path, response_path, pr_inventory_path, manifest_path, assignments_path, File.join(root, "packet-manifest.json")]) do
      snap = JSON.parse(File.read(snapshot_path)); snap["pull_requests"] = []; write_json(snapshot_path, snap)
      write_json(response_path, {"issues" => snap.fetch("issues"), "pull_requests" => []}); snap.fetch("api_receipt")["response_sha256"] = Digest::SHA256.file(response_path).hexdigest; write_json(snapshot_path, snap)
      write_json(pr_inventory_path, {"rows" => []})
      run = JSON.parse(File.read(manifest_path)); run["milestone_pull_request_count"] = 0; write_json(manifest_path, run)
      assignments_doc = JSON.parse(File.read(assignments_path)); assignments_doc.fetch("assignments").each { |row| row.fetch("denominator_refs").delete("pr:527") }; write_json(assignments_path, assignments_doc)
    end
    reject_mutation.call("incomplete_pagination", [snapshot_path, File.join(root, "packet-manifest.json")]) do
      snap = JSON.parse(File.read(snapshot_path)); snap["pagination_complete"] = false; snap.fetch("api_receipt")["final_has_next_page"] = true; write_json(snapshot_path, snap)
    end
    reject_mutation.call("duplicate_review_gate", [manifest_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(manifest_path)); doc.fetch("review_gate_observations") << doc.fetch("review_gate_observations").last.dup; write_json(manifest_path, doc)
    end
    reject_mutation.call("forged_review_gate", [manifest_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(manifest_path)); doc.fetch("review_gate_observations").first["head_sha"] = base; write_json(manifest_path, doc)
    end
    reject_mutation.call("self_authored_mutable_base", [manifest_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(manifest_path)); doc["base_sha"] = candidate; doc.fetch("opening_authority")["base_sha"] = candidate; write_json(manifest_path, doc)
    end
    acceptance_path = File.join(root, "acceptance_coverage.json")
    reject_mutation.call("jointly_omitted_acceptance", [acceptance_path, assignments_path, File.join(root, "packet-manifest.json")]) do
      write_json(acceptance_path, {"rows" => []})
      doc = JSON.parse(File.read(assignments_path)); doc.fetch("assignments").each { |row| row.fetch("denominator_refs").delete("ac:1") }; write_json(assignments_path, doc)
    end
    reject_mutation.call("non_terminal_acceptance", [acceptance_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(acceptance_path)); doc.fetch("rows").first["proof_disposition"] = "requires_specialist_evidence_review"; write_json(acceptance_path, doc)
    end
    report_path = results.first.fetch("report_path")
    reject_mutation.call("content_free_lane", [report_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(report_path)); doc["observations"] = []; write_json(report_path, doc)
    end
    reject_mutation.call("missing_review_basis", [report_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(report_path)); doc.fetch("observations").first.delete("review_basis"); write_json(report_path, doc)
    end
    lane_results_path = File.join(root, "lane-results.json")
    reject_mutation.call("missing_dependency_lane", [assignments_path, lane_results_path, File.join(root, "packet-manifest.json")]) do
      assignments_doc = JSON.parse(File.read(assignments_path))
      dependency = assignments_doc.fetch("assignments").find { |row| row.fetch("lane") == "dependency" }
      architecture = assignments_doc.fetch("assignments").find { |row| row.fetch("lane") == "architecture" }
      architecture.fetch("denominator_refs").concat(dependency.fetch("denominator_refs"))
      assignments_doc.fetch("assignments").delete(dependency)
      write_json(assignments_path, assignments_doc)
      results_doc = JSON.parse(File.read(lane_results_path))
      results_doc.fetch("results").reject! { |row| row.fetch("assignment_id") == dependency.fetch("id") }
      write_json(lane_results_path, results_doc)
    end
    quality_path = File.join(root, "quality-report.json")
    reject_mutation.call("summary_outcome_contradiction", [quality_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(quality_path)); doc["outcome"] = "findings"; write_json(quality_path, doc)
    end
  end
end
