#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 181
manifest_path = File.join(root, ".csdlc/evidence/181/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "contract"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt01.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

topology = payload.fetch("topology")
nodes = topology.fetch("nodes")
expected_roles = {"voter" => 3, "governed_agent" => 3, "shepherd" => 1, "observatory" => 1}
actual_roles = nodes.group_by { |node| node.fetch("role") }.transform_values(&:length)
abort("topology role denominator mismatch") unless actual_roles == expected_roles
%w[id identity credential_ref port state_root storage failure_domain].each do |field|
  values = nodes.map { |node| node.fetch(field) }
  abort("non-unique topology #{field}") unless values.uniq.length == values.length
end
abort("Shepherd must be non-voting") unless nodes.one? { |n| n["role"] == "shepherd" && n["voting"] == false }
abort("Observatory must be quorum-leased") unless nodes.one? { |n| n["role"] == "observatory" && n["lease_authority"] == "quorum" }
required_scenarios = %w[election quorum_loss stale_lease voter_restart snapshot_restore partition healing replay cleanup]
scenarios = payload.fetch("scenarios")
abort("scenario denominator mismatch") unless scenarios.map { |s| s.fetch("id") }.sort == required_scenarios.sort
%w[setup action expected_behavior timeout_seconds receipt_fields cleanup fail_closed_outcome proof_owner].each do |field|
  abort("scenario missing #{field}") unless scenarios.all? { |s| present?(s[field]) }
end
abort("invalid production proof boundary") unless payload.dig("proof_boundary", "production_processes") == true && payload.dig("proof_boundary", "in_process_substitutes") == false && payload.dig("proof_boundary", "hard_coded_counts") == false

puts "PASS: issue 181 producer artifacts and receipts recomputed at #{revision}"
