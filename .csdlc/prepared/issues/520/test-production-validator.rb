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

    specs = {"issue_specifications" => [{"id" => "WP-01", "acceptance_criteria" => ["observable result"]}]}
    spec_path = "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
    FileUtils.mkdir_p(File.dirname(spec_path)); File.write(spec_path, specs.to_yaml)
    receipt_path = "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"
    write_json(receipt_path, {"live_verified" => true, "child_count" => 0, "children" => []})
    %w[documentation demo provider_cloud retained_evidence].each { |kind| File.write("#{kind}.txt", "#{kind}\n") }
    sh!("git", "add", "."); sh!("git", "commit", "-qm", "candidate")
    candidate = `git rev-parse HEAD`.strip
    changed = `git diff --name-only #{base}...#{candidate}`.lines.map(&:strip).sort
    blob_evidence = lambda do |path|
      content = `git show #{candidate}:#{path}`
      {"path" => path, "sha256" => Digest::SHA256.hexdigest(content), "source" => "candidate", "revision" => candidate}
    end

    root = File.join(repo, "packet")
    FileUtils.mkdir_p(root)
    issue = {"number" => 480, "title" => "WP-01", "state" => "CLOSED", "pull_requests" => [527]}
    response_path = File.join(root, "github-response.json")
    write_json(response_path, {"issues" => [issue]})
    query = "issues pageInfo hasNextPage endCursor closedByPullRequestsReferences"
    manifest = {
      "base_sha" => base, "candidate_sha" => candidate, "candidate_source_issue" => 519,
      "candidate_merge_sha" => candidate,
      "opening_authority" => {"issue" => 480, "pull_request" => 527, "merge_sha" => opening, "base_sha" => base},
      "tail_03_observation" => {"issue" => 519, "state" => "CLOSED", "merge_sha" => candidate, "retrieved_at" => "2026-09-07T00:00:00Z"},
      "execution_spec_sha256" => Digest::SHA256.hexdigest(File.read(spec_path))
    }
    write_json(File.join(root, "run_manifest.json"), manifest)
    snapshot = {"repository" => "agent-logic/agent-design-language", "milestone" => "v0.92.1",
      "api_receipt" => {"transport" => "github_graphql", "page_size" => 100, "page_count" => 1, "final_has_next_page" => false, "retrieved_at" => "now", "query" => query, "query_sha256" => Digest::SHA256.hexdigest(query), "response_path" => response_path, "response_sha256" => Digest::SHA256.file(response_path).hexdigest},
      "pagination_complete" => true, "next_cursor" => nil, "query_limit" => nil, "issues" => [issue]}
    write_json(File.join(root, "live-milestone-snapshot.json"), snapshot)
    repo_rows = changed.map.with_index { |path, i| {"path" => path, "denominator_ref" => "repo:#{i}", "classification" => "code", "disposition" => "review", "review_lane" => "code", "evidence" => blob_evidence.call(path)} }
    write_json(File.join(root, "repo_inventory.json"), {"rows" => repo_rows})
    canonical = %w[documentation demo provider_cloud retained_evidence].map { |kind| {"kind" => kind, "path" => "#{kind}.txt", "denominator_ref" => "canonical:#{kind}", "evidence" => blob_evidence.call("#{kind}.txt")} }
    write_json(File.join(root, "canonical-surface-inventory.json"), {"rows" => canonical})
    issue_row = issue.merge("planned_id" => "WP-01", "issue" => 480, "denominator_ref" => "issue:480", "retrieved_at" => "now", "disposition" => "review", "evidence" => blob_evidence.call(spec_path))
    write_json(File.join(root, "issue_inventory.json"), {"rows" => [issue_row]})
    criterion = "observable result"
    ac = {"planned_id" => "WP-01", "acceptance_id" => "AC-1", "criterion" => criterion, "criterion_sha256" => Digest::SHA256.hexdigest(JSON.generate(criterion)), "denominator_ref" => "ac:1", "implementation_disposition" => "implemented", "proof_disposition" => "proved", "evidence" => blob_evidence.call(spec_path)}
    write_json(File.join(root, "acceptance_coverage.json"), {"rows" => [ac]})
    refs = (repo_rows + canonical + [issue_row, ac]).map { |row| row.fetch("denominator_ref") }
    lanes = %w[code tests documentation security architecture provider_cloud demos retained_evidence]
    assignments = lanes.map.with_index { |lane, i| {"id" => "lane-#{i}", "lane" => lane, "denominator_refs" => []} }
    refs.each_with_index { |ref, i| assignments[i % assignments.length]["denominator_refs"] << ref }
    results = assignments.map do |assignment|
      report_path = File.join(root, "#{assignment.fetch('id')}.json")
      observations = assignment.fetch("denominator_refs").map { |ref| {"ref" => ref, "evidence" => blob_evidence.call(spec_path), "conclusion" => "reviewed"} }
      write_json(report_path, {"candidate_sha" => candidate, "denominator_refs" => assignment.fetch("denominator_refs"), "observations" => observations, "findings" => []})
      {"assignment_id" => assignment.fetch("id"), "lane" => assignment.fetch("lane"), "outcome" => "passed", "candidate_sha" => candidate, "reviewer" => "fixture", "evidence" => "retained", "report_path" => report_path, "report_sha256" => Digest::SHA256.file(report_path).hexdigest, "tests_run" => 1}
    end
    write_json(File.join(root, "assignments.json"), {"assignments" => assignments})
    write_json(File.join(root, "lane-results.json"), {"results" => results})
    write_json(File.join(root, "findings.json"), {"candidate_sha" => candidate, "outcome" => "passed", "findings" => []})
    %w[proof-results.json validation-results.json redaction-report.json quality-report.json].each { |name| write_json(File.join(root, name), {"candidate_sha" => candidate, "outcome" => "passed", "observations" => ["checked"]}) }
    required = %w[run_manifest.json live-milestone-snapshot.json repo_inventory.json canonical-surface-inventory.json issue_inventory.json acceptance_coverage.json assignments.json lane-results.json findings.json proof-results.json validation-results.json redaction-report.json quality-report.json]
    paths = required.map { |name| File.join(root, name) } + results.map { |row| row.fetch("report_path") } + [response_path]
    write_json(File.join(root, "packet-manifest.json"), {"entries" => paths.map { |path| {"path" => path, "sha256" => Digest::SHA256.file(path).hexdigest} }})

    bin = File.join(repo, "bin"); FileUtils.mkdir_p(bin)
    File.write(File.join(bin, "gh"), "#!/bin/sh\nprintf '%s\\n' '[{\"number\":480,\"title\":\"WP-01\",\"state\":\"CLOSED\",\"closedByPullRequestsReferences\":[{\"number\":527}]}]'\n")
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
    reject_mutation.call("non_resolving_evidence", [issue_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(issue_path)); doc.fetch("rows").first["evidence"] = "looks convincing"; write_json(issue_path, doc)
    end
    reject_mutation.call("duplicate_planned_mapping", [issue_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(issue_path)); doc.fetch("rows") << doc.fetch("rows").first.dup; write_json(issue_path, doc)
    end
    snapshot_path = File.join(root, "live-milestone-snapshot.json")
    reject_mutation.call("omitted_live_pr", [snapshot_path, response_path, File.join(root, "packet-manifest.json")]) do
      snap = JSON.parse(File.read(snapshot_path)); snap.fetch("issues").first["pull_requests"] = []; write_json(snapshot_path, snap)
      write_json(response_path, {"issues" => snap.fetch("issues")}); snap.fetch("api_receipt")["response_sha256"] = Digest::SHA256.file(response_path).hexdigest; write_json(snapshot_path, snap)
    end
    reject_mutation.call("incomplete_pagination", [snapshot_path, File.join(root, "packet-manifest.json")]) do
      snap = JSON.parse(File.read(snapshot_path)); snap["pagination_complete"] = false; snap.fetch("api_receipt")["final_has_next_page"] = true; write_json(snapshot_path, snap)
    end
    manifest_path = File.join(root, "run_manifest.json")
    reject_mutation.call("self_authored_mutable_base", [manifest_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(manifest_path)); doc["base_sha"] = candidate; doc.fetch("opening_authority")["base_sha"] = candidate; write_json(manifest_path, doc)
    end
    acceptance_path = File.join(root, "acceptance_coverage.json")
    assignments_path = File.join(root, "assignments.json")
    reject_mutation.call("jointly_omitted_acceptance", [acceptance_path, assignments_path, File.join(root, "packet-manifest.json")]) do
      write_json(acceptance_path, {"rows" => []})
      doc = JSON.parse(File.read(assignments_path)); doc.fetch("assignments").each { |row| row.fetch("denominator_refs").delete("ac:1") }; write_json(assignments_path, doc)
    end
    report_path = results.first.fetch("report_path")
    reject_mutation.call("content_free_lane", [report_path, File.join(root, "packet-manifest.json")]) do
      doc = JSON.parse(File.read(report_path)); doc["observations"] = []; write_json(report_path, doc)
    end
  end
end
