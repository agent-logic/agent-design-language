#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

ROOT = "docs/milestones/v0.92.1/evidence/release/tail-06"
COMMANDS = [
  [["ruby", ".csdlc/prepared/issues/833/validate-external-review-handoff.rb", "--all"], 8],
  [["ruby", ".csdlc/prepared/issues/833/test-issue-818-candidate-current-supersession.rb"], 8],
  [["python3", "docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py"], 14],
  [["cargo", "test", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "gate9"], 11]
].freeze

candidate = `git rev-parse HEAD`.strip
results = COMMANDS.map do |argv, denominator|
  stdout, stderr, status = Open3.capture3(*argv)
  {
    "argv" => argv, "exit_code" => status.exitstatus, "denominator" => denominator,
    "stdout_sha256" => Digest::SHA256.hexdigest(stdout),
    "stderr_sha256" => Digest::SHA256.hexdigest(stderr),
    "stdout_bytes" => stdout.bytesize, "stderr_bytes" => stderr.bytesize
  }
end
outcome = results.all? { |row| row.fetch("exit_code") == 0 } ? "passed" : "failed"
document = {
  "schema" => "adl.v0921.candidate_validation.v2", "candidate_sha" => candidate,
  "producer" => __FILE__, "producer_sha256" => Digest::SHA256.file(__FILE__).hexdigest,
  "outcome" => outcome, "failures" => results.reject { |row| row.fetch("exit_code") == 0 },
  "commands" => results, "observations" => []
}
File.write(File.join(ROOT, "candidate-validation.json"), JSON.pretty_generate(document) + "\n")
abort("candidate validation failed") unless outcome == "passed"
