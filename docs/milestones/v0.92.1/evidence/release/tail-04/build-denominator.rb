#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "time"
require "yaml"

ROOT = File.expand_path("../../../../../..", __dir__)
PACKET = __dir__
REPOSITORY = "agent-logic/agent-design-language"
OPENING_ISSUE = 480
OPENING_PR = 527
REVIEW_GATES = [
  {"issue" => 718, "pull_request" => 809},
  {"issue" => 758, "pull_request" => 805}
].freeze
SPEC_PATH = "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
CREATION_RECEIPT_PATH = "docs/milestones/v0.92.1/evidence/wp-01/final-creation-receipt.json"
HANDOFF_PATH = "docs/milestones/v0.92.1/evidence/release/tail-02/handoff-content.json"

def run!(*argv, chdir: ROOT)
  stdout, stderr, status = Open3.capture3(*argv, chdir: chdir)
  abort("#{argv.join(' ')} failed: #{stderr.strip}") unless status.success?
  stdout
end

def json_command!(*argv, chdir: ROOT)
  JSON.parse(run!(*argv, chdir: chdir))
end

def git_blob(revision, path)
  run!("git", "show", "#{revision}:#{path}")
end

def sha256(content)
  Digest::SHA256.hexdigest(content)
end

def write_json(name, value)
  File.write(File.join(PACKET, name), JSON.pretty_generate(value) + "\n")
end

def candidate_evidence(candidate, path, subject_id, line: 1)
  content = git_blob(candidate, path)
  {
    "subject_id" => subject_id,
    "path" => path,
    "source" => "candidate",
    "revision" => candidate,
    "sha256" => sha256(content),
    "locator" => {"path" => path, "line" => line}
  }
end

def packet_evidence(path, subject_id, command:)
  {
    "subject_id" => subject_id,
    "path" => path,
    "source" => "packet",
    "sha256" => Digest::SHA256.file(path).hexdigest,
    "locator" => {"command" => command}
  }
end

def classify(path)
  return ["test/proof", "tests"] if path.match?(%r{(^|/)(tests?|testdata|fixtures)(/|$)|_test\.|\.test\.|test_|(^|/)\.github/workflows/})
  return ["production_code", "security"] if path.match?(/\.(rs|js|mjs|ts|tsx|py|rb|sh|go|swift)$/) && path.match?(/auth|secret|credential|security|tls|iam|redact|sign|permission/i)
  return ["production_code", "provider_cloud"] if path.match?(/\.(rs|js|mjs|ts|tsx|py|rb|sh|go|swift|tf)$/) && path.match?(/aws|gcp|cloud|provider|ollama|terraform/i)
  return ["production_code", "code"] if path.match?(/\.(rs|js|mjs|ts|tsx|py|rb|sh|go|swift)$/)
  return ["dependency", "dependency"] if File.basename(path).match?(/\A(Cargo\.toml|Cargo\.lock|package(?:-lock)?\.json|pyproject\.toml|requirements.*|.*\.lock\.hcl)\z/)
  return ["architecture", "architecture"] if path.end_with?(".tf", ".mmd") || path.match?(/architecture|diagram/i)
  return ["documentation", "demos"] if path.end_with?(".md") && path.match?(/demo|observatory|podcast/i)
  return ["documentation", "documentation"] if path.end_with?(".md")
  return ["lifecycle/evidence", "retained_evidence"] if path.start_with?(".csdlc/", "docs/milestones/v0.92.1/evidence/")
  ["other", "retained_evidence"]
end

def acceptance_lane(planned_id)
  return "provider_cloud" if planned_id.match?(/\A(?:AWS|GCP|XCL|PROV)/)
  return "demos" if planned_id.match?(/\A(?:OBS|DRT)/)
  return "architecture" if planned_id.match?(/\A(?:V3|DEC|RUST|HOT)/)
  "retained_evidence"
end

candidate = ARGV.fetch(0) { abort("usage: build-denominator.rb <merged-candidate-sha>") }
abort("candidate must be a full SHA") unless candidate.match?(/\A[0-9a-f]{40}\z/)
origin_main = run!("git", "rev-parse", "refs/remotes/origin/main").strip
abort("candidate must equal the exact fetched origin/main revision") unless candidate == origin_main

