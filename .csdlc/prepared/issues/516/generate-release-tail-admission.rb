#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
OUT = ROOT.join("docs/milestones/v0.92.1/evidence/integration")
REPO = "agent-logic/agent-design-language"
TAIL = (516..526).to_a.freeze
UMBRELLAS = (529..539).to_a.freeze
OPERATOR_DEFERRED = [84, 251].freeze

def capture(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT.to_s)
  abort("#{argv.join(' ')} failed: #{stderr}") unless status.success?
  stdout
end

def ancestor?(commit, candidate)
  _stdout, _stderr, status = Open3.capture3(
    "git", "merge-base", "--is-ancestor", commit, candidate, chdir: ROOT.to_s
  )
  status.success?
end

candidate = capture("git", "rev-parse", "origin/main").strip
issues = JSON.parse(capture(
  "gh", "issue", "list", "--repo", REPO, "--milestone", "v0.92.1",
  "--state", "all", "--limit", "200", "--json",
  "number,title,state,url,body,closedByPullRequestsReferences"
))

issues.select! do |issue|
  issue["title"].start_with?("[v0.92.1]") &&
    !TAIL.include?(issue["number"]) &&
    !UMBRELLAS.include?(issue["number"])
end
issues.sort_by! { |issue| issue["number"] }

rows = issues.map do |issue|
  number = issue.fetch("number")
  prs = issue.fetch("closedByPullRequestsReferences", [])
  pr = prs.last
  pr_data = if pr
              JSON.parse(capture(
                "gh", "pr", "view", pr.fetch("number").to_s, "--repo", REPO,
                "--json", "number,url,state,mergedAt,headRefOid,mergeCommit"
              ))
            end

  deferred = OPERATOR_DEFERRED.include?(number)
  closed = issue.fetch("state") == "CLOSED"
  merged = pr_data && pr_data["mergedAt"]
  merge_oid = pr_data&.dig("mergeCommit", "oid")
  ancestral = merged && merge_oid && ancestor?(merge_oid, candidate)
  disposition =
    if deferred
      "operator_deferred_backlog"
    elsif closed && ((merged && ancestral) || prs.empty?)
      "satisfied"
    else
      "release_blocker"
    end

  issue_record = ROOT.join(".csdlc/issues/#{number}")
  artifacts = [issue.fetch("url")]
  artifacts << ".csdlc/issues/#{number}" if issue_record.directory?
  artifacts << pr_data["url"] if pr_data

  {
    "issue" => number,
    "title" => issue.fetch("title"),
    "acceptance_authority" => "#{issue.fetch('url')}#issue-body",
    "revision" => pr_data&.fetch("headRefOid", nil),
    "merge_ancestry" => ancestral ? "ancestor_of_candidate:#{merge_oid}" :
      (deferred ? "not_applicable_operator_deferred" : "not_proven"),
    "artifacts" => artifacts,
    "implementation_evidence" => pr_data ? pr_data.fetch("url") : issue.fetch("url"),
    "validation_evidence" => issue_record.directory? ?
      ".csdlc/issues/#{number}/cards/sor.md" : "github_issue_terminal_state",
    "review_evidence" => issue_record.directory? ?
      ".csdlc/issues/#{number}/cards/srp.md" : (pr_data&.fetch("url", nil) || issue.fetch("url")),
    "closeout_evidence" => closed ? "github:issue_closed" : "github:issue_open",
    "disposition" => disposition
  }
end

findings = rows.map do |row|
  next if row["disposition"] == "satisfied"

  if row["disposition"] == "operator_deferred_backlog"
    {
      "id" => "issue-#{row['issue']}-operator-deferred",
      "type" => "scope_ambiguity",
      "severity" => "P2",
      "classification" => "routed_work",
      "summary" => "#{row['title']} is explicitly deferred to backlog and excluded from the v0.92.1 release gate.",
      "evidence" => [row["acceptance_authority"]],
      "uncertainty" => "none; operator disposition is explicit",
      "disposition" => "routed_to_backlog",
      "owner" => "issue ##{row['issue']}"
    }
  else
    {
      "id" => "issue-#{row['issue']}-not-terminal",
      "type" => "closeout_drift",
      "severity" => "P1",
      "classification" => "release_blockers",
      "summary" => "#{row['title']} is not yet closed by a merged PR.",
      "evidence" => row["artifacts"],
      "uncertainty" => "none; live GitHub state was queried",
      "disposition" => "open",
      "owner" => "issue ##{row['issue']}"
    }
  end
