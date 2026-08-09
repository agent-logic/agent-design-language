#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "yaml"

EXPECTED = (1..16).to_h { |number| [format("WP-04.%02d", number), 5862 + number] }.freeze
EXPECTED_DEPENDENCIES = {
  5863 => [],
  5864 => [5863],
  5865 => [5864],
  5866 => [5865],
  5867 => [5866],
  5868 => [5867],
  5869 => [5867],
  5870 => [5868, 5869],
  5871 => [5865],
  5872 => [5865],
  5873 => [5867, 5870, 5871, 5872],
  5874 => [5864, 5870],
  5875 => [5873, 5874],
  5876 => [5875],
  5877 => [5876],
  5878 => (5863..5877).to_a
}.freeze
SESSION_PROMPT = ".adl/docs/TBD/V092_SPRINT_5862_DISTRIBUTED_GUARDIAN_SESSION_PROMPT.md"
MEDIUM_ESTIMATES = {
  "elapsed_seconds" => 21_600,
  "total_tokens" => 80_000,
  "validation_seconds" => 3_600
}.freeze
SHA = /\A[0-9a-f]{40}\z/
SHA256 = /\A[0-9a-f]{64}\z/
PREFLIGHT = ARGV.delete("--preflight")
TOPOLOGY_REQUEST_INDEX = ARGV.index("--validate-topology")
TOPOLOGY_REQUEST = if TOPOLOGY_REQUEST_INDEX
  path = ARGV.fetch(TOPOLOGY_REQUEST_INDEX + 1) { abort "missing topology request path" }
  ARGV.slice!(TOPOLOGY_REQUEST_INDEX, 2)
  path
end

def git_capture(repo, *argv, allow_failure: false)
  stdout, stderr, status = Open3.capture3("git", "-C", repo, *argv)
  return stdout if status.success?
  return nil if allow_failure
  abort "git #{argv.join(' ')} failed: #{stderr}"
end

def safe_evidence_path!(path, issue, label)
  candidate = path.to_s
  expected_prefix = ".csdlc/evidence/#{issue}/"
  components = candidate.split("/")
  abort "invalid #{label} mapping for ##{issue}" if candidate.empty? || candidate.start_with?("/") || components.any? { |part| part.empty? || part == "." || part == ".." }
  abort "#{label} mapping escapes issue ##{issue}" unless candidate.start_with?(expected_prefix)
  candidate
end

def git_path_exists?(repo, revision, path)
  !git_capture(repo, "cat-file", "-e", "#{revision}:#{path}", allow_failure: true).nil?
end

def git_ancestor?(repo, ancestor, descendant)
  _stdout, _stderr, status = Open3.capture3("git", "-C", repo, "merge-base", "--is-ancestor", ancestor, descendant)
  status.success?
end

def git_paths_unchanged?(repo, from, to, paths)
  _stdout, _stderr, status = Open3.capture3("git", "-C", repo, "diff", "--quiet", from, to, "--", *paths)
  status.success?
end

def git_path_entries(repo, revision, path)
  git_capture(repo, "ls-tree", "-r", revision, "--", path).lines.map do |line|
    metadata, name = line.chomp.split("\t", 2)
    mode, type, oid = metadata.to_s.split(" ", 3)
    abort "unsafe symlink or gitlink evidence at #{revision}:#{name}" unless type == "blob" && %w[100644 100755].include?(mode)
    [name, mode, oid]
  end
end

def no_path_touches?(repo, from_exclusive, to_inclusive, paths)
  git_capture(repo, "log", "--format=%H", "#{from_exclusive}..#{to_inclusive}", "--", *paths).lines.none? { |line| !line.strip.empty? }
end

