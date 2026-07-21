#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = [5344, 5343, 5358, 5361].freeze
MANIFESTS = {
  5346 => ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json"),
  5347 => ROOT.join("docs/milestones/v0.91.8/evidence/wp13/5347-deletion-eligibility.v1.json")
}.freeze
HEX40 = /\A[0-9a-f]{40}\z/
HEX64 = /\A[0-9a-f]{64}\z/

def fail_gate(message)
  warn("#5346 dependency gate: #{message}")
  exit 1
end

def capture_git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

def relative_path(value, label)
  fail_gate("#{label} must be a non-empty repository-relative path") unless value.is_a?(String) && !value.empty?
  path = Pathname.new(value)
  fail_gate("#{label} must be repository-relative") if path.absolute?
  clean = path.cleanpath.to_s
  fail_gate("#{label} contains traversal or is not normalized: #{value}") if clean == "." || clean.start_with?("../") || clean != value
  clean
end

def load_json(path, label)
  fail_gate("missing #{label}: #{path.relative_path_from(ROOT)}") unless path.file?
  JSON.parse(path.read)
rescue JSON::ParserError => e
  fail_gate("invalid #{label}: #{e.message}")
end

common_dir = Pathname.new(capture_git("rev-parse", "--git-common-dir"))
common_dir = ROOT.join(common_dir) unless common_dir.absolute?
head = capture_git("rev-parse", "HEAD")

receipts = {}
DEPENDENCIES.each do |issue|
  path = common_dir.join("csdlc-v2/closeout/#{issue}.json")
  receipt = load_json(path, "retained terminal receipt for ##{issue}")
  record = receipt.fetch("record") { fail_gate("##{issue} receipt has no typed record") }
  fail_gate("##{issue} is not typed closed_out") unless record["phase"] == "closed_out"
  fail_gate("##{issue} still has an active claim") unless record["claim"].nil?
  terminal = record.fetch("terminal") { fail_gate("##{issue} receipt has no terminal evidence") }
  fail_gate("##{issue} is not merged") unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
  sha = terminal["observed_sha"]
  fail_gate("##{issue} receipt has invalid merged SHA") unless sha&.match?(HEX40)
  _out, status = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", sha, head)
  fail_gate("##{issue} merged SHA is not ancestral to #{head}") unless status.success?
  receipts[issue.to_s] = { "sha" => sha, "sha256" => Digest::SHA256.file(path).hexdigest }
end

