#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

EXPECTED = [
  "cargo", "nextest", "run", "--manifest-path", "adl-runtime/Cargo.toml",
  "--test", "distributed_migration", "--no-tests=fail"
].freeze
ROOT = Pathname.new(__dir__).join("../../..").cleanpath.expand_path
abort "exact child runner argv mismatch" unless ARGV == EXPECTED
stdout, stderr, status = Open3.capture3({"CARGO_TERM_COLOR" => "never"}, *EXPECTED, chdir: ROOT.to_s)
$stderr.write(stdout)
$stderr.write(stderr)
abort "exact child tests failed" unless status.success?
summary = (stdout + stderr).match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
abort "exact child test denominator mismatch" unless summary && summary[1].to_i.positive? && summary[1] == summary[2]
puts JSON.generate({"schema" => "adl.wp04.exact_child_summary.v1", "selected_tests" => summary[1].to_i, "passed_tests" => summary[2].to_i, "skipped_tests" => 0})