gate_observations = REVIEW_GATES.map do |gate|
  pr_number = gate.fetch("pull_request")
  issue_number = gate.fetch("issue")
  pr = json_command!("gh", "pr", "view", pr_number.to_s, "--repo", REPOSITORY,
                     "--json", "number,state,headRefOid,mergeCommit,mergedAt,url")
  merge_sha = pr.dig("mergeCommit", "oid")
  abort("PR ##{pr_number} is not merged") unless pr.fetch("state") == "MERGED" && merge_sha&.match?(/\A[0-9a-f]{40}\z/)
  issue = json_command!("gh", "issue", "view", issue_number.to_s, "--repo", REPOSITORY,
                        "--json", "number,state,closedAt,closedByPullRequestsReferences,url")
  closing_prs = issue.fetch("closedByPullRequestsReferences").map { |row| row.fetch("number") }
  abort("issue ##{issue_number} is not closed by PR ##{pr_number}") unless issue.fetch("state") == "CLOSED" && closing_prs.include?(pr_number)
  system("git", "merge-base", "--is-ancestor", merge_sha, candidate, chdir: ROOT) ||
    abort("PR ##{pr_number} merge is not ancestral to the frozen candidate")
  {
    "issue" => issue_number,
    "pull_request" => pr_number,
    "issue_state" => issue.fetch("state"),
    "pr_state" => pr.fetch("state"),
    "head_sha" => pr.fetch("headRefOid"),
    "merge_sha" => merge_sha,
    "merged_at" => pr.fetch("mergedAt"),
    "issue_closed_at" => issue.fetch("closedAt")
  }
end

opening = json_command!("gh", "pr", "view", OPENING_PR.to_s, "--repo", REPOSITORY,
                        "--json", "number,state,mergeCommit,mergedAt,url")
opening_merge = opening.dig("mergeCommit", "oid")
base = run!("git", "show", "-s", "--format=%P", opening_merge).strip
abort("opening merge must have one parent") unless base.match?(/\A[0-9a-f]{40}\z/)

spec_blob = git_blob(candidate, SPEC_PATH)
specs = YAML.safe_load(spec_blob).fetch("issue_specifications")
creation_receipt = JSON.parse(git_blob(candidate, CREATION_RECEIPT_PATH))
handoff = JSON.parse(git_blob(candidate, HANDOFF_PATH))

issue_query = <<~GRAPHQL
  query($owner:String!, $name:String!, $cursor:String) {
    repository(owner:$owner, name:$name) {
      issues(first:100, after:$cursor, filterBy:{milestoneNumber:"1"}) {
        nodes { number title state }
        pageInfo { hasNextPage endCursor }
      }
    }
  }
GRAPHQL

closing_query = <<~GRAPHQL
  query($owner:String!, $name:String!, $issueNumber:Int!, $cursor:String) {
    repository(owner:$owner, name:$name) {
      issue(number:$issueNumber) {
        closedByPullRequestsReferences(first:100, after:$cursor) {
          nodes { number }
          pageInfo { hasNextPage endCursor }
        }
      }
    }
  }
GRAPHQL

pull_request_query = <<~GRAPHQL
  query($owner:String!, $name:String!, $cursor:String) {
    repository(owner:$owner, name:$name) {
      pullRequests(first:100, after:$cursor, orderBy:{field:CREATED_AT,direction:ASC}) {
        nodes { number title state mergedAt url milestone { title } }
        pageInfo { hasNextPage endCursor }
      }
    }
  }
GRAPHQL

issue_pages = []
issues = []
cursor = nil
loop do
  variables = ["-F", "owner=agent-logic", "-F", "name=agent-design-language"]
  variables += ["-F", "cursor=#{cursor}"] if cursor
  response = json_command!("gh", "api", "graphql", "-f", "query=#{issue_query}", *variables)
  page = response.dig("data", "repository", "issues")
  issue_pages << page
  issues.concat(page.fetch("nodes").map do |row|
    {
      "number" => row.fetch("number"),
      "title" => row.fetch("title"),
      "state" => row.fetch("state"),
      "pull_requests" => []
    }
  end)
  break unless page.dig("pageInfo", "hasNextPage")
  cursor = page.dig("pageInfo", "endCursor")
end
issues.sort_by! { |row| row.fetch("number") }

closing_reference_pages = {}
issues.each do |issue|
  issue_cursor = nil
  pages = []
  loop do
    variables = ["-F", "owner=agent-logic", "-F", "name=agent-design-language", "-F", "issueNumber=#{issue.fetch('number')}"]
    variables += ["-F", "cursor=#{issue_cursor}"] if issue_cursor
    response = json_command!("gh", "api", "graphql", "-f", "query=#{closing_query}", *variables)
    page = response.dig("data", "repository", "issue", "closedByPullRequestsReferences")
    pages << page
    issue["pull_requests"].concat(page.fetch("nodes").map { |row| row.fetch("number") })
    break unless page.dig("pageInfo", "hasNextPage")
    issue_cursor = page.dig("pageInfo", "endCursor")
  end
  issue["pull_requests"] = issue.fetch("pull_requests").uniq.sort
  closing_reference_pages[issue.fetch("number").to_s] = pages
end