def validate_manifest(issue, path, head, receipts)
  manifest = load_json(path, "##{issue} deletion manifest")
  fail_gate("##{issue} manifest schema mismatch") unless manifest["schema"] == "adl.wp13.deletion_eligibility.v1"
  fail_gate("##{issue} manifest issue mismatch") unless manifest["issue"] == issue
  %w[baseline_revision execution_revision reviewed_revision].each do |field|
    fail_gate("##{issue} #{field} is invalid") unless manifest[field]&.match?(HEX40)
  end
  fail_gate("##{issue} execution revision is not current HEAD") unless manifest["execution_revision"] == head
  review = manifest.fetch("review") { fail_gate("##{issue} manifest has no review") }
  fail_gate("##{issue} manifest review is not a pass") unless review["result"] == "pass" && !review["reviewer"].to_s.empty?
  fail_gate("##{issue} review revision mismatch") unless review["reviewed_revision"] == manifest["reviewed_revision"] && manifest["reviewed_revision"] == head
  rollback = manifest.fetch("rollback") { fail_gate("##{issue} manifest has no rollback evidence") }
  rollback_refs = Array(rollback["evidence_refs"])
  fail_gate("##{issue} rollback window is not complete") unless rollback["window_complete"] == true && !rollback_refs.empty? && rollback_refs.all? { |ref| !ref.to_s.empty? }
  if issue == 5346
    accepted = manifest.fetch("acceptance_receipts") { fail_gate("#5346 manifest has no acceptance receipt bindings") }
    %w[5358 5361].each do |dependency|
      fail_gate("#5346 acceptance receipt #{dependency} does not bind retained truth") unless accepted[dependency] == receipts.fetch(dependency)
    end
  end
  request = relative_path(manifest["eligibility_request"], "##{issue} eligibility request")
  decision = relative_path(manifest["eligibility_decision"], "##{issue} eligibility decision")
  [request, decision].each { |ref| fail_gate("##{issue} missing eligibility artifact #{ref}") unless ROOT.join(ref).file? }
  decision_json = load_json(ROOT.join(decision), "##{issue} eligibility decision")
  fail_gate("##{issue} is not eligible") unless decision_json["eligible"] == true && decision_json["deletion_executed"] == false
  fail_gate("##{issue} eligibility decision revision mismatch") unless decision_json["code_revision"] == head

  rows = manifest.fetch("paths") { fail_gate("##{issue} manifest has no paths") }
  fail_gate("##{issue} manifest paths must be a non-empty array") unless rows.is_a?(Array) && !rows.empty?
  seen = {}
  rows.map do |row|
    path_value = relative_path(row["path"], "##{issue} path")
    fail_gate("##{issue} duplicate path #{path_value}") if seen[path_value]
    seen[path_value] = true
    fail_gate("#5346 Runtime v2 is categorically outside scope: #{path_value}") if issue == 5346 && path_value.split("/").include?("runtime_v2")
    fail_gate("##{issue} invalid Git mode for #{path_value}") unless row["git_mode"].to_s.match?(/\A(?:100644|100755|120000|160000)\z/)
    fail_gate("##{issue} invalid Git object id for #{path_value}") unless row["git_object_id"]&.match?(HEX40)
    tree = capture_git("ls-tree", manifest["baseline_revision"], "--", path_value)
    object_type = row["git_mode"] == "160000" ? "commit" : "blob"
    expected = "#{row['git_mode']} #{object_type} #{row['git_object_id']}\t#{path_value}"
    fail_gate("##{issue} Git identity mismatch for #{path_value}") unless tree == expected
    fail_gate("##{issue} invalid baseline LoC for #{path_value}") unless row["baseline_physical_loc"].is_a?(Integer) && row["baseline_physical_loc"] >= 0
    fail_gate("##{issue} invalid disposition for #{path_value}") unless %w[remove retain].include?(row["disposition"])
    if row["disposition"] == "retain"
      fail_gate("##{issue} retained path lacks owner/justification: #{path_value}") if row["retained_owner"].to_s.empty? || row["retained_justification"].to_s.empty?
    else
      replacement = row.fetch("replacement") { fail_gate("##{issue} removed path lacks replacement proof: #{path_value}") }
      fail_gate("##{issue} removed path lacks replacement owner/path/proof: #{path_value}") if replacement["owner"].to_s.empty? || replacement["path"].to_s.empty? || Array(replacement["proof_refs"]).empty?
    end
    symlink_target = row["symlink_target"]
    relative_path(symlink_target, "##{issue} symlink target") unless symlink_target.nil?
    generated_owner = row["generated_owner"]
    fail_gate("##{issue} generated owner must be explicit or null: #{path_value}") unless generated_owner.nil? || !generated_owner.to_s.empty?
    cargo = Array(row["cargo_memberships"])
    fail_gate("##{issue} Cargo membership must be normalized strings: #{path_value}") unless cargo.all? { |member| relative_path(member, "Cargo membership") == member }
    { "path" => path_value, "symlink_target" => symlink_target, "generated_owner" => generated_owner, "cargo_memberships" => cargo, "retained_owner" => row["retained_owner"] }
  end
rescue KeyError => e
  fail_gate("invalid ##{issue} manifest: #{e.message}")
end

surfaces = MANIFESTS.to_h { |issue, path| [issue, validate_manifest(issue, path, head, receipts)] }
left = surfaces.fetch(5346)
right = surfaces.fetch(5347)

left.product(right).each do |a, b|
  pairs = [
    [a["path"], b["path"]],
    [a["symlink_target"], b["path"]],
    [a["path"], b["symlink_target"]],
    [a["symlink_target"], b["symlink_target"]]
  ]
  overlap = pairs.find { |x, y| x && y && (x == y || x.start_with?("#{y}/") || y.start_with?("#{x}/")) }
  fail_gate("manifest path/symlink overlap: #{overlap.join(' <-> ')}") if overlap
  fail_gate("generated ownership overlap: #{a['generated_owner']}") if a["generated_owner"] && a["generated_owner"] == b["generated_owner"]
  cargo_overlap = a["cargo_memberships"] & b["cargo_memberships"]
  fail_gate("Cargo membership overlap: #{cargo_overlap.join(', ')}") unless cargo_overlap.empty?
  if a["retained_owner"] && a["retained_owner"] == b["retained_owner"]
    fail_gate("retained ownership overlap: #{a['retained_owner']}")
  end
end

puts JSON.generate(status: "pass", issue: 5346, revision: head, dependencies: DEPENDENCIES, disjoint: true)