def validate_child_topology!(repo:, issue:, entry:, product_paths:, candidate:, expected_head: nil, expected_merge: nil)
  head = entry.fetch("head_sha")
  evidence = entry.fetch("evidence_sha")
  merge = entry.fetch("merge_sha")
  proof_path = safe_evidence_path!(entry.fetch("execution_proof_path"), issue, "execution proof")
  evidence_path = safe_evidence_path!(entry.fetch("evidence_path"), issue, "evidence")
  abort "proof mapping is outside evidence mapping for ##{issue}" unless proof_path == evidence_path || proof_path.start_with?("#{evidence_path}/")
  abort "invalid head/evidence/merge/candidate revision for ##{issue}" unless [head, evidence, merge, candidate].all? { |revision| revision.to_s.match?(SHA) }
  abort "exact PR head mapping drift for ##{issue}" if expected_head && head != expected_head
  abort "exact merge mapping drift for ##{issue}" if expected_merge && merge != expected_merge
  abort "missing proof at child head for ##{issue}" unless git_path_exists?(repo, head, proof_path)

  proof_bytes = git_capture(repo, "show", "#{head}:#{proof_path}")
  expected_proof_digest = entry.fetch("execution_proof_sha256")
  abort "invalid proof digest mapping for ##{issue}" unless expected_proof_digest.to_s.match?(SHA256)
  abort "proof digest drift for ##{issue}" unless Digest::SHA256.hexdigest(proof_bytes) == expected_proof_digest
  proof = JSON.parse(proof_bytes)
  source = proof.fetch("source_revision")
  abort "invalid source revision for ##{issue}" unless source.to_s.match?(SHA)
  abort "self-referential or collapsed proof topology for ##{issue}" unless [source, evidence, head].uniq.length == 3
  abort "proof issue mapping drift for ##{issue}" unless proof["issue"] == issue
  abort "proof schema drift for ##{issue}" unless %w[adl.wp04.execution_proof.v2 adl.wp04.execution_proof.v3].include?(proof["schema"])
  abort "proof protected-path mapping drift for ##{issue}" unless Array(proof["protected_paths"]).sort == product_paths.sort

  abort "source is not ancestral to evidence for ##{issue}" unless git_ancestor?(repo, source, evidence)
  abort "evidence is not ancestral to head for ##{issue}" unless git_ancestor?(repo, evidence, head)
  abort "head is not ancestral to merge for ##{issue}" unless git_ancestor?(repo, head, merge)
  abort "merge is not ancestral to candidate for ##{issue}" unless git_ancestor?(repo, merge, candidate)
  product_snapshots = [source, evidence, head, merge, candidate].map do |revision|
    product_paths.flat_map { |path| git_path_entries(repo, revision, path) }
  end
  abort "product object or mode drift after source for ##{issue}" unless product_snapshots.uniq.length == 1
  abort "transient product drift after source for ##{issue}" unless no_path_touches?(repo, source, candidate, product_paths)
  abort "evidence already existed at source for ##{issue}" if git_path_exists?(repo, source, evidence_path)
  abort "evidence mapping missing at introduction for ##{issue}" unless git_path_exists?(repo, evidence, evidence_path)
  evidence_snapshots = [evidence, head, merge, candidate].map { |revision| git_path_entries(repo, revision, evidence_path) }
  abort "evidence object or mode drift after introduction for ##{issue}" unless evidence_snapshots.first.any? && evidence_snapshots.uniq.length == 1
  abort "transient evidence drift after introduction for ##{issue}" unless no_path_touches?(repo, evidence, candidate, [evidence_path])

  introduction_touches = git_capture(repo, "log", "--format=%H", "#{source}..#{evidence}", "--", evidence_path).lines.map(&:strip).reject(&:empty?)
  abort "whole evidence mapping was not introduced once at E for ##{issue}" unless introduction_touches == [evidence]
  introductions = git_capture(repo, "log", "--format=%H", "--diff-filter=A", "#{source}..#{head}", "--", proof_path).lines.map(&:strip).reject(&:empty?)
  abort "execution proof mapping is missing or ambiguous for ##{issue}" unless introductions == [evidence]

  artifacts = Array(proof["source_artifacts"])
  abort "source artifact denominator drift for ##{issue}" unless artifacts.map { |artifact| artifact["path"] }.sort == product_paths.sort
  artifacts.each do |artifact|
    bytes = git_capture(repo, "show", "#{source}:#{artifact.fetch('path')}")
    abort "source artifact digest drift for ##{issue}" unless Digest::SHA256.hexdigest(bytes) == artifact.fetch("sha256")
  end
  commands = Array(proof["commands"])
  abort "proof command denominator missing for ##{issue}" if commands.empty?
  abort "proof command failed or selected zero tests for ##{issue}" unless commands.all? { |command| command["exit_code"] == 0 } && commands.any? { |command| command["selected_tests"].to_i.positive? }
  abort "negative-case proof missing for ##{issue}" if Array(proof["negative_cases"]).empty?
  proof
