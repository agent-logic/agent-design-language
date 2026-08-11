#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "yaml"

ROOT = File.expand_path("../../../..", __dir__)
ISSUE = 149
UMBRELLA = "CORP-U"
LANE = "corporate"
EXPECTED_CHILDREN = {"CORP-01" => 153, "CORP-02" => 154, "CORP-03" => 155, "CORP-04" => 156, "CORP-05" => 157, "CORP-06" => 158, "CORP-07" => 159, "CORP-08" => 160}.freeze
DEFERRED_CHILDREN = [].freeze
EXTERNAL_GATES = {}.freeze
LEDGER_PATH = File.join(ROOT, "docs/milestones/v0.92.1/evidence/umbrellas/corp-u/child-ledger.json")
ALLOWED_PREFIXES = [
  "docs/milestones/v0.92.1/evidence/umbrellas/corp-u/",
  ".csdlc/issues/149/",
  ".csdlc/prepared/issues/149/",
  ".csdlc/evidence/149/"
].freeze

def fail!(message)
  abort("FAIL: #{message}")
end

def git!(*args)
  output, status = Open3.capture2e("git", "-C", ROOT, *args)
  fail!("git #{args.join(' ')}: #{output.strip}") unless status.success?
  output.strip
end

def safe_artifact!(artifact)
  path = artifact.fetch("path")
  fail!("unsafe artifact path #{path.inspect}") if Pathname.new(path).absolute? || path.split("/").include?("..")
  absolute = File.join(ROOT, path)
  fail!("missing artifact #{path}") unless File.file?(absolute)
  actual = Digest::SHA256.file(absolute).hexdigest
  fail!("artifact digest mismatch #{path}") unless actual == artifact.fetch("sha256")
end

wave = YAML.safe_load_file(File.join(ROOT, "docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml"))
declared = wave.fetch("work_packages").select { |entry| entry.fetch("lane") == LANE }
declared_map = declared.to_h { |entry| [entry.fetch("id"), entry.fetch("issue")] }
fail!("wave child denominator mismatch") unless declared_map == EXPECTED_CHILDREN

ledger = JSON.parse(File.read(LEDGER_PATH))
fail!("wrong ledger schema") unless ledger["schema"] == "adl.umbrella.child_ledger.v1"
fail!("wrong umbrella identity") unless ledger["umbrella"] == UMBRELLA && ledger["issue"] == ISSUE
head = git!("rev-parse", "HEAD")
fail!("ledger is not exact-head bound") unless ledger["revision"] == head
base = ledger.fetch("base_revision")
git!("cat-file", "-e", "#{base}^{commit}")
entries = ledger.fetch("children")
fail!("ledger child denominator mismatch") unless entries.map { |entry| [entry["id"], entry["issue"]] }.to_h == EXPECTED_CHILDREN
fail!("duplicate child entries") unless entries.map { |entry| entry["id"] }.uniq.length == EXPECTED_CHILDREN.length

entry_by_id = entries.to_h { |entry| [entry.fetch("id"), entry] }
declared.each do |package|
  id = package.fetch("id")
  entry = entry_by_id.fetch(id)
  if DEFERRED_CHILDREN.include?(id)
    fail!("#{id} must remain deferred") unless entry["status"] == "deferred"
    next
  end

  index_path = File.join(ROOT, ".csdlc/issues/#{entry.fetch('issue')}/index.json")
  fail!("missing child index #{id}") unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  fail!("#{id} is not closed out") unless index["phase"] == "closed_out"
  fail!("#{id} index digest mismatch") unless entry["index_digest"] == index["digest"]
  terminal_sha = index.dig("terminal", "observed_sha")
  fail!("#{id} lacks merged terminal revision") unless index.dig("terminal", "disposition") == "merged" && terminal_sha&.match?(/\A[0-9a-f]{40}\z/)
  fail!("#{id} terminal revision mismatch") unless entry["terminal_revision"] == terminal_sha
  git!("cat-file", "-e", "#{terminal_sha}^{commit}")
  git!("merge-base", "--is-ancestor", terminal_sha, head)

  validate = ENV.fetch("CSDLC_VALIDATE", "csdlc-validate")
  output, status = Open3.capture2e(validate, "--root", ROOT, "issue", "--issue", entry.fetch("issue").to_s)
  fail!("#{id} typed validation failed: #{output.strip}") unless status.success?

  artifacts = entry.fetch("artifacts")
  fail!("#{id} has no producer artifacts") unless artifacts.is_a?(Array) && !artifacts.empty?
  artifacts.each { |artifact| safe_artifact!(artifact) }

  package.fetch("depends_on").each do |dependency|
    if EXTERNAL_GATES.key?(dependency)
      next
    end
    predecessor = entry_by_id.fetch(dependency)
    fail!("#{id} started before #{dependency} terminal") unless predecessor.fetch("terminal_sequence") < entry.fetch("start_sequence")
  end
end

gates = ledger.fetch("external_gates", {})
fail!("external gate denominator mismatch") unless gates.keys.sort == EXTERNAL_GATES.keys.sort
EXTERNAL_GATES.each do |id, policy|
  gate = gates.fetch(id)
  fail!("#{id} issue mismatch") unless gate["issue"] == policy.fetch("issue")
  fail!("#{id} is not terminal") unless gate["state"] == "terminal"
  revision = gate.fetch("terminal_revision")
  fail!("#{id} revision is invalid") unless revision.match?(/\A[0-9a-f]{40}\z/)
  git!("cat-file", "-e", "#{revision}^{commit}")
  git!("merge-base", "--is-ancestor", revision, head)
  safe_artifact!(gate.fetch("receipt"))
  first = entry_by_id.fetch(policy.fetch("before"))
  fail!("#{policy.fetch('before')} crossed #{id} early") unless gate.fetch("terminal_sequence") < first.fetch("start_sequence")
end

actual_paths = git!("diff", "--name-only", "#{base}...#{head}").lines.map(&:strip).reject(&:empty?)
declared_paths = ledger.fetch("umbrella_changed_paths").sort
fail!("changed-path receipt mismatch") unless actual_paths.sort == declared_paths
outside = actual_paths.reject { |path| ALLOWED_PREFIXES.any? { |prefix| path.start_with?(prefix) } }
fail!("umbrella changed child or product paths: #{outside.join(', ')}") unless outside.empty?

puts "PASS: #{UMBRELLA} recomputed #{EXPECTED_CHILDREN.length} child records at #{head}"
