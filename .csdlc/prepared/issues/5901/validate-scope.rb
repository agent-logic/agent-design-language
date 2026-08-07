#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "set"

ROOT = File.expand_path("../../../..", __dir__)
Dir.chdir(ROOT)

allowed_files = Set.new(%w[
  csdlc-v2/src/cards.rs
  csdlc-v2/src/store.rs
  csdlc-v2/src/bin/csdlc-finish.rs
  csdlc-v2/tests/gate2.rs
  csdlc-v2/tests/gate_finish.rs
  .csdlc/issues/5865/cards/sip.values.json
  .csdlc/issues/5865/cards/stp.values.json
  .csdlc/issues/5865/cards/spp.md
  .csdlc/issues/5865/cards/spp.values.json
  .csdlc/issues/5865/cards/vpp.values.json
  .csdlc/issues/5865/cards/srp.values.json
  .csdlc/issues/5865/cards/sor.values.json
  .csdlc/issues/5865/index.json
  .csdlc/issues/5865/audit.jsonl
  .csdlc/prepared/issues/5862/validate-implementation-wave.rb
  .csdlc/prepared/issues/5901/test-implementation-wave.rb
  .csdlc/prepared/issues/5901/validate-scope.rb
])
allowed_prefixes = %w[
  .csdlc/issues/5901/
  .csdlc/prepared/issues/5901/
  .csdlc/evidence/5901/
  .csdlc/evidence/.csdlc-finalize-5901-
].freeze

base, status = Open3.capture2("git", "merge-base", "origin/main", "HEAD")
abort "cannot resolve exact origin/main base" unless status.success?
changed, status = Open3.capture2("git", "diff", "--name-only", "#{base.strip}...HEAD")
abort "cannot inspect committed change set" unless status.success?
working, status = Open3.capture2("git", "status", "--porcelain=v1", "--untracked-files=all")
abort "cannot inspect working change set" unless status.success?
paths = changed.lines.map(&:strip)
paths.concat(working.lines.map { |line| line[3..]&.strip }.compact)
paths.uniq!

unexpected = paths.reject { |path| allowed_files.include?(path) || allowed_prefixes.any? { |prefix| path.start_with?(prefix) } }
product = paths.select { |path| path.start_with?("adl-runtime/src/distributed/") || path.start_with?("adl-runtime/tests/distributed_") }
other_children = paths.select do |path|
  match = path.match(%r{\A\.csdlc/issues/(586[3-9]|587[0-8])/})
  match && !path.start_with?(".csdlc/issues/5865/")
end
abort "unexpected changed paths: #{unexpected.join(', ')}" unless unexpected.empty?
abort "Guardian product paths changed: #{product.join(', ')}" unless product.empty?
abort "other Sprint 3 child topology changed: #{other_children.join(', ')}" unless other_children.empty?
puts "PASS: #{paths.length} changed paths stay inside issue #5901 readiness scope"