end

def checked_historical_evidence!(repo, head, issue, path, digest, label)
  safe_path = safe_evidence_path!(path, issue, label)
  abort "invalid #{label} digest for ##{issue}" unless digest.to_s.match?(SHA256)
  bytes = git_capture(repo, "show", "#{head}:#{safe_path}")
  abort "#{label} digest drift for ##{issue}" unless Digest::SHA256.hexdigest(bytes) == digest
end

def validate_full_integrated_proof!(repo:, head:, proof:)
  commands = Array(proof["commands"])
  commands.each do |command|
    checked_historical_evidence!(repo, head, 5878, command.fetch("stdout_path"), command.fetch("stdout_sha256"), "command stdout")
    checked_historical_evidence!(repo, head, 5878, command.fetch("stderr_path"), command.fetch("stderr_sha256"), "command stderr")
  end
  Array(proof["negative_cases"]).each do |negative|
    checked_historical_evidence!(repo, head, 5878, negative.fetch("evidence_path"), negative.fetch("evidence_sha256"), "negative-case evidence")
  end
  artifacts = Array(proof["artifacts"])
  abort "WP-04.16 integrated artifact denominator missing" if artifacts.empty?
  artifacts.each do |artifact|
    checked_historical_evidence!(repo, head, 5878, artifact.fetch("path"), artifact.fetch("sha256"), "integrated artifact")
  end

  receipts = Array(proof["native_receipts"])
  abort "WP-04.16 native platform denominator drift" unless receipts.map { |receipt| receipt["platform"] }.sort == %w[linux macos windows]
  source = proof.fetch("source_revision")
  abort "WP-04.16 native receipt source drift" unless receipts.all? { |receipt| receipt["source_revision"] == source }
  abort "WP-04.16 native run IDs are missing or duplicated" unless receipts.all? { |receipt| !receipt["run_id"].to_s.empty? } && receipts.map { |receipt| receipt["run_id"] }.uniq.length == receipts.length
  abort "WP-04.16 native runner identities are missing or duplicated" unless receipts.all? { |receipt| receipt["runner_identity_sha256"].to_s.match?(SHA256) } && receipts.map { |receipt| receipt["runner_identity_sha256"] }.uniq.length == receipts.length
  receipts.each do |receipt|
    checked_historical_evidence!(repo, head, 5878, receipt.fetch("path"), receipt.fetch("sha256"), "#{receipt.fetch('platform')} native receipt")
  end
end

if TOPOLOGY_REQUEST
  request = JSON.parse(File.read(TOPOLOGY_REQUEST))
  mappings = Array(request.fetch("mappings"))
  identities = mappings.map { |mapping| mapping.fetch("issue") }
  abort "topology mapping is missing or ambiguous" unless !mappings.empty? && identities.uniq.length == identities.length
  mappings.each do |mapping|
    proof = validate_child_topology!(
      repo: request.fetch("repository_root"),
      issue: mapping.fetch("issue"),
      entry: mapping,
      product_paths: mapping.fetch("product_paths"),
      candidate: request.fetch("candidate_sha")
    )
    if request["require_integrated_proof"]
      abort "integrated proof request must map only issue #5878" unless mapping.fetch("issue") == 5878
      validate_full_integrated_proof!(repo: request.fetch("repository_root"), head: mapping.fetch("head_sha"), proof: proof)
    end
  end
  puts "PASS: #{mappings.length} exact v3 source-evidence-head topology mapping(s)"
  exit 0
