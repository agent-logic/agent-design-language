#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 184
manifest_path = File.join(root, ".csdlc/evidence/184/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "hybrid"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt04.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

expected_account = ENV.fetch("ADL_AGENT_LOGIC_AWS_ACCOUNT_SHA256", "")
abort("missing approved business AWS account digest") unless sha?(expected_account)
identity = payload.fetch("aws_identity")
abort("wrong AWS profile") unless identity["profile"] == "agent-logic-admin"
abort("wrong business AWS account") unless identity.fetch("account_sha256") == expected_account
voters = payload.fetch("voters")
abort("hybrid voter denominator mismatch") unless voters.map { |v| v["location"] }.sort == %w[aws_az_a aws_az_b wuji]
aws = voters.select { |v| v["location"].start_with?("aws_") }
abort("AWS voters not in separate AZs") unless aws.map { |v| v.fetch("availability_zone") }.uniq.length == 2
abort("AWS network is not private") unless aws.all? { |v| v["public_ip"] == false && v["public_ingress"] == false && v["transport"] == "mtls" }
%w[state_root snapshot_sha256 snapshot_materialization_id].each { |field| vals = voters.map { |v| v.fetch(field) }; abort("shared #{field}") unless vals.uniq.length == vals.length }
phases = payload.fetch("phases").to_h { |p| [p.fetch("id"), p] }
abort("AWS-only quorum continuity missing") unless phases.dig("wuji_isolated", "aws_quorum_committed") == true
abort("stale Wuji mutation not fenced") unless phases.dig("wuji_isolated", "wuji_typed_outcome") == "stale_fence"
abort("quorum-loss mutation did not halt") unless phases.dig("quorum_loss", "committed") == false
heal = phases.fetch("healed")
%w[term commit_index state_sha256 fence observatory_owner].each { |field| abort("healing did not converge #{field}") unless heal.fetch("members").map { |m| m.fetch(field) }.uniq.one? }
cleanup = payload.fetch("cleanup")
abort("provider cleanup incomplete") unless cleanup["enumerated_before"] > 0 && cleanup["enumerated_after"] == 0 && cleanup["verified"] == true

puts "PASS: issue 184 producer artifacts and receipts recomputed at #{revision}"
