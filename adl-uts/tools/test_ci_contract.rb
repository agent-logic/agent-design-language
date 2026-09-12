#!/usr/bin/env ruby
# PVF: tooling contract; deterministic local shell; required #877 publication gate.
# Exercises the actual workflow block, including rejected missing/failed lanes.
require 'yaml'
require 'open3'

root = File.expand_path('../..', __dir__)
workflow = YAML.load_file(File.join(root, '.github/workflows/ci.yaml'))
jobs = workflow.fetch('jobs')
aggregate = jobs.fetch('adl-ci')
abort 'package job missing from aggregate needs' unless aggregate.fetch('needs').include?('adl_uts_package')
step = aggregate.fetch('steps').find { |s| s['name'] == 'Aggregate split adl-ci lanes' }
script = step.fetch('run')
start = script.index('if [ "$UTS_PACKAGE_REQUIRED" = true ]; then') or abort 'UTS gate missing'
finish = script.index('case "$CSDLC_V2_STANDALONE_REQUIRED" in', start) or abort 'UTS gate boundary missing'
block = script[start...finish]
%w[UTS_PACKAGE_REQUIRED UTS_PACKAGE_RESULT].each do |key|
  abort "missing aggregate env #{key}" unless step.fetch('env').key?(key)
end
jobs.fetch('adl_path_policy').fetch('steps').each do |s|
  abort 'UTS result gate must not run before package job' if s.fetch('run', '').include?('$UTS_PACKAGE_RESULT')
end
[
  ['true', 'success', true], ['true', 'failure', false], ['true', 'skipped', false],
  ['true', '', false], ['false', 'skipped', true], ['false', 'success', false]
].each do |required, result, success|
  _, _, status = Open3.capture3({'UTS_PACKAGE_REQUIRED' => required, 'UTS_PACKAGE_RESULT' => result}, 'bash', '-euc', block)
  abort "gate mismatch #{required}/#{result}" unless status.success? == success
end
puts 'PASS UTS package CI gate: six outcomes and declaration scope'
