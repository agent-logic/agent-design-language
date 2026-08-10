#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 185
manifest_path = File.join(root, ".csdlc/evidence/185/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "security"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt05.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

identities = payload.fetch("identities")
abort("security identity denominator mismatch") unless identities.map { |i| i.fetch("role") }.sort == %w[agent observatory operator shepherd voter]
abort("identity keys are not separated") unless identities.map { |i| i.fetch("key_sha256") }.uniq.length == identities.length
abort("Shepherd has voting authority") unless identities.find { |i| i["role"] == "shepherd" }.fetch("can_vote") == false
certificates = payload.fetch("certificates")
abort("missing production certificate evidence") if certificates.empty?
abort("unapproved production TLS") unless certificates.all? { |c| c["production"] == true && c["self_signed"] == false && c["chain_verified"] == true && present?(c["trust_anchor_sha256"]) && c["hostname_verified"] == true }
required = %w[forged stale wrong_domain missing_capability cross_polis malformed pre_auth_disclosure provider_timeout provider_stall provider_malformed_output provider_partial_failure]
cases = payload.fetch("cases")
abort("security/failure denominator mismatch") unless cases.map { |c| c.fetch("id") }.sort == required.sort
abort("negative case lacks producer proof") unless cases.all? { |c| present?(c["input_sha256"]) && present?(c["typed_outcome"]) && present?(c["receipt_sha256"]) }
abort("state or authority invariant changed") unless cases.all? { |c| c["state_before_sha256"] == c["state_after_sha256"] && c["authority_before_sha256"] == c["authority_after_sha256"] }

puts "PASS: issue 185 producer artifacts and receipts recomputed at #{revision}"
