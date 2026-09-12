#!/usr/bin/env ruby
# PVF contract: actual aggregate shell outcomes and job transport wiring, no network.
require 'yaml'
require 'open3'
require 'pathname'
root = Pathname.new(__dir__).join('../../..').cleanpath
workflow = YAML.load_file(root.join('.github/workflows/ci.yaml'))
jobs = workflow.fetch('jobs')
aggregate = jobs.fetch('adl-ci')
raise 'missing required CI acquisition need' unless aggregate.fetch('needs').include?('codefriend_ci_acquisition')
script = aggregate.fetch('steps').find { |s| s['name'] == 'Aggregate split adl-ci lanes' }.fetch('run')
block = script.split('# BEGIN CI acquisition aggregate contract', 2)[1].split('# END CI acquisition aggregate contract', 2)[0]
%w[true false garbage].product(%w[success skipped failure cancelled]).each do |selected, result|
  _, status = Open3.capture2e({'CODEFRIEND_CI_REQUIRED'=>selected, 'CODEFRIEND_CI_RESULT'=>result}, 'bash', '-eu', '-c', block)
  expected = (selected == 'true' && result == 'success') || (selected == 'false' && result == 'skipped')
  raise "incorrect gate #{selected}:#{result}" unless status.success? == expected
end
job = jobs.fetch('codefriend_ci_acquisition')
raise 'permissions' unless job.fetch('permissions') == {'contents'=>'read'}
raise 'selection' unless job.fetch('if').include?("outputs.codefriend_ci_required == 'true'")
upload = job.fetch('steps').find { |s| s.fetch('uses', '').start_with?('actions/upload-artifact@') }
raise 'upload missing-file policy' unless upload.fetch('with').fetch('if-no-files-found') == 'error'
raise 'upload may fail silently' if upload['continue-on-error'] || job['continue-on-error']
raise 'installed route absent' unless job.fetch('steps').any? { |s| s.fetch('run','').include?('--binary .adl/bin/adl') }
%w[adl/src/codefriend/ingestion/ci.rs adl/src/cli/codefriend_ci_cmd.rs adl/Cargo.lock .github/workflows/ci.yaml].each do |path|
  value, status = Open3.capture2('python3', root.join('adl/tools/codefriend/ci_select.py').to_s, stdin_data: path+"\n")
  raise 'selected path skipped' unless status.success? && value.strip == 'true'
end
value, status = Open3.capture2('python3', root.join('adl/tools/codefriend/ci_select.py').to_s, stdin_data: "README.md\n")
raise 'unrelated doc selected' unless status.success? && value.strip == 'false'
puts 'PASS: 12 aggregate outcomes, 5 path selections, required installed/upload wiring'
