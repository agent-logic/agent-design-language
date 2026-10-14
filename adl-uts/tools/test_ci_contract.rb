#!/usr/bin/env ruby
# PVF: deterministic local contract; preserves #877 six result cases after consolidation.
require 'yaml'
require 'open3'
root = File.expand_path('../..', __dir__)
jobs = YAML.load_file(File.join(root, '.github/workflows/ci.yaml')).fetch('jobs')
aggregate = jobs.fetch('adl-ci')
abort 'public producer missing' unless aggregate.fetch('needs').include?('public_adl_validation')
step = aggregate.fetch('steps').find { |s| s['name'] == 'Require every selected public lane' }
abort 'wrong aggregate command' unless step.fetch('run') == 'python3 tools/public_adl/ci_policy.py --aggregate ci'
expected = {'PUBLIC_REQUIRED' => '${{ needs.adl_path_policy.outputs.public_adl_validation_required }}',
            'PUBLIC_RESULT' => '${{ needs.public_adl_validation.result }}'}
expected.each { |key, value| abort "wrong binding #{key}" unless step.fetch('env')[key] == value }
abort 'UTS package dropped' unless File.read(File.join(root, 'tools/public_adl/run_ci.sh')).include?("  adl-uts\n")
jobs.fetch('adl_path_policy').fetch('steps').each do |s|
  abort 'premature result gate' if s.fetch('run', '').include?('$PUBLIC_RESULT')
end
[
  ['true', 'success', true], ['true', 'failure', false], ['true', 'skipped', false],
  ['true', '', false], ['false', 'skipped', true], ['false', 'success', false]
].each do |required, result, success|
  env = {'PATH_POLICY_RESULT' => 'success', 'CONTRACT_RESULT' => 'success',
         'TRANSPILE_REQUIRED' => 'false', 'TRANSPILE_RESULT' => 'skipped',
         'PUBLIC_REQUIRED' => required, 'PUBLIC_RESULT' => result}
  _, _, status = Open3.capture3(env, 'bash', '-euc', step.fetch('run'), chdir: root)
  abort "gate mismatch #{required}/#{result}" unless status.success? == success
end
puts 'PASS UTS public producer gate: six outcomes and declaration scope'
