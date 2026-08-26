#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

source_root = ARGV.fetch(0)
packet_root = ARGV.fetch(1)
target_sha = ARGV.fetch(2)

stdout, status = Open3.capture2("git", "-C", source_root, "ls-tree", "-r", "--name-only", target_sha)
abort "cannot inventory target" unless status.success?
paths = stdout.lines.map(&:strip).reject(&:empty?).sort

code = paths.select { |p| p.match?(/\.(rs|rb|py|sh|js|mjs|ts|tsx|cs)$/) && !p.match?(%r{(^|/)(test|tests|fixtures?)(/|_)}) }
tests = paths.select { |p| p.match?(%r{(^|/)(test|tests|fixtures?)(/|_)}) || p.match?(/(_test|\.test)\./) }
docs = paths.select { |p| p.end_with?(".md") }
dependencies = paths.select { |p| p.match?(%r{(^|/)(Cargo\.(toml|lock)|package(-lock)?\.json|Gemfile(\.lock)?|requirements[^/]*\.txt|Dockerfile|\.github/workflows/[^/]+\.ya?ml)$}) }
architecture = (paths.select { |p| p.end_with?("/lib.rs", "/mod.rs") } + paths.select { |p| p.match?(%r{(^|/)(ADR|architecture|design)[^/]*\.(md|mmd)$}i) } + dependencies).uniq.sort
security = paths.select { |p| p.match?(/(auth|security|privacy|secret|sign|tls|credential|sandbox|permission|redact)/i) }
lifecycle = paths.select { |p| p.start_with?("csdlc-v2/", ".csdlc/issues/") }
demos = paths.select { |p| p.start_with?("demos/") || p.match?(/(demo|proof)/i) }
release = paths.select { |p| p.match?(%r{(^|/)(release|publication|external_launch|milestones)/}i) }

assignments = {
  "architecture" => architecture,
  "code" => code,
  "dependencies" => dependencies,
  "docs" => docs,
  "security" => security,
  "tests" => tests,
  "lifecycle" => lifecycle,
  "demos" => demos,
  "release_publication" => release
}
assignments.each { |lane, rows| abort "empty #{lane} assignment" if rows.empty? }

output = {
  "schema" => "adl.internal_review.specialist_assignments.v1",
  "target_sha" => target_sha,
  "source_checkout_kind" => "clean_primary_checkout",
  "output_worktree" => "issue_313_registered_fastwork_worktree",
  "assignments" => assignments,
  "counts" => assignments.transform_values(&:length)
}
File.write(File.join(packet_root, "specialist_assignments.json"), JSON.pretty_generate(output) + "\n")
puts JSON.generate("status" => "passed", "target_sha" => target_sha, "counts" => output["counts"])
