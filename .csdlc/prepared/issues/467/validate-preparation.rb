#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../..", __dir__)
issue_dir = File.join(root, ".csdlc", "issues", "467")
prepared_dir = File.join(root, ".csdlc", "prepared", "issues", "467")

required = [
  File.join(issue_dir, "index.json"),
  File.join(prepared_dir, "design.md"),
  File.join(prepared_dir, "diagram.mmd"),
  File.join(prepared_dir, "validate-quality-gate.rb"),
  File.join(prepared_dir, "test-validate-quality-gate.rb")
]
missing = required.reject { |path| File.file?(path) }
abort("missing #467 preparation targets: #{missing.join(', ')}") unless missing.empty?
index = JSON.parse(File.read(File.join(issue_dir, "index.json")))
abort("wrong issue") unless index.fetch("issue") == 467
puts "issue #467 preparation bundle is ready"
