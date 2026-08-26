#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "json"
require "fileutils"

script = ".csdlc/prepared/issues/312/validate-doc-release-truth.rb"
abort "validator missing" unless File.file?(script)

_out, _err, status = Open3.capture3("ruby", "-c", script)
abort "validator syntax failed" unless status.success?

fixture_root = ".csdlc/prepared/issues/312/test-fixtures"
FileUtils.rm_rf(fixture_root)
FileUtils.mkdir_p(fixture_root)
inventory_path = "docs/reviews/v0.92/docs-release-truth-312/inventory.json"
inventory = JSON.parse(File.read(inventory_path))
cases = []

def rejected?(script, mode, env = {})
  _out, _err, status = Open3.capture3(env, "ruby", script, mode)
  !status.success?
end

missing = Marshal.load(Marshal.dump(inventory))
missing["rows"].pop
path = File.join(fixture_root, "missing.json")
File.write(path, JSON.pretty_generate(missing))
cases << rejected?(script, "packet", "ADL_DOC_INVENTORY" => path)

duplicate = Marshal.load(Marshal.dump(inventory))
duplicate["rows"] << duplicate["rows"].first
path = File.join(fixture_root, "duplicate.json")
File.write(path, JSON.pretty_generate(duplicate))
cases << rejected?(script, "packet", "ADL_DOC_INVENTORY" => path)

tampered = Marshal.load(Marshal.dump(inventory))
tampered["rows"].first["evidence_sha256"] = "0" * 64
path = File.join(fixture_root, "tampered.json")
File.write(path, JSON.pretty_generate(tampered))
cases << rejected?(script, "packet", "ADL_DOC_INVENTORY" => path)

handoff = File.read("docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md")
path = File.join(fixture_root, "local-path.md")
File.write(path, handoff + "\n/Users/example/private\n")
cases << rejected?(script, "structure-handoff", "ADL_DOC_HANDOFF" => path)

path = File.join(fixture_root, "missing-section.md")
File.write(path, handoff.sub("## Findings Format", "## Output"))
cases << rejected?(script, "structure-handoff", "ADL_DOC_HANDOFF" => path)

cases << rejected?(script, "unknown")

readme_manifest = ".csdlc/evidence/312/readme-paths.txt"
readmes = File.readlines(readme_manifest, chomp: true)
File.write(readme_manifest, readmes.drop(1).join("\n") + "\n")
cases << rejected?(script, "packet")
File.write(readme_manifest, readmes.join("\n") + "\n")

File.write(readme_manifest, (readmes + ["stale/README.md"]).join("\n") + "\n")
cases << rejected?(script, "packet")
File.write(readme_manifest, readmes.join("\n") + "\n")

File.write(readme_manifest, (readmes + [readmes.first]).join("\n") + "\n")
cases << rejected?(script, "packet")
File.write(readme_manifest, readmes.join("\n") + "\n")

scope_path = "docs/milestones/v0.92/unexpected-scope-file.txt"
File.write(scope_path, "unexpected\n")
cases << rejected?(script, "structure-handoff")
FileUtils.rm_f(scope_path)

adl_path = "docs/milestones/v0.92/local-authority-fixture.md"
File.write(adl_path, "Depends on .adl/docs/TBD/private.md\n")
cases << rejected?(script, "packet", "ADL_DOC_EXTRA_SCAN" => adl_path)
FileUtils.rm_f(adl_path)

FileUtils.rm_rf(fixture_root)
abort "negative suite accepted invalid evidence" unless cases.all?

puts({ schema: "adl.v0.92.doc_release_truth_negative_suite.v1", status: "passed", cases: cases.length }.to_json)
