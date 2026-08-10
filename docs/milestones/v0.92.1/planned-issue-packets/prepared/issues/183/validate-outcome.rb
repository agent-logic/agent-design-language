#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 183
manifest_path = File.join(root, ".csdlc/evidence/183/proof-manifest.json")
abort("missing proof manifest: #{manifest_path}") unless File.file?(manifest_path)

def present?(value)
  !(value.nil? || (value.respond_to?(:empty?) && value.empty?))
end

def sha?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{64}\z/)
end

def git_sha?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{40}\z/)
end

def read_json(path)
  JSON.parse(File.read(path))
rescue JSON::ParserError => error
  abort("invalid JSON #{path}: #{error.message}")
end

def digest_file(path)
  Digest::SHA256.file(path).hexdigest
end

def artifact_path(manifest, name)
  entry = manifest.fetch("artifacts").fetch(name)
  entry.fetch("resolved_path")
end

manifest = read_json(manifest_path)
abort("wrong manifest schema") unless manifest["schema"] == "adl.v0921.proof_manifest.v1"
abort("wrong issue") unless manifest["issue"] == issue
revision = `git -C #{Shellwords.escape(root)} rev-parse HEAD`.strip
abort("invalid exact revision") unless revision.match?(/\A[0-9a-f]{40}\z/)
abort("proof revision does not match HEAD") unless manifest["revision"] == revision

artifacts = manifest.fetch("artifacts")
abort("artifact manifest empty") if artifacts.empty?
artifacts.each do |name, entry|
  relative = entry.fetch("path")
  abort("absolute or escaping artifact path: #{name}") if Pathname.new(relative).absolute? || relative.split("/").include?("..")
  abort("artifact outside issue evidence root: #{name}") unless relative.start_with?(".csdlc/evidence/#{issue}/")
  path = File.join(root, relative)
  abort("missing artifact #{name}: #{relative}") unless File.file?(path)
  abort("artifact digest mismatch: #{name}") unless digest_file(path) == entry.fetch("sha256")
  entry["resolved_path"] = path
end

receipts = manifest.fetch("producer_receipts")
abort("producer receipt denominator empty") if receipts.empty?
receipts.each do |entry|
  relative = entry.fetch("path")
  abort("receipt outside issue evidence root: #{relative}") unless relative.start_with?(".csdlc/evidence/#{issue}/")
  path = File.join(root, relative)
  abort("missing producer receipt: #{relative}") unless File.file?(path)
  abort("producer receipt digest mismatch: #{relative}") unless digest_file(path) == entry.fetch("sha256")
  receipt = read_json(path)
  abort("receipt revision mismatch: #{relative}") unless receipt["revision"] == revision
  abort("receipt command failed: #{relative}") unless receipt["exit_code"] == 0
  abort("receipt missing producer/command: #{relative}") unless present?(receipt["producer"]) && receipt["command"].is_a?(Array) && !receipt["command"].empty?
  abort("receipt uses asserted pass flag: #{relative}") if receipt.key?("passed")
end

payload = read_json(artifact_path(manifest, "wuji"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt03.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

dependency = payload.fetch("dependency_142")
abort("#142 is not terminal merged proof") unless dependency["issue"] == 142 && dependency["state"] == "closed" && dependency["disposition"] == "merged"
merge_sha = dependency.fetch("merge_sha")
abort("invalid #142 merge SHA") unless git_sha?(merge_sha)
system("git", "-C", root, "merge-base", "--is-ancestor", merge_sha, revision, out: File::NULL, err: File::NULL) or abort("#142 merge is not ancestral")
proofs = dependency.fetch("proofs")
abort("#142 retained proof denominator mismatch") unless proofs.keys.sort == %w[api guardian wp04_16 wss]
abort("#142 retained proof digest invalid") unless proofs.values.all? { |value| sha?(value) }
topology = payload.fetch("topology")
actors = topology.fetch("actors")
expected = {"voter" => 3, "governed_agent" => 3, "shepherd" => 1, "observatory" => 1}
abort("production actor denominator mismatch") unless actors.group_by { |a| a.fetch("role") }.transform_values(&:length) == expected
abort("in-process or direct-executor substitute") unless actors.all? { |a| a["process_kind"] == "production" && a["direct_executor_bypass"] == false }
%w[identity credential_ref port state_root].each { |field| vals = actors.map { |a| a.fetch(field) }; abort("shared #{field}") unless vals.uniq.length == vals.length }
phases = payload.fetch("phases").to_h { |p| [p.fetch("id"), p] }
required = %w[three_voter_commit two_voter_commit one_voter_denial lease_expiry successor_binding stale_write_denial snapshot_restore voter_restart agent_continuity replay cleanup]
abort("Wuji phase denominator mismatch") unless phases.keys.sort == required.sort
abort("3-voter commit missing") unless phases.dig("three_voter_commit", "committed") == true && phases.dig("three_voter_commit", "voters") == 3
abort("2-voter continuity missing") unless phases.dig("two_voter_commit", "committed") == true && phases.dig("two_voter_commit", "voters") == 2
abort("1-voter mutation was not denied") unless phases.dig("one_voter_denial", "committed") == false && phases.dig("one_voter_denial", "typed_outcome") == "quorum_unavailable"
abort("old lease did not expire before successor") unless phases.dig("lease_expiry", "ended_at") < phases.dig("successor_binding", "started_at")
abort("stale Observatory write was not denied") unless phases.dig("stale_write_denial", "typed_outcome") == "stale_lease"
abort("snapshot/restart continuity mismatch") unless phases.dig("snapshot_restore", "state_sha256") == phases.dig("voter_restart", "state_sha256") && phases.dig("agent_continuity", "preserved") == true
abort("replay digest mismatch") unless phases.dig("replay", "producer_sha256") == phases.dig("replay", "independent_sha256")
cleanup = phases.fetch("cleanup")
abort("cleanup incomplete") unless cleanup["processes_remaining"] == 0 && cleanup["ports_remaining"] == 0 && cleanup["verified"] == true

puts "PASS: issue 183 producer artifacts and receipts recomputed at #{revision}"
