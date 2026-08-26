#!/usr/bin/env ruby
require "json"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
path = root.join(".csdlc/evidence/451/production-birthday-evidence.json")
evidence = JSON.parse(path.read)
expected_keys = %w[schema issue source_revision kernel_tests resident_tests proof redaction]
abort "wrong evidence keys" unless evidence.keys.sort == expected_keys.sort
abort "wrong schema" unless evidence["schema"] == "adl.runtime.production_birthday_evidence.v1" && evidence["issue"] == 451
revision = evidence.fetch("source_revision")
abort "invalid source revision" unless revision.match?(/\A[0-9a-f]{40}\z/) && system("git", "cat-file", "-e", "#{revision}^{commit}", chdir: root.to_s, out: File::NULL, err: File::NULL)
%w[kernel_tests resident_tests].each do |name|
  row = evidence.fetch(name)
  abort "non-proving test denominator" unless row["selected"].is_a?(Integer) && row["selected"] > 0 && row["passed"] == row["selected"] && row["failed"] == 0
end
abort "incomplete production proof" unless evidence.fetch("proof").values.all?(true)
redaction = evidence.fetch("redaction")
abort "redaction failed" unless redaction == {"private_state_retained"=>false, "provider_content_retained"=>false, "tool_arguments_retained"=>false, "repository_relative_paths"=>true}
serialized = JSON.generate(evidence)
abort "machine-local path retained" if serialized.match?(%r{/(Users|Volumes|private|tmp)/})
puts JSON.generate({schema: "adl.runtime.production_birthday_evidence_result.v1", issue: 451, result: "passed"})
