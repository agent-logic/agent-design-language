#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

def section(text, heading)
  text[/^#{Regexp.escape(heading)}\s*$\n(.*?)(?=^## |\z)/m, 1].to_s
end

def dependency_ids(text)
  expanded = text.scan(/(WP-04\.\d{2})\s+through\s+(WP-04\.\d{2})/).flat_map do |first, last|
    (first[-2..].to_i..last[-2..].to_i).map { |number| format("WP-04.%02d", number) }
  end
  (text.scan(/WP-04\.\d{2}/) + expanded).uniq.sort
end

design_path = File.expand_path("design.md", __dir__)
text = File.read(design_path)
rows = text.lines.grep(/^\| WP-04\.\d{2} \|/)
abort "expected 16 child rows, found #{rows.length}" unless rows.length == 16

children = rows.map do |line|
  cells = line.split("|").map(&:strip).reject(&:empty?)
  abort "malformed seven-field child row: #{line}" unless cells.length == 7
  id, issue_cell, owner, dependencies, protected, proof, rollback = cells
  issue = issue_cell[/#(\d+)/, 1]&.to_i
  paths = protected.scan(/`([^`]+)`/).flatten
  abort "#{id} missing live issue" unless issue
  abort "#{id} missing issue owner" unless owner.include?("Issue ##{issue}")
  abort "#{id} missing dependency declaration" if dependencies.empty?
  abort "#{id} missing protected paths" if paths.empty?
  abort "#{id} missing proving boundary" unless proof.include?("Exact nonzero") || id == "WP-04.16"
  abort "#{id} missing rollback responsibility" if rollback.length < 20
  [id, issue, dependencies, paths, proof, rollback]
end

expected_ids = (1..16).map { |number| format("WP-04.%02d", number) }
expected_issues = (5863..5878).to_a
expected_dependencies = {
  "WP-04.01" => [],
  "WP-04.02" => ["WP-04.01"],
  "WP-04.03" => ["WP-04.02"],
  "WP-04.04" => ["WP-04.03"],
  "WP-04.05" => ["WP-04.04"],
  "WP-04.06" => ["WP-04.05"],
  "WP-04.07" => ["WP-04.05"],
  "WP-04.08" => ["WP-04.06", "WP-04.07"],
  "WP-04.09" => ["WP-04.03"],
  "WP-04.10" => ["WP-04.03"],
  "WP-04.11" => ["WP-04.05", "WP-04.08", "WP-04.09", "WP-04.10"],
  "WP-04.12" => ["WP-04.02", "WP-04.08"],
  "WP-04.13" => ["WP-04.08", "WP-04.11", "WP-04.12"],
  "WP-04.14" => ["WP-04.13"],
  "WP-04.15" => ["WP-04.05", "WP-04.08", "WP-04.13", "WP-04.14"],
  "WP-04.16" => (1..15).map { |number| format("WP-04.%02d", number) }
}.freeze
authority_contracts = {
  5869 => ["authoritycertificatev1", "joint membership", "majority-committed", "majorities of both", "union majority", "ed25519", "adl-authority-certificate-v1", "32-byte", "64-byte", "unknown/duplicate/non-minimal", "activation-key possession", "mutation-sink", "malicious-leader/minority"],
  5870 => ["authoritycertificatev1", "mutation sink", "majority-certificate", "activation possession", "quorum-committed", "lease safety window"],
  5875 => ["before fence", "after fence", "source-permit revocation", "majority-committed fencing", "activation-key", "non-authoritative"],
  5876 => ["majority-committed", "authoritycertificatev1", "divergent local histories", "malicious-leader/minority", "quorum proof", "trust-domain recovery"]
}.freeze
cots_contracts = {
  "design.md" => ["quinn", "rustls", "prost", "openraft", "sole distributed manifest and", "adl-runtime/cargo.toml", "adl-runtime/cargo.lock"],
  "sip.values.json" => ["quinn", "rustls", "prost", "openraft", "adl-runtime/cargo.toml", "adl-runtime/cargo.lock"],
  "stp.values.json" => ["quinn", "rustls", "prost", "openraft", "manifest", "lockfile"],
  "spp.values.json" => ["quinn", "rustls", "prost", "openraft", "adl-runtime/cargo.toml", "adl-runtime/cargo.lock"],
  "vpp.values.json" => ["quinn", "rustls", "prost", "openraft", "dependency-lock parity"]
}.freeze
abort "child identities drifted" unless children.map(&:first) == expected_ids
abort "live issue mapping drifted" unless children.map { |row| row[1] } == expected_issues

cots_contracts.each do |surface, terms|
  path = if surface == "design.md"
           File.expand_path("../5865/design.md", __dir__)
         else
           File.expand_path("../../../issues/5865/cards/#{surface}", __dir__)
         end
  content = File.read(path).downcase
  terms.each { |term| abort "child #5865 #{surface} COTS contract omits #{term}" unless content.include?(term) }
end

authority_contracts.each do |issue, terms|
  surfaces = [
    File.read(File.expand_path("../#{issue}/design.md", __dir__)),
    File.read(File.expand_path("../../../issues/#{issue}/cards/sip.values.json", __dir__)),
    File.read(File.expand_path("../../../issues/#{issue}/cards/stp.values.json", __dir__)),
    File.read(File.expand_path("../../../issues/#{issue}/cards/vpp.values.json", __dir__))
  ].join("\n").downcase
  terms.each { |term| abort "child ##{issue} authority contract omits #{term}" unless surfaces.include?(term) }
  record = JSON.parse(File.read(File.expand_path("../../../issues/#{issue}/index.json", __dir__)))
  reviewer = record.dig("design_review", "approved", "reviewer").to_s.strip
  abort "child ##{issue} authority design is not reapproved" if reviewer.empty?
end

all_paths = children.flat_map { |id, _, _, paths, _, _| paths.map { |path| [path, id] } }
duplicates = all_paths.group_by(&:first).select { |_, entries| entries.length > 1 }
abort "duplicate protected paths: #{duplicates.keys.join(', ')}" unless duplicates.empty?
overlaps = all_paths.combination(2).select do |(left, left_id), (right, right_id)|
  left_id != right_id && (left.start_with?("#{right}/") || right.start_with?("#{left}/"))
end
abort "overlapping protected paths: #{overlaps.inspect}" unless overlaps.empty?

dependency_graph = {}
children.each do |id, _, dependencies, _|
  child_dependencies = dependency_ids(dependencies)
  abort "#{id} dependency set drifted" unless child_dependencies == expected_dependencies.fetch(id)
  dependency_graph[id] = child_dependencies
  child_dependencies.each do |dependency|
    abort "#{id} references unknown dependency #{dependency}" unless expected_ids.include?(dependency)
    abort "#{id} depends on itself" if dependency == id
  end
end
visiting = {}
visited = {}
visit = lambda do |id|
  abort "dependency cycle reaches #{id}" if visiting[id]
  return if visited[id]
  visiting[id] = true
  dependency_graph.fetch(id).each { |dependency| visit.call(dependency) }
  visiting.delete(id)
  visited[id] = true
end
expected_ids.each { |id| visit.call(id) }

children.each do |id, issue, _, _, _, _|
  index_path = File.expand_path("../../../issues/#{issue}/index.json", __dir__)
  abort "missing typed record for #{id} ##{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  abort "#{id} design not approved" unless index.dig("design_review", "approved", "revision").to_s.match?(/\A[0-9a-f]{64}\z/)
  abort "#{id} preparation claim active" unless index["claim"].nil?
  abort "#{id} is already bound" unless index["branch"].nil? && index["worktree"].nil?
end

umbrella = JSON.parse(File.read(File.expand_path("../../../issues/5862/index.json", __dir__)))
abort "WP-04-IMP claim active" unless umbrella["claim"].nil?
abort "WP-04-IMP is already bound" unless umbrella["branch"].nil? && umbrella["worktree"].nil?
abort "missing final integration registration owner" unless text.include?("`adl-runtime/src/distributed/mod.rs`") && text.include?("`adl-runtime/src/lib.rs`")

git_common, git_status = Open3.capture2("git", "rev-parse", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_status.success?
github_binary = ENV.fetch("CSDLC_GITHUB_ISSUE_BIN", File.join(File.expand_path("..", git_common.strip), ".adl/bin/csdlc-v2/csdlc-github-issue"))
abort "missing typed GitHub issue binary" unless File.executable?(github_binary)
expected_titles = {5862 => "[v0.92][WP-04-IMP][umbrella] Execute distributed Guardian child wave"}
children.each do |id, issue, dependencies, paths, proof, rollback|
  local_title = JSON.parse(File.read(File.expand_path("../../../issues/#{issue}/cards/stp.values.json", __dir__))).dig("identity", "title")
  test_target = paths.grep(%r{/tests/.*\.rs\z}).first&.then { |path| File.basename(path, ".rs") }
  abort "#{id} proof omits exact owned test target" if test_target && !proof.include?(test_target)
  expected_titles[issue] = [id, local_title, dependencies, paths, test_target, rollback]
end
expected_titles[5862] = ["WP-04-IMP", expected_titles.fetch(5862), "", [], nil, ""]
expected_titles.each do |issue, (id, title, dependencies, paths, test_target, rollback)|
  request_path = File.join(__dir__, "wp04-implementation-wave", "read", "#{issue}.json")
  abort "missing live read request for ##{issue}" unless File.file?(request_path)
  stdout, stderr, status = Open3.capture3(github_binary, "run", "--request", request_path)
  abort "live read failed for ##{issue}: #{stderr} #{stdout}" unless status.success?
  packet = JSON.parse(stdout).fetch("issue")
  abort "live issue ##{issue} is not open" unless packet["state"] == "open"
  abort "live title drift for ##{issue}" unless packet["title"] == title
  body = packet.fetch("body")
  ["## Required Outcome", "## Dependencies", "## Owned Paths", "## Validation And Proof", "## Rollback"].each do |heading|
    abort "live issue ##{issue} omits #{heading}" unless body.include?(heading)
  end
  paths.each do |path|
    abort "live issue ##{issue} omits owned path #{path}" unless body.include?("`#{path}`")
  end
  if issue != 5862
    abort "live body lost #{id} identity for ##{issue}" unless body.include?(id)
    abort "live body lost canonical WP-04-IMP dependency for ##{issue}" unless body.include?("WP-04-IMP issue 5862")
    if issue == 5865
      %w[quinn rustls prost openraft].each do |dependency|
        abort "live issue #5865 COTS contract omits #{dependency}" unless body.downcase.include?(dependency)
      end
    end
    live_dependencies = dependency_ids(section(body, "## Dependencies"))
    abort "live dependency drift for ##{issue}" unless live_dependencies == dependency_ids(dependencies)
    abort "live proof omits #{test_target} for ##{issue}" if test_target && !section(body, "## Validation And Proof").include?(test_target)
    live_rollback = section(body, "## Rollback").downcase
    rollback.downcase.split(/[,;]/).map(&:strip).reject(&:empty?).each do |clause|
      keywords = clause.scan(/[a-z0-9-]+/).reject { |word| %w[the a an and or to from only].include?(word) }
      abort "live rollback drift for ##{issue}: #{clause}" unless keywords.count { |word| live_rollback.include?(word) } >= [keywords.length, 3].min
    end
  end
end
umbrella_request = File.join(__dir__, "wp04-implementation-wave", "read", "5862.json")
stdout, = Open3.capture2(github_binary, "run", "--request", umbrella_request)
live_umbrella = JSON.parse(stdout).fetch("issue").fetch("body")
abort "umbrella lost canonical child denominator" unless live_umbrella.include?("WP-04.01 through WP-04.16")
umbrella_design = File.read(File.expand_path("../5862/design.md", __dir__))
umbrella_rows = umbrella_design.lines.grep(/^\| WP-04\.\d{2} \|/).map do |line|
  cells = line.split("|").map(&:strip).reject(&:empty?)
  [cells.fetch(0), cells.fetch(1)[/#(\d+)/, 1].to_i]
end
abort "umbrella exact denominator drifted" unless umbrella_rows == expected_ids.zip(expected_issues)

rollback_by_id = children.to_h { |id, _, _, _, _, rollback| [id, rollback.downcase] }
abort "WP-04.07 rollback is not majority-committed" unless rollback_by_id.fetch("WP-04.07").include?("majority-committed")
abort "WP-04.08 rollback is not quorum-committed" unless rollback_by_id.fetch("WP-04.08").include?("quorum-committed")
migration_rollback = rollback_by_id.fetch("WP-04.13")
abort "WP-04.13 rollback crosses the fence boundary" unless ["before fence", "after fence", "non-authoritative", "wp-04.14"].all? { |term| migration_rollback.include?(term) }
recovery_rollback = rollback_by_id.fetch("WP-04.14")
abort "WP-04.14 rollback can select uncommitted authority" unless recovery_rollback.include?("majority-committed") && recovery_rollback.include?("quorum-committed")

puts "PASS: live #5862 plus 16 mapped approved claim-null children, #{all_paths.length} exclusive paths, complete owner/proof/rollback fields"