end.compact

decision = findings.any? { |finding| %w[P0 P1].include?(finding["severity"]) && finding["disposition"] != "resolved" } ?
  "blocked" : "admitted"
observed = rows.map do |row|
  {
    "issue" => row["issue"],
    "revision" => row["revision"],
    "merge_ancestry" => row["merge_ancestry"],
    "disposition" => row["disposition"]
  }
end
generated_at = Time.now.utc.iso8601

admission = {
  "schema" => "adl.v0921.release_tail_admission.v1",
  "candidate" => candidate,
  "generated_at" => generated_at,
  "denominator_policy" => "All milestone issues titled [v0.92.1], excluding release-tail children and sprint umbrellas; explicit backlog issues remain visible as operator-deferred rows.",
  "denominator" => rows,
  "decision" => decision
}

gap = {
  "schema" => "adl.gap_analysis_report.v1",
  "mode" => "compare_milestone_to_evidence",
  "generated_at" => generated_at,
  "expected_baseline" => rows.map { |row| { "issue" => row["issue"], "acceptance_authority" => row["acceptance_authority"] } },
  "observed_evidence" => observed,
  "findings" => findings,
  "limitations" => [
    "Operational and paid-cloud evidence is consumed from issue, PR, and retained lifecycle records; this admission pass does not repeat live or paid execution.",
    "Issues #84 and #251 are retained in the denominator but follow the operator's explicit backlog disposition."
  ],
  "decision" => decision
}

OUT.mkpath
# Keep machine evidence compact; the Markdown report is the human review surface.
OUT.join("release-tail-admission.json").write(JSON.generate(admission) + "\n")
OUT.join("gap_analysis_report.json").write(JSON.generate(gap) + "\n")

finding_lines = if findings.empty?
                  ["No unresolved findings."]
                else
                  findings.map do |finding|
                    "- **#{finding['severity']} #{finding['id']}** — #{finding['summary']} Owner: #{finding['owner']}. Disposition: #{finding['disposition']}."
                  end
                end
denominator_lines = [
  "| Issue | State | Revision | Merge ancestry | Disposition |",
  "|---:|---|---|---|---|"
] + rows.map do |row|
  state = row["closeout_evidence"].delete_prefix("github:issue_")
  revision = row["revision"] ? row["revision"][0, 12] : "none"
  "| ##{row['issue']} | #{state} | `#{revision}` | #{row['merge_ancestry']} | #{row['disposition']} |"
end
markdown = <<~MD
  # v0.92.1 Release-tail Gap Analysis

  Generated at `#{generated_at}` against `#{admission['candidate']}`.

  ## Findings

  #{finding_lines.join("\n")}

  ## Denominator

  #{rows.length} milestone execution issues were evaluated. Sprint umbrellas and release-tail children are coordination or downstream outputs and are excluded from the admission input denominator. Explicit backlog issues remain visible.

  #{denominator_lines.join("\n")}

  ## Limitations

  - This pass consumes existing issue, PR, lifecycle, and retained proof; it does not repeat paid or disruptive execution.
  - #84 and #251 remain operator-deferred backlog work and do not gate v0.92.1.

  ## Decision

  **#{decision.upcase}**
MD
OUT.join("gap_analysis_report.md").write(markdown)

puts JSON.generate({
  "schema" => "adl.v0921.release_tail_generation.v1",
  "status" => "pass",
  "denominator_count" => rows.length,
  "finding_count" => findings.length,
  "decision" => decision
})