pull_request_pages = []
pull_requests = []
cursor = nil
loop do
  variables = ["-F", "owner=agent-logic", "-F", "name=agent-design-language"]
  variables += ["-F", "cursor=#{cursor}"] if cursor
  response = json_command!("gh", "api", "graphql", "-f", "query=#{pull_request_query}", *variables)
  page = response.dig("data", "repository", "pullRequests")
  pull_request_pages << page
  pull_requests.concat(page.fetch("nodes").map do |row|
    next unless row.dig("milestone", "title") == "v0.92.1"
    row.slice("number", "title", "state", "mergedAt", "url")
  end.compact)
  break unless page.dig("pageInfo", "hasNextPage")
  cursor = page.dig("pageInfo", "endCursor")
end
pull_requests.sort_by! { |row| row.fetch("number") }

raw_response_path = File.join(PACKET, "live-milestone-response.json")
File.write(raw_response_path, JSON.pretty_generate({"issues" => issues, "pull_requests" => pull_requests,
                                                     "issue_pages" => issue_pages,
                                                     "closing_reference_pages" => closing_reference_pages,
                                                     "pull_request_pages" => pull_request_pages}) + "\n")
retrieved_at = Time.now.utc.iso8601

snapshot = {
  "schema" => "adl.v0921.live_milestone_snapshot.v1",
  "repository" => REPOSITORY,
  "milestone" => "v0.92.1",
  "pagination_complete" => true,
  "next_cursor" => nil,
  "query_limit" => nil,
  "issues" => issues,
  "pull_requests" => pull_requests,
  "api_receipt" => {
    "transport" => "github_graphql",
    "page_size" => 100,
    "page_count" => issue_pages.length + closing_reference_pages.values.sum(&:length) + pull_request_pages.length,
    "issue_page_count" => issue_pages.length,
    "closing_reference_page_count" => closing_reference_pages.values.sum(&:length),
    "pull_request_page_count" => pull_request_pages.length,
    "final_has_next_page" => false,
    "retrieved_at" => retrieved_at,
    "query" => [issue_query, closing_query, pull_request_query].join("\n"),
    "query_sha256" => sha256([issue_query, closing_query, pull_request_query].join("\n")),
    "response_path" => raw_response_path.sub(ROOT + "/", ""),
    "response_sha256" => Digest::SHA256.file(raw_response_path).hexdigest
  }
}
write_json("live-milestone-snapshot.json", snapshot)

changed_paths = run!("git", "diff", "--name-only", "#{base}...#{candidate}").lines.map(&:strip).reject(&:empty?).sort
repo_rows = changed_paths.each_with_index.map do |path, index|
  ref = format("FILE-%05d", index + 1)
  classification, lane = classify(path)
  {
    "denominator_ref" => ref,
    "path" => path,
    "classification" => classification,
    "disposition" => "assigned_for_exact-candidate_review",
    "review_lane" => lane,
    "evidence" => candidate_evidence(candidate, path, ref)
  }
end
write_json("repo_inventory.json", {"schema" => "adl.v0921.review_repo_inventory.v1", "rows" => repo_rows})

canonical_rows = handoff.fetch("documents").each_with_index.map do |document, index|
  path = document.fetch("path")
  kind = if path.match?(/demo|observatory|podcast/i)
           "demo"
         elsif path.match?(/aws|gcp|cloud|provider|terraform|ollama/i)
           "provider_cloud"
         elsif path.start_with?(".csdlc/", "docs/milestones/v0.92.1/evidence/")
           "retained_evidence"
         else
           "documentation"
         end
  ref = format("CANON-%04d", index + 1)
  {
    "denominator_ref" => ref,
    "kind" => kind,
    "path" => path,
    "review_lane" => kind == "demo" ? "demos" : kind,
    "evidence" => candidate_evidence(candidate, path, ref)
  }
end
write_json("canonical-surface-inventory.json", {"schema" => "adl.v0921.canonical_surface_inventory.v1", "rows" => canonical_rows})

mapping = {"WP-01" => OPENING_ISSUE}.merge(creation_receipt.fetch("children").to_h { |row| [row.fetch("planned_id"), row.fetch("issue")] })
reverse_mapping = mapping.invert
issue_rows = issues.map do |row|
  ref = format("ISSUE-%04d", row.fetch("number"))
  state = row.fetch("state")
  labels = json_command!("gh", "issue", "view", row.fetch("number").to_s, "--repo", REPOSITORY, "--json", "labels").fetch("labels").map { |item| item.fetch("name") }
  disposition = if labels.include?("track:backlog")
                  "explicit_backlog"
                elsif state == "CLOSED"
                  row.fetch("pull_requests").empty? ? "closed_without_closing_pr" : "closed_with_closing_pr"
                else
                  "open_milestone_work"
                end
  {
    "denominator_ref" => ref,
    "issue" => row.fetch("number"),
    "planned_id" => reverse_mapping[row.fetch("number")],
    "title" => row.fetch("title"),
    "state" => state,
    "pull_requests" => row.fetch("pull_requests"),
    "retrieved_at" => retrieved_at,
    "disposition" => disposition,
    "review_lane" => "retained_evidence",
    "evidence" => packet_evidence("docs/milestones/v0.92.1/evidence/release/tail-04/live-milestone-response.json", ref,
                                  command: "GitHub milestone GraphQL snapshot page")
  }
