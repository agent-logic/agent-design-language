#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 188
manifest_path = File.join(root, ".csdlc/evidence/188/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "integrated_review"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.int01.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

lanes = payload.fetch("lanes")
abort("integrated lane denominator mismatch") unless lanes.map { |l| l.fetch("id") }.sort == %w[CORP-08 DRT-07 V3-16]
lanes.each do |lane|
  abort("lane nonterminal") unless lane["terminal"] == true && git_sha?(lane["revision"])
  system("git", "-C", root, "merge-base", "--is-ancestor", lane["revision"], revision, out: File::NULL, err: File::NULL) or abort("lane revision not ancestral: #{lane['id']}")
  abort("lane evidence not independently recomputed") unless lane["recomputed"] == true && present?(lane["evidence_inventory_sha256"]) && present?(lane["validator_receipt_sha256"])
end
abort("lane evidence was cross-credited") unless payload["cross_lane_credit"] == false
findings = payload.fetch("findings")
abort("finding identifiers not unique") unless findings.map { |f| f.fetch("id") }.uniq.length == findings.length
blockers = findings.select { |f| %w[p1 p2].include?(f["severity"]) }
abort("blocking finding lacks terminal verified disposition") unless blockers.all? { |f| f["disposition"] == "fixed" && sha?(f["fix_revision"]) && present?(f["rerun_receipt_sha256"]) }

puts "PASS: issue 188 producer artifacts and receipts recomputed at #{revision}"
