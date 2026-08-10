#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 182
manifest_path = File.join(root, ".csdlc/evidence/182/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "conformance"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt02.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

families = payload.fetch("message_families")
abort("missing message-family denominator") if families.empty? || families.uniq.length != families.length
vectors_path = artifact_path(manifest, "vectors")
vectors = File.readlines(vectors_path, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
abort("missing positive family vector") unless families.all? { |family| vectors.any? { |v| v["family"] == family && v["kind"] == "positive" } }
vectors.select { |v| v["kind"] == "positive" }.each { |v| abort("canonical round-trip drift") unless v.fetch("encoded_sha256") == v.fetch("reencoded_sha256") }
required_mutations = %w[identity authority permit causation correlation sequence term polis duplicate reordered stale malformed unsigned wrong_domain cross_polis]
actual_mutations = vectors.select { |v| v["kind"] == "negative" }.map { |v| v.fetch("mutation") }.uniq
abort("mutation denominator mismatch") unless actual_mutations.sort == required_mutations.sort
abort("negative vector lacks typed outcome") unless vectors.select { |v| v["kind"] == "negative" }.all? { |v| present?(v["typed_outcome"]) && present?(v["input_sha256"]) }
replay = read_json(artifact_path(manifest, "replay"))
%w[input_sha256 committed_outcome_sha256 receipt_sha256].each { |k| abort("replay digest mismatch: #{k}") unless replay.dig("producer", k) == replay.dig("independent", k) }
abort("replay used hidden mutable state") unless replay["hidden_mutable_state"] == false

puts "PASS: issue 182 producer artifacts and receipts recomputed at #{revision}"
