#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
manifest = JSON.parse(Pathname.new(__dir__).join("product-paths.json").read)
own = manifest.fetch("planned_paths")
siblings = manifest.fetch("siblings")

def normalized(path)
  value = Pathname.new(path).cleanpath.to_s.sub(%r{/+$}, "")
  abort("unsafe product path #{path.inspect}") if value.empty? || value == "." || value.start_with?("/", "../") || value.split("/").include?("..")
  value
end

def overlaps?(left, right)
  left == right || left.start_with?("#{right}/") || right.start_with?("#{left}/")
end

own = own.map { |path| normalized(path) }
reserved = siblings.flat_map { |issue, paths| paths.map { |path| [issue, normalized(path)] } }
collisions = own.flat_map do |path|
  reserved.map { |issue, sibling| "##{issue}: #{path} overlaps #{sibling}" if overlaps?(path, sibling) }.compact
end

worktrees, _stderr, status = Open3.capture3("git", "worktree", "list", "--porcelain", chdir: root.to_s)
abort("cannot enumerate worktrees") unless status.success?
active = []
worktrees.scan(/^worktree (.+)$/).flatten.each do |worktree|
  Dir.glob(File.join(worktree, ".csdlc", "issues", "*", "index.json")).sort.each do |index_path|
    record = JSON.parse(File.read(index_path)) rescue next
    claim = record["claim"]
    next if claim.nil? || record["issue"] == 5500
    claim.fetch("protected_paths", []).each do |claimed|
      candidate = normalized(claimed)
      own.each do |path|
        active << "##{record['issue']}: #{path} overlaps active #{candidate}" if overlaps?(path, candidate)
      end
    end
  end
end

failures = (collisions + active).uniq
abort("product path collision:\n#{failures.join("\n")}") unless failures.empty?
puts JSON.generate(status: "pass", issue: 5500, planned_paths: own, sibling_reservations: reserved.length, active_collisions: 0)
