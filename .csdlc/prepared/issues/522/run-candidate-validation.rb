#!/usr/bin/env ruby
require "digest"
require "json"
require "open3"

ROOT = "docs/milestones/v0.92.1/evidence/release/tail-06"
COMMANDS = [
  [["ruby", ".csdlc/prepared/issues/833/validate-external-review-handoff.rb", "--all"], "negative_cases_plus_success"],
  [["ruby", ".csdlc/prepared/issues/833/test-issue-818-candidate-current-supersession.rb"], "fraction"],
  [["python3", "docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py"], "json_findings"],
  [["cargo", "test", "--locked", "--manifest-path", "csdlc-v3/Cargo.toml", "--all-targets"], "cargo_tests"],
  [["bash", "adl/tools/test_release_ceremony.sh"], "shell_assertions"],
  [["bash", "adl/tools/test_owner_binary_install.sh"], "shell_assertions"],
  [["bash", "adl/tools/test_check_milestone_closed_issue_sor_truth.sh"], "shell_assertions"],
  [["cargo", "test", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "card_identity"], "cargo_tests"],
  [["cargo", "test", "--locked", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "gate9"], "cargo_tests"]
].freeze

def denominator(output, parser)
  value = case parser
  when "negative_cases_plus_success" then output[/([0-9]+) negative cases/, 1].to_i + 1
  when "fraction" then output[/([0-9]+)\/[0-9]+/, 1].to_i
  when "json_findings" then JSON.parse(output.lines.last).fetch("findings")
  when "cargo_tests" then output.scan(/test result: ok\. ([0-9]+) passed/).flatten.sum(&:to_i)
  when "shell_assertions"
    output.lines.count { |line| line.match?(/\b(?:PASS|passed|ok)\b/i) }
  else 0
  end
  abort("could not derive a positive denominator for #{parser}") unless value.positive?
  value
end

if __FILE__ == $PROGRAM_NAME
  candidate = `git rev-parse HEAD`.strip
  results = COMMANDS.map do |argv, parser|
    stdout, stderr, status = Open3.capture3(*argv)
    combined = stdout + stderr
    {
      "argv" => argv, "exit_code" => status.exitstatus,
      "denominator_parser" => parser,
      "denominator" => status.success? ? denominator(combined, parser) : 0,
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
end
