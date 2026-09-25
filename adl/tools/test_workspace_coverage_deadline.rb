#!/usr/bin/env ruby
# frozen_string_literal: true
# PVF #916: deterministic local CI-policy contract; small temporary YAML fixtures.
# Required focused prepublication gate; no jobs, workloads or clock waits execute.
require "fileutils"
require "json"
require "open3"
require "tmpdir"
require "yaml"

root = File.expand_path("../..", __dir__)
validator = File.join(__dir__, "validate_ci_workflow_policy.rb")
source = File.read(File.join(root, ".github/workflows/ci.yaml"))
job_header = "  adl_coverage_workspace_hosted:\n"
raise "missing fixture job" unless source.include?(job_header)

Dir.mktmpdir("coverage-deadline-contract") do |fixture|
  FileUtils.mkdir_p(File.join(fixture, ".github"))
  FileUtils.cp_r(File.join(root, ".github/workflows"), File.join(fixture, ".github"))
  FileUtils.cp(File.join(root, "rust-toolchain.toml"), fixture)
  FileUtils.mkdir_p(File.join(fixture, "adl/tools"))
  FileUtils.cp(File.join(root, "adl/tools/ci_path_policy.sh"), File.join(fixture, "adl/tools"))
  workflow = File.join(fixture, ".github/workflows/ci.yaml")
  cases = {
    "current complete workflow" => [source, true],
    "original 35 minute job deadline" => [source.sub(job_header, job_header + "    timeout-minutes: 35\n"), false],
    "quoted deadline key" => [source.sub(job_header, job_header + "    'timeout-minutes': 35\n"), false],
    "expression deadline" => [source.sub(job_header, job_header + "    timeout-minutes: ${{ vars.COVERAGE_TIMEOUT }}\n"), false],
    "null deadline" => [source.sub(job_header, job_header + "    timeout-minutes:\n"), false],
    "malformed YAML" => [source + "\nbroken: [\n", false]
  }
  before, job = source.split(job_header, 2)
  step_limited = before + job_header + job.sub("    steps:\n", "    steps:\n      - name: shorter coverage deadline\n        timeout-minutes: 35\n        run: echo fixture\n")
  cases["step deadline"] = [step_limited, false]
  cases.each do |name, (content, expected)|
    File.write(workflow, content)
    stdout, stderr, status = Open3.capture3("ruby", validator, fixture)
    raise "#{name}: unexpected policy result\n#{stdout}\n#{stderr}" unless status.success? == expected
    unless expected
      result = JSON.parse(stdout)
      raise "#{name}: rejected for unrelated reason" unless result.fetch("errors").any? { |e| e.include?("deadline") }
    end
  end
  puts "PASS workspace coverage deadline contract: #{cases.length} cases"
end
