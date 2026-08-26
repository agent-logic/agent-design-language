#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "set"
require "time"

ROOT = File.expand_path("../../../..", __dir__)
EVIDENCE = File.join(ROOT, ".csdlc/evidence/317")
UNIVERSE_PATH = File.join(EVIDENCE, "issue-universe.json")
DAG_PATH = File.join(EVIDENCE, "closeout-dag.json")
NEGATIVE_PATH = File.join(EVIDENCE, "negative-cases.json")
OBSERVATION_PATH = File.join(EVIDENCE, "github-observation-envelope.json")
RAW_DIR = File.join(EVIDENCE, "github-raw")
REPOSITORY = "agent-logic/agent-design-language"
EXPECTED_MAPPING = {
  5856 => [307, "SPRINT-6"], 5840 => [308, "WP-20"],
  5786 => [309, "WP-21"], 5841 => [310, "WP-21A"],
  5842 => [311, "WP-22"], 5843 => [312, "WP-23"],
  5846 => [313, "WP-25"], 5847 => [314, "WP-26"],
  5848 => [315, "WP-27"], 5849 => [316, "WP-28"],
  5850 => [317, "WP-28A"], 5851 => [318, "WP-29"],
  5852 => [319, "WP-30"]
}.freeze
SUCCESS = %w[SUCCESS NEUTRAL SKIPPED].freeze

class Invalid < StandardError
  attr_reader :code

  def initialize(code, detail = nil)
    @code = code
    super([code, detail].compact.join(":"))
  end
end

def read_json(path)
  JSON.parse(File.read(path))
rescue Errno::ENOENT
  raise Invalid.new("missing_artifact", path.delete_prefix("#{ROOT}/"))
rescue JSON::ParserError => e
  raise Invalid.new("invalid_json", "#{path}:#{e.message}")
end

def canonical_json(value)
  JSON.generate(value, sort_keys: true)
end

def sha(value)
  Digest::SHA256.hexdigest(value)
end

def capture_json(*argv)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT)
  raise Invalid.new("observation_failed", "#{argv.join(' ')}:#{stderr.strip}") unless status.success?

  [JSON.parse(stdout), stdout]
end

def observe!
  Dir.mkdir(RAW_DIR) unless Dir.exist?(RAW_DIR)
  observations = EXPECTED_MAPPING.map do |legacy, (canonical, wp)|
    issue, issue_raw = capture_json(
      "gh", "issue", "view", canonical.to_s, "--repo", REPOSITORY,
      "--json", "number,state,title,url,body,closedAt,closedByPullRequestsReferences,labels"
    )
    issue_path = File.join(RAW_DIR, "issue-#{canonical}.json")
    File.write(issue_path, issue_raw)
    prs = Array(issue["closedByPullRequestsReferences"]).map do |ref|
      pr, pr_raw = capture_json(
        "gh", "pr", "view", ref.fetch("number").to_s, "--repo", REPOSITORY,
        "--json", "number,state,url,baseRefName,headRefOid,mergeCommit,mergedAt,statusCheckRollup,reviews"
      )
      pr_path = File.join(RAW_DIR, "pr-#{pr.fetch('number')}.json")
      File.write(pr_path, pr_raw)
      {
        "path" => pr_path.delete_prefix("#{ROOT}/"),
        "response_sha256" => sha(pr_raw),
        "pr" => pr.fetch("number")
      }
    end
    {
      "wp" => wp,
      "legacy_issue" => legacy,
      "canonical_issue" => canonical,
      "issue_path" => issue_path.delete_prefix("#{ROOT}/"),
      "issue_response_sha256" => sha(issue_raw),
      "closing_pull_requests" => prs
    }
  end
  envelope = {
    "schema" => "adl.v092.github-observation-envelope.v1",
    "repository" => REPOSITORY,
    "observed_at" => Time.now.utc.iso8601,
    "collector" => "gh read-only issue/pr views",
    "nondeterministic" => true,
    "observations" => observations
  }
  File.write(OBSERVATION_PATH, JSON.pretty_generate(envelope) + "\n")
  relative_path = OBSERVATION_PATH.delete_prefix("#{ROOT}/")
  puts "PASS observe issues=#{observations.length} path=#{relative_path}"
