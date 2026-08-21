#!/usr/bin/env ruby
require "json"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
path = root.join(".csdlc/evidence/451/runtime-feature-wiring-audit.json")
audit = JSON.parse(path.read)
expected = %w[identity continuity birthday_decision birth_witness memory_palace capability_envelope governed_cognitive_profile adaptive_learning acc_tool_authority]
abort "wrong schema" unless audit.keys.sort == %w[issue rows schema source_revision].sort && audit["schema"] == "adl.runtime.feature_wiring_audit.v1" && audit["issue"] == 451
abort "wrong feature denominator" unless audit["rows"].map { |row| row["feature"] } == expected
revision = audit.fetch("source_revision")
abort "invalid source revision" unless revision.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{revision}^{commit}", chdir: root.to_s, out: File::NULL, err: File::NULL)
required = %w[feature construction production_consumption behavioral_proof negative_proof disposition]
audit["rows"].each do |row|
  abort "wrong row keys" unless row.keys.sort == required.sort
  abort "non-live feature: #{row["feature"]}" unless row["disposition"] == "live"
  required[1..4].each do |field|
    value = row.fetch(field)
    candidate = Pathname.new(value)
    abort "unsafe path: #{value}" if candidate.absolute? || value.split("/").include?("..")
    abort "missing path: #{value}" unless root.join(candidate).file?
  end
end
abort "production composition missing" unless root.join("adl-runtime-kernel/src/production_birthday.rs").read.include?("ProductionBirthdayStore")
abort "ACC live consumer missing" unless root.join("adl/src/long_lived_agent.rs").read.include?("govern_resident_tool_output_v1")
puts JSON.generate({schema: "adl.runtime.feature_wiring_audit_result.v1", issue: 451, rows: expected.length, source_revision: audit["source_revision"], result: "passed"})
