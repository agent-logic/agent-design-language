#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 187
manifest_path = File.join(root, ".csdlc/evidence/187/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "soak"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt07.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

attempts = payload.fetch("attempts")
abort("soak attempt ledger empty") if attempts.empty?
abort("attempt identifiers not unique") unless attempts.map { |a| a.fetch("id") }.uniq.length == attempts.length
completed = attempts.select { |a| a["outcome"] == "completed" }.to_h { |a| [a.fetch("kind"), a] }
abort("local soak shorter than two hours") unless completed.dig("local", "duration_seconds").to_i >= 7_200
abort("hybrid soak shorter than four hours") unless completed.dig("hybrid", "duration_seconds").to_i >= 14_400
attempts.each do |attempt|
  %w[command_sha256 source_revision model_digest clock_start clock_end terms committed_indexes envelopes_sha256 resource_receipt_sha256 cleanup_receipt_sha256].each { |field| abort("attempt missing #{field}") unless present?(attempt[field]) }
  abort("soak threshold exceeded") unless attempt.fetch("thresholds").all? { |_, value| value["observed"] <= value["maximum"] }
  abort("attempt cleanup incomplete") unless attempt["cleanup_verified"] == true
end
replay = payload.fetch("replay")
abort("replay used live provider") unless replay["live_provider"] == false
%w[outcome_sha256 terms_sha256 indexes_sha256 envelopes_sha256 state_sha256].each { |field| abort("soak replay mismatch #{field}") unless replay.dig("producer", field) == replay.dig("independent", field) }
cleanup = payload.fetch("final_cleanup")
abort("surviving local or cloud resource") unless cleanup["local_processes"] == 0 && cleanup["open_ports"] == 0 && cleanup["cloud_resources"] == 0

puts "PASS: issue 187 producer artifacts and receipts recomputed at #{revision}"