end

def exact_owned_paths(design, wp)
  section = design[/## Owned Paths\n\n(.*?)\n\n## /m, 1]
  abort "#{wp} missing exact Owned Paths" unless section
  paths = section.scan(/`([^`]+)`/).flatten
  abort "#{wp} has no owned paths" if paths.empty?
  paths
end

def checked_digest(path, expected, label)
  abort "missing #{label}: #{path}" unless File.file?(path)
  abort "invalid #{label} digest" unless expected.to_s.match?(SHA256)
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected
end

wave = YAML.safe_load(File.read("docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"), aliases: true)
canonical_rows = Array(wave["work_packages"]).select { |row| EXPECTED.key?(row["wp"]) }
canonical = canonical_rows.to_h { |row| [row["wp"], row["issue"]] }
abort "canonical WP-04 child denominator drift" unless canonical == EXPECTED

prompt = File.read(SESSION_PROMPT)
canonical_rows.each do |row|
  expected_line = "- ##{row.fetch("issue")} #{row.fetch("wp")}: #{row.fetch("title")}"
  abort "session prompt scope drift for #{row.fetch("wp")}" unless prompt.lines.map(&:chomp).include?(expected_line)
end

all_paths = {}
owned_paths_by_issue = {}
records = {}
estimate_sections = []
EXPECTED.each do |wp, issue|
  index_path = ".csdlc/issues/#{issue}/index.json"
  abort "missing index for #{wp} ##{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  records[issue] = index
  abort "issue mismatch for #{wp}" unless index["issue"] == issue
  design = File.read(".csdlc/prepared/issues/#{issue}/design.md")
  approval_digest = index.dig("design_review", "approved", "revision")
  abort "#{wp} design not approved" unless approval_digest.to_s.match?(SHA256)
  if PREFLIGHT
    abort "#{wp} is not initialized and unbound" unless index["phase"] == "initialized" && index["branch"].nil? && index["worktree"].nil?
  end
  %w[sip stp spp vpp].each do |card|
    values = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json"))
    abort "#{wp} #{card} not ready" unless values["status"] == "ready"
  end
  spp = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/spp.values.json")).dig("content", "values")
  stp = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/stp.values.json")).dig("content", "values")
  vpp = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/vpp.values.json")).dig("content", "values")
  child_dependencies = Array(stp["dependencies"]).flat_map { |dependency| dependency.scan(/#(58(?:6[3-9]|7[0-8]))\b/).flatten.map(&:to_i) }.uniq.sort
  abort "#{wp} dependency DAG drift" unless child_dependencies == EXPECTED_DEPENDENCIES.fetch(issue)
  abort "#{wp} approved design digest is not projected to SPP/VPP" unless spp["design_digest"] == approval_digest && vpp["design_digest"] == approval_digest
  abort "#{wp} SPP estimate is not the typed medium profile" unless spp["execution_estimates"] == MEDIUM_ESTIMATES
  abort "#{wp} VPP estimate is not the typed medium profile" unless vpp["planned_validation_seconds"] == 3_600 && vpp["planned_validation_tokens"] == 25_000
  estimate = design[/## Estimate\n\n(.*?)\n\n## /m, 1]
  abort "#{wp} lacks an issue-specific Estimate section" unless estimate
  normalized_estimate = estimate.gsub(/\s+/, " ")
  abort "#{wp} design estimate does not name the typed medium profile" unless normalized_estimate.include?("typed medium profile")
  abort "#{wp} design/SPP estimate mismatch" unless normalized_estimate.include?("6 elapsed hours, 80,000 reasoning tokens, and 60 minutes")
  abort "#{wp} estimate lacks a bounded scope rationale" unless estimate.match?(/scope|boundary|contract|module|fixture|flow|state machine|handoff|owns/i)
  estimate_sections << estimate
  paths = exact_owned_paths(design, wp)
  owned_paths_by_issue[issue] = paths
  paths.each do |path|
    abort "path collision #{path}: #{all_paths[path]} and #{wp}" if all_paths.key?(path)
    all_paths[path] = wp
  end
end
abort "WP-04 estimate rationales were copied rather than reasoned per child" unless estimate_sections.uniq.length == EXPECTED.length

umbrella = JSON.parse(File.read(".csdlc/issues/5862/index.json"))
gate = File.read(".csdlc/prepared/issues/5821/design.md")
EXPECTED.each { |wp, issue| abort "gate mapping missing #{wp} ##{issue}" unless gate.include?("| #{wp} | ##{issue} |") }

if PREFLIGHT
  abort "umbrella is not initialized and unbound" unless umbrella["phase"] == "initialized" && umbrella["branch"].nil? && umbrella["worktree"].nil?
  puts "PASS: WP-04-IMP preflight, sixteen approved claim-free unbound children, #{all_paths.length} exact owned paths"
  exit 0
end

manifest_path = ".csdlc/evidence/5862/terminal-child-envelopes.json"
abort "missing terminal reconciliation manifest" unless File.file?(manifest_path)
manifest = JSON.parse(File.read(manifest_path))
abort "wrong terminal manifest schema" unless manifest["schema"] == "adl.wp04.terminal_child_envelopes.v1"
entries = Array(manifest["children"])
abort "terminal manifest denominator drift" unless entries.map { |entry| entry["issue"] }.sort == EXPECTED.values
abort "terminal manifest contains ambiguous evidence mappings" unless entries.map { |entry| entry["execution_proof_path"] }.uniq.length == entries.length && entries.map { |entry| entry["evidence_path"] }.uniq.length == entries.length
candidate_head = git_capture(".", "rev-parse", "HEAD").strip

git_common, git_status = Open3.capture2("git", "rev-parse", "--path-format=absolute", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_status.success?
repo_root = File.expand_path("..", git_common.strip)
finish_binary = ENV.fetch("CSDLC_FINISH_BIN", File.join(repo_root, ".adl/bin/csdlc-v2/csdlc-finish"))
pr_binary = ENV.fetch("CSDLC_GITHUB_PR_BIN", File.join(repo_root, ".adl/bin/csdlc-v2/csdlc-github-pr"))
abort "missing typed finish binary" unless File.executable?(finish_binary)
abort "missing typed GitHub PR binary" unless File.executable?(pr_binary)
request_dir = ".csdlc/evidence/5862/pr-state-requests"
FileUtils.mkdir_p(request_dir)

proofs = {}
entries.each do |entry|
  issue = entry.fetch("issue")
  index = records.fetch(issue)
  stdout, stderr, status = Open3.capture3(finish_binary, "--root", ".", "--validate-cached-issue", issue.to_s)
  abort "typed terminal validation failed for ##{issue}: #{stderr} #{stdout}" unless status.success?
  validation = JSON.parse(stdout)
  abort "wrong terminal validation schema for ##{issue}" unless validation["schema"] == "csdlc.derived_terminal_validation.v1" && validation["canonical_match"] == true
  terminal = validation.fetch("terminal")
  abort "issue ##{issue} is not derived terminal merged" unless terminal["disposition"] == "merged" && terminal["issue_state"] == "closed_by_merged_pr" && terminal["source"] == "live_github"
  abort "terminal canonical identity drift for ##{issue}" unless terminal["repository"] == index["repository"] && terminal["initialization_digest"] == index["initialization_digest"] && terminal["canonical_generation"] == index["generation"] && terminal["canonical_digest"] == index["digest"]
  pr = terminal.fetch("pull_request")
  child_head = terminal.fetch("head_sha")
  issue_repository = entry.fetch("issue_repository")
  code_repository = entry.fetch("code_repository")
  abort "manifest issue repository drift for ##{issue}" unless issue_repository == index["repository"]
  abort "terminal issue repository drift for ##{issue}" unless terminal["repository"] == issue_repository
  abort "manifest code repository drift for ##{issue}" unless code_repository == index.fetch("code_repository", issue_repository)
  abort "invalid child head for ##{issue}" unless child_head.match?(SHA)
  abort "manifest PR drift for ##{issue}" unless entry["pull_request"] == pr
  abort "manifest head drift for ##{issue}" unless entry["head_sha"] == child_head
  abort "manifest envelope digest drift for ##{issue}" unless entry["envelope_digest"] == terminal["digest"]

  request_path = File.join(request_dir, "#{issue}.json")
  File.write(request_path, JSON.pretty_generate({repository: code_repository, pull_request: pr, required_checks: [], require_review: false, linked_issue: issue, linked_issue_repository: issue_repository}) + "\n")
  stdout, stderr, status = Open3.capture3(pr_binary, "state", "--request", request_path)
  abort "typed PR read failed for ##{issue}: #{stderr} #{stdout}" unless status.success?
  packet = JSON.parse(stdout)
  abort "PR ##{pr} does not close ##{issue}" unless packet["linked_issue"] == issue
  abort "PR repository drift for ##{issue}" unless packet["repository"] == code_repository
  closing_reference = issue_repository == code_repository ? "Closes ##{issue}" : "Closes #{issue_repository}##{issue}"
  abort "PR ##{pr} lacks exact qualified closure for ##{issue}" unless packet["body"].to_s.lines.map(&:strip).include?(closing_reference)
  abort "PR ##{pr} is not merged" unless packet["state"] == "closed" && packet["merged"] == true
  abort "PR head drift for ##{issue}" unless packet["head_sha"] == child_head
  merge_sha = packet["merge_commit_sha"]
  abort "invalid merge SHA for ##{issue}" unless merge_sha.to_s.match?(SHA)
  abort "manifest merge drift for ##{issue}" unless entry["merge_sha"] == merge_sha
  abort "terminal merge drift for ##{issue}" unless terminal["merge_sha"] == merge_sha
  system("git", "merge-base", "--is-ancestor", merge_sha, "HEAD") or abort "merge for ##{issue} is not ancestral to candidate HEAD"
  proofs[issue] = validate_child_topology!(repo: ".", issue: issue, entry: entry, product_paths: owned_paths_by_issue.fetch(issue), candidate: candidate_head, expected_head: child_head, expected_merge: merge_sha)
end

integrated_entry = entries.find { |entry| entry["issue"] == 5878 }
proof_path = integrated_entry.fetch("execution_proof_path")
abort "WP-04.16 execution proof manifest digest drift" unless integrated_entry.fetch("execution_proof_sha256") == manifest.fetch("wp04_16_execution_proof_sha256")
proof = proofs.fetch(5878)
commands = Array(proof["commands"])
required = [["bash", "adl/tools/validate_v092_distributed_guardian.sh"], ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"]]
required.each do |argv|
  matches = commands.select { |candidate| candidate["argv"] == argv && candidate["exit_code"] == 0 }
  abort "WP-04.16 missing or duplicate #{argv.join(' ')}" unless matches.one?
end
validate_full_integrated_proof!(repo: ".", head: integrated_entry.fetch("head_sha"), proof: proof)
puts "PASS: sixteen live merged child PRs, derived terminal envelopes, exact heads, and WP-04.16 integrated proof authorize WP-14 handoff"