end
write_json("issue_inventory.json", {"schema" => "adl.v0921.issue_inventory.v1", "rows" => issue_rows})

pull_request_rows = pull_requests.map do |row|
  ref = format("PR-%04d", row.fetch("number"))
  {
    "denominator_ref" => ref,
    "pull_request" => row.fetch("number"),
    "title" => row.fetch("title"),
    "state" => row.fetch("state"),
    "merged_at" => row.fetch("mergedAt"),
    "url" => row.fetch("url"),
    "retrieved_at" => retrieved_at,
    "disposition" => "assigned_for_exact-candidate_review",
    "review_lane" => "retained_evidence",
    "evidence" => packet_evidence("docs/milestones/v0.92.1/evidence/release/tail-04/live-milestone-response.json", ref,
                                  command: "Fully paginated GitHub milestone pull-request snapshot")
  }
end
write_json("pull_request_inventory.json", {"schema" => "adl.v0921.pull_request_inventory.v1", "rows" => pull_request_rows})

acceptance_rows = specs.flat_map do |spec|
  spec.fetch("acceptance_criteria").each_with_index.map do |criterion, index|
    acceptance_id = "AC-#{index + 1}"
    criterion_id = "#{spec.fetch('id')}:#{acceptance_id}"
    ref = "ACCEPT-#{criterion_id}"
    evidence = candidate_evidence(candidate, SPEC_PATH, ref)
    evidence["criterion_id"] = criterion_id
    {
      "denominator_ref" => ref,
      "planned_id" => spec.fetch("id"),
      "acceptance_id" => acceptance_id,
      "criterion" => criterion,
      "criterion_sha256" => sha256(JSON.generate(criterion)),
      "implementation_disposition" => "requires_specialist_evidence_review",
      "proof_disposition" => "requires_specialist_evidence_review",
      "review_lane" => acceptance_lane(spec.fetch("id")),
      "evidence" => evidence
    }
  end
end
write_json("acceptance_coverage.json", {"schema" => "adl.v0921.acceptance_coverage.v1", "rows" => acceptance_rows})

all_rows = repo_rows + canonical_rows + issue_rows + pull_request_rows + acceptance_rows
assignments = all_rows.group_by { |row| row.fetch("review_lane") }.sort.map do |lane, rows|
  {
    "id" => "LANE-#{lane.upcase.tr('_', '-')}",
    "lane" => lane,
    "candidate_sha" => candidate,
    "denominator_refs" => rows.map { |row| row.fetch("denominator_ref") },
    "status" => "assigned",
    "review_depth" => "deep"
  }
end
write_json("assignments.json", {"schema" => "adl.v0921.review_assignments.v1", "assignments" => assignments})

manifest = {
  "schema" => "adl.v0921.internal_review_run.v1",
  "run_id" => "tail-04-#{candidate[0, 12]}",
  "repository" => REPOSITORY,
  "base_sha" => base,
  "candidate_sha" => candidate,
  "candidate_source" => "origin_main_after_review_gates",
  "candidate_merge_sha" => candidate,
  "opening_authority" => {"issue" => OPENING_ISSUE, "pull_request" => OPENING_PR, "merge_sha" => opening_merge, "base_sha" => base},
  "review_gate_observations" => gate_observations,
  "execution_spec_sha256" => sha256(spec_blob),
  "changed_path_count" => changed_paths.length,
  "canonical_surface_count" => canonical_rows.length,
  "milestone_issue_count" => issue_rows.length,
  "milestone_pull_request_count" => pull_request_rows.length,
  "acceptance_surface_count" => acceptance_rows.length,
  "publication_allowed" => false,
  "skills_used" => ["sprint-review", "repo-packet-builder", "gap-analysis", "repo-review-code", "repo-review-tests", "repo-review-security", "repo-review-docs", "repo-architecture-review", "repo-dependency-review", "repo-review-synthesis", "redaction-and-evidence-auditor", "review-quality-evaluator", "finding-to-issue-planner"]
}
write_json("run_manifest.json", manifest)

puts JSON.generate({"status" => "prepared", "candidate_sha" => candidate, "changed_paths" => changed_paths.length,
                    "canonical_surfaces" => canonical_rows.length, "issues" => issue_rows.length,
                    "acceptance_surfaces" => acceptance_rows.length, "assignments" => assignments.length})
