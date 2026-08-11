#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
issue = 186
manifest_path = File.join(root, ".csdlc/evidence/186/proof-manifest.json")
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

payload = read_json(artifact_path(manifest, "portability"))
abort("wrong proof schema") unless payload["schema"] == "adl.v0921.drt06.proof.v1"
abort("proof revision mismatch") unless payload["revision"] == revision

required = [["macos","arm64","hosted"],["macos","arm64","local"],["linux","x86_64","hosted"],["linux","x86_64","local"]]
cells = payload.fetch("cells")
actual = cells.map { |c| [c["os"], c["arch"], c["model_kind"]] }
abort("portability matrix denominator mismatch") unless actual.sort == required.sort
abort("matrix cell lacks exact artifact/model identity") unless cells.all? { |c| sha?(c["artifact_sha256"]) && git_sha?(c["source_revision"]) && present?(c["model_id"]) && sha?(c["model_digest"]) && c["production_path"] == true }
abort("machine-local or hand-repair dependency") unless cells.all? { |c| c["tracked_command"] == true && c["hand_repair"] == false && c["machine_local_path"] == false }
required_events = %w[disconnect reconnect voter_restart agent_continuity shepherd_continuity observatory_reattach replay]
abort("continuity event denominator mismatch") unless cells.all? { |c| c.fetch("events").map { |e| e["id"] }.sort == required_events.sort }
abort("continuity or replay mismatch") unless cells.all? { |c| c.fetch("events").all? { |e| e["producer_sha256"] == e["independent_sha256"] } }

puts "PASS: issue 186 producer artifacts and receipts recomputed at #{revision}"
