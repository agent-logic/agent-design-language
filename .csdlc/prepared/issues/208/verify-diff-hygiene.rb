#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
BASE = ENV.fetch("ISSUE_208_EXECUTION_BASE", "4460ec8157da7a53decf28f41e20af8afd19f611")
SOURCE = ENV.fetch("ISSUE_208_PROVING_SOURCE") do
  out, status = Open3.capture2("git", "rev-parse", "HEAD", chdir: ROOT.to_s)
  abort("issue 208 diff: cannot resolve source") unless status.success?
  out.strip
end
PROTECTED = %w[
  adl-runtime-kernel/Cargo.toml adl-runtime-kernel/Cargo.lock
  adl-runtime-kernel/src/continuity_control.rs adl-runtime-kernel/src/assembly.rs
  adl-runtime-kernel/src/bin/adl-runtime-kernel.rs adl-runtime-kernel/src/config.rs
  adl-runtime-kernel/src/governance.rs adl-runtime-kernel/src/lib.rs
  adl-runtime-kernel/src/reasoning.rs adl-runtime-kernel/tests/kernel_continuity_control.rs
  adl-runtime/Cargo.toml adl-runtime/Cargo.lock adl-runtime/src/kernel_continuity_client.rs
  adl-runtime/src/bin/adl-runtime-guardian.rs adl-runtime/src/distributed/polis_runtime.rs
  adl-runtime/src/guardian.rs adl-runtime/src/lib.rs adl-runtime/tests/kernel_continuity_client.rs
  .csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json
  .csdlc/prepared/issues/208/verify-diff-hygiene.rb
  .csdlc/prepared/issues/208/produce-proof-receipt.rb
  .csdlc/prepared/issues/208/validate-proof-receipt.rb
].freeze

def git!(*args)
  out, err, status = Open3.capture3("git", *args, chdir: ROOT.to_s)
  abort("issue 208 diff: git #{args.join(' ')} failed: #{err.strip}") unless status.success?
  out
end

abort("issue 208 diff: malformed revision") unless [BASE, SOURCE].all? { |revision| revision.match?(/\A[0-9a-f]{40}\z/) }
git!("cat-file", "-e", "#{BASE}^{commit}")
git!("cat-file", "-e", "#{SOURCE}^{commit}")
system("git", "merge-base", "--is-ancestor", BASE, SOURCE, chdir: ROOT.to_s, out: File::NULL, err: File::NULL) ||
  abort("issue 208 diff: execution base is not an ancestor of source")
whitespace = git!("diff", "--check", "#{BASE}..#{SOURCE}")
abort("issue 208 diff: whitespace/EOF diagnostics:\n#{whitespace}") unless whitespace.empty?
dirty = git!("status", "--porcelain=v1", "--untracked-files=all", "--", *PROTECTED)
abort("issue 208 diff: protected source is dirty:\n#{dirty}") unless dirty.empty?
puts "PASS: issue #208 exact-range diff hygiene #{BASE}..#{SOURCE}"
