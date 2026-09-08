#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "shellwords"

root = File.expand_path("../../../..", __dir__)
source = File.join(root, ".csdlc/evidence/516/semantic-criterion-evidence.json")
fixture_dir = File.join(`git -C #{root.shellescape} rev-parse --git-common-dir`.strip, "csdlc-v3/test-fixtures/516/#{Process.pid}")
fixture_dir = File.expand_path(fixture_dir, root)
FileUtils.mkdir_p(fixture_dir)
fixture = File.join(fixture_dir, "semantic.json")
script = File.join(__dir__, "refresh-semantic-evidence.rb")

def run_refresh(script, fixture)
  Open3.capture3({ "ADL_SEMANTIC_EVIDENCE_PATH" => fixture }, "ruby", script)
end

begin
  FileUtils.cp(source, fixture)
  original = File.binread(fixture)
  2.times do
    _stdout, stderr, status = run_refresh(script, fixture)
    abort "idempotent refresh failed: #{stderr}" unless status.success?
  end
  abort "already-migrated refresh changed bytes" unless File.binread(fixture) == original

  document = JSON.parse(original)
  entries = document.fetch("entries").to_h { |entry| [entry.fetch("criterion_id"), entry] }
  old_runtime = Marshal.load(Marshal.dump(entries.fetch("OBS-B-ac-3"))).merge("criterion_id" => "OBS-B-ac-2")
  old_accessibility = Marshal.load(Marshal.dump(entries.fetch("OBS-B-ac-4"))).merge("criterion_id" => "OBS-B-ac-3")
  conflict = Marshal.load(Marshal.dump(entries.fetch("OBS-B-ac-4"))).merge("criterion_digest" => "0" * 64)
  document["entries"] = entries.reject { |id, _| id.start_with?("OBS-B-ac-") }.values + [old_runtime, old_accessibility, conflict]
  File.write(fixture, JSON.pretty_generate(document) + "\n")
  before = File.binread(fixture)
  _stdout, stderr, status = run_refresh(script, fixture)
  abort "conflicting ac-4 was accepted" if status.success?
  abort "wrong conflict diagnostic: #{stderr}" unless stderr.include?("unexpected OBS-B criterion identities")
  abort "conflict rejection modified fixture" unless File.binread(fixture) == before
  puts JSON.generate(schema: "csdlc.semantic_refresh_test.v1", status: "pass")
ensure
  FileUtils.rm_rf(fixture_dir) if fixture_dir.include?("/csdlc-v3/test-fixtures/516/")
end
