#!/usr/bin/env ruby
# frozen_string_literal: true

ROOT = File.expand_path("../../../..", __dir__)
cmd = [
  "cargo",
  "test",
  "--manifest-path",
  File.join(ROOT, "adl/Cargo.toml"),
  "resilience::tests::",
  "--",
  "--test-threads=1"
]

puts cmd.join(" ")
abort "RUST-01 resilience positive/negative proof failed" unless system(*cmd, chdir: ROOT)
puts "RUST-01 resilience positive/negative proof passed"
