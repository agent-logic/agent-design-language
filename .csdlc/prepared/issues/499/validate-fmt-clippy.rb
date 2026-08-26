#!/usr/bin/env ruby
# frozen_string_literal: true

ROOT = File.expand_path("../../../..", __dir__)
MANIFEST = File.join(ROOT, "adl/Cargo.toml")
commands = [
  ["cargo", "fmt", "--manifest-path", MANIFEST, "--check"],
  ["cargo", "clippy", "--manifest-path", MANIFEST, "--lib", "--", "-D", "warnings"]
]

commands.each do |cmd|
  puts cmd.join(" ")
  abort "RUST-01 fmt/clippy proof failed for #{cmd.join(" ")}" unless system(*cmd, chdir: ROOT)
end

puts "RUST-01 fmt/clippy proof passed"