end

def validate_mapping!(rows)
  raise Invalid.new("wrong_repository") unless rows.all? { |row| row["repository"] == REPOSITORY }
  raise Invalid.new("wrong_row_count", rows.length) unless rows.length == EXPECTED_MAPPING.length
  canonical = rows.map { |row| row["canonical_issue"] }
  legacy = rows.map { |row| row["legacy_issue"] }
  raise Invalid.new("duplicate_canonical_issue") unless canonical.uniq.length == canonical.length
  raise Invalid.new("duplicate_legacy_issue") unless legacy.uniq.length == legacy.length
  raise Invalid.new("mapping_gap") unless legacy.sort == EXPECTED_MAPPING.keys.sort
  EXPECTED_MAPPING.each do |old, (current, wp)|
    row = rows.find { |candidate| candidate["legacy_issue"] == old }
    raise Invalid.new("mapping_gap", old) unless row
    raise Invalid.new("ambiguous_mapping", old) unless row["canonical_issue"] == current && row["wp"] == wp
  end
end

def validate_planning_authority!(rows)
  wave = File.read(File.join(ROOT, "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"))
  sprint = File.read(File.join(ROOT, ".csdlc/prepared/issues/5856/sprint-execution-packet.yaml"))
  rows.each do |row|
    legacy = row.fetch("legacy_issue")
    wp = row.fetch("wp")
    unless wave.match?(/(?:issue|legacy_issue):\s*#{legacy}\b/) &&
           (wp == "SPRINT-6" || wave.include?("wp: #{wp}"))
      raise Invalid.new("planning_authority_gap", "#{legacy}:#{wp}")
    end
    raise Invalid.new("sprint_authority_gap", legacy) unless sprint.match?(/\b#{legacy}\b/)
  end
end

def validate_observation!(universe, envelope)
  raise Invalid.new("self_attested_evidence") unless envelope["schema"] == "adl.v092.github-observation-envelope.v1"
  raise Invalid.new("observation_repository_mismatch") unless envelope["repository"] == REPOSITORY
  Time.iso8601(envelope.fetch("observed_at"))
  observations = envelope.fetch("observations")
  raise Invalid.new("observation_denominator_mismatch") unless observations.length == EXPECTED_MAPPING.length
  expected_digest = sha(File.binread(OBSERVATION_PATH))
  raise Invalid.new("unbound_observation") unless universe["observation_envelope_sha256"] == expected_digest
  universe.fetch("issues").each do |row|
    observed = observations.find { |item| item["canonical_issue"] == row["canonical_issue"] }
    raise Invalid.new("observation_mapping_gap", row["canonical_issue"]) unless observed
    issue_path = File.join(ROOT, observed.fetch("issue_path"))
    issue_raw = File.binread(issue_path)
    raise Invalid.new("source_digest_mismatch", row["canonical_issue"]) unless sha(issue_raw) == observed.fetch("issue_response_sha256")
    issue = JSON.parse(issue_raw)
    raise Invalid.new("stale_issue_state", row["canonical_issue"]) unless issue["state"] == row["github_state"]
    raise Invalid.new("issue_identity_mismatch", row["canonical_issue"]) unless issue["number"] == row["canonical_issue"]
    legacy_url = "danielbaustin/agent-design-language/issues/#{row['legacy_issue']}"
    sprint_identity = row["wp"] == "SPRINT-6" && issue.fetch("title").include?("Quality and release tail")
    wp_identity = issue.fetch("title").include?("[#{row['wp']}]")
    unless issue.fetch("body").include?(legacy_url) || sprint_identity || wp_identity
      raise Invalid.new("canonical_legacy_provenance_gap", row["canonical_issue"])
    end
    merge = row["merge"]
    next unless merge

    observed_pr = observed.fetch("closing_pull_requests").find do |entry|
      entry["pr"] == merge["pr"]
    end
    raise Invalid.new("unbound_merge_identity", row["canonical_issue"]) unless observed_pr
    pr_raw = File.binread(File.join(ROOT, observed_pr.fetch("path")))
    raise Invalid.new("source_digest_mismatch", "pr-#{merge['pr']}") unless sha(pr_raw) == observed_pr.fetch("response_sha256")
    pr = JSON.parse(pr_raw)
    unless pr["baseRefName"] == merge["base"] && pr["headRefOid"] == merge["publication_head"] &&
           pr.dig("mergeCommit", "oid") == merge["merge_commit"] && pr["state"] == "MERGED"
      raise Invalid.new("unbound_merge_identity", row["canonical_issue"])
    end
    conclusions = pr.fetch("statusCheckRollup").map { |check| check["conclusion"] }.compact
    raise Invalid.new("red_checks", row["canonical_issue"]) if conclusions.empty? || (conclusions - SUCCESS).any?
    if merge["review_status"] == "independent_reviewed_revision_ancestor"
      review_path = File.join(ROOT, merge.fetch("review_evidence"))
      raise Invalid.new("absent_review", row["canonical_issue"]) unless File.file?(review_path)
      review_values = read_json(review_path.sub(/\.md\z/, ".values.json"))
      review = review_values.dig("content", "values") || {}
      reviewed = merge.fetch("reviewed_revision")
      unless review["review_result"] == "pass" && review["review_revision"].to_s.include?(reviewed) && !review["reviewer"].to_s.empty?
        raise Invalid.new("absent_review", row["canonical_issue"])
      end
    end
  end
rescue KeyError, ArgumentError => e
  raise Invalid.new("malformed_observation", e.message)
end

def git_ancestor?(ancestor, descendant)
  system("git", "merge-base", "--is-ancestor", ancestor, descendant, chdir: ROOT,
         out: File::NULL, err: File::NULL)
end

def validate_universe!(universe, envelope)
  raise Invalid.new("wrong_universe_schema") unless universe["schema"] == "adl.v092.terminal-issue-universe.v1"
  raise Invalid.new("self_attested_evidence") unless universe["authority"] == "canonical tracked planning plus retained read-only observation"
  rows = universe.fetch("issues")
  validate_mapping!(rows)
  validate_planning_authority!(rows)
  ledger = universe.fetch("mutation_ledger")
  raise Invalid.new("duplicate_mutation") unless ledger.uniq.length == ledger.length
  allowed = %w[review_complete remediation_in_progress reviewed_green_merged active_planning queued ceremony_queued]
  rows.each do |row|
    raise Invalid.new("unknown_classification", row["canonical_issue"]) unless allowed.include?(row["classification"])
    raise Invalid.new("unowned_action", row["canonical_issue"]) if row["owner"].to_s.empty? || row["next_action"].to_s.empty?
    lifecycle = row.fetch("lifecycle")
    receipt = lifecycle.fetch("terminal_receipt")
    topology = lifecycle.fetch("topology")
    if row["classification"] == "reviewed_green_merged" && receipt["state"] == "missing_async"
      raise Invalid.new("missing_receipt") unless lifecycle["cleanup"] == "async_pending"
    end
    if lifecycle["cleanup"] == "complete"
      raise Invalid.new("active_worktree", row["canonical_issue"]) unless topology["worktree"] == "cleaned"
      raise Invalid.new("dirty_worktree", row["canonical_issue"]) unless topology["cleanliness"] == "not_applicable"
    end
    if %w[derived_terminal recordless_closeout].include?(receipt["state"])
      common = File.expand_path(`git rev-parse --git-common-dir`.strip, ROOT)
      receipt_path = File.join(common, receipt.fetch("path").sub(%r{\A\.git/}, ""))
      raise Invalid.new("missing_receipt", row["canonical_issue"]) unless File.file?(receipt_path)
    end
    merge = row["merge"]
    next unless merge

    %w[pr base publication_head merge_commit review_status checks_status ancestry].each do |field|
      raise Invalid.new("partial_merge_identity", "#{row['canonical_issue']}:#{field}") if merge[field].nil?
    end
    raise Invalid.new("stale_head", row["canonical_issue"]) unless merge["publication_head"].match?(/\A[0-9a-f]{40}\z/)
    raise Invalid.new("red_checks", row["canonical_issue"]) unless merge["checks_status"] == "green"
    if merge["review_status"] == "independent_reviewed_revision_ancestor"
      reviewed = merge.fetch("reviewed_revision")
      raise Invalid.new("absent_review", row["canonical_issue"]) unless reviewed.match?(/\A[0-9a-f]{40}\z/)
      raise Invalid.new("review_not_ancestor", row["canonical_issue"]) unless git_ancestor?(reviewed, merge["publication_head"])
    elsif merge["review_status"] != "recordless_closeout_no_tracked_review"
      raise Invalid.new("absent_review", row["canonical_issue"])
    end
    raise Invalid.new("non_ancestral_merge", row["canonical_issue"]) unless merge["ancestry"] == "ancestral_to_main" && git_ancestor?(merge["merge_commit"], "main")
  end
  validate_observation!(universe, envelope)
  true
rescue KeyError => e
  raise Invalid.new("malformed_universe", e.message)
end

def validate_dag!(dag, universe)
  raise Invalid.new("wrong_dag_schema") unless dag["schema"] == "adl.v092.closeout-dag.v1"
  nodes = dag.fetch("nodes")
  edges = dag.fetch("edges")
  ids = nodes.map { |node| node["id"] }
  raise Invalid.new("duplicate_node") unless ids.uniq.length == ids.length
  issue_ids = universe.fetch("issues").map { |row| "issue-#{row['canonical_issue']}" }
  raise Invalid.new("missing_issue_node") unless (issue_ids - ids).empty?
  nodes.each { |node| raise Invalid.new("unowned_action", node["id"]) if node["owner"].to_s.empty? }
  edges.each do |edge|
    raise Invalid.new("unknown_node", edge.inspect) unless ids.include?(edge["from"]) && ids.include?(edge["to"])
    if edge["gate"] && edge["kind"] != "reviewed_green_merge_ancestry"
      raise Invalid.new("closeout_as_gate", edge.inspect)
    end
    if edge["kind"].match?(/finish|cleanup|reconciliation|bookkeeping/) && edge["gate"]
      raise Invalid.new("closeout_as_gate", edge.inspect)
    end
  end
  graph = Hash.new { |hash, key| hash[key] = [] }
  edges.each { |edge| graph[edge["from"]] << edge["to"] }
  visiting = Set.new
  visited = Set.new
  visit = lambda do |node|
    raise Invalid.new("dependency_cycle", node) if visiting.include?(node)
    return if visited.include?(node)

    visiting << node
    graph[node].each { |child| visit.call(child) }
    visiting.delete(node)
    visited << node
  end
  ids.each { |id| visit.call(id) }
  true
rescue KeyError => e
  raise Invalid.new("malformed_dag", e.message)
end

def mutate(universe, dag, mutation)
  u = Marshal.load(Marshal.dump(universe))
  d = Marshal.load(Marshal.dump(dag))
  case mutation
  when "duplicate-row" then u["issues"] << Marshal.load(Marshal.dump(u["issues"].first))
  when "mapping-gap" then u["issues"].delete_at(0)
  when "ambiguous-mapping" then u["issues"].first["canonical_issue"] = 315
  when "self-attestation" then u["authority"] = "author declaration"
  when "stale-head" then u["issues"].find { |row| row["merge"] }["merge"]["publication_head"] = "stale"
  when "red-checks" then u["issues"].find { |row| row["merge"] }["merge"]["checks_status"] = "failure"
  when "absent-review" then u["issues"].find { |row| row["merge"] }["merge"]["review_status"] = "missing"
  when "non-ancestral-merge" then u["issues"].find { |row| row["merge"] }["merge"]["ancestry"] = "not_ancestral"
  when "review-not-ancestor" then u["issues"].find { |row| row.dig("merge", "reviewed_revision") }["merge"]["reviewed_revision"] = "0000000000000000000000000000000000000000"
  when "missing-receipt" then u["issues"].find { |row| %w[derived_terminal recordless_closeout].include?(row.dig("lifecycle", "terminal_receipt", "state")) }["lifecycle"]["terminal_receipt"] = { "state" => "missing_async", "path" => nil }
  when "active-worktree" then u["issues"].find { |row| row.dig("lifecycle", "cleanup") == "complete" }["lifecycle"]["topology"]["worktree"] = "active"
  when "dirty-worktree" then u["issues"].find { |row| row.dig("lifecycle", "cleanup") == "complete" }["lifecycle"]["topology"]["cleanliness"] = "dirty"
  when "partial-release-identity" then u["issues"].find { |row| row["merge"] }["merge"]["merge_commit"] = nil
  when "duplicate-mutation" then u["mutation_ledger"] = %w[retry-1 retry-1]
  when "arbitrary-envelope-digest" then u["observation_envelope_sha256"] = "0" * 64
  when "unknown-node" then d["edges"].first["to"] = "issue-999999"
  when "cycle" then d["edges"] << { "from" => "issue-319", "to" => "issue-317", "kind" => "reviewed_green_merge_ancestry", "gate" => true }
  when "unowned-action" then d["nodes"].first["owner"] = ""
  when "closeout-as-gate" then d["edges"] << { "from" => "finish-317", "to" => "issue-318", "kind" => "typed_finish", "gate" => true }
  else raise Invalid.new("unknown_negative_fixture", mutation)
  end
  [u, d]
end

def validate_negatives!(universe, dag, envelope, negative)
  raise Invalid.new("wrong_negative_schema") unless negative["schema"] == "adl.v092.closeout-negative-cases.v1"
  cases = negative.fetch("cases")
  raise Invalid.new("duplicate_negative_case") unless cases.map { |item| item["id"] }.uniq.length == cases.length
  cases.each do |item|
    mutated_universe, mutated_dag = mutate(universe, dag, item.fetch("mutation"))
    begin
      validate_universe!(mutated_universe, envelope)
      validate_dag!(mutated_dag, mutated_universe)
      raise Invalid.new("negative_case_accepted", item["id"])
    rescue Invalid => e
      raise if e.code == "negative_case_accepted"
      raise Invalid.new("wrong_negative_result", "#{item['id']}:#{e.code}") unless e.code == item["expected_blocker"]
    end
  end
  true
end

def deterministic!(mode)
  universe = read_json(UNIVERSE_PATH)
  dag = read_json(DAG_PATH)
  envelope = read_json(OBSERVATION_PATH)
  validate_universe!(universe, envelope) if %w[universe all negative].include?(mode)
  validate_dag!(dag, universe) if %w[dag all negative].include?(mode)
  validate_negatives!(universe, dag, envelope, read_json(NEGATIVE_PATH)) if %w[negative all].include?(mode)
  puts "PASS #{mode} issues=#{universe.fetch('issues').length} nodes=#{dag.fetch('nodes').length} edges=#{dag.fetch('edges').length}"
end

begin
  mode = ARGV.fetch(0, "all")
  if mode == "observe"
    observe!
  elsif %w[universe dag negative all].include?(mode)
    deterministic!(mode)
  else
    warn "usage: #{$PROGRAM_NAME} [observe|universe|dag|negative|all]"
    exit 64
  end
rescue Invalid => e
  warn "FAIL #{e.message}"
  exit 1
end
