#!/usr/bin/env ruby
# PVF: deterministic local documentation contract; small resource profile; no network.
# Proof role: inventory, relative links and source-snapshot parity. Not release acceptance.
require 'json'
require 'digest'
require 'open3'
require 'pathname'
root = File.expand_path('../../../..', __dir__)
Dir.chdir(root)
packet = 'docs/milestones/v0.92.1/evidence/release/tail-02'
def read_json(path); JSON.parse(File.read(path)); end
def require_true(condition, message); raise message unless condition; end
mode = ARGV.fetch(0, '--all')
require_true(%w[--inventory --links --claims --all --final].include?(mode), 'unknown validation mode')
if mode == '--final'
  abort 'Final acceptance unavailable: refresh the candidate after issue 517 passing reviewed merge and record independent exact-revision review.'
end
checks = 0
if %w[--inventory --all].include?(mode)
  inventory = read_json("#{packet}/document-inventory.json")
  rows = inventory.fetch('documents')
  paths = rows.map { |r| r.fetch('path') }
  require_true(rows.length == 737 && paths.uniq.length == 737, 'audit denominator changed')
  queries = paths.map { |p| "#{inventory.fetch('baseline')}:#{p}\n" }.join
  raw, err, status = Open3.capture3('git', 'cat-file', '--batch', stdin_data: queries)
  require_true(status.success?, "baseline objects unavailable: #{err}")
  raw = raw.b
  offset = 0
  rows.each do |row|
    ending = raw.index("\n", offset); require_true(ending, 'missing object header')
    header = raw[offset...ending].split; require_true(header[1] == 'blob', "missing baseline #{row['path']}")
    size = Integer(header[2]); body = raw.byteslice(ending + 1, size)
    require_true(Digest::SHA256.hexdigest(body) == row.fetch('sha256'), "baseline hash mismatch #{row['path']}")
    require_true(File.file?(row.fetch('path')), "current document missing #{row['path']}")
    offset = ending + 1 + size + 1
    checks += 1
  end
end
if %w[--links --all].include?(mode)
  inventory = read_json("#{packet}/document-inventory.json")
  paths = inventory.fetch('documents').map { |r| r['path'] }.select { |p| p.end_with?('.md') }
  paths += Dir.glob("#{packet}/*.md")
  paths.uniq.each do |path|
    text = File.read(path).gsub(/^```.*?^```[^\n]*$/m, '')
    text.scan(/(?<!!)\[[^\]\n]*\]\(([^\s)]+)(?:\s+"[^"]*")?\)/).flatten.each do |target|
      next if target.start_with?('#') || target.match?(/\A[a-zA-Z][a-zA-Z0-9+.-]*:/)
      target = target.split('#', 2).first.split('?', 2).first
      next if target.empty? || target.include?('<')
      require_true(!target.start_with?('/'), "absolute link in #{path}: #{target}")
      resolved = File.expand_path(target, File.dirname(path))
      require_true(resolved.start_with?(root + '/') && File.exist?(resolved), "broken relative link #{path}: #{target}")
      checks += 1
    end
  end
  require_true(checks > 0, 'empty link denominator')
end
if %w[--claims --all].include?(mode)
  snapshot = read_json("#{packet}/source-proof-snapshot.json")
  source_path = snapshot.fetch('source_path'); source = read_json(source_path)
  require_true(Digest::SHA256.file(source_path).hexdigest == snapshot.fetch('source_sha256'), 'source diagnostic changed; refresh snapshot')
  require_true(snapshot['source_candidate'] == source['candidate'] && snapshot['source_decision'] == source['decision'], 'source claim mismatch')
  require_true(snapshot['status'] == 'historical_diagnostic_not_current_acceptance', 'diagnostic promoted to acceptance')
  require_true(snapshot['rows'].map { |r| r['issue'] } == source['execution_issues'].map { |r| r['issue'] }, 'proof row denominator mismatch')
  snapshot['rows'].zip(source['execution_issues']).each do |row, original|
    %w[planned_id issue revision merge_revision disposition review_truth artifacts closure_disposition].each do |key|
      require_true(row[key] == original[key], "source field mismatch #{row['issue']} #{key}")
      checks += 1
    end
  end
  creation = read_json("#{packet}/creation-map.json"); original = read_json(creation['source_path'])
  require_true(Digest::SHA256.file(creation['source_path']).hexdigest == creation['source_sha256'], 'creation source changed')
  expected = original['children'].map { |r| r.slice('planned_id', 'issue', 'title') }
  require_true(creation['children'] == expected && expected.length == 45, 'creation mapping mismatch')
  findings = read_json("#{packet}/finding-dispositions.json")
  require_true(findings.map { |f| f['id'] } == (1..15).map { |n| format('D%02d', n) }, 'finding denominator mismatch')
  require_true(findings.find { |f| f['id'] == 'D07' }['status'] == 'source_snapshot_mapped_final_proof_pending', 'final proof gap hidden')
  require_true(snapshot['explicit_current_scope_exclusions'].map { |r| r['issue'] }.sort == [84, 251], 'deferrals lost')
  require_true(File.read("#{packet}/README.md").include?('not a final external-review'), 'handoff status overclaimed')
  checks += 6
end
puts JSON.generate(schema: 'adl.tail02.local_validation.v1', mode: mode, status: 'pass', checks: checks, final_acceptance: false)
